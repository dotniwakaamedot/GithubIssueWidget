use super::*;
use crate::creation::{send_write_locked, status_label, WriteFailure};

pub(super) async fn move_in_repo(client: &reqwest::Client, repo_url: &str, token: &str, number: u64, expected_updated_at: &str, status: &str, cooldown: &RateCooldown) -> Result<(), String> {
    if number == 0 || expected_updated_at.is_empty() { return Err("Issue の更新情報が無効です。再読込してください。".into()); }
    let target_label = status_label(status)?;
    let mut label_url = reqwest::Url::parse(repo_url).map_err(|_| "接続先 URL が無効です。")?;
    label_url.path_segments_mut().map_err(|_| "接続先 URL が無効です。")?.pop_if_empty().push("labels").push(target_label);
    if let Err(error) = get_api_response_locked(client, label_url, token, cooldown).await {
        return Err(if error.contains("HTTP 404") { format!("必要なラベル {target_label} が存在しないか、リポジトリにアクセスできません。GitHub で確認してください。") } else { error });
    }
    let issue_url = reqwest::Url::parse(&format!("{repo_url}/issues/{number}")).map_err(|_| "Issue URL が無効です。")?;
    let latest: ApiIssue = get_api_response_locked(client, issue_url.clone(), token, cooldown).await?.json().await.map_err(|_| "Issue の状態を読み取れませんでした。")?;
    if latest.number != number || latest.pull_request.is_some() { return Err("更新対象の Issue が一致しません。再読込してください。".into()); }
    if latest.updated_at != expected_updated_at { return Err("Issue が他の場所で更新されました。再読込してから操作してください。".into()); }
    let target_state = if status == "done" { "closed" } else { "open" };
    let mut labels_url = issue_url.clone();
    labels_url.path_segments_mut().map_err(|_| "Issue URL が無効です。")?.push("labels");
    let mut writes_started = false;
    if !latest.labels.iter().any(|label| label.name == target_label) {
        let response = write_step(client.post(labels_url.clone()).json(&serde_json::json!({"labels": [target_label]})), token, cooldown, writes_started).await?;
        let labels: Vec<ApiLabel> = response.json().await.map_err(|_| "追加結果を確認できません。一部が反映された可能性があります。最新状態を再取得してください。")?;
        if !labels.iter().any(|label| label.name == target_label) {
            return Err("移動先ラベルの追加を確認できません。一部が反映された可能性があります。最新状態を再取得してください。".into());
        }
        writes_started = true;
    }
    for old in ["status:todo", "status:in-progress", "status:done"] {
        if old == target_label || !latest.labels.iter().any(|label| label.name == old) { continue; }
        let mut old_url = labels_url.clone();
        old_url.path_segments_mut().map_err(|_| "Issue URL が無効です。")?.push(old);
        let response = write_step(client.delete(old_url), token, cooldown, writes_started).await?;
        let labels: Vec<ApiLabel> = response.json().await.map_err(|_| "削除結果を確認できません。一部が反映された可能性があります。最新状態を再取得してください。")?;
        if !labels.iter().any(|label| label.name == target_label) || labels.iter().any(|label| label.name == old) {
            return Err("削除後のラベルを確認できません。一部が反映された可能性があります。最新状態を再取得してください。".into());
        }
        writes_started = true;
    }
    if latest.state != target_state {
        write_step(client.patch(issue_url.clone()).json(&serde_json::json!({"state": target_state})), token, cooldown, writes_started).await?;
    }
    let updated: ApiIssue = get_api_response_locked(client, issue_url, token, cooldown).await
        .map_err(|_| "更新後の状態を確認できません。一部が反映された可能性があります。最新状態を再取得してください。")?
        .json().await.map_err(|_| "更新後の状態を確認できません。一部が反映された可能性があります。最新状態を再取得してください。")?;
    let statuses: Vec<_> = updated.labels.iter().map(|label| label.name.as_str())
        .filter(|name| matches!(*name, "status:todo" | "status:in-progress" | "status:done")).collect();
    if updated.number != number || updated.pull_request.is_some() || updated.state != target_state || statuses != [target_label] {
        return Err("更新後の状態またはラベルが一致しません。最新状態を確認してください。".into());
    }
    Ok(())
}

