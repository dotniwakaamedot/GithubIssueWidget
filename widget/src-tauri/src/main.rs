#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use fs2::FileExt;
use std::{fs, io::Write, path::{Path, PathBuf}, sync::atomic::{AtomicU64, Ordering}, time::{Duration, SystemTime, UNIX_EPOCH}};
use tauri::State;
use tokio::sync::Mutex;
mod creation;
use creation::{acknowledge_creation, create_issue, pending_creation};
mod movement;
use movement::move_issue;

const SERVICE: &str = "github-task-widget";

#[derive(Default)]
struct RateCooldown(AtomicU64, bool);

fn now_seconds() -> u64 { SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_secs() }

fn rate_deadline(now: u64, retry: Option<u64>, reset: Option<u64>, primary: bool) -> u64 {
    now.saturating_add(retry.unwrap_or(60)).max(if primary { reset.unwrap_or(now.saturating_add(60)) } else { 0 })
}

fn check_cooldown(cooldown: &RateCooldown) -> Result<(), String> {
    let mut deadline = cooldown.0.load(Ordering::Relaxed);
    if cooldown.1 {
        let path = settings_path()?.with_file_name("rate-limit.json");
        match fs::read(path) {
            Ok(data) => { let stored: u64 = serde_json::from_slice(&data).map_err(|_| "レート制限の記録を読み取れません。")?; deadline = deadline.max(stored); },
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => (),
            Err(_) => return Err("レート制限の記録を読み取れません。".into()),
        }
    }
    if deadline > now_seconds() { Err("GitHub API のレート制限中です。指定された待機時間の後に再試行します。".into()) } else { Ok(()) }
}

fn record_cooldown(cooldown: &RateCooldown, deadline: u64) -> Result<(), String> {
    let previous = cooldown.0.fetch_max(deadline, Ordering::Relaxed);
    if cooldown.1 { write_json_at(&settings_path()?.with_file_name("rate-limit.json"), &deadline.max(previous))?; }
    Ok(())
}

#[derive(Clone, Deserialize, Serialize, PartialEq, Eq)]
struct Settings { owner: String, repo: String, credential_id: String }

fn matches_connection(settings: &Settings, owner: &str, repo: &str) -> bool {
    settings.owner == owner && settings.repo == repo
}

fn matches_connection_snapshot(expected: &Settings, actual: &Settings, token: &str, actual_token: Option<&str>) -> bool {
    matches_connection(actual, &expected.owner, &expected.repo)
        && actual.credential_id == expected.credential_id
        && actual_token == Some(token)
}

#[derive(Deserialize, Serialize)]
struct Recovery { old_id: String, new_id: String }

#[derive(Serialize)]
struct ConnectionInfo { owner: String, repo: String, has_token: bool, revision: String }

#[derive(Deserialize)]
struct ApiLabel { name: String }

#[derive(Deserialize)]
struct ApiRepository { id: u64, full_name: String }

#[derive(Deserialize)]
struct ApiIssue {
    number: u64,
    title: String,
    html_url: String,
    state: String,
    updated_at: String,
    labels: Vec<ApiLabel>,
    #[serde(default)]
    pull_request: Option<serde_json::Value>,
}

#[derive(Serialize)]
struct BoardIssue { number: u64, title: String, url: String, status: String, state: String, updated_at: String }

fn validate_board_snapshot(issues: Vec<BoardIssue>, expected: &Settings, current: &Settings, token: &str, current_token: Option<&str>) -> Result<Vec<BoardIssue>, String> {
    if !matches_connection_snapshot(expected, current, token, current_token) {
        return Err("接続設定が変更されました。設定を確認して再読み込みしてください。".into());
    }
    Ok(issues)
}

fn classify_issue(issue: &ApiIssue) -> &'static str {
    if issue.state == "closed" { return "done"; }
    for (name, status) in [("status:todo", "todo"), ("status:in-progress", "in-progress"), ("status:done", "done")] {
        if issue.labels.iter().any(|label| label.name == name) { return status; }
    }
    "todo"
}

fn link_has_next(params: &str) -> Result<bool, String> {
    let (mut start, mut quoted, mut escaped, mut found) = (0, false, false, false);
    let mut parts = Vec::new();
    for (index, character) in params.char_indices() {
        if escaped { escaped = false; continue; }
        if quoted && character == '\\' { escaped = true; continue; }
        match character {
            '"' => quoted = !quoted,
            ';' if !quoted => { parts.push(&params[start..index]); start = index + 1; },
            _ => (),
        }
    }
    if quoted || escaped { return Err("ページ情報が無効です。".into()); }
    parts.push(&params[start..]);
    for param in parts {
        if let Some(value) = param.trim().strip_prefix("rel=") {
            let value = value.trim().trim_matches('"');
            if value.split_ascii_whitespace().any(|relation| relation == "next") { found = true; }
        }
    }
    Ok(found)
}

fn next_page(headers: &reqwest::header::HeaderMap, endpoint: &str, repository_id: u64) -> Result<Option<reqwest::Url>, String> {
    let base = reqwest::Url::parse(endpoint).map_err(|_| "Issue 取得先が無効です。")?;
    let mut next = None;
    for header in headers.get_all(reqwest::header::LINK).iter() {
        let text = header.to_str().map_err(|_| "ページ情報を読み取れませんでした。")?;
        let mut parts = Vec::new();
        let (mut start, mut angled, mut quoted, mut escaped) = (0, false, false, false);
        for (index, character) in text.char_indices() {
            if escaped { escaped = false; continue; }
            if quoted && character == '\\' { escaped = true; continue; }
            match character {
                '"' if !angled => quoted = !quoted,
                '<' if !quoted => { if angled { return Err("ページ情報が無効です。".into()); } angled = true; },
                '>' if !quoted => { if !angled { return Err("ページ情報が無効です。".into()); } angled = false; },
                ',' if !quoted && !angled => { parts.push(&text[start..index]); start = index + 1; },
                _ => (),
            }
        }
        if angled || quoted || escaped { return Err("ページ情報が無効です。".into()); }
        parts.push(&text[start..]);
        for part in parts {
            let (address, params) = part.trim().split_once('>').ok_or("ページ情報が無効です。")?;
            let is_next = link_has_next(params)?;
            if !is_next { continue; }
            if next.is_some() { return Err("ページ情報が重複しています。".into()); }
            let url = reqwest::Url::parse(address.strip_prefix('<').ok_or("ページ情報が無効です。")?).map_err(|_| "ページ URL が無効です。")?;
            let alternate = format!("/repositories/{repository_id}/issues");
            if url.scheme() != base.scheme() || url.host_str() != base.host_str() || url.port_or_known_default() != base.port_or_known_default() || (url.path() != base.path() && url.path() != alternate) || !url.username().is_empty() || url.password().is_some() || url.fragment().is_some() { return Err("ページ URL が許可されていません。".into()); }
            let pairs: Vec<_> = url.query_pairs().collect();
            let size = pairs.iter().filter(|(key,_)| key == "per_page").collect::<Vec<_>>();
            let state = pairs.iter().filter(|(key,_)| key == "state").collect::<Vec<_>>();
            let mut names = std::collections::HashSet::new();
            if url.as_str().len() > 8192 || pairs.iter().any(|(key,value)| !matches!(key.as_ref(), "state" | "per_page" | "page" | "after" | "before" | "since") || !names.insert(key.to_string()) || value.is_empty()) || size.len() != 1 || state.len() != 1 || size[0].1 != "100" || state[0].1 != "all" { return Err("次のページ情報が無効です。".into()); }
            next = Some(url);
        }
    }
    Ok(next)
}

async fn get_api_response(client: &reqwest::Client, url: reqwest::Url, token: &str, cooldown: &RateCooldown) -> Result<reqwest::Response, String> {
    let _rate_guard = if cooldown.1 { Some(process_lock()?) } else { None };
    get_api_response_locked(client, url, token, cooldown).await
}

async fn get_api_response_locked(client: &reqwest::Client, url: reqwest::Url, token: &str, cooldown: &RateCooldown) -> Result<reqwest::Response, String> {
        check_cooldown(cooldown)?;
        let response = client.get(url)
            .bearer_auth(token).header("Accept", "application/vnd.github+json").header("X-GitHub-Api-Version", "2022-11-28")
            .send().await.map_err(|_| "Issue を取得できませんでした。通信状態を確認してください。")?;
        check_api_response(response, cooldown).await
}

async fn check_api_response(response: reqwest::Response, cooldown: &RateCooldown) -> Result<reqwest::Response, String> {
        if !response.status().is_success() {
            let status = response.status().as_u16();
            let retry = response.headers().get("retry-after").and_then(|v| v.to_str().ok()).and_then(|v| v.parse::<u64>().ok());
            let reset = response.headers().get("x-ratelimit-reset").and_then(|v| v.to_str().ok()).and_then(|v| v.parse::<u64>().ok());
            let primary = response.headers().get("x-ratelimit-remaining").and_then(|v| v.to_str().ok()) == Some("0");
            let body_limited = if status == 403 {
                response.json::<serde_json::Value>().await.ok().and_then(|v| v.get("message").and_then(|m| m.as_str()).map(str::to_lowercase)).map(|m| m.contains("rate limit")).unwrap_or(false)
            } else { false };
            let limited = primary || status == 429 || retry.is_some() || body_limited;
            if limited {
                let now = now_seconds();
                let deadline = rate_deadline(now, retry, reset, primary);
                record_cooldown(cooldown, deadline)?;
            }
            return Err(if limited { "GitHub API のレート制限に達しました。時間をおいて再試行してください。".into() } else { format!("GitHub の要求に失敗しました (HTTP {status})。") });
        }
        Ok(response)
}

#[cfg(all(test, windows))]
static TEST_AFTER_HEADERS: std::sync::Mutex<Option<Box<dyn FnOnce() + Send>>> = std::sync::Mutex::new(None);

async fn fetch_pages(client: &reqwest::Client, endpoint: &str, repository_id: u64, token: &str, cooldown: &RateCooldown) -> Result<Vec<BoardIssue>, String> {
    let mut result = Vec::new();
    let mut seen_urls = std::collections::HashSet::new();
    let mut seen_issues = std::collections::HashSet::new();
    let mut url = reqwest::Url::parse(endpoint).map_err(|_| "Issue 取得先が無効です。")?;
    url.query_pairs_mut().append_pair("state", "all").append_pair("per_page", "100").append_pair("page", "1");
    loop {
        if seen_urls.len() >= 10_000 || !seen_urls.insert(url.to_string()) { return Err("ページが繰り返されたか上限を超えました。全件取得できません。".into()); }
        let response = get_api_response(client, url.clone(), token, cooldown).await?;
        #[cfg(all(test, windows))]
        if let Some(hook) = TEST_AFTER_HEADERS.lock().unwrap().take() { hook(); }
        let next = next_page(response.headers(), endpoint, repository_id)?;
        let issues: Vec<ApiIssue> = response.json().await.map_err(|_| "Issue の応答を読み取れませんでした。".to_string())?;
        for issue in issues {
            if issue.pull_request.is_some() { continue; }
            if !seen_issues.insert(issue.number) { continue; }
            if !matches!(issue.state.as_str(), "open" | "closed") { return Err("Issue の状態が無効です。".into()); }
            let parsed = reqwest::Url::parse(&issue.html_url).map_err(|_| "Issue の URL が無効です。")?;
            if parsed.scheme() != "https" || parsed.host_str() != Some("github.com") || parsed.port().is_some() || !parsed.username().is_empty() || parsed.password().is_some() { return Err("Issue の URL が許可されていません。".into()); }
            let status = classify_issue(&issue).to_owned();
            result.push(BoardIssue { number: issue.number, title: issue.title, url: issue.html_url, status, state: issue.state, updated_at: issue.updated_at });
        }
        match next { Some(value) => url = value, None => break }
    }
    Ok(result)
}

