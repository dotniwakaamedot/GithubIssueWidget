use super::*;

#[derive(Serialize)]
pub struct CreateResult {
    kind: &'static str,
    number: Option<u64>,
    url: Option<String>,
    message: String,
    operation_id: String,
}

#[derive(Serialize, Deserialize)]
pub struct PendingCreation { operation_id: String, #[serde(default = "default_host")] pub(super) host: String, owner: String, repo: String, status: String, number: Option<u64>, phase: String }

pub(super) fn pending_path() -> Result<PathBuf, String> { Ok(settings_path()?.with_file_name("pending-issue.json")) }

pub(super) fn read_pending_at(path: &Path) -> Result<Option<PendingCreation>, String> {
    let data = match fs::read(path) {
        Ok(data) => data,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(None),
        Err(_) => return Err("作成操作の確認記録を読み取れません。".into()),
    };
    let pending: PendingCreation = serde_json::from_slice(&data).map_err(|_| "作成操作の確認記録が無効です。")?;
    if uuid::Uuid::parse_str(&pending.operation_id).is_err() || normalize_host(&pending.host).ok().as_deref() != Some(&pending.host) || !valid_component(&pending.owner) || !valid_component(&pending.repo) || status_label(&pending.status).is_err() || !matches!(pending.phase.as_str(), "pending" | "created_open" | "partial" | "completed") || (pending.phase == "pending" && pending.number.is_some()) || (pending.phase != "pending" && pending.number.filter(|number| *number > 0).is_none()) {
        return Err("作成操作の確認記録が無効です。".into());
    }
    Ok(Some(pending))
}

fn clear_pending_at(path: &Path, expected_id: &str) -> Result<(), String> {
    let pending = read_pending_at(path)?.ok_or("確認対象の作成記録がありません。")?;
    if pending.operation_id != expected_id { return Err("別の作成操作があるため、再度状態を確認してください。".into()); }
    fs::remove_file(path).map_err(|_| "作成操作の確認記録を更新できません。".to_string())
}

#[tauri::command]
pub async fn pending_creation(lock: State<'_, ConnectionLock>) -> Result<Option<PendingCreation>, String> {
    let _guard = lock.0.lock().await;
    let _process_guard = process_lock()?;
    read_pending_at(&pending_path()?)
}

#[tauri::command]
pub async fn acknowledge_creation(operation_id: String, lock: State<'_, ConnectionLock>) -> Result<(), String> {
    let _guard = lock.0.lock().await;
    let _process_guard = process_lock()?;
    clear_pending_at(&pending_path()?, &operation_id)
}

pub(super) enum WriteFailure { Rejected(String), Unknown }

pub(super) fn status_label(status: &str) -> Result<&'static str, String> {
    match status {
        "todo" => Ok("status:todo"),
        "in-progress" => Ok("status:in-progress"),
        "done" => Ok("status:done"),
        _ => Err("ステータスを選択してください。".into()),
    }
}

fn matches_created_issue(value: &serde_json::Value, number: u64, label: &str, state: &str) -> bool {
    value.get("number").and_then(|v| v.as_u64()) == Some(number)
        && value.get("state").and_then(|v| v.as_str()) == Some(state)
        && value.get("labels").and_then(|v| v.as_array()).map(|labels| {
            let mut found = false;
            for item in labels {
                let Some(name) = item.get("name").and_then(|v| v.as_str()) else { return false; };
                if matches!(name, "status:todo" | "status:in-progress" | "status:done") {
                    if name != label || found { return false; }
                    found = true;
                }
            }
            found
        }).unwrap_or(false)
}

pub(super) async fn send_write_locked(builder: reqwest::RequestBuilder, token: &str, cooldown: &RateCooldown) -> Result<reqwest::Response, WriteFailure> {
    check_cooldown(cooldown).map_err(WriteFailure::Rejected)?;
    let response = builder.bearer_auth(token)
        .header("Accept", "application/vnd.github+json")
        .header("X-GitHub-Api-Version", "2022-11-28")
        .send().await.map_err(|_| WriteFailure::Unknown)?;
    let status = response.status();
    match check_api_response(response, cooldown).await {
        Ok(response) => Ok(response),
        Err(_) if status.is_server_error() => Err(WriteFailure::Unknown),
        Err(error) => Err(WriteFailure::Rejected(error)),
    }
}