async fn write_step(builder: reqwest::RequestBuilder, token: &str, cooldown: &RateCooldown, previous_write: bool) -> Result<reqwest::Response, String> {
    match send_write_locked(builder, token, cooldown).await {
        Ok(response) => Ok(response),
        Err(WriteFailure::Rejected(error)) if !previous_write => Err(format!("更新できませんでした。最新状態を再取得してください。{error}")),
        Err(WriteFailure::Rejected(error)) => Err(format!("更新の一部が反映された可能性があります。最新状態を再取得してください。{error}")),
        Err(WriteFailure::Unknown) => Err("更新結果が不明です。一部が反映された可能性があります。最新状態を再取得してください。".into()),
    }
}

#[tauri::command]
pub async fn move_issue(expected_owner: String, expected_repo: String, expected_revision: String, number: u64, expected_updated_at: String, status: String, lock: State<'_, ConnectionLock>, cooldown: State<'_, RateCooldown>) -> Result<(), String> {
    let guard = lock.0.lock().await;
    let _process_guard = process_lock()?;
    recover_pending()?;
    let settings = read_settings()?.ok_or("接続設定を保存してください。")?;
    if !matches_connection(&settings, &expected_owner, &expected_repo) { return Err("別のウィンドウで接続先が変更されました。設定を開いて確認してください。".into()); }
    if !valid_component(&settings.owner) || !valid_component(&settings.repo) { return Err("保存された接続先が無効です。".into()); }
    let token = read_token(&settings.credential_id)?;
    validate_connection_revision(guard.as_ref(), &expected_revision, &Some(settings.clone()), token.as_deref())?;
    let token = token.ok_or("トークンを設定してください。")?;
    let client = reqwest::Client::builder().timeout(Duration::from_secs(20)).redirect(reqwest::redirect::Policy::none()).retry(reqwest::retry::never()).user_agent("github-task-widget").build().map_err(|_| "通信の準備に失敗しました。")?;
    let endpoint = format!("https://api.github.com/repos/{}/{}", settings.owner, settings.repo);
    move_in_repo(&client, &endpoint, &token, number, &expected_updated_at, &status, &cooldown).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{io::{self, Read, Write}, net::{TcpListener, TcpStream}, sync::{Arc, Mutex}, thread, time::Instant};

    fn issue(state: &str, labels: &[&str], updated: &str) -> String {
        serde_json::json!({"number": 42, "title": "test", "html_url": "https://github.com/a/b/issues/42", "state": state, "updated_at": updated, "labels": labels.iter().map(|name| serde_json::json!({"name": name})).collect::<Vec<_>>()}).to_string()
    }

    fn accept_with_timeout(listener: &TcpListener, timeout: Duration) -> io::Result<TcpStream> {
        let deadline = Instant::now() + timeout;
        loop {
            match listener.accept() {
                Ok((stream, _)) => {
                    stream.set_nonblocking(false)?;
                    stream.set_write_timeout(Some(Duration::from_secs(3)))?;
                    return Ok(stream);
                }
                Err(error) if error.kind() == io::ErrorKind::WouldBlock && Instant::now() < deadline => thread::sleep(Duration::from_millis(10)),
                Err(error) if error.kind() == io::ErrorKind::WouldBlock => return Err(io::Error::new(io::ErrorKind::TimedOut, "mock accept timed out")),
                Err(error) => return Err(error),
            }
        }
    }

    fn read_request(stream: &mut TcpStream, timeout: Duration) -> io::Result<String> {
        let deadline = Instant::now() + timeout;
        let mut bytes = Vec::new();
        let mut buffer = [0; 4096];
        loop {
            let remaining = deadline.saturating_duration_since(Instant::now());
            if remaining.is_zero() { return Err(io::Error::new(io::ErrorKind::TimedOut, "mock request timed out")); }
            stream.set_read_timeout(Some(remaining))?;
            let count = stream.read(&mut buffer)?;
            if count == 0 { return Err(io::Error::new(io::ErrorKind::UnexpectedEof, "mock request ended early")); }
            bytes.extend_from_slice(&buffer[..count]);
            if bytes.len() > 65536 { return Err(io::Error::new(io::ErrorKind::InvalidData, "mock request too large")); }
            let text = String::from_utf8_lossy(&bytes);
            if let Some(end) = text.find("\r\n\r\n") {
                let length = text[..end].lines().find_map(|line| line.to_ascii_lowercase().strip_prefix("content-length:").and_then(|n| n.trim().parse::<usize>().ok())).unwrap_or(0);
                if bytes.len() >= end + 4 + length {
                    return String::from_utf8(bytes).map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "mock request is not UTF-8"));
                }
            }
        }
    }

    fn server(responses: Vec<(u16, String)>) -> (String, Arc<Mutex<Vec<String>>>, thread::JoinHandle<()>) {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        listener.set_nonblocking(true).unwrap();
        let address = listener.local_addr().unwrap();
        let requests = Arc::new(Mutex::new(Vec::new()));
        let seen = requests.clone();
        let handle = thread::spawn(move || {
            for (status, body) in responses {
                let mut stream = accept_with_timeout(&listener, Duration::from_secs(5)).expect("mock accept failed");
                let request = read_request(&mut stream, Duration::from_secs(3)).expect("mock request failed");
                seen.lock().unwrap().push(request);
                if status != 0 { write!(stream, "HTTP/1.1 {status} Test\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}", body.len(), body).unwrap(); }
            }
        });
        (format!("http://{address}/repos/a/b"), requests, handle)
    }

    fn run(url: &str, status: &str) -> Result<(), String> {
        let client = reqwest::Client::builder().no_proxy().timeout(Duration::from_secs(3)).retry(reqwest::retry::never()).build().unwrap();
        tauri::async_runtime::block_on(move_in_repo(&client, url, "synthetic", 42, "v1", status, &RateCooldown::default()))
    }

    #[test]
    fn mock_accept_stops_when_no_client_connects() {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        listener.set_nonblocking(true).unwrap();
        assert_eq!(accept_with_timeout(&listener, Duration::from_millis(50)).unwrap_err().kind(), io::ErrorKind::TimedOut);
    }

    #[test]
    fn mock_request_stops_on_early_eof() {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        listener.set_nonblocking(true).unwrap();
        let client = TcpStream::connect(listener.local_addr().unwrap()).unwrap();
        client.shutdown(std::net::Shutdown::Write).unwrap();
        let mut server = accept_with_timeout(&listener, Duration::from_secs(1)).unwrap();
        assert_eq!(read_request(&mut server, Duration::from_secs(1)).unwrap_err().kind(), io::ErrorKind::UnexpectedEof);
    }

    #[test]
    fn mock_request_stops_when_client_sends_nothing() {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        listener.set_nonblocking(true).unwrap();
        let _client = TcpStream::connect(listener.local_addr().unwrap()).unwrap();
        let mut server = accept_with_timeout(&listener, Duration::from_secs(1)).unwrap();
        let error = read_request(&mut server, Duration::from_millis(50)).unwrap_err();
        assert!(matches!(error.kind(), io::ErrorKind::TimedOut | io::ErrorKind::WouldBlock));
    }

    fn transition_server(source_state: &str, source_label: &str) -> (String, thread::JoinHandle<(Vec<String>, String, Vec<String>)>) {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        listener.set_nonblocking(true).unwrap();
        let url = format!("http://{}/repos/a/b", listener.local_addr().unwrap());
        let mut state = source_state.to_string();
        let mut labels = vec!["bug".to_string(), source_label.to_string()];
        let handle = thread::spawn(move || {
            let mut requests = Vec::new();
            let mut issue_reads = 0;
            loop {
                let mut stream = accept_with_timeout(&listener, Duration::from_secs(5)).expect("mock accept failed");
                let request = read_request(&mut stream, Duration::from_secs(3)).expect("mock request failed");
                let first = request.lines().next().unwrap().to_string();
                requests.push(request.clone());
                let body = if first.starts_with("GET /repos/a/b/labels/") {
                    "{}".to_string()
                } else if first.starts_with("GET /repos/a/b/issues/42 ") {
                    issue_reads += 1;
                    issue(&state, &labels.iter().map(String::as_str).collect::<Vec<_>>(), if issue_reads == 1 { "v1" } else { "v2" })
                } else if first.starts_with("POST /repos/a/b/issues/42/labels ") {
                    let payload: serde_json::Value = serde_json::from_str(request.split("\r\n\r\n").nth(1).unwrap()).unwrap();
                    let label = payload["labels"][0].as_str().unwrap().to_string();
                    if !labels.contains(&label) { labels.push(label); }
                    serde_json::json!(labels.iter().map(|name| serde_json::json!({"name": name})).collect::<Vec<_>>()).to_string()
                } else if first.starts_with("DELETE /repos/a/b/issues/42/labels/") {
                    let path = first.split_whitespace().nth(1).unwrap().replace("%3A", ":").replace("%3a", ":");
                    let name = path.rsplit('/').next().unwrap();
                    labels.retain(|label| label != name);
                    serde_json::json!(labels.iter().map(|name| serde_json::json!({"name": name})).collect::<Vec<_>>()).to_string()
                } else if first.starts_with("PATCH /repos/a/b/issues/42 ") {
                    let payload: serde_json::Value = serde_json::from_str(request.split("\r\n\r\n").nth(1).unwrap()).unwrap();
                    assert!(payload.get("labels").is_none());
                    state = payload["state"].as_str().unwrap().to_string();
                    "{}".to_string()
                } else { panic!("unexpected request: {first}"); };
                write!(stream, "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}", body.len(), body).unwrap();
                if issue_reads == 1 && first.starts_with("GET /repos/a/b/issues/42 ") {
                    labels.push("external".to_string());
                }
                if issue_reads == 2 { break; }
            }
            (labels, state, requests)
        });
        (url, handle)
    }

    #[test]
    fn all_six_transitions_preserve_labels_added_after_the_initial_get() {
        let statuses = [("todo", "status:todo", "open"), ("in-progress", "status:in-progress", "open"), ("done", "status:done", "closed")];
        for (source, source_label, source_state) in statuses {
            for (status, target_label, target_state) in statuses {
                if source == status { continue; }
                let (url, handle) = transition_server(source_state, source_label);
                assert!(run(&url, status).is_ok());
                let (labels, state, requests) = handle.join().unwrap();
                assert_eq!(state, target_state);
                assert!(labels.iter().any(|x| x == "bug"));
                assert!(labels.iter().any(|x| x == "external"));
                assert!(labels.iter().any(|x| x == target_label));
                assert!(!labels.iter().any(|x| x == source_label));
                assert!(requests.iter().all(|r| !r.starts_with("PUT ")));
            }
        }
    }

    #[test]
    fn a_rejected_step_after_a_success_reports_partial_application() {
        let (url, requests, handle) = server(vec![(200, "{}".into()), (200, issue("open", &["bug", "status:todo"], "v1")), (200, "[{\"name\":\"status:done\"}]".into()), (403, "{}".into())]);
        let error = run(&url, "done").unwrap_err();
        handle.join().unwrap();
        assert!(error.contains("一部"));
        assert_eq!(requests.lock().unwrap().len(), 4);
    }

    #[test]
    fn unconfirmed_label_addition_stops_before_removal_or_state_change() {
        for response in ["[]", "not-json"] {
            let (url, requests, handle) = server(vec![
                (200, "{}".into()),
                (200, issue("open", &["bug", "status:todo"], "v1")),
                (200, response.into()),
            ]);
            let error = run(&url, "done").unwrap_err();
            handle.join().unwrap();
            assert!(error.contains("確認できません") && error.contains("一部"));
            let requests = requests.lock().unwrap();
            assert_eq!(requests.len(), 3);
            assert!(requests[2].starts_with("POST /repos/a/b/issues/42/labels "));
        }
    }

    #[test]
    fn unconfirmed_label_removal_stops_before_state_change() {
        for response in ["[]", "[{\"name\":\"status:done\"},{\"name\":\"status:todo\"}]", "not-json"] {
            let (url, requests, handle) = server(vec![
                (200, "{}".into()),
                (200, issue("open", &["status:todo"], "v1")),
                (200, "[{\"name\":\"status:done\"}]".into()),
                (200, response.into()),
            ]);
            let error = run(&url, "done").unwrap_err();
            handle.join().unwrap();
            assert!(error.contains("確認できません") && error.contains("一部"));
            let requests = requests.lock().unwrap();
            assert_eq!(requests.len(), 4);
            assert!(requests[3].starts_with("DELETE /repos/a/b/issues/42/labels/"));
        }
    }

    #[test]
    fn multiple_status_labels_remove_only_obsolete_statuses() {
        let (url, requests, handle) = server(vec![
            (200, "{}".into()),
            (200, issue("closed", &["bug", "status:todo", "status:in-progress", "status:done"], "v1")),
            (200, "[{\"name\":\"status:done\"},{\"name\":\"status:in-progress\"}]".into()), (200, "[{\"name\":\"status:done\"}]".into()),
            (200, issue("closed", &["bug", "external", "status:done"], "v2")),
        ]);
        assert!(run(&url, "done").is_ok());
        handle.join().unwrap();
        let requests = requests.lock().unwrap();
        assert_eq!(requests.len(), 5);
        assert!(requests[2].starts_with("DELETE "));
        assert!(requests[3].starts_with("DELETE "));
        assert!(!requests[2].contains("/bug"));
        assert!(!requests[3].contains("/bug"));
    }

    #[test]
    fn final_read_failure_is_not_reported_as_success() {
        let (url, _, handle) = server(vec![
            (200, "{}".into()), (200, issue("open", &["status:todo"], "v1")),
            (200, "[{\"name\":\"status:done\"}]".into()), (200, "[{\"name\":\"status:done\"}]".into()), (200, "{}".into()), (500, "{}".into()),
        ]);
        let error = run(&url, "done").unwrap_err();
        handle.join().unwrap();
        assert!(error.contains("一部"));
    }

    #[test]
    fn final_status_conflict_is_not_reported_as_success() {
        let (url, requests, handle) = server(vec![(200, "{}".into()), (200, issue("open", &["bug"], "v1")), (200, "[{\"name\":\"status:done\"}]".into()), (200, "{}".into()), (200, issue("closed", &["bug", "status:done", "status:todo"], "v2"))]);
        assert!(run(&url, "done").is_err());
        handle.join().unwrap();
        assert_eq!(requests.lock().unwrap().len(), 5);
    }

    #[test]
    fn changed_issue_and_missing_label_never_send_patch() {
        let (url, requests, handle) = server(vec![(200, "{}".into()), (200, issue("open", &["bug"], "v2"))]);
        assert!(run(&url, "done").is_err());
        handle.join().unwrap();
        assert_eq!(requests.lock().unwrap().len(), 2);
        let (url, requests, handle) = server(vec![(404, "{}".into())]);
        assert!(run(&url, "done").is_err());
        handle.join().unwrap();
        assert_eq!(requests.lock().unwrap().len(), 1);
    }

    #[test]
    fn lost_label_response_is_not_reported_as_success() {
        let (url, requests, handle) = server(vec![(200, "{}".into()), (200, issue("open", &["bug"], "v1")), (0, String::new())]);
        assert!(run(&url, "done").is_err());
        handle.join().unwrap();
        assert_eq!(requests.lock().unwrap().len(), 3);
    }

    #[test]
    fn lost_state_patch_response_reports_unknown_partial_result() {
        let (url, requests, handle) = server(vec![
            (200, "{}".into()), (200, issue("open", &["status:todo"], "v1")),
            (200, "[{\"name\":\"status:done\"}]".into()), (200, "[{\"name\":\"status:done\"}]".into()), (0, String::new()),
        ]);
        let error = run(&url, "done").unwrap_err();
        handle.join().unwrap();
        assert!(error.contains("不明") && error.contains("一部"));
        assert!(requests.lock().unwrap()[4].starts_with("PATCH "));
    }
}