async fn fetch_pages_checked<F, Fut>(client: &reqwest::Client, endpoint: &str, repository_id: u64, token: &str, cooldown: &RateCooldown, finish: F) -> Result<Vec<BoardIssue>, String>
where
    F: FnOnce(Vec<BoardIssue>) -> Fut,
    Fut: std::future::Future<Output = Result<Vec<BoardIssue>, String>>,
{
    let issues = fetch_pages(client, endpoint, repository_id, token, cooldown).await?;
    finish(issues).await
}

#[tauri::command]
async fn list_issues(expected_owner: String, expected_repo: String, expected_revision: String, lock: State<'_, ConnectionLock>, cooldown: State<'_, RateCooldown>) -> Result<Vec<BoardIssue>, String> {
    let (settings, token) = {
        let guard = lock.0.lock().await;
        let _process_guard = process_lock()?;
        recover_pending()?;
        let settings = read_settings()?.ok_or("接続設定を保存してください。")?;
        if !matches_connection(&settings, &expected_owner, &expected_repo) { return Err("別のウィンドウで接続先が変更されました。設定を開いて確認してください。".into()); }
        if !valid_component(&settings.owner) || !valid_component(&settings.repo) { return Err("保存された接続先が無効です。".into()); }
        let token = read_token(&settings.credential_id)?;
        validate_connection_revision(guard.as_ref(), &expected_revision, &Some(settings.clone()), token.as_deref())?;
        let token = token.ok_or("トークンを設定してください。")?;
        (settings, token)
    };
    let client = reqwest::Client::builder().timeout(Duration::from_secs(20)).redirect(reqwest::redirect::Policy::none()).user_agent("github-task-widget").build().map_err(|_| "通信の準備に失敗しました。")?;
    let repo_endpoint = format!("https://api.github.com/repos/{}/{}", settings.owner, settings.repo);
    let repo_url = reqwest::Url::parse(&repo_endpoint).map_err(|_| "リポジトリ URL が無効です。")?;
    let repository: ApiRepository = get_api_response(&client, repo_url, &token, &cooldown).await?.json().await.map_err(|_| "リポジトリ情報を読み取れませんでした。")?;
    if repository.id == 0 || !repository.full_name.eq_ignore_ascii_case(&format!("{}/{}", settings.owner, settings.repo)) { return Err("リポジトリ情報が接続先と一致しません。設定を確認してください。".into()); }
    let endpoint = format!("{repo_endpoint}/issues");
    let settings_ref = &settings;
    let token_ref = &token;
    let lock_ref: &ConnectionLock = &lock;
    fetch_pages_checked(&client, &endpoint, repository.id, &token, &cooldown, |issues| finish_board(lock_ref, settings_ref, token_ref, issues)).await
}

#[derive(Default)]
struct ConnectionLock(Mutex<Option<ConnectionSnapshot>>);

struct ConnectionSnapshot {
    revision: String,
    settings: Option<Settings>,
    token_digest: Option<[u8; 32]>,
}

fn token_digest(token: Option<&str>) -> Option<[u8; 32]> {
    token.map(|value| Sha256::digest(value.as_bytes()).into())
}

fn capture_connection(settings: Option<Settings>, token: Option<&str>) -> (ConnectionInfo, ConnectionSnapshot) {
    let revision = uuid::Uuid::new_v4().to_string();
    let info = ConnectionInfo {
        owner: settings.as_ref().map(|value| value.owner.clone()).unwrap_or_default(),
        repo: settings.as_ref().map(|value| value.repo.clone()).unwrap_or_default(),
        has_token: token.is_some(), revision: revision.clone(),
    };
    (info, ConnectionSnapshot { revision, settings, token_digest: token_digest(token) })
}

fn refresh_connection_snapshot(slot: &mut Option<ConnectionSnapshot>, settings: Option<Settings>, token: Option<&str>) -> ConnectionInfo {
    let (mut info, candidate) = capture_connection(settings, token);
    if let Some(current) = slot.as_ref() {
        if current.settings == candidate.settings && current.token_digest == candidate.token_digest {
            info.revision = current.revision.clone();
            return info;
        }
    }
    *slot = Some(candidate);
    info
}

fn validate_connection_revision(snapshot: Option<&ConnectionSnapshot>, revision: &str, settings: &Option<Settings>, token: Option<&str>) -> Result<(), String> {
    if snapshot.is_some_and(|value| value.revision == revision && value.settings == *settings && value.token_digest == token_digest(token)) {
        Ok(())
    } else {
        Err("接続設定が変更されました。設定を再読み込みして確認してください。".into())
    }
}

async fn finish_board(lock: &ConnectionLock, expected: &Settings, token: &str, issues: Vec<BoardIssue>) -> Result<Vec<BoardIssue>, String> {
    let _guard = lock.0.lock().await;
    let _process_guard = process_lock()?;
    recover_pending()?;
    let current = read_settings()?.ok_or("接続設定が変更されました。設定を確認してください。")?;
    let current_token = read_token(&current.credential_id)?;
    validate_board_snapshot(issues, expected, &current, token, current_token.as_deref())
}

#[cfg(all(test, windows))]
fn validate_ipc_test_identity(service: &str, id: &str) -> Result<(), String> {
    let suffix = service.strip_prefix("github-task-widget-ipc-test-").ok_or("テスト用資格情報 service が無効です。")?;
    let parsed = uuid::Uuid::parse_str(suffix).map_err(|_| "テスト用資格情報 service が無効です。")?;
    if parsed.to_string() != suffix { return Err("テスト用資格情報 service が無効です。".into()); }
    let parsed_id = uuid::Uuid::parse_str(id).map_err(|_| "テスト用資格情報 ID が無効です。")?;
    if parsed_id.to_string() != id { return Err("テスト用資格情報 ID が無効です。".into()); }
    Ok(())
}

fn entry(id: &str) -> Result<keyring::Entry, String> {
    #[cfg(all(test, windows))]
    if std::env::var_os("WIDGET_IPC_CONTEXT").is_some() {
        let service = std::env::var("WIDGET_IPC_SERVICE").map_err(|_| "テスト用資格情報 service がありません。")?;
        validate_ipc_test_identity(&service, id)?;
        return keyring::Entry::new(&service, id).map_err(|_| "資格情報ストアを利用できません。".into());
    }
    keyring::Entry::new(SERVICE, id).map_err(|_| "資格情報ストアを利用できません。".into())
}

fn read_token(id: &str) -> Result<Option<String>, String> {
    if id.is_empty() { return Ok(None); }
    match entry(id)?.get_password() {
        Ok(token) => Ok(Some(token)),
        Err(keyring::Error::NoEntry) => Ok(None),
        Err(_) => Err("資格情報を読み取れませんでした。".into()),
    }
}

fn process_lock() -> Result<fs::File, String> {
    let path = settings_path()?;
    let dir = path.parent().ok_or("設定保存先が無効です。")?;
    fs::create_dir_all(dir).map_err(|_| "設定フォルダーを作成できません。")?;
    process_lock_at(&dir.join("settings.lock"))
}

fn process_lock_at(path: &Path) -> Result<fs::File, String> {
    let file = fs::OpenOptions::new().read(true).write(true).create(true).open(path).map_err(|_| "設定のロックを取得できません。")?;
    file.try_lock_exclusive().map_err(|_| "別のウィンドウで設定を操作中です。もう一度お試しください。")?;
    Ok(file)
}

fn settings_path() -> Result<PathBuf, String> {
    let appdata = std::env::var_os("APPDATA").ok_or("設定保存先を取得できません。")?;
    Ok(PathBuf::from(appdata).join("github-task-widget").join("settings.json"))
}

fn read_settings() -> Result<Option<Settings>, String> {
    match fs::read(settings_path()?) {
        Ok(data) => serde_json::from_slice(&data).map(Some).map_err(|_| "設定ファイルを読み取れません。".into()),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(None),
        Err(_) => Err("設定ファイルを読み取れません。".into()),
    }
}

fn write_settings(settings: &Settings) -> Result<(), String> {
    let path = settings_path()?;
    write_settings_at(&path, settings)
}

fn write_settings_at(path: &Path, settings: &Settings) -> Result<(), String> {
    write_json_at(path, settings)
}

fn write_json_at<T: Serialize>(path: &Path, value: &T) -> Result<(), String> {
    let dir = path.parent().ok_or("設定保存先が無効です。")?;
    fs::create_dir_all(dir).map_err(|_| "設定フォルダーを作成できません。")?;
    let mut temp = tempfile::NamedTempFile::new_in(dir).map_err(|_| "設定を保存できません。")?;
    serde_json::to_writer(&mut temp, value).map_err(|_| "設定を保存できません。")?;
    temp.flush().map_err(|_| "設定を保存できません。")?;
    temp.as_file().sync_all().map_err(|_| "設定を保存できません。")?;
    temp.persist(path).map_err(|_| "設定を保存できません。")?;
    Ok(())
}

fn recovery_path() -> Result<PathBuf, String> {
    Ok(settings_path()?.with_file_name("credential-recovery.json"))
}

fn obsolete_generation<'a>(recovery: &'a Recovery, current: &str) -> Result<&'a str, String> {
    if uuid::Uuid::parse_str(&recovery.new_id).is_err() || (!recovery.old_id.is_empty() && (uuid::Uuid::parse_str(&recovery.old_id).is_err() || recovery.old_id == recovery.new_id)) {
        return Err("資格情報の復旧記録が無効です。".into());
    }
    if current == recovery.new_id { Ok(&recovery.old_id) } else if current == recovery.old_id { Ok(&recovery.new_id) } else { Err("設定と資格情報の復旧記録が一致しません。設定を確認してください。".into()) }
}

fn recover_pending() -> Result<(), String> {
    let path = recovery_path()?;
    let current = read_settings()?.map(|s| s.credential_id).unwrap_or_default();
    recover_at(&path, &current, |id| match entry(id)?.delete_credential() {
        Ok(()) | Err(keyring::Error::NoEntry) => Ok(()),
        Err(_) => Err("古い資格情報を削除できません。復旧記録を保持しています。".into()),
    })
}

fn recover_at<D>(path: &Path, current: &str, delete: D) -> Result<(), String>
where D: Fn(&str) -> Result<(), String> {
    let data = match fs::read(&path) {
        Ok(value) => value,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(()),
        Err(_) => return Err("資格情報の復旧記録を読み取れません。".into()),
    };
    let recovery: Recovery = serde_json::from_slice(&data).map_err(|_| "資格情報の復旧記録が無効です。" )?;
    let obsolete = obsolete_generation(&recovery, current)?;
    if !obsolete.is_empty() {
        delete(obsolete)?;
    }
    fs::remove_file(path).map_err(|_| "資格情報の復旧記録を更新できません。".to_string())?;
    Ok(())
}

