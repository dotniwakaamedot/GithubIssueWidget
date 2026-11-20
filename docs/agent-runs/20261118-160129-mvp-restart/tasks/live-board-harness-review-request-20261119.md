# 接続・ボードの実 GitHub 読取りテスト補助コードの独立レビュー依頼

対象: `C:/Users/semis/orca/workspaces/GithubIssueWidget/issue-6-move-resume`、HEAD `ab6122ef26bd62d5ec8dffbda1d35cd1a09606e0`。`widget/src-tauri/src/main.rs` の差分は ignored テスト追加のみ、SHA256 `C140670F7DCD568DEF33401EFE5B409973E44347D12F41D8CBC20AA611CCE4BC`。`movement.rs` の差分は別途承認・実施済みのテスト補助コードで本依頼対象外。

ユーザー指定リポジトリ `dotniwakaamedot/orcaDiversion` に対し、実装の接続確認・全ページ取得を読み取り専用で試し、テスト Issue #13 だけの番号・一意タイトル・分類を照合する。秘密と他 Issue の内容を出力・保存しない。通常テスト時の誤実行防止、HTTP が読み取りだけであること、token 保護、接続先、テストが証明する範囲を確認する。`cargo test --locked --no-run` は作者側で成功。テスト実行・GitHub 書込み・対象コード編集をせず、結果を `tasks/live-board-harness-review-20261119.md` に記録し親へ通知する。参照前には存在確認する。