async fn create_in_repo(client: &reqwest::Client, repo_url: &str, owner: &str, repo: &str, token: &str, title: &str, body: &str, status: &str, cooldown: &RateCooldown, pending_path: &Path) -> Result<CreateResult, String> {
    create_with_writer(client, repo_url, owner, repo, token, title, body, status, cooldown, pending_path, |path, record| write_json_at(path, record)).await
}

async fn create_with_writer<W>(client: &reqwest::Client, repo_url: &str, owner: &str, repo: &str, token: &str, title: &str, body: &str, status: &str, cooldown: &RateCooldown, pending_path: &Path, write_pending: W) -> Result<CreateResult, String>
where W: Fn(&Path, &PendingCreation) -> Result<(), String> {
    let title = title.trim();
    if title.is_empty() { return Err("タイトルを入力してください。".into()); }
    let label = status_label(status)?;
    if read_pending_at(pending_path)?.is_some() { return Err("前の Issue 作成結果を確認してから、次の作成を行ってください。".into()); }
    let mut label_url = reqwest::Url::parse(repo_url).map_err(|_| "接続先 URL が無効です。")?;
    label_url.path_segments_mut().map_err(|_| "接続先 URL が無効です。")?.pop_if_empty().push("labels").push(label);
    if let Err(error) = get_api_response_locked(client, label_url, token, cooldown).await {
        return Err(if error.contains("HTTP 404") { format!("必要なラベル {label} が存在しないか、リポジトリにアクセスできません。GitHub で確認してください。") } else { error });
    }
    let issues_url = format!("{repo_url}/issues");
    let host = web_root_for_api(repo_url)?;
    let mut pending = PendingCreation { operation_id: uuid::Uuid::new_v4().to_string(), host: host.clone(), owner: owner.into(), repo: repo.into(), status: status.into(), number: None, phase: "pending".into() };
    write_pending(pending_path, &pending).map_err(|_| "作成操作の確認記録を保存できません。".to_string())?;
    let payload = serde_json::json!({"title": title, "body": body, "labels": [label]});
    let created = match send_write_locked(client.post(&issues_url).json(&payload), token, cooldown).await {
        Ok(response) => match response.json::<serde_json::Value>().await {
            Ok(issue) if issue.get("number").and_then(|v| v.as_u64()).unwrap_or(0) > 0 => issue,
            _ => return Ok(CreateResult { kind: "uncertain", number: None, url: None, message: "作成結果を確認できません。重複を避けるため、GitHub で確認してから操作してください。".into(), operation_id: pending.operation_id }),
        },
        Err(WriteFailure::Rejected(error)) => { clear_pending_at(pending_path, &pending.operation_id)?; return Err(error); },
        Err(WriteFailure::Unknown) => return Ok(CreateResult { kind: "uncertain", number: None, url: None, message: "通信が途中で終了し、作成結果は不明です。重複を避けるため、GitHub で確認してください。".into(), operation_id: pending.operation_id }),
    };
    let number = created.get("number").and_then(|v| v.as_u64()).expect("number was validated");
    let url = format!("{host}/{owner}/{repo}/issues/{number}");
    pending.number = Some(number);
    pending.phase = "created_open".into();
    if write_pending(pending_path, &pending).is_err() {
        return Ok(CreateResult { kind: "partial", number: Some(number), url: Some(url), message: "Issue は作成されましたが、確認記録を更新できません。GitHub で状態を確認してください。".into(), operation_id: pending.operation_id });
    }
    if status == "done" {
        let response = send_write_locked(client.patch(format!("{issues_url}/{number}")).json(&serde_json::json!({"state":"closed"})), token, cooldown).await;
        let confirmed = match response {
            Ok(response) => matches!(response.json::<serde_json::Value>().await, Ok(value) if matches_created_issue(&value, number, label, "closed")),
            Err(_) => false,
        };
        if !confirmed {
            pending.phase = "partial".into();
            let _ = write_pending(pending_path, &pending);
            return Ok(CreateResult { kind: "partial", number: Some(number), url: Some(url), message: "Issue は作成されましたが、クローズとラベルの結果を確認できません。再取得して状態を確認してください。".into(), operation_id: pending.operation_id });
        }
    } else if !matches_created_issue(&created, number, label, "open") {
        pending.phase = "partial".into();
        let _ = write_pending(pending_path, &pending);
        return Ok(CreateResult { kind: "partial", number: Some(number), url: Some(url), message: "Issue は作成されましたが、ステータスラベルまたは状態を確認できません。GitHub で確認してください。".into(), operation_id: pending.operation_id });
    }
    pending.phase = "completed".into();
    if write_pending(pending_path, &pending).is_err() {
        return Ok(CreateResult { kind: "partial", number: Some(number), url: Some(url), message: "Issue は作成されましたが、確認記録を更新できません。GitHub で状態を確認してください。".into(), operation_id: pending.operation_id });
    }
    Ok(CreateResult { kind: "created", number: Some(number), url: Some(url), message: "Issue を作成しました。".into(), operation_id: pending.operation_id })
}