fn commit_settings<P, W, S, F>(settings: &Settings, token: Option<&str>, old_id: &str, put: P, write: W, start: S, finish: F) -> Result<(), String>
where
    P: Fn(&str, &str) -> Result<(), String>,
    W: Fn(&Settings) -> Result<(), String>,
    S: Fn(&Recovery) -> Result<(), String>,
    F: Fn() -> Result<(), String>,
{
    if let Some(value) = token {
        start(&Recovery { old_id: old_id.to_owned(), new_id: settings.credential_id.clone() })?;
        if put(&settings.credential_id, value).is_err() {
            if finish().is_err() { return Err("資格情報の保存に失敗しました。復旧記録を保持しています。".into()); }
            return Err("トークンを保存できませんでした。".into());
        }
    }
    if write(settings).is_err() {
        if token.is_some() && finish().is_err() {
            return Err("設定の保存に失敗しました。資格情報の復旧記録を保持しています。".into());
        }
        return Err("設定を保存できませんでした。以前の接続設定は変更されていません。".into());
    }
    if token.is_some() && finish().is_err() { return Err("設定は保存されましたが、資格情報の復旧が未完了です。復旧記録を保持しています。".into()); }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn mock_pages(pages: Vec<(u16, String)>) -> (String, std::thread::JoinHandle<()>) {
        use std::{io::{Read, Write}, net::TcpListener};
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let endpoint = format!("http://{}/issues", listener.local_addr().unwrap());
        let server_endpoint = endpoint.clone();
        let server = std::thread::spawn(move || {
            let total = pages.len();
            for (index, (status, body)) in pages.into_iter().enumerate() {
                let (mut stream, _) = listener.accept().unwrap();
                let mut request = [0; 4096];
                let len = stream.read(&mut request).unwrap();
                let request_line = String::from_utf8_lossy(&request[..len]);
                assert!(request_line.lines().next().unwrap_or_default().contains(&format!("&page={} ", index + 1)), "unexpected requested page");
                let link = if index + 1 < total { format!("Link: <{server_endpoint}?state=all&per_page=100&page={}>; rel=\"next\"\r\n", index + 2) } else { String::new() };
                write!(stream, "HTTP/1.1 {status} Test\r\n{link}Content-Length: {}\r\nConnection: close\r\n\r\n{}", body.len(), body).unwrap();
            }
        });
        (endpoint, server)
    }

    fn issue_json(number: u64, state: &str, labels: &[&str], pull: bool) -> serde_json::Value {
        let mut issue = serde_json::json!({"number":number,"title":format!("Issue {number}"),"html_url":format!("https://github.com/a/b/issues/{number}"),"state":state,"updated_at":"2026-11-19T00:00:00Z","labels":labels.iter().map(|name| serde_json::json!({"name":name})).collect::<Vec<_>>()});
        if pull { issue["pull_request"] = serde_json::json!({}); }
        issue
    }

    #[test]
    fn opener_rejects_local_url_via_mock_ipc() {
        use std::{sync::mpsc, time::Duration};
        use tauri::{ipc::{CallbackFn, InvokeBody}, test::{get_ipc_response, mock_builder, INVOKE_KEY}, webview::InvokeRequest};

        let (sender, receiver) = mpsc::sync_channel(1);
        std::thread::spawn(move || {
            let app = mock_builder().plugin(tauri_plugin_opener::init())
                .build(tauri::generate_context!()).expect("mock app setup failed");
            let window = tauri::WebviewWindowBuilder::new(&app, "main", Default::default())
                .build().expect("mock window setup failed");
            let request = InvokeRequest {
                cmd: "plugin:opener|open_url".into(), callback: CallbackFn(0), error: CallbackFn(1),
                url: "http://tauri.localhost".parse().unwrap(),
                body: InvokeBody::Json(serde_json::json!({"url": "http://127.0.0.1:1/blocked"})),
                headers: Default::default(), invoke_key: INVOKE_KEY.to_string(),
            };
            let _ = sender.send(get_ipc_response(&window, request));
        });
        let result = receiver.recv_timeout(Duration::from_secs(10)).expect("mock IPC did not complete");
        let error = result.expect_err("unapproved URL was accepted");
        assert!(error.as_str().unwrap_or_default().contains("Not allowed to open url"), "unexpected rejection: {error}");
    }

    #[test]
    fn board_classification_prioritizes_status_labels_and_excludes_pull_requests() {
        let labeled: ApiIssue = serde_json::from_value(issue_json(1, "open", &["status:done", "status:todo", "status:in-progress"], false)).unwrap();
        let closed: ApiIssue = serde_json::from_value(issue_json(2, "closed", &[], false)).unwrap();
        let open: ApiIssue = serde_json::from_value(issue_json(3, "open", &[], false)).unwrap();
        let closed_labeled: ApiIssue = serde_json::from_value(issue_json(4, "closed", &["status:todo"], false)).unwrap();
        let closed_multiple: ApiIssue = serde_json::from_value(issue_json(5, "closed", &["status:todo", "status:in-progress", "status:done"], false)).unwrap();
        assert_eq!(classify_issue(&labeled), "todo");
        assert_eq!(classify_issue(&closed), "done");
        assert_eq!(classify_issue(&closed_labeled), "done");
        assert_eq!(classify_issue(&closed_multiple), "done");
        assert_eq!(classify_issue(&open), "todo");
        let page = serde_json::json!([issue_json(1, "open", &[], false), issue_json(2, "closed", &["status:todo"], false), issue_json(3, "open", &[], true)]).to_string();
        let (url, server) = mock_pages(vec![(200,page)]);
        let client = reqwest::Client::builder().no_proxy().build().unwrap();
        let result = tauri::async_runtime::block_on(fetch_pages(&client, &url, 42, "synthetic", &RateCooldown::default())).unwrap();
        server.join().unwrap();
        assert_eq!(result.len(), 2);
        assert_eq!(result[0].number, 1);
        assert_eq!(result[0].state, "open");
        assert_eq!(result[1].state, "closed");
        assert_eq!(result[1].status, "done");
    }

    #[test]
    fn board_rejects_unknown_github_state() {
        let page = serde_json::json!([issue_json(1, "unknown", &[], false)]).to_string();
        let (url, server) = mock_pages(vec![(200, page)]);
        let client = reqwest::Client::builder().no_proxy().build().unwrap();
        let result = tauri::async_runtime::block_on(fetch_pages(&client, &url, 42, "synthetic", &RateCooldown::default()));
        server.join().unwrap();
        assert!(matches!(result, Err(message) if message.contains("状態が無効")));
    }

    #[test]
    fn board_request_must_match_the_current_shared_connection() {
        let settings = Settings { owner: "new-owner".into(), repo: "new-repo".into(), credential_id: String::new() };
        assert!(!matches_connection(&settings, "old-owner", "old-repo"));
        assert!(matches_connection(&settings, "new-owner", "new-repo"));
    }

    #[test]
    fn board_snapshot_rejects_changed_or_removed_credentials() {
        let initial = Settings { owner: "owner".into(), repo: "repo".into(), credential_id: "generation-1".into() };
        let same = initial.clone();
        let new_token = Settings { owner: "owner".into(), repo: "repo".into(), credential_id: "generation-2".into() };
        let other_repo = Settings { owner: "owner".into(), repo: "other".into(), credential_id: "generation-1".into() };
        assert!(matches_connection_snapshot(&initial, &same, "synthetic-old", Some("synthetic-old")));
        assert!(!matches_connection_snapshot(&initial, &new_token, "synthetic-old", Some("synthetic-new")));
        assert!(!matches_connection_snapshot(&initial, &same, "synthetic-old", None));
        assert!(!matches_connection_snapshot(&initial, &same, "synthetic-old", Some("synthetic-new")));
        assert!(!matches_connection_snapshot(&initial, &other_repo, "synthetic-old", Some("synthetic-old")));
    }

    #[test]
    fn mutation_revision_rejects_changed_settings_and_credentials() {
        let initial = Some(Settings { owner: "owner".into(), repo: "repo".into(), credential_id: "synthetic-id".into() });
        let (info, snapshot) = capture_connection(initial.clone(), Some("synthetic-token"));
        let serialized = serde_json::to_string(&info).unwrap();
        assert!(!serialized.contains("synthetic-token") && !serialized.contains("synthetic-id"));
        assert!(validate_connection_revision(Some(&snapshot), &info.revision, &initial, Some("synthetic-token")).is_ok());
        assert!(validate_connection_revision(None, &info.revision, &initial, Some("synthetic-token")).is_err());
        assert!(validate_connection_revision(Some(&snapshot), "wrong", &initial, Some("synthetic-token")).is_err());
        assert!(validate_connection_revision(Some(&snapshot), &info.revision, &initial, None).is_err());
        assert!(validate_connection_revision(Some(&snapshot), &info.revision, &initial, Some("different-token")).is_err());
        let changed_repo = Some(Settings { owner: "owner".into(), repo: "other".into(), credential_id: "synthetic-id".into() });
        let changed_id = Some(Settings { owner: "owner".into(), repo: "repo".into(), credential_id: "other-id".into() });
        assert!(validate_connection_revision(Some(&snapshot), &info.revision, &changed_repo, Some("synthetic-token")).is_err());
        assert!(validate_connection_revision(Some(&snapshot), &info.revision, &changed_id, Some("synthetic-token")).is_err());
        let (fresh, _) = capture_connection(initial, Some("synthetic-token"));
        assert_ne!(info.revision, fresh.revision);
    }

    #[test]
    fn repeated_reads_reuse_revision_only_while_connection_is_unchanged() {
        let settings = Some(Settings { owner: "owner".into(), repo: "repo".into(), credential_id: "synthetic-id".into() });
        let mut slot = None;
        let first = refresh_connection_snapshot(&mut slot, settings.clone(), Some("synthetic-token"));
        let second = refresh_connection_snapshot(&mut slot, settings.clone(), Some("synthetic-token"));
        assert_eq!(first.revision, second.revision);
        let changed = refresh_connection_snapshot(&mut slot, settings.clone(), Some("different-token"));
        assert_ne!(second.revision, changed.revision);
        assert!(validate_connection_revision(slot.as_ref(), &second.revision, &settings, Some("different-token")).is_err());
    }

    #[cfg(windows)]
    #[test]
    fn ipc_test_identity_rejects_product_and_noncanonical_values() {
        let uuid = "abcdefab-1234-4234-8234-abcdefabcdef";
        let service = format!("github-task-widget-ipc-test-{uuid}");
        assert!(validate_ipc_test_identity(&service, uuid).is_ok());
        for invalid in ["github-task-widget", "", "wrong-abcdefab-1234-4234-8234-abcdefabcdef", "github-task-widget-ipc-test-invalid", "github-task-widget-ipc-test-ABCDEFAB-1234-4234-8234-ABCDEFABCDEF", "github-task-widget-ipc-test-abcdefab-1234-4234-8234-abcdefabcdef-extra"] {
            assert!(validate_ipc_test_identity(invalid, uuid).is_err());
        }
        for invalid in ["", "invalid", "ABCDEFAB-1234-4234-8234-ABCDEFABCDEF"] {
            assert!(validate_ipc_test_identity(&service, invalid).is_err());
        }
    }

    #[cfg(windows)]
    struct RaceCredentials {
        entries: Vec<(String, &'static str)>,
        service: Option<String>,
        cleanup_allowed: bool,
        cleanup_done: bool,
    }

    #[cfg(windows)]
    impl RaceCredentials {
        fn cleanup(&self) -> Result<(), String> {
            for (id, expected) in &self.entries {
                let credential = match &self.service {
                    Some(service) => keyring::Entry::new(service, id).map_err(|_| "資格情報ストアを利用できません。")?,
                    None => entry(id)?,
                };
                match credential.get_password() {
                    Ok(value) if value == *expected => credential.delete_credential().map_err(|_| "合成資格情報を削除できません。")?,
                    Err(keyring::Error::NoEntry) => (),
                    _ => return Err("合成資格情報の値を確認できないため削除しません。".into()),
                }
                if !matches!(credential.get_password(), Err(keyring::Error::NoEntry)) {
                    return Err("合成資格情報の削除を確認できません。".into());
                }
            }
            Ok(())
        }
    }

    #[cfg(windows)]
    impl Drop for RaceCredentials {
        fn drop(&mut self) {
            if self.cleanup_done { return; }
            if !self.cleanup_allowed {
                eprintln!("Synthetic credential cleanup unconfirmed; inspect recorded IDs only.");
                return;
            }
            self.cleanup_allowed = false;
            if self.cleanup().is_err() {
                eprintln!("Synthetic credential cleanup unconfirmed; inspect recorded IDs only.");
            }
        }
    }

    #[cfg(windows)]
    fn accept_race_connection(listener: &std::net::TcpListener) -> std::io::Result<std::net::TcpStream> {
        listener.set_nonblocking(true)?;
        let deadline = std::time::Instant::now() + Duration::from_secs(20);
        loop {
            match listener.accept() {
                Ok((stream, _)) => return Ok(stream),
                Err(error) if error.kind() == std::io::ErrorKind::WouldBlock && std::time::Instant::now() < deadline => std::thread::sleep(Duration::from_millis(20)),
                Err(error) if error.kind() == std::io::ErrorKind::WouldBlock => return Err(std::io::Error::new(std::io::ErrorKind::TimedOut, "test listener timed out")),
                Err(error) => return Err(error),
            }
        }
    }

    #[cfg(windows)]
    fn race_thread_finished<T>(thread: &std::thread::JoinHandle<T>, timeout: Duration) -> bool {
        let deadline = std::time::Instant::now() + timeout;
        while !thread.is_finished() {
            if std::time::Instant::now() >= deadline { return false; }
            std::thread::sleep(Duration::from_millis(20));
        }
        true
    }

    #[cfg(windows)]
    fn verify_race_fixture(root: &Path, marker: &str) -> Result<(), String> {
        if uuid::Uuid::parse_str(marker).is_err() { return Err("invalid fixture marker".into()); }
        let marker_path = root.join("fixture.marker");
        let marker_meta = fs::symlink_metadata(&marker_path).map_err(|_| "fixture marker unavailable")?;
        if !marker_meta.file_type().is_file() || marker_meta.file_type().is_symlink() { return Err("invalid fixture marker file".into()); }
        if fs::read_to_string(marker_path).map_err(|_| "fixture marker unreadable")? != marker { return Err("fixture marker mismatch".into()); }
        for item in fs::read_dir(root).map_err(|_| "fixture root unreadable")? {
            let item = item.map_err(|_| "fixture entry unreadable")?;
            let kind = item.file_type().map_err(|_| "fixture type unreadable")?;
            let name = item.file_name().to_string_lossy().into_owned();
            if kind.is_symlink() || !matches!(name.as_str(), "fixture.marker" | "github-task-widget") { return Err("unexpected fixture entry".into()); }
            if (name == "fixture.marker" && !kind.is_file()) || (name == "github-task-widget" && !kind.is_dir()) { return Err("unexpected fixture type".into()); }
        }
        let settings_dir = root.join("github-task-widget");
        if settings_dir.canonicalize().map_err(|_| "fixture settings unavailable")?.parent() != Some(root) { return Err("fixture settings path mismatch".into()); }
        for item in fs::read_dir(&settings_dir).map_err(|_| "fixture settings unreadable")? {
            let item = item.map_err(|_| "fixture settings entry unreadable")?;
            let kind = item.file_type().map_err(|_| "fixture settings type unreadable")?;
            let name = item.file_name().to_string_lossy().into_owned();
            if kind.is_symlink() || !kind.is_file() || !matches!(name.as_str(), "settings.json" | "settings.lock") { return Err("unexpected fixture settings entry".into()); }
        }
        match fs::symlink_metadata(settings_dir.join("credential-recovery.json")) {
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(()),
            _ => Err("fixture recovery record must be absent".into()),
        }
    }

    #[cfg(windows)]
    #[test]
    #[ignore = "uses isolated test-service credentials and a mock IPC child"]
    fn windows_ipc_guard() {
        use std::{io::Write, os::windows::process::CommandExt, process::{Command, Stdio}, thread, time::Instant};
        let temp_root = std::env::temp_dir().canonicalize().unwrap();
        let root = temp_root.join(format!("giw-ipc-{}", uuid::Uuid::new_v4()));
        fs::create_dir(&root).unwrap();
        let root = root.canonicalize().unwrap();
        assert!(root.starts_with(&temp_root) && root != temp_root);
        assert_eq!(fs::read_dir(&root).unwrap().count(), 0);
        let marker = uuid::Uuid::new_v4().to_string();
        fs::write(root.join("fixture.marker"), &marker).unwrap();
        let settings_dir = root.join("github-task-widget");
        fs::create_dir(&settings_dir).unwrap();
        let settings_dir = settings_dir.canonicalize().unwrap();
        assert_eq!(settings_dir.parent(), Some(root.as_path()));
        let settings_path = settings_dir.join("settings.json");
        assert!(!settings_path.exists());

        let service = format!("github-task-widget-ipc-test-{}", uuid::Uuid::new_v4());
        let id = uuid::Uuid::new_v4().to_string();
        let credential = keyring::Entry::new(&service, &id).expect("test credential setup failed");
        assert!(matches!(credential.get_password(), Err(keyring::Error::NoEntry)), "test credential already exists or cannot be checked");
        let mut credentials = RaceCredentials { entries: vec![(id.clone(), "synthetic-ipc-secret")], service: Some(service.clone()), cleanup_allowed: true, cleanup_done: false };
        println!("fixture_root={} test_service={} synthetic_id={}", root.display(), service, id);
        println!("stage=before-credential-write"); std::io::stdout().flush().unwrap();
        if let Err(error) = credential.set_password("synthetic-ipc-secret") {
            let category = match error {
                keyring::Error::PlatformFailure(_) => "platform_failure",
                keyring::Error::NoStorageAccess(_) => "no_storage_access",
                keyring::Error::NoEntry => "no_entry",
                keyring::Error::BadEncoding(_) => "bad_encoding",
                keyring::Error::TooLong(_, _) => "too_long",
                keyring::Error::Invalid(_, _) => "invalid",
                keyring::Error::Ambiguous(_) => "ambiguous",
                _ => "other",
            };
            panic!("synthetic credential write failed; category={category}");
        }
        write_settings_at(&settings_path, &Settings { owner: "a".into(), repo: "b".into(), credential_id: id.clone() }).unwrap();

        let mut command = Command::new(std::env::current_exe().unwrap());
        command.args(["--exact", "tests::windows_ipc_guard_child", "--ignored", "--test-threads=1", "--nocapture"])
            .env("APPDATA", &root).env("WIDGET_RACE_ROOT", &root).env("WIDGET_RACE_MARKER", &marker)
            .env("WIDGET_IPC_CONTEXT", "1").env("WIDGET_IPC_SERVICE", &service).env("WIDGET_IPC_ID", &id)
            .env("WIDGET_RACE_EXPECTED_SETTINGS", &settings_path)
            .stdout(Stdio::piped()).stderr(Stdio::piped()).creation_flags(0x08000000);
        credentials.cleanup_allowed = false;
        let mut child = command.spawn().expect("isolated IPC child could not be started; synthetic ID retained");
        println!("stage=ipc-child-started pid={}", child.id()); std::io::stdout().flush().unwrap();
        let deadline = Instant::now() + Duration::from_secs(40);
        let status = loop {
            match child.try_wait() {
                Ok(Some(status)) => break Some(status),
                Ok(None) if Instant::now() < deadline => thread::sleep(Duration::from_millis(20)),
                Ok(None) => {
                    if child.kill().is_err() { break None; }
                    let stop_deadline = Instant::now() + Duration::from_secs(5);
                    break loop {
                        match child.try_wait() {
                            Ok(Some(status)) => break Some(status),
                            Ok(None) if Instant::now() < stop_deadline => thread::sleep(Duration::from_millis(20)),
                            _ => break None,
                        }
                    };
                }
                Err(_) => break None,
            }
        };
        if status.is_none() { panic!("IPC child termination is unconfirmed; synthetic ID and fixture retained"); }
        credentials.cleanup_allowed = true;
        let output = child.wait_with_output().unwrap();
        assert!(output.status.success(), "isolated IPC child test failed");
        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(stdout.contains("IPC_INFO_OK") && stdout.contains("IPC_STALE_BOARD_ACTIONS_REJECTED") && stdout.contains("IPC_STALE_SAVE_REJECTED") && stdout.contains("IPC_STALE_DELETE_REJECTED") && stdout.contains("IPC_DELETE_OK"));
        println!("stage=ipc-info-and-delete-confirmed");
        credentials.cleanup_allowed = false;
        let cleanup_result = credentials.cleanup();
        if cleanup_result.is_ok() { credentials.cleanup_done = true; }
        cleanup_result.expect("synthetic credential cleanup failed; inspect recorded ID only");
        println!("stage=ipc-credential-cleanup-confirmed");
    }

    #[cfg(windows)]
    #[test]
    #[ignore = "child of the isolated test-service IPC test"]
    fn windows_ipc_guard_child() {
        use tauri::test::{get_ipc_response, mock_builder, mock_context, noop_assets, INVOKE_KEY};
        use tauri::{ipc::{CallbackFn, InvokeBody}, webview::InvokeRequest};
        assert!(std::env::var("WIDGET_IPC_CONTEXT").ok().as_deref() == Some("1"), "missing IPC fixture context");
        let root = PathBuf::from(std::env::var_os("WIDGET_RACE_ROOT").expect("missing isolated root")).canonicalize().unwrap();
        let temp = std::env::temp_dir().canonicalize().unwrap();
        assert!(root.starts_with(&temp) && root != temp);
        assert!(PathBuf::from(std::env::var_os("APPDATA").unwrap()).canonicalize().unwrap() == root, "APPDATA does not match fixture");
        let expected_path = PathBuf::from(std::env::var_os("WIDGET_RACE_EXPECTED_SETTINGS").unwrap());
        assert!(settings_path().unwrap() == expected_path, "settings path does not match fixture");
        let marker = std::env::var("WIDGET_RACE_MARKER").expect("missing fixture marker");
        verify_race_fixture(&root, &marker).expect("isolated fixture was not accepted");
        let id = std::env::var("WIDGET_IPC_ID").expect("missing fixture ID");
        let parsed_id = uuid::Uuid::parse_str(&id).expect("invalid fixture ID");
        assert!(parsed_id.to_string() == id, "noncanonical fixture ID");
        let settings = read_settings().unwrap().unwrap();
        assert!(settings.credential_id == id && settings.owner == "a" && settings.repo == "b", "settings do not match fixture");
        assert!(read_token(&id).unwrap().as_deref() == Some("synthetic-ipc-secret"), "credential is not the expected synthetic value");

        let app = mock_builder().manage(ConnectionLock::default()).manage(RateCooldown::default())
            .invoke_handler(tauri::generate_handler![connection_info, save_connection, delete_token, list_issues, create_issue, move_issue])
            .build(mock_context(noop_assets())).expect("mock app setup failed");
        let window = tauri::WebviewWindowBuilder::new(&app, "main", Default::default()).build().expect("mock window setup failed");
        let invoke = |command: &str, body: serde_json::Value| -> serde_json::Value {
            let request = InvokeRequest { cmd: command.into(), callback: CallbackFn(0), error: CallbackFn(1),
                url: "http://tauri.localhost".parse().unwrap(), body: InvokeBody::Json(body), headers: Default::default(), invoke_key: INVOKE_KEY.to_string() };
            let response = match get_ipc_response(&window, request) { Ok(value) => value, Err(_) => panic!("mock IPC returned an error") };
            match response.deserialize::<serde_json::Value>() { Ok(value) => value, Err(_) => panic!("mock IPC returned invalid JSON") }
        };
        let before = invoke("connection_info", serde_json::json!({}));
        let before_text = serde_json::to_string(&before).unwrap();
        assert!(before.get("owner").and_then(|v| v.as_str()) == Some("a") && before.get("repo").and_then(|v| v.as_str()) == Some("b") && before.get("has_token").and_then(|v| v.as_bool()) == Some(true), "unexpected IPC connection state");
        assert!(!before_text.contains("synthetic-ipc-secret") && !before_text.contains(&id), "credential data was returned by IPC");
        println!("IPC_INFO_OK");
        verify_race_fixture(&root, &marker).expect("isolated fixture changed");
        let current = read_settings().unwrap().unwrap();
        assert!(current.credential_id == id, "credential ID changed before deletion");
        assert!(read_token(&id).unwrap().as_deref() == Some("synthetic-ipc-secret"), "credential changed before deletion");
        let revision = before.get("revision").and_then(|v| v.as_str()).expect("missing revision");
        // Invalid authorization data cannot be sent if a revision guard regresses.
        let invalid_token = "synthetic\ninvalid";
        assert!(reqwest::Client::new().get("http://127.0.0.1:1").bearer_auth(invalid_token).build().is_err(), "synthetic header must be invalid");
        entry(&id).unwrap().set_password(invalid_token).expect("synthetic credential update failed");
        for (command, body) in [
            ("list_issues", serde_json::json!({"expectedOwner": "a", "expectedRepo": "b", "expectedRevision": revision})),
            ("create_issue", serde_json::json!({"expectedOwner": "a", "expectedRepo": "b", "expectedRevision": revision, "title": "test", "body": "", "status": "todo"})),
            ("move_issue", serde_json::json!({"expectedOwner": "a", "expectedRepo": "b", "expectedRevision": revision, "number": 42, "expectedUpdatedAt": "v1", "status": "done"})),
        ] {
            let request = InvokeRequest { cmd: command.into(), callback: CallbackFn(0), error: CallbackFn(1),
                url: "http://tauri.localhost".parse().unwrap(), body: InvokeBody::Json(body),
                headers: Default::default(), invoke_key: INVOKE_KEY.to_string() };
            let error = get_ipc_response(&window, request).expect_err("stale board action must be rejected");
            assert!(error.as_str() == Some("接続設定が変更されました。設定を再読み込みして確認してください。"), "unexpected stale board response");
        }
        assert!(read_token(&id).unwrap().as_deref() == Some(invalid_token), "stale board action changed credential");
        entry(&id).unwrap().set_password("synthetic-ipc-secret").expect("synthetic credential restore failed");
        println!("IPC_STALE_BOARD_ACTIONS_REJECTED");
        let changed = Settings { owner: "other".into(), repo: current.repo.clone(), credential_id: current.credential_id.clone() };
        write_settings_at(&expected_path, &changed).expect("fixture settings update failed");
        let stale_save = InvokeRequest { cmd: "save_connection".into(), callback: CallbackFn(0), error: CallbackFn(1),
            url: "http://tauri.localhost".parse().unwrap(), body: InvokeBody::Json(serde_json::json!({"owner": "a", "repo": "b", "token": invalid_token, "expectedRevision": revision})),
            headers: Default::default(), invoke_key: INVOKE_KEY.to_string() };
        let save_error = get_ipc_response(&window, stale_save).expect_err("stale save must be rejected");
        assert!(save_error.as_str() == Some("接続設定が変更されました。設定を再読み込みして確認してください。"), "unexpected stale save response");
        println!("IPC_STALE_SAVE_REJECTED");
        let stale = InvokeRequest { cmd: "delete_token".into(), callback: CallbackFn(0), error: CallbackFn(1),
            url: "http://tauri.localhost".parse().unwrap(), body: InvokeBody::Json(serde_json::json!({"expectedRevision": revision})),
            headers: Default::default(), invoke_key: INVOKE_KEY.to_string() };
        assert!(get_ipc_response(&window, stale).is_err(), "stale deletion must be rejected");
        assert!(read_token(&id).unwrap().as_deref() == Some("synthetic-ipc-secret"), "stale deletion changed credential");
        println!("IPC_STALE_DELETE_REJECTED");
        write_settings_at(&expected_path, &current).expect("fixture settings restore failed");
        let refreshed = invoke("connection_info", serde_json::json!({}));
        let revision = refreshed.get("revision").and_then(|v| v.as_str()).expect("missing refreshed revision");
        let _ = invoke("delete_token", serde_json::json!({"expectedRevision": revision}));
        assert!(read_token(&id).unwrap().is_none(), "synthetic credential remains after deletion");
        let after = invoke("connection_info", serde_json::json!({}));
        let after_text = serde_json::to_string(&after).unwrap();
        assert!(after.get("has_token").and_then(|v| v.as_bool()) == Some(false), "unexpected IPC state after deletion");
        assert!(!after_text.contains("synthetic-ipc-secret") && !after_text.contains(&id), "credential data was returned by IPC");
        println!("IPC_DELETE_OK");
    }

    #[cfg(windows)]
    #[test]
    #[ignore = "uses isolated Windows Credential Manager entries and child process"]
    fn windows_inflight_guard() {
        use std::{io::{Read, Write}, net::TcpListener, os::windows::process::CommandExt, process::{Command, Stdio}, sync::mpsc, thread, time::Instant};

        let temp_root = std::env::temp_dir().canonicalize().unwrap();
        let root = temp_root.join(format!("giw-race-{}", uuid::Uuid::new_v4()));
        fs::create_dir(&root).unwrap();
        let root = root.canonicalize().unwrap();
        assert!(root.starts_with(&temp_root) && root != temp_root);
        assert_eq!(fs::read_dir(&root).unwrap().count(), 0);
        let marker = uuid::Uuid::new_v4().to_string();
        fs::write(root.join("fixture.marker"), &marker).unwrap();
        let settings_dir = root.join("github-task-widget");
        fs::create_dir(&settings_dir).unwrap();
        let settings_dir = settings_dir.canonicalize().unwrap();
        assert_eq!(settings_dir.parent(), Some(root.as_path()));
        let settings_path = settings_dir.join("settings.json");
        let lock_path = settings_dir.join("settings.lock");
        assert!(!settings_path.exists() && !lock_path.exists());

        let old_id = uuid::Uuid::new_v4().to_string();
        let new_id = uuid::Uuid::new_v4().to_string();
        assert_ne!(old_id, new_id);
        for id in [&old_id, &new_id] {
            assert!(matches!(entry(id).unwrap().get_password(), Err(keyring::Error::NoEntry)), "candidate credential already exists or cannot be checked");
        }
        let mut credentials = RaceCredentials { entries: vec![(old_id.clone(), "synthetic-old"), (new_id.clone(), "synthetic-new")], service: None, cleanup_allowed: true, cleanup_done: false };
        println!("fixture_root={} synthetic_old_id={} synthetic_new_id={}", root.display(), old_id, new_id);
        println!("stage=before-old-credential-write"); std::io::stdout().flush().unwrap();
        entry(&old_id).unwrap().set_password("synthetic-old").unwrap();
        println!("stage=before-new-credential-write"); std::io::stdout().flush().unwrap();
        entry(&new_id).unwrap().set_password("synthetic-new").unwrap();
        let initial = Settings { owner: "a".into(), repo: "b".into(), credential_id: old_id.clone() };
        let changed = Settings { owner: "a".into(), repo: "b".into(), credential_id: new_id };
        write_settings_at(&settings_path, &initial).unwrap();

        let http = TcpListener::bind("127.0.0.1:0").unwrap();
        let control = TcpListener::bind("127.0.0.1:0").unwrap();
        let endpoint = format!("http://{}/issues", http.local_addr().unwrap());
        let control_addr = control.local_addr().unwrap();
        let page = serde_json::json!([issue_json(42, "open", &[], false)]).to_string();
        let (resume_tx, resume_rx) = mpsc::channel();
        credentials.cleanup_allowed = false;
        let server = thread::spawn(move || -> Result<(), String> {
            for index in 0..2 {
                let mut socket = accept_race_connection(&http).map_err(|_| "HTTP accept failed")?;
                socket.set_read_timeout(Some(Duration::from_secs(15))).map_err(|_| "HTTP timeout setup failed")?;
                socket.set_write_timeout(Some(Duration::from_secs(15))).map_err(|_| "HTTP write timeout setup failed")?;
                let mut request = [0; 4096];
                socket.read(&mut request).map_err(|_| "HTTP request failed")?;
                write!(socket, "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nConnection: close\r\n\r\n", page.len()).map_err(|_| "HTTP headers failed")?;
                socket.flush().map_err(|_| "HTTP header flush failed")?;
                if index == 1 { resume_rx.recv_timeout(Duration::from_secs(15)).map_err(|_| "mutation timed out")?; }
                socket.write_all(page.as_bytes()).map_err(|_| "HTTP body failed")?;
            }
            Ok(())
        });
        let changer = thread::spawn(move || -> Result<(), String> {
            let mut socket = accept_race_connection(&control).map_err(|_| "control accept failed")?;
            socket.set_read_timeout(Some(Duration::from_secs(15))).map_err(|_| "control timeout setup failed")?;
            socket.set_write_timeout(Some(Duration::from_secs(15))).map_err(|_| "control write timeout setup failed")?;
            let mut ready = [0; 5];
            socket.read_exact(&mut ready).map_err(|_| "control request failed")?;
            if &ready != b"READY" { return Err("unexpected control message".into()); }
            {
                let _guard = process_lock_at(&lock_path)?;
                write_settings_at(&settings_path, &changed)?;
            }
            socket.write_all(b"CONTINUE").map_err(|_| "control response failed")?;
            resume_tx.send(()).map_err(|_| "HTTP resume failed")?;
            Ok(())
        });

        let mut command = Command::new(std::env::current_exe().unwrap());
        command.args(["--exact", "tests::windows_inflight_guard_child", "--ignored", "--test-threads=1", "--nocapture"])
            .env("APPDATA", &root).env("WIDGET_RACE_ROOT", &root).env("WIDGET_RACE_ENDPOINT", &endpoint)
            .env("WIDGET_RACE_CONTROL", control_addr.to_string()).env("WIDGET_RACE_OLD_ID", &old_id).env("WIDGET_RACE_MARKER", &marker)
            .env("WIDGET_RACE_EXPECTED_SETTINGS", root.join("github-task-widget").join("settings.json"))
            .stdout(Stdio::piped()).stderr(Stdio::piped()).creation_flags(0x08000000);
        let mut child = command.spawn().expect("isolated child could not be started; synthetic IDs retained");
        println!("stage=child-started pid={}", child.id()); std::io::stdout().flush().unwrap();
        let deadline = Instant::now() + Duration::from_secs(40);
        let status = loop {
            match child.try_wait() {
                Ok(Some(status)) => break Some(status),
                Ok(None) if Instant::now() < deadline => thread::sleep(Duration::from_millis(20)),
                Ok(None) => {
                    if child.kill().is_err() { break None; }
                    let stop_deadline = Instant::now() + Duration::from_secs(5);
                    break loop {
                        match child.try_wait() {
                            Ok(Some(status)) => break Some(status),
                            Ok(None) if Instant::now() < stop_deadline => thread::sleep(Duration::from_millis(20)),
                            _ => break None,
                        }
                    };
                }
                Err(_) => break None,
            }
        };
        let terminated = status.is_some();
        let server_finished = race_thread_finished(&server, Duration::from_secs(20));
        let changer_finished = race_thread_finished(&changer, Duration::from_secs(20));
        if !terminated || !server_finished || !changer_finished {
            panic!("test process or thread termination is unconfirmed; synthetic IDs and fixture retained");
        }
        credentials.cleanup_allowed = true;
        let server_result = server.join().unwrap_or_else(|_| Err("HTTP thread panicked".into()));
        let changer_result = changer.join().unwrap_or_else(|_| Err("control thread panicked".into()));
        let output = child.wait_with_output().unwrap();
        assert!(server_result.is_ok(), "isolated HTTP test failed");
        assert!(changer_result.is_ok(), "isolated settings update failed");
        assert!(output.status.success(), "isolated child test failed");
        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(stdout.contains("RACE_NORMAL_OK") && stdout.contains("RACE_STALE_REJECTED"));
        println!("stage=normal-and-stale-cases-confirmed");
        credentials.cleanup_allowed = false;
        let cleanup_result = credentials.cleanup();
        if cleanup_result.is_ok() { credentials.cleanup_done = true; }
        cleanup_result.expect("synthetic credential cleanup failed; inspect recorded IDs only");
        println!("stage=credential-cleanup-confirmed");
    }

    #[cfg(windows)]
    #[test]
    #[ignore = "child of the isolated Windows in-flight test"]
    fn windows_inflight_guard_child() {
        use std::{io::{Read, Write}, net::{SocketAddr, TcpStream}};
        let root = PathBuf::from(std::env::var_os("WIDGET_RACE_ROOT").expect("missing isolated root")).canonicalize().unwrap();
        let temp = std::env::temp_dir().canonicalize().unwrap();
        assert!(root.starts_with(&temp) && root != temp);
        assert_eq!(PathBuf::from(std::env::var_os("APPDATA").unwrap()).canonicalize().unwrap(), root);
        let expected_path = PathBuf::from(std::env::var_os("WIDGET_RACE_EXPECTED_SETTINGS").unwrap());
        assert_eq!(settings_path().unwrap(), expected_path);
        let marker = std::env::var("WIDGET_RACE_MARKER").expect("missing fixture marker");
        verify_race_fixture(&root, &marker).expect("isolated fixture was not accepted");
        let expected = read_settings().unwrap().unwrap();
        assert!(expected.credential_id == std::env::var("WIDGET_RACE_OLD_ID").unwrap(), "credential id does not match the test fixture");
        assert!(uuid::Uuid::parse_str(&expected.credential_id).is_ok());
        let token = read_token(&expected.credential_id).unwrap().unwrap();
        assert!(token == "synthetic-old", "credential is not the expected synthetic value");
        let endpoint = std::env::var("WIDGET_RACE_ENDPOINT").unwrap();
        let parsed = reqwest::Url::parse(&endpoint).unwrap();
        assert_eq!(parsed.scheme(), "http");
        assert_eq!(parsed.host_str(), Some("127.0.0.1"));
        let control: SocketAddr = std::env::var("WIDGET_RACE_CONTROL").unwrap().parse().unwrap();
        assert!(control.ip().is_loopback());
        let client = reqwest::Client::builder().no_proxy().redirect(reqwest::redirect::Policy::none()).timeout(Duration::from_secs(20)).build().unwrap();
        let lock = ConnectionLock::default();
        let cooldown = RateCooldown(AtomicU64::new(0), true);
        let root_ref = &root;
        let marker_ref = &marker;
        let lock_ref = &lock;
        let expected_ref = &expected;
        let token_ref = &token;
        let good = tauri::async_runtime::block_on(fetch_pages_checked(&client, &endpoint, 42, &token, &cooldown, |issues| async move {
            verify_race_fixture(root_ref, marker_ref)?;
            finish_board(lock_ref, expected_ref, token_ref, issues).await
        })).unwrap();
        assert_eq!(good.len(), 1);
        assert_eq!(good[0].number, 42);
        println!("RACE_NORMAL_OK");
        *TEST_AFTER_HEADERS.lock().unwrap() = Some(Box::new(move || {
            let mut socket = TcpStream::connect_timeout(&control, Duration::from_secs(10)).unwrap();
            socket.set_read_timeout(Some(Duration::from_secs(15))).unwrap();
            socket.set_write_timeout(Some(Duration::from_secs(15))).unwrap();
            socket.write_all(b"READY").unwrap();
            let mut response = [0; 8];
            socket.read_exact(&mut response).unwrap();
            assert_eq!(&response, b"CONTINUE");
        }));
        let result = tauri::async_runtime::block_on(fetch_pages_checked(&client, &endpoint, 42, &token, &cooldown, |issues| async move {
            verify_race_fixture(root_ref, marker_ref)?;
            finish_board(lock_ref, expected_ref, token_ref, issues).await
        }));
        match result {
            Err(error) => {
                assert!(error.contains("接続設定が変更"));
                assert!(!error.contains("synthetic-old") && !error.contains("Issue 42"));
            }
            Ok(_) => panic!("stale board data was returned"),
        }
        println!("RACE_STALE_REJECTED");
    }

    #[test]
    fn in_flight_board_is_not_returned_after_credential_change() {
        use std::{io::{Read, Write}, net::TcpListener, sync::{mpsc, Arc, Mutex}, thread};
        let initial = Settings { owner: "a".into(), repo: "b".into(), credential_id: "generation-1".into() };
        let changed = Settings { owner: "a".into(), repo: "b".into(), credential_id: "generation-2".into() };
        let page = serde_json::json!([issue_json(42, "open", &[], false)]).to_string();
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let url = format!("http://{}/issues", listener.local_addr().unwrap());
        let (request_tx, request_rx) = mpsc::channel();
        let (resume_tx, resume_rx) = mpsc::channel();
        let server = thread::spawn(move || {
            let (mut socket, _) = listener.accept().unwrap();
            socket.set_read_timeout(Some(Duration::from_secs(10))).unwrap();
            let mut request = [0; 4096];
            socket.read(&mut request).unwrap();
            request_tx.send(()).unwrap();
            resume_rx.recv_timeout(Duration::from_secs(10)).unwrap();
            write!(socket, "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}", page.len(), page).unwrap();
        });
        let store = Arc::new(Mutex::new((initial.clone(), Some("synthetic-old".to_owned()))));
        let changer_store = Arc::clone(&store);
        let changer = thread::spawn(move || {
            request_rx.recv_timeout(Duration::from_secs(10)).unwrap();
            *changer_store.lock().unwrap() = (changed, Some("synthetic-new".to_owned()));
            resume_tx.send(()).unwrap();
        });
        let client = reqwest::Client::builder().timeout(Duration::from_secs(10)).no_proxy().build().unwrap();
        let result = tauri::async_runtime::block_on(fetch_pages_checked(&client, &url, 42, "synthetic-old", &RateCooldown::default(), |issues| async {
            let current = store.lock().unwrap();
            validate_board_snapshot(issues, &initial, &current.0, "synthetic-old", current.1.as_deref())
        }));
        server.join().unwrap();
        changer.join().unwrap();
        match result {
            Err(error) => {
                assert!(error.contains("接続設定が変更"));
                assert!(!error.contains("synthetic-old"));
                assert!(!error.contains("Issue 42"));
            }
            Ok(_) => panic!("stale issue data was returned after the credential changed"),
        }
    }

    #[test]
    fn fetches_more_than_one_thousand_issues() {
        let pages = (0..11).map(|page| {
            let count = if page == 10 { 50 } else { 100 };
            let values: Vec<_> = (0..count).map(|i| issue_json((page * 100 + i + 1) as u64, "open", &[], false)).collect();
            (200, serde_json::to_string(&values).unwrap())
        }).collect();
        let (url, server) = mock_pages(pages);
        let client = reqwest::Client::builder().no_proxy().build().unwrap();
        let result = tauri::async_runtime::block_on(fetch_pages(&client, &url, 42, "synthetic", &RateCooldown::default())).unwrap();
        server.join().unwrap();
        assert_eq!(result.len(), 1050);
    }

    #[test]
    fn later_page_failure_never_returns_partial_results() {
        let values: Vec<_> = (1..=100).map(|i| issue_json(i, "open", &[], false)).collect();
        let (url, server) = mock_pages(vec![(200,serde_json::to_string(&values).unwrap()), (429, "".into())]);
        let client = reqwest::Client::builder().no_proxy().build().unwrap();
        let result = tauri::async_runtime::block_on(fetch_pages(&client, &url, 42, "synthetic", &RateCooldown::default()));
        server.join().unwrap();
        assert!(matches!(result, Err(error) if error.contains("レート制限")));
    }

    #[test]
    fn short_page_with_next_still_fetches_all_pages() {
        let first = serde_json::json!([issue_json(1, "open", &[], false)]).to_string();
        let second = serde_json::json!([issue_json(2, "open", &[], false)]).to_string();
        let (url, server) = mock_pages(vec![(200, first), (200, second)]);
        let client = reqwest::Client::builder().no_proxy().build().unwrap();
        let result = tauri::async_runtime::block_on(fetch_pages(&client, &url, 42, "synthetic", &RateCooldown::default())).unwrap();
        server.join().unwrap();
        assert_eq!(result.len(), 2);
    }

    #[test]
    fn follows_repository_id_cursor_without_reconstructing_it() {
        use std::{io::{Read, Write}, net::TcpListener, thread};
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let address = listener.local_addr().unwrap();
        let server = thread::spawn(move || {
            for index in 0..2 {
                let (mut stream, _) = listener.accept().unwrap();
                let mut buffer = [0; 4096];
                let len = stream.read(&mut buffer).unwrap();
                if index == 1 {
                    let line = String::from_utf8_lossy(&buffer[..len]);
                    assert!(line.lines().next().unwrap_or_default().contains("/repositories/42/issues?state=all&per_page=100&page=2&after=opaque%2Bcursor"));
                }
                let body = serde_json::json!([issue_json(index + 1, "open", &[], false)]).to_string();
                let link = if index == 0 { format!("Link: <http://{address}/repositories/42/issues?state=all&per_page=100&page=2&after=opaque%2Bcursor>; rel=\"next\"\r\n") } else { String::new() };
                write!(stream, "HTTP/1.1 200 OK\r\n{link}Content-Length: {}\r\nConnection: close\r\n\r\n{}", body.len(), body).unwrap();
            }
        });
        let client = reqwest::Client::builder().no_proxy().build().unwrap();
        let result = tauri::async_runtime::block_on(fetch_pages(&client, &format!("http://{address}/issues"), 42, "synthetic", &RateCooldown::default())).unwrap();
        server.join().unwrap();
        assert_eq!(result.len(), 2);
    }

    #[test]
    fn duplicate_page_link_is_an_error() {
        use std::{io::{Read, Write}, net::TcpListener, thread};
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let address = listener.local_addr().unwrap();
        let server = thread::spawn(move || {
            let (mut stream, _) = listener.accept().unwrap();
            let mut buffer = [0; 4096];
            let _ = stream.read(&mut buffer).unwrap();
            write!(stream, "HTTP/1.1 200 OK\r\nLink: <http://{address}/issues?state=all&per_page=100&page=1>; rel=\"next\"\r\nContent-Length: 2\r\nConnection: close\r\n\r\n[]").unwrap();
        });
        let client = reqwest::Client::builder().no_proxy().build().unwrap();
        let result = tauri::async_runtime::block_on(fetch_pages(&client, &format!("http://{address}/issues"), 42, "synthetic", &RateCooldown::default()));
        server.join().unwrap();
        assert!(matches!(result, Err(error) if error.contains("繰り返")));
    }

    #[test]
    #[ignore = "contacts a public GitHub repository"]
    fn public_github_next_link_smoke() {
        tauri::async_runtime::block_on(async {
            let client = reqwest::Client::builder().user_agent("github-task-widget-test").timeout(Duration::from_secs(20)).redirect(reqwest::redirect::Policy::none()).build().unwrap();
            let endpoint = "https://api.github.com/repos/octocat/Spoon-Knife/issues";
            let first = client.get(endpoint).query(&[("state", "all"), ("per_page", "100"), ("page", "1")]).send().await.unwrap();
            assert!(first.status().is_success(), "public first-page request failed");
            let next = next_page(first.headers(), endpoint, 1300192).unwrap().expect("public repository returned no next link");
            let second = client.get(next).send().await.unwrap();
            assert!(second.status().is_success(), "public second-page request failed");
            let _: Vec<ApiIssue> = second.json().await.unwrap();
        });
    }

    #[test]
    #[ignore = "contacts public GitHub with an invalid synthetic token"]
    fn public_github_rejects_invalid_synthetic_token() {
        tauri::async_runtime::block_on(async {
            let client = reqwest::Client::builder().user_agent("github-task-widget-test")
                .timeout(Duration::from_secs(20)).redirect(reqwest::redirect::Policy::none())
                .retry(reqwest::retry::never()).build().unwrap();
            let synthetic = "invalid-github-issue-widget-synthetic";
            let error = verify_connection(&client, "https://api.github.com/repos/octocat/Spoon-Knife", synthetic, &RateCooldown::default())
                .await.expect_err("invalid synthetic token was accepted");
            assert!(error.contains("HTTP 401"), "unexpected response to invalid synthetic token");
            assert!(!error.contains(synthetic), "synthetic token was returned in the error");
        });
    }

    #[test]
    #[ignore = "reads a specified GitHub repository with a supplied token; never writes"]
    fn authenticated_github_readonly_repository_and_issues() {
        let owner = std::env::var("GIW_READONLY_OWNER").expect("read-only owner is required");
        let repo = std::env::var("GIW_READONLY_REPO").expect("read-only repository is required");
        let token = std::env::var("GIW_READONLY_TOKEN").expect("read-only token is required");
        assert!(valid_component(&owner) && valid_component(&repo), "invalid repository identifier");
        assert!(!token.is_empty(), "read-only token is empty");
        tauri::async_runtime::block_on(async {
            let client = reqwest::Client::builder().user_agent("github-task-widget")
                .timeout(Duration::from_secs(20)).redirect(reqwest::redirect::Policy::none())
                .build().expect("HTTP client setup failed");
            let endpoint = format!("https://api.github.com/repos/{owner}/{repo}");
            let url = reqwest::Url::parse(&endpoint).expect("invalid repository URL");
            let cooldown = RateCooldown::default();
            let response = get_api_response(&client, url, &token, &cooldown).await;
            assert!(response.is_ok(), "repository read failed");
            let repository: Result<ApiRepository, _> = response.unwrap().json().await;
            assert!(repository.is_ok(), "repository response invalid");
            let repository = repository.unwrap();
            assert!(repository.id != 0 && repository.full_name.eq_ignore_ascii_case(&format!("{owner}/{repo}")), "repository identity mismatch");
            let issues = fetch_pages(&client, &format!("{endpoint}/issues"), repository.id, &token, &cooldown).await;
            assert!(issues.is_ok(), "issue read failed");
            println!("repository_and_issues_read_ok count={}", issues.unwrap().len());
        });
    }

    #[test]
    fn next_page_cannot_change_host_or_repository() {
        let endpoint = "https://api.github.com/repos/a/b/issues";
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(reqwest::header::LINK, "<https://example.com/repos/a/b/issues?state=all&per_page=100&page=2>; rel=\"next\"".parse().unwrap());
        assert!(next_page(&headers, endpoint, 42).is_err());
        headers.insert(reqwest::header::LINK, "<https://api.github.com/repositories/43/issues?state=all&per_page=100&page=2>; rel=\"next\"".parse().unwrap());
        assert!(next_page(&headers, endpoint, 42).is_err());
    }

    #[test]
    fn quoted_parameters_do_not_forge_a_next_relation() {
        let endpoint = "https://api.github.com/repos/a/b/issues";
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(reqwest::header::LINK, "<https://api.github.com/repositories/42/issues?state=all&per_page=100&page=2&after=abc>; title=\"text; rel=next, text\"; rel=\"last\"".parse().unwrap());
        assert!(next_page(&headers, endpoint, 42).unwrap().is_none());
        headers.insert(reqwest::header::LINK, "<https://api.github.com/repositories/42/issues?state=all&per_page=100&page=2&after=abc>; title=\"x, y\"; rel=\"next\"".parse().unwrap());
        assert!(next_page(&headers, endpoint, 42).unwrap().is_some());
    }

    #[test]
    fn secondary_limit_403_starts_cooldown_before_any_retry() {
        let body = serde_json::json!({"message": "You have exceeded a secondary rate limit"}).to_string();
        let (url, server) = mock_pages(vec![(403, body)]);
        let client = reqwest::Client::builder().no_proxy().timeout(Duration::from_secs(2)).build().unwrap();
        let cooldown = RateCooldown::default();
        let first = tauri::async_runtime::block_on(fetch_pages(&client, &url, 42, "synthetic", &cooldown));
        server.join().unwrap();
        assert!(matches!(first, Err(error) if error.contains("レート制限")));
        assert!(cooldown.0.load(Ordering::Relaxed) >= now_seconds() + 59);
        let second = tauri::async_runtime::block_on(fetch_pages(&client, &url, 42, "synthetic", &cooldown));
        assert!(matches!(second, Err(error) if error.contains("レート制限中")));
    }

    #[test]
    fn ordinary_permission_error_does_not_start_cooldown() {
        let body = serde_json::json!({"message": "Resource not accessible by integration"}).to_string();
        let (url, server) = mock_pages(vec![(403, body)]);
        let client = reqwest::Client::builder().no_proxy().build().unwrap();
        let cooldown = RateCooldown::default();
        let result = tauri::async_runtime::block_on(fetch_pages(&client, &url, 42, "synthetic", &cooldown));
        server.join().unwrap();
        assert!(matches!(result, Err(error) if error.contains("HTTP 403")));
        assert_eq!(cooldown.0.load(Ordering::Relaxed), 0);
    }

    #[test]
    fn rate_limit_waits_for_retry_after_and_primary_reset() {
        assert_eq!(rate_deadline(1_000, Some(120), None, false), 1_120);
        assert_eq!(rate_deadline(1_000, Some(10), Some(1_500), true), 1_500);
        assert_eq!(rate_deadline(1_000, None, None, false), 1_060);
    }

    #[test]
    fn retry_after_header_controls_cooldown() {
        use std::{io::{Read, Write}, net::TcpListener, thread};
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let address = listener.local_addr().unwrap();
        let server = thread::spawn(move || {
            let (mut stream, _) = listener.accept().unwrap();
            let mut buffer = [0; 4096];
            let _ = stream.read(&mut buffer).unwrap();
            write!(stream, "HTTP/1.1 403 Forbidden\r\nRetry-After: 120\r\nContent-Length: 2\r\nConnection: close\r\n\r\n{{}}").unwrap();
        });
        let client = reqwest::Client::builder().no_proxy().build().unwrap();
        let cooldown = RateCooldown::default();
        let result = tauri::async_runtime::block_on(fetch_pages(&client, &format!("http://{address}/issues"), 42, "synthetic", &cooldown));
        server.join().unwrap();
        assert!(matches!(result, Err(error) if error.contains("レート制限")));
        assert!(cooldown.0.load(Ordering::Relaxed) >= now_seconds() + 119);
    }

    #[test]
    fn connection_rate_limit_blocks_later_issue_fetch() {
        use std::{io::{Read, Write}, net::TcpListener, thread};
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let address = listener.local_addr().unwrap();
        let server = thread::spawn(move || {
            let (mut stream, _) = listener.accept().unwrap();
            let mut buffer = [0; 4096];
            let _ = stream.read(&mut buffer).unwrap();
            write!(stream, "HTTP/1.1 429 Too Many Requests\r\nRetry-After: 120\r\nContent-Length: 0\r\nConnection: close\r\n\r\n").unwrap();
        });
        let client = reqwest::Client::builder().no_proxy().build().unwrap();
        let cooldown = RateCooldown::default();
        let url = format!("http://{address}/issues");
        let first = tauri::async_runtime::block_on(verify_connection(&client, &url, "synthetic", &cooldown));
        server.join().unwrap();
        assert!(matches!(first, Err(error) if error.contains("レート制限")));
        let second = tauri::async_runtime::block_on(fetch_pages(&client, &url, 42, "synthetic", &cooldown));
        assert!(matches!(second, Err(error) if error.contains("レート制限中")));
    }

    #[test]
    fn issue_rate_limit_blocks_later_connection_check() {
        let (url, server) = mock_pages(vec![(429, String::new())]);
        let client = reqwest::Client::builder().no_proxy().build().unwrap();
        let cooldown = RateCooldown::default();
        let first = tauri::async_runtime::block_on(fetch_pages(&client, &url, 42, "synthetic", &cooldown));
        server.join().unwrap();
        assert!(first.is_err());
        let second = tauri::async_runtime::block_on(verify_connection(&client, &url, "synthetic", &cooldown));
        assert!(matches!(second, Err(error) if error.contains("レート制限中")));
    }

    #[test]
    fn validates_repository_components() {
        for valid in ["octocat", "hello-world", "repo.name", "a_b"] { assert!(valid_component(valid)); }
        for invalid in ["", ".", "..", "a/b", "a?b", "a b", "日本語"] { assert!(!valid_component(invalid)); }
    }

    #[test]
    fn replaces_settings_without_saving_a_token() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("settings.json");
        write_settings_at(&path, &Settings { owner: "one".into(), repo: "first".into(), credential_id: "id1".into() }).unwrap();
        write_settings_at(&path, &Settings { owner: "two".into(), repo: "second".into(), credential_id: "id2".into() }).unwrap();
        let data = fs::read_to_string(path).unwrap();
        let value: serde_json::Value = serde_json::from_str(&data).unwrap();
        assert_eq!(value, serde_json::json!({"owner": "two", "repo": "second", "credential_id": "id2"}));
    }

    #[test]
    fn separate_handles_cannot_mutate_configuration_at_once() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("settings.lock");
        let first = process_lock_at(&path).unwrap();
        assert!(process_lock_at(&path).is_err());
        drop(first);
        assert!(process_lock_at(&path).is_ok());
    }

    #[test]
    #[ignore = "helper launched by the separate-process lock test"]
    fn lock_child() {
        let path = std::env::var_os("WIDGET_LOCK_TEST_PATH").expect("missing test path");
        let expect_locked = std::env::var_os("WIDGET_LOCK_EXPECT_BUSY").is_some();
        assert_eq!(process_lock_at(Path::new(&path)).is_err(), expect_locked);
    }

    #[test]
    fn lock_excludes_an_independent_process() {
        use std::process::Command;
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("settings.lock");
        let first = process_lock_at(&path).unwrap();
        let run_child = |busy: bool| {
            let mut command = Command::new(std::env::current_exe().unwrap());
            command.args(["--exact", "tests::lock_child", "--ignored"])
                .env("WIDGET_LOCK_TEST_PATH", &path);
            if busy { command.env("WIDGET_LOCK_EXPECT_BUSY", "1"); }
            else { command.env_remove("WIDGET_LOCK_EXPECT_BUSY"); }
            command.output().unwrap().status.success()
        };
        assert!(run_child(true));
        drop(first);
        assert!(run_child(false));
    }

    #[test]
    #[ignore = "helper launched by the shared rate limit test"]
    fn rate_child() {
        let mode = std::env::var("WIDGET_RATE_TEST_MODE").unwrap();
        let cooldown = RateCooldown(AtomicU64::new(0), true);
        let _guard = process_lock().unwrap();
        if mode == "write" { record_cooldown(&cooldown, now_seconds() + 120).unwrap(); }
        else { assert!(check_cooldown(&cooldown).is_err()); }
    }

    #[test]
    fn rate_limit_deadline_is_shared_across_processes() {
        use std::process::Command;
        let dir = tempfile::tempdir().unwrap();
        for mode in ["write", "read"] {
            let result = Command::new(std::env::current_exe().unwrap())
                .args(["--exact", "tests::rate_child", "--ignored"])
                .env("APPDATA", dir.path())
                .env("WIDGET_RATE_TEST_MODE", mode)
                .output().unwrap();
            assert!(result.status.success(), "rate child failed");
        }
    }

    #[test]
    fn failed_settings_commit_keeps_previous_credential() {
        use std::{cell::RefCell, collections::HashMap};
        let credentials = RefCell::new(HashMap::from([("old".to_owned(), "old-secret".to_owned())]));
        let candidate = Settings { owner: "new".into(), repo: "repo".into(), credential_id: "new".into() };
        let result = commit_settings(&candidate, Some("new-secret"), "old",
            |id, secret| { credentials.borrow_mut().insert(id.into(), secret.into()); Ok(()) },
            |_| Err("simulated settings failure".into()),
            |_| Ok(()),
            || { credentials.borrow_mut().remove("new"); Ok(()) });
        assert!(result.is_err());
        assert_eq!(credentials.borrow().get("old").map(String::as_str), Some("old-secret"));
        assert!(!credentials.borrow().contains_key("new"));
    }

    #[test]
    fn successful_commit_switches_reference_before_old_secret_is_removed() {
        use std::cell::RefCell;
        let events = RefCell::new(Vec::new());
        let candidate = Settings { owner: "new".into(), repo: "repo".into(), credential_id: "new".into() };
        commit_settings(&candidate, Some("new-secret"), "old",
            |_, _| { events.borrow_mut().push("put"); Ok(()) },
            |_| { events.borrow_mut().push("write"); Ok(()) },
            |_| { events.borrow_mut().push("journal"); Ok(()) },
            || { events.borrow_mut().push("cleanup"); Ok(()) }).unwrap();
        assert_eq!(*events.borrow(), vec!["journal", "put", "write", "cleanup"]);
    }

    #[test]
    fn recovery_selects_only_unreferenced_generation() {
        let old = uuid::Uuid::new_v4().to_string();
        let new = uuid::Uuid::new_v4().to_string();
        let recovery = Recovery { old_id: old.clone(), new_id: new.clone() };
        assert_eq!(obsolete_generation(&recovery, &old).unwrap(), new);
        assert_eq!(obsolete_generation(&recovery, &new).unwrap(), old);
        assert!(obsolete_generation(&recovery, "unrelated").is_err());
        assert!(obsolete_generation(&Recovery { old_id: String::new(), new_id: "invalid".into() }, "").is_err());
        assert!(obsolete_generation(&Recovery { old_id: old.clone(), new_id: old.clone() }, &old).is_err());
    }

    #[test]
    fn journal_failure_prevents_writing_credentials() {
        use std::cell::Cell;
        let wrote = Cell::new(false);
        let candidate = Settings { owner: "new".into(), repo: "repo".into(), credential_id: "new".into() };
        assert!(commit_settings(&candidate, Some("synthetic"), "old",
            |_, _| { wrote.set(true); Ok(()) }, |_| Ok(()),
            |_| Err("journal failed".into()), || Ok(())).is_err());
        assert!(!wrote.get());
    }

    #[test]
    fn recovery_survives_deletion_failure_and_can_be_retried() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("recovery.json");
        let old = uuid::Uuid::new_v4().to_string();
        let new = uuid::Uuid::new_v4().to_string();
        write_json_at(&path, &Recovery { old_id: old.clone(), new_id: new.clone() }).unwrap();
        assert!(recover_at(&path, &old, |_| Err("simulated failure".into())).is_err());
        assert!(path.exists());
        let selected = std::cell::RefCell::new(String::new());
        recover_at(&path, &old, |id| { *selected.borrow_mut() = id.into(); Ok(()) }).unwrap();
        assert_eq!(*selected.borrow(), new);
        assert!(!path.exists());
        recover_at(&path, &old, |_| panic!("no entry to delete")).unwrap();
    }

    #[test]
    fn recovery_handles_interruption_on_both_sides_of_commit() {
        for committed in [false, true] {
            let dir = tempfile::tempdir().unwrap();
            let path = dir.path().join("recovery.json");
            let old = uuid::Uuid::new_v4().to_string();
            let new = uuid::Uuid::new_v4().to_string();
            write_json_at(&path, &Recovery { old_id: old.clone(), new_id: new.clone() }).unwrap();
            let current = if committed { &new } else { &old };
            let selected = std::cell::RefCell::new(String::new());
            recover_at(&path, current, |id| { *selected.borrow_mut() = id.into(); Ok(()) }).unwrap();
            assert_eq!(*selected.borrow(), if committed { old } else { new });
        }
    }

    #[test]
    fn http_statuses_do_not_leak_token_in_errors() {
        use std::{io::{Read, Write}, net::TcpListener, thread};
        let synthetic = "synthetic-secret-not-for-production";
        let client = reqwest::Client::builder().timeout(Duration::from_secs(3)).redirect(reqwest::redirect::Policy::none()).no_proxy().build().unwrap();
        for status in [200, 401, 403, 404, 429, 500] {
            let listener = TcpListener::bind("127.0.0.1:0").unwrap();
            let address = listener.local_addr().unwrap();
            let server = thread::spawn(move || {
                let (mut socket, _) = listener.accept().unwrap();
                let mut request = [0; 4096];
                let _ = socket.read(&mut request).unwrap();
                write!(socket, "HTTP/1.1 {status} Test\r\nContent-Length: 0\r\nConnection: close\r\n\r\n").unwrap();
            });
            let result = tauri::async_runtime::block_on(verify_connection(&client, &format!("http://{address}/"), synthetic, &RateCooldown::default()));
            server.join().unwrap();
            if status == 200 { assert!(result.is_ok()); } else {
                let error = result.unwrap_err();
                assert!(error.contains(&status.to_string()) || (status == 429 && error.contains("レート制限")));
                assert!(!error.contains(synthetic));
            }
        }
    }

    #[test]
    fn api_error_bodies_do_not_leak_tokens_or_issue_titles() {
        let synthetic_token = "synthetic-secret-not-for-production";
        let synthetic_title = "synthetic-private-issue-title";
        let client = reqwest::Client::builder().timeout(Duration::from_secs(3)).redirect(reqwest::redirect::Policy::none()).no_proxy().build().unwrap();
        for (status, message) in [(401, "bad credentials"), (403, "resource not accessible"), (403, "secondary rate limit"), (404, "not found"), (429, "too many requests"), (500, "server error")] {
            let body = serde_json::json!({"message": format!("{message} {synthetic_token} {synthetic_title}")}).to_string();
            let (url, server) = mock_pages(vec![(status, body)]);
            let result = tauri::async_runtime::block_on(fetch_pages(&client, &url, 42, synthetic_token, &RateCooldown::default()));
            server.join().unwrap();
            match result {
                Err(error) => {
                    assert!(!error.contains(synthetic_token));
                    assert!(!error.contains(synthetic_title));
                    if status == 403 && message == "resource not accessible" { assert!(error.contains("HTTP 403")); }
                    if status == 403 && message == "secondary rate limit" { assert!(error.contains("レート制限")); }
                }
                Ok(_) => panic!("error response was accepted"),
            }
        }
    }

    #[test]
    #[ignore = "writes a unique synthetic Windows Credential Manager entry"]
    fn isolated_windows_credential_lifecycle() {
        let service = format!("github-task-widget-test-{}", uuid::Uuid::new_v4());
        let entry = keyring::Entry::new(&service, "synthetic").unwrap();
        let first = "synthetic-value-one";
        let second = "synthetic-value-two";
        let outcome = (|| -> Result<bool, keyring::Error> {
            entry.set_password(first)?;
            if entry.get_password()? != first { return Ok(false); }
            entry.set_password(second)?;
            Ok(entry.get_password()? == second)
        })();
        let cleanup = entry.delete_credential();
        assert!(cleanup.is_ok(), "Synthetic test entry cleanup failed for service {service}");
        assert!(matches!(outcome, Ok(true)), "Synthetic credential lifecycle failed");
        assert!(matches!(entry.get_password(), Err(keyring::Error::NoEntry)));
    }
}

