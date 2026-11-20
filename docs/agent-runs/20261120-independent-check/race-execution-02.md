# 隔離競合テスト再確認記録

- 対象 `widget/src-tauri/src/main.rs` SHA256: `D4C7C833AA003967D34B994033B74189EF7F4D42FDE8F910C6A16569EC63799C`。前回の承認対象から、合成トークンと Issue 名を含む API エラー本文の非露出テストを追加した。
- 作業場所 `D:/Projects/GithubIssueWidget/widget/src-tauri` で `cargo test --locked -- --exact tests::windows_inflight_guard --ignored --test-threads=1 --nocapture` を実行し終了コード0、1 passed。通常系、別プロセスによる資格情報変更、合成資格情報の削除・不存在確認をテスト内で実行した。
- ログ `%TEMP%/githubissuewidget-real-lock-recheck-20261120.log` SHA256 `0951D31EB3E603DBF456F0DBCECE0EDA221E0CFEAF2AD641E025F5587AA8EE6F`。テスト実行ファイル SHA256 `FE3E32180DB7371B802D7727F8B4FBFF11AE88F5F791BAC6C96A19D53A35D59E`。fixture と合成 ID はログに記録し、実トークンは使用していない。
- `cargo test --locked` は 49 passed / 6 ignored。新しいエラー本文の非露出テストを含む。画面側は 20 passed だが、WebView と Rust を結合した検証ではない。
- この試験は `list_issues` の Tauri 入口全体、`save_connection`、実 WebView、実 GitHub、最終照合後の別プロセス変更を対象としない。
