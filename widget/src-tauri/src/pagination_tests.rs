use super::*;
use std::{collections::BTreeMap, io::{Read, Write}, net::TcpListener, thread, time::{Duration, Instant}};

// The listener and stream reads have deadlines; the caller also uses a client timeout.
// Each fixture contains at most 100 GitHub API elements per page.
fn serve(pages: Vec<Option<(u16, String)>>) -> (String, thread::JoinHandle<Result<(), String>>) {
    let listener = TcpListener::bind("127.0.0.1:0").expect("bind synthetic server");
    listener.set_nonblocking(true).expect("nonblocking listener");
    let endpoint = format!("http://{}/issues", listener.local_addr().unwrap());
    let base = endpoint.clone();
    let handle = thread::spawn(move || -> Result<(), String> {
        let deadline = Instant::now() + Duration::from_secs(15);
        let total = pages.len();
        for (index, page) in pages.into_iter().enumerate() {
            let (mut stream, _) = loop {
                match listener.accept() {
                    Ok(value) => break value,
                    Err(error) if error.kind() == std::io::ErrorKind::WouldBlock => {
                        if Instant::now() >= deadline { return Err("accept deadline exceeded".into()); }
                        thread::sleep(Duration::from_millis(10));
                    }
                    Err(error) => return Err(format!("accept: {error}")),
                }
            };
            stream.set_nonblocking(false).map_err(|e| e.to_string())?;
            stream.set_read_timeout(Some(Duration::from_secs(2))).map_err(|e| e.to_string())?;
            stream.set_write_timeout(Some(Duration::from_secs(2))).map_err(|e| e.to_string())?;
            let mut request = Vec::new();
            let mut buf = [0u8; 1024];
            while !request.windows(2).any(|w| w == b"\r\n") {
                let count = stream.read(&mut buf).map_err(|e| format!("read: {e}"))?;
                if count == 0 || request.len() + count > 8192 { return Err("invalid request".into()); }
                request.extend_from_slice(&buf[..count]);
            }
            let request = String::from_utf8(request).map_err(|e| e.to_string())?;
            if !request.lines().next().unwrap_or_default().contains(&format!("&page={} ", index + 1)) {
                return Err("unexpected request page".into());
            }
            let Some((status, body)) = page else { continue }; // Drop the socket without a response.
            let link = if index + 1 < total {
                format!("Link: <{base}?state=all&per_page=100&page={}>; rel=\"next\"\r\n", index + 2)
            } else { String::new() };
            write!(stream, "HTTP/1.1 {status} Test\r\n{link}Content-Length: {}\r\nConnection: close\r\n\r\n{body}", body.len()).map_err(|e| format!("write: {e}"))?;
        }
        Ok(())
    });
    (endpoint, handle)
}

fn fixture_issue(number: u64, state: &str, labels: &[&str], pr: bool) -> serde_json::Value {
    let mut value = serde_json::json!({
        "number": number, "title": format!("Issue {number}"),
        "html_url": format!("https://github.com/a/b/issues/{number}"),
        "state": state, "updated_at": "2026-01-01T00:00:00Z",
        "labels": labels.iter().map(|name| serde_json::json!({"name": name})).collect::<Vec<_>>()
    });
    if pr { value["pull_request"] = serde_json::json!({}); }
    value
}

fn client() -> reqwest::Client {
    reqwest::Client::builder().no_proxy().timeout(Duration::from_secs(3)).build().unwrap()
}

#[test]
fn mixed_pages_preserve_exact_issue_set_and_classification() {
    let mut pages: Vec<Vec<serde_json::Value>> = Vec::new();
    for page in 0..11 {
        let mut items = Vec::new();
        for number in (page * 100 + 1)..=(page * 100 + 100) {
            let issue = match number {
                10 => fixture_issue(number, "open", &[], true),
                20 => fixture_issue(number, "closed", &["status:todo"], false),
                30 => fixture_issue(number, "open", &["status:done", "status:in-progress", "status:todo"], false),
                40 => fixture_issue(number, "open", &["status:in-progress"], false),
                50 => fixture_issue(number, "open", &["status:done"], false),
                _ => fixture_issue(number, "open", &[], false),
            };
            items.push(issue);
        }
        pages.push(items);
    }
    pages.push(vec![fixture_issue(1, "open", &[], false), fixture_issue(2000, "open", &[], true),
        fixture_issue(1101, "open", &[], false), fixture_issue(1102, "open", &[], false),
        fixture_issue(1103, "open", &[], false), fixture_issue(1104, "open", &[], false)]);
    assert!(pages.iter().all(|page| page.len() <= 100));
    let responses = pages.into_iter().map(|page| Some((200, serde_json::to_string(&page).unwrap()))).collect();
    let (url, server) = serve(responses);
    let result = tauri::async_runtime::block_on(fetch_pages(&client(), &url, 42, "synthetic", &RateCooldown::default()));
    let server_result = server.join().expect("synthetic server panicked");
    assert!(server_result.is_ok(), "synthetic server failed: {server_result:?}");
    let issues = result.expect("all synthetic pages should succeed");
    assert_eq!(issues.len(), 1103);
    let actual: BTreeMap<_, _> = issues.iter().map(|issue| (issue.number, (issue.state.as_str(), issue.status.as_str()))).collect();
    assert_eq!(actual.len(), issues.len(), "duplicate issue numbers");
    let mut expected: BTreeMap<u64, (&str, &str)> = (1..=1104).filter(|number| *number != 10).map(|number| (number, ("open", "todo"))).collect();
    expected.insert(20, ("closed", "done"));
    expected.insert(40, ("open", "in-progress"));
    expected.insert(50, ("open", "done"));
    assert_eq!(actual, expected);
    assert!(!actual.contains_key(&2000));
}

#[test]
fn later_failures_never_return_the_first_page_as_success() {
    let first = serde_json::to_string(&(1..=100).map(|number| fixture_issue(number, "open", &[], false)).collect::<Vec<_>>()).unwrap();
    let failures = [Some((500, "{}".to_owned())), None, Some((200, "not-json".to_owned()))];
    for failure in failures {
        let (url, server) = serve(vec![Some((200, first.clone())), failure]);
        let result = tauri::async_runtime::block_on(fetch_pages(&client(), &url, 42, "synthetic", &RateCooldown::default()));
        let server_result = server.join().expect("synthetic server panicked");
        assert!(server_result.is_ok(), "synthetic server failed: {server_result:?}");
        assert!(result.is_err(), "a later failure must not return partial success");
    }
}