fn valid_component(s: &str) -> bool {
    !s.is_empty() && s.len() <= 100 && s != "." && s != ".." &&
        s.bytes().all(|b| b.is_ascii_alphanumeric() || b"-_.".contains(&b))
}

async fn verify_connection(client: &reqwest::Client, url: &str, token: &str, cooldown: &RateCooldown) -> Result<(), String> {
    let url = reqwest::Url::parse(url).map_err(|_| "接続先 URL が無効です。")?;
    get_api_response_locked(client, url, token, cooldown).await.map(|_| ())
}

#[tauri::command]
async fn connection_info(lock: State<'_, ConnectionLock>) -> Result<ConnectionInfo, String> {
    let mut guard = lock.0.lock().await;
    let _process_guard = process_lock()?;
    recover_pending()?;
    let settings = read_settings()?;
    let token = read_token(settings.as_ref().map(|value| value.credential_id.as_str()).unwrap_or(""))?;
    Ok(refresh_connection_snapshot(&mut guard, settings, token.as_deref()))
}

#[tauri::command]
async fn save_connection(owner: String, repo: String, token: String, expected_revision: String, lock: State<'_, ConnectionLock>, cooldown: State<'_, RateCooldown>) -> Result<ConnectionInfo, String> {
    let mut guard = lock.0.lock().await;
    let _process_guard = process_lock()?;
    recover_pending()?;
    let owner = owner.trim().to_owned();
    let repo = repo.trim().to_owned();
    if !valid_component(&owner) || !valid_component(&repo) { return Err("オーナー名またはリポジトリ名が無効です。".into()); }
    let old_settings = read_settings()?;
    let old_id = old_settings.as_ref().map(|s| s.credential_id.as_str()).unwrap_or("");
    let old_token = read_token(old_id)?;
    validate_connection_revision(guard.as_ref(), &expected_revision, &old_settings, old_token.as_deref())?;
    let supplied = !token.trim().is_empty();
    let active_token = if supplied { token.trim() } else { old_token.as_deref().ok_or("トークンを入力してください。")? };
    let client = reqwest::Client::builder().timeout(Duration::from_secs(20)).redirect(reqwest::redirect::Policy::none()).user_agent("github-task-widget").build().map_err(|_| "通信の準備に失敗しました。")?;
    let url = format!("https://api.github.com/repos/{owner}/{repo}");
    verify_connection(&client, &url, active_token, &cooldown).await?;
    let new_id = if supplied { uuid::Uuid::new_v4().to_string() } else { old_id.to_owned() };
    let settings = Settings { owner, repo, credential_id: new_id.clone() };
    commit_settings(&settings, if supplied { Some(active_token) } else { None }, old_id,
        |id, value| entry(id)?.set_password(value).map_err(|_| "トークンを保存できませんでした。".into()),
        write_settings,
        |journal| write_json_at(&recovery_path()?, journal),
        recover_pending)?;
    let (info, snapshot) = capture_connection(Some(settings), Some(active_token));
    *guard = Some(snapshot);
    Ok(info)
}

