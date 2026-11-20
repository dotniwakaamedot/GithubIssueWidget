use glob::Pattern;
use serde_json::Value;

#[test]
fn opener_url_scope_allows_only_expected_github_urls() {
    let capability: Value = serde_json::from_str(include_str!("../capabilities/default.json")).unwrap();
    let opener = capability["permissions"].as_array().unwrap().iter()
        .find(|entry| entry["identifier"] == "opener:allow-open-url")
        .expect("missing opener URL permission");
    let patterns: Vec<_> = opener["allow"].as_array().unwrap().iter()
        .map(|entry| Pattern::new(entry["url"].as_str().expect("URL scope must be a string")).unwrap())
        .collect();
    let allowed = |url: &str| patterns.iter().any(|pattern| pattern.matches(url));

    for url in ["https://github.com/", "https://github.com/example/repo/issues/123"] {
        assert!(allowed(url), "expected allowed URL: {url}");
    }
    for url in [
        "http://github.com/example/repo/issues/123",
        "https://github.com.evil.example/",
        "https://github.com@evil.example/",
        "https://github.com:443/example/repo/issues/123",
        "https://evil.example/",
        "file:///github.com/example/repo",
    ] {
        assert!(!allowed(url), "expected rejected URL: {url}");
    }
}

