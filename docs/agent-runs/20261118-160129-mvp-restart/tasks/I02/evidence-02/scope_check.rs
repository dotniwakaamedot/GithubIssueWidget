extern crate glob;

fn main() {
    let allowed = ["https://github.com/", "https://github.com/**"]
        .iter().map(|s| glob::Pattern::new(s).unwrap()).collect::<Vec<_>>();
    let cases = [
        ("https://github.com/", true),
        ("https://github.com/owner/repo/issues/1", true),
        ("https://github.com.evil.example/", false),
        ("https://github.com@evil.example/", false),
        ("https://evil.example/github.com/", false),
        ("http://github.com/", false),
        ("javascript:alert(1)", false),
        ("file:///C:/example.txt", false),
        ("https://sub.github.com/", false),
    ];
    for (url, expected) in cases {
        let actual = allowed.iter().any(|p| p.matches(url));
        println!("{url} expected={expected} actual={actual}");
        assert_eq!(actual, expected);
    }
}