#[tauri::command]
async fn delete_token(expected_revision: String, lock: State<'_, ConnectionLock>) -> Result<ConnectionInfo, String> {
    let mut guard = lock.0.lock().await;
    let _process_guard = process_lock()?;
    recover_pending()?;
    let settings = read_settings()?;
    let token = read_token(settings.as_ref().map(|value| value.credential_id.as_str()).unwrap_or(""))?;
    validate_connection_revision(guard.as_ref(), &expected_revision, &settings, token.as_deref())?;
    if let Some(value) = &settings {
        if !value.credential_id.is_empty() {
            match entry(&value.credential_id)?.delete_credential() {
                Ok(()) | Err(keyring::Error::NoEntry) => (),
                Err(_) => return Err("トークンを削除できませんでした。".into()),
            }
        }
    }
    let (info, snapshot) = capture_connection(settings, None);
    *guard = Some(snapshot);
    Ok(info)
}

fn main() {
    tauri::Builder::default()
        .manage(ConnectionLock::default())
        .manage(RateCooldown(AtomicU64::new(0), true))
        .invoke_handler(tauri::generate_handler![connection_info, save_connection, delete_token, list_issues, create_issue, pending_creation, acknowledge_creation, move_issue])
        .plugin(tauri_plugin_opener::init())
        .run(tauri::generate_context!())
        .expect("failed to run widget");
}
