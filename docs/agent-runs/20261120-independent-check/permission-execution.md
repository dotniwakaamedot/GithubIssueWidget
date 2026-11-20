# 権限不足と隔離競合の再検証

- 対象 `widget/src-tauri/src/main.rs` SHA256 `5DE67CA065A45C3637B4DD0755D6ED9846E57C903ED93099214E9C03A16C0022`。前版から、合成データを含む通常の権限不足 403 と二次レート制限 403 を区別して検証するケースを追加した。
- Windows x64 開発環境、作業場所 `D:/Projects/GithubIssueWidget/widget/src-tauri`。`cargo test --locked` は終了コード0、49 passed / 6 ignored。`%TEMP%/githubissuewidget-permission-tests-20261120.log` SHA256 `F5518D054C374393F4D39EBD0B1AE00B94AC84DC2788B932100756D2A42A1003`。
- 続けて `cargo test --locked -- --exact tests::windows_inflight_guard --ignored --test-threads=1 --nocapture` は終了コード0、1 passed。通常・旧応答拒否と合成資格情報の削除確認を示す各 stage をログに記録した。`%TEMP%/githubissuewidget-permission-isolated-20261120.log` SHA256 `027C56517F1E86A80AECCE148222222F134DFCC0BC79D2B283759C5C3CFBF656`。
- テスト実行ファイル `widget/src-tauri/target/debug/deps/github_issue_widget-263288f3cb360ecd.exe` SHA256 `1A4DD972A85C5135286313427436B8B7320E9E4DDAEDFB60FF41E056226B5339`。合成値とローカル通信を使用し、実トークンや実 GitHub への書込みは行っていない。
- 実 WebView と Rust の結合、実 GitHub の権限・レート制限、設定画面からの実 token 更新や、最終照合後の他プロセス変更はこの試験の対象外。
