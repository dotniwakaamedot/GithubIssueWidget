# Issue #6 独立レビュー依頼（第3回）

第2回 `tasks/issue-06-review-20261119-02.md` の blocking 2件を対象版全体とともに再確認。対象・基準・他ファイルの SHA256 は `issue-06-review-request-20261119-02.md` と同じ。`widget/src-tauri/src/movement.rs` の SHA256 は `A099EB5E405F7BA9894BB85A9B60653B572619628E43B161C178E7ECB354FF71`。今回の修正はテストのみ。作者の `cargo test --locked` 46 passed / 4 ignored。実 GitHub 移動と Windows UI は未検証。

模擬サーバー上で GET 応答後に外部ラベルを追加し、PATCH 前後の模擬状態と戻り値を確認しているか、複数 status ラベル除去と非ステータスラベル保持のテストがあるか確認する。繰り返した競合テスト不足の独立調査結果は `tasks/issue-06-race-diagnosis-20261119.md`。結果を `tasks/issue-06-review-20261119-03.md` に記録し通知する。コード・GitHub は変更しない。パスを列挙・存在確認し、未確認パスを検索対象としない。