#[tauri::command]
pub async fn create_issue(expected_owner: String, expected_repo: String, expected_revision: String, title: String, body: String, status: String, lock: State<'_, ConnectionLock>, cooldown: State<'_, RateCooldown>) -> Result<CreateResult, String> {
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
    let endpoint = format!("{}/repos/{}/{}", api_root(&settings.host)?, settings.owner, settings.repo);
    create_in_repo(&client, &endpoint, &settings.owner, &settings.repo, &token, &title, &body, &status, &cooldown, &pending_path()?).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{io::{Read, Write}, net::TcpListener, sync::{Arc, Mutex as StdMutex}, thread};

    fn mock_server(responses: Vec<(u16, &'static str)>) -> (String, Arc<StdMutex<Vec<String>>>, thread::JoinHandle<()>) {
        mock_server_with_timeout(responses, Duration::from_secs(5))
    }

    fn mock_server_with_timeout(responses: Vec<(u16, &'static str)>, accept_timeout: Duration) -> (String, Arc<StdMutex<Vec<String>>>, thread::JoinHandle<()>) {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        listener.set_nonblocking(true).unwrap();
        let address = listener.local_addr().unwrap();
        let methods = Arc::new(StdMutex::new(Vec::new()));
        let seen = Arc::clone(&methods);
        let thread = thread::spawn(move || {
            for (status, body) in responses {
                let deadline = std::time::Instant::now() + accept_timeout;
                let (mut stream, _) = loop {
                    match listener.accept() {
                        Ok(value) => break value,
                        Err(error) if error.kind() == std::io::ErrorKind::WouldBlock && std::time::Instant::now() < deadline => {
                            thread::sleep(Duration::from_millis(10));
                        }
                        Err(_) => panic!("mock server accept failed or timed out"),
                    }
                };
                stream.set_nonblocking(false).unwrap();
                stream.set_read_timeout(Some(Duration::from_secs(5))).unwrap();
                stream.set_write_timeout(Some(Duration::from_secs(5))).unwrap();
                let mut buffer = [0; 8192];
                let count = stream.read(&mut buffer).unwrap();
                let request = String::from_utf8_lossy(&buffer[..count]);
                let method = request.lines().next().unwrap().split_whitespace().next().unwrap().to_owned();
                seen.lock().unwrap().push(method);
                if status == 0 { continue; }
                write!(stream, "HTTP/1.1 {status} Test\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}", body.len(), body).unwrap();
            }
        });
        (format!("http://{address}/repos/a/b"), methods, thread)
    }

    #[test]
    fn mock_server_stops_if_the_expected_request_never_arrives() {
        let (_url, _methods, server) = mock_server_with_timeout(vec![(200, "{}")], Duration::from_millis(50));
        assert!(server.join().is_err());
    }

    fn client() -> reqwest::Client {
        reqwest::Client::builder().no_proxy().timeout(Duration::from_secs(3)).redirect(reqwest::redirect::Policy::none()).retry(reqwest::retry::never()).build().unwrap()
    }

    fn run_create(url: &str, title: &str, body: &str, status: &str) -> Result<CreateResult, String> {
        let dir = tempfile::tempdir().unwrap();
        tauri::async_runtime::block_on(create_in_repo(&client(), url, "a", "b", "synthetic", title, body, status, &RateCooldown::default(), &dir.path().join("pending.json")))
    }

    #[test]
    fn blank_title_is_rejected_without_network() {
        let result = run_create("http://127.0.0.1:9/repos/a/b", "  ", "", "todo");
        assert!(matches!(result, Err(error) if error.contains("タイトル")));
    }

    #[test]
    fn only_supported_statuses_are_accepted() {
        assert_eq!(status_label("todo").unwrap(), "status:todo");
        assert_eq!(status_label("in-progress").unwrap(), "status:in-progress");
        assert_eq!(status_label("done").unwrap(), "status:done");
        assert!(status_label("unknown").is_err());
    }

    #[test]
    fn missing_label_prevents_creation() {
        let (url, methods, server) = mock_server(vec![(404, "{}")]);
        let result = run_create(&url, "title", "", "todo");
        server.join().unwrap();
        assert!(matches!(result, Err(error) if error.contains("ラベル")));
        assert_eq!(*methods.lock().unwrap(), vec!["GET"]);
    }

    #[test]
    fn failed_done_close_reports_created_number_without_second_post() {
        let (url, methods, server) = mock_server(vec![(200, "{}"), (201, "{\"number\":123}"), (403, "{\"message\":\"forbidden\"}")]);
        let result = run_create(&url, "title", "body", "done").unwrap();
        server.join().unwrap();
        assert_eq!(result.kind, "partial");
        assert_eq!(result.number, Some(123));
        assert_eq!(result.url.as_deref(), Some("https://github.com/a/b/issues/123"));
        assert_eq!(*methods.lock().unwrap(), vec!["GET", "POST", "PATCH"]);
    }

    #[test]
    fn done_creation_requires_confirmed_closed_response() {
        let (url, methods, server) = mock_server(vec![(200, "{}"), (201, "{\"number\":123}"), (200, "{\"number\":123,\"state\":\"closed\",\"labels\":[{\"name\":\"status:done\"}]}")]);
        let result = run_create(&url, "title", "", "done").unwrap();
        server.join().unwrap();
        assert_eq!(result.kind, "created");
        assert_eq!(*methods.lock().unwrap(), vec!["GET", "POST", "PATCH"]);
    }

    #[test]
    fn ignored_label_on_created_issue_is_partial_not_success() {
        let (url, methods, server) = mock_server(vec![(200, "{}"), (201, "{\"number\":124,\"state\":\"open\",\"labels\":[]}")]);
        let result = run_create(&url, "title", "", "in-progress").unwrap();
        server.join().unwrap();
        assert_eq!(result.kind, "partial");
        assert_eq!(result.number, Some(124));
        assert_eq!(*methods.lock().unwrap(), vec!["GET", "POST"]);
    }

    #[test]
    fn conflicting_status_labels_after_creation_are_partial() {
        let response = "{\"number\":126,\"state\":\"open\",\"labels\":[{\"name\":\"status:todo\"},{\"name\":\"status:in-progress\"},{\"name\":\"bug\"}]}";
        let (url, methods, server) = mock_server(vec![(200, "{}"), (201, response)]);
        let result = run_create(&url, "title", "", "in-progress").unwrap();
        server.join().unwrap();
        assert_eq!(result.kind, "partial");
        assert_eq!(result.number, Some(126));
        assert_eq!(*methods.lock().unwrap(), vec!["GET", "POST"]);
    }

    #[test]
    fn conflicting_status_labels_after_done_close_are_partial() {
        let response = "{\"number\":127,\"state\":\"closed\",\"labels\":[{\"name\":\"status:todo\"},{\"name\":\"status:done\"}]}";
        let (url, methods, server) = mock_server(vec![(200, "{}"), (201, "{\"number\":127}"), (200, response)]);
        let result = run_create(&url, "title", "", "done").unwrap();
        server.join().unwrap();
        assert_eq!(result.kind, "partial");
        assert_eq!(result.number, Some(127));
        assert_eq!(*methods.lock().unwrap(), vec!["GET", "POST", "PATCH"]);
    }

    #[test]
    fn unrelated_labels_do_not_prevent_creation_success() {
        let response = "{\"number\":128,\"state\":\"open\",\"labels\":[{\"name\":\"status:in-progress\"},{\"name\":\"bug\"}]}";
        let (url, methods, server) = mock_server(vec![(200, "{}"), (201, response)]);
        let result = run_create(&url, "title", "", "in-progress").unwrap();
        server.join().unwrap();
        assert_eq!(result.kind, "created");
        assert_eq!(*methods.lock().unwrap(), vec!["GET", "POST"]);
    }

    #[test]
    fn done_issue_missing_label_after_close_is_partial() {
        let (url, methods, server) = mock_server(vec![(200, "{}"), (201, "{\"number\":125}"), (200, "{\"number\":125,\"state\":\"closed\",\"labels\":[]}")]);
        let result = run_create(&url, "title", "", "done").unwrap();
        server.join().unwrap();
        assert_eq!(result.kind, "partial");
        assert_eq!(result.number, Some(125));
        assert_eq!(*methods.lock().unwrap(), vec!["GET", "POST", "PATCH"]);
    }

    #[test]
    fn lost_create_response_is_uncertain_without_retry() {
        let (url, methods, server) = mock_server(vec![(200, "{}"), (0, "")]);
        let result = run_create(&url, "title", "", "todo").unwrap();
        server.join().unwrap();
        assert_eq!(result.kind, "uncertain");
        assert_eq!(*methods.lock().unwrap(), vec!["GET", "POST"]);
    }

    #[test]
    fn partial_result_is_persisted_and_blocks_duplicate_creation() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("pending.json");
        let (url, methods, server) = mock_server(vec![(200, "{}"), (201, "{\"number\":123}"), (403, "{\"message\":\"forbidden\"}")]);
        let result = tauri::async_runtime::block_on(create_in_repo(&client(), &url, "a", "b", "synthetic-secret", "private-title", "private-body", "done", &RateCooldown::default(), &path)).unwrap();
        server.join().unwrap();
        let raw = fs::read_to_string(&path).unwrap();
        assert!(!raw.contains("synthetic-secret") && !raw.contains("private-title") && !raw.contains("private-body"));
        let pending = read_pending_at(&path).unwrap().unwrap();
        assert_eq!(pending.number, Some(123));
        assert_eq!(pending.phase, "partial");
        let next = tauri::async_runtime::block_on(create_in_repo(&client(), &url, "a", "b", "synthetic-secret", "another", "", "todo", &RateCooldown::default(), &path));
        assert!(matches!(next, Err(error) if error.contains("確認")));
        assert!(clear_pending_at(&path, &uuid::Uuid::new_v4().to_string()).is_err());
        clear_pending_at(&path, &result.operation_id).unwrap();
        assert!(read_pending_at(&path).unwrap().is_none());
        assert_eq!(*methods.lock().unwrap(), vec!["GET", "POST", "PATCH"]);
    }

    #[test]
    fn lost_response_record_survives_restart_until_acknowledged() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("pending.json");
        let (url, methods, server) = mock_server(vec![(200, "{}"), (0, "")]);
        let result = tauri::async_runtime::block_on(create_in_repo(&client(), &url, "a", "b", "synthetic", "title", "", "todo", &RateCooldown::default(), &path)).unwrap();
        server.join().unwrap();
        let after_restart = read_pending_at(&path).unwrap().unwrap();
        assert_eq!(after_restart.phase, "pending");
        assert_eq!(after_restart.operation_id, result.operation_id);
        assert_eq!(*methods.lock().unwrap(), vec!["GET", "POST"]);
    }

    #[test]
    fn successful_creation_remains_recorded_until_acknowledged() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("pending.json");
        let (url, methods, server) = mock_server(vec![(200, "{}"), (201, "{\"number\":42,\"state\":\"open\",\"labels\":[{\"name\":\"status:todo\"}]}")]);
        let result = tauri::async_runtime::block_on(create_in_repo(&client(), &url, "a", "b", "synthetic", "title", "", "todo", &RateCooldown::default(), &path)).unwrap();
        server.join().unwrap();
        let pending = read_pending_at(&path).unwrap().unwrap();
        assert_eq!(result.kind, "created");
        assert_eq!(pending.phase, "completed");
        assert_eq!(pending.number, Some(42));
        clear_pending_at(&path, &result.operation_id).unwrap();
        assert!(read_pending_at(&path).unwrap().is_none());
        assert_eq!(*methods.lock().unwrap(), vec!["GET", "POST"]);
    }

    #[test]
    fn rejected_write_clears_pending_record_without_retry() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("pending.json");
        let (url, methods, server) = mock_server(vec![(200, "{}"), (403, "{\"message\":\"Resource not accessible\"}")]);
        let result = tauri::async_runtime::block_on(create_in_repo(&client(), &url, "a", "b", "synthetic", "title", "", "todo", &RateCooldown::default(), &path));
        server.join().unwrap();
        assert!(matches!(result, Err(error) if error.contains("403")));
        assert!(read_pending_at(&path).unwrap().is_none());
        assert_eq!(*methods.lock().unwrap(), vec!["GET", "POST"]);
    }

    #[test]
    fn failed_record_update_preserves_known_issue_number_and_blocks_retry() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("pending.json");
        let (url, methods, server) = mock_server(vec![(200, "{}"), (201, "{\"number\":123,\"state\":\"open\",\"labels\":[{\"name\":\"status:todo\"}]}")]);
        let writes = std::cell::Cell::new(0);
        let result = tauri::async_runtime::block_on(create_with_writer(&client(), &url, "a", "b", "synthetic", "title", "", "todo", &RateCooldown::default(), &path, |target, record| {
            writes.set(writes.get() + 1);
            if writes.get() == 2 { Err("simulated record failure".into()) } else { write_json_at(target, record) }
        })).unwrap();
        server.join().unwrap();
        assert_eq!(result.kind, "partial");
        assert_eq!(result.number, Some(123));
        let pending = read_pending_at(&path).unwrap().unwrap();
        assert_eq!(pending.phase, "pending");
        assert_eq!(pending.number, None);
        let next = tauri::async_runtime::block_on(create_in_repo(&client(), &url, "a", "b", "synthetic", "title", "", "todo", &RateCooldown::default(), &path));
        assert!(next.is_err());
        assert_eq!(*methods.lock().unwrap(), vec!["GET", "POST"]);
    }

    #[test]
    fn submitted_payload_contains_the_selected_label() {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let address = listener.local_addr().unwrap();
        let server = thread::spawn(move || {
            for index in 0..2 {
                let (mut stream, _) = listener.accept().unwrap();
                stream.set_read_timeout(Some(Duration::from_secs(3))).unwrap();
                let mut request = Vec::new();
                let mut buffer = [0; 4096];
                loop {
                    let count = stream.read(&mut buffer).unwrap();
                    assert!(count > 0);
                    request.extend_from_slice(&buffer[..count]);
                    if let Some(end) = request.windows(4).position(|bytes| bytes == b"\r\n\r\n") {
                        let headers = String::from_utf8_lossy(&request[..end]);
                        let length = headers.lines().find_map(|line| line.split_once(':').and_then(|(name, value)| if name.eq_ignore_ascii_case("content-length") { value.trim().parse::<usize>().ok() } else { None })).unwrap_or(0);
                        if request.len() >= end + 4 + length {
                            if index == 0 { assert!(headers.starts_with("GET /repos/a/b/labels/")); }
                            else {
                                assert!(headers.starts_with("POST /repos/a/b/issues "));
                                let body: serde_json::Value = serde_json::from_slice(&request[end + 4..end + 4 + length]).unwrap();
                                assert_eq!(body, serde_json::json!({"title":"Example","body":"Text","labels":["status:in-progress"]}));
                            }
                            break;
                        }
                    }
                }
                let body = if index == 0 { "{}" } else { "{\"number\":7,\"state\":\"open\",\"labels\":[{\"name\":\"status:in-progress\"}]}" };
                write!(stream, "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}", body.len(), body).unwrap();
            }
        });
        let result = run_create(&format!("http://{address}/repos/a/b"), "Example", "Text", "in-progress").unwrap();
        server.join().unwrap();
        assert_eq!(result.kind, "created");
    }
}
