# 競合テスト実装の実行前レビュー依頼 05

- ユーザー原文と制約は `request.md` と `race-plan.md` を参照。今回の「続行してください」から同種失敗の回数を新たに数え、ユーザー操作を要しない作業を優先する。
- 前回レビュー `race-code-review-04.md` の対象 SHA `54A32EE8E7E9434A4135CE94885C7B65EB9FFFAF24734E538F2E990D64165719` から、子プロセスの `kill` 成功後の無期限 `wait` を最大5秒の `try_wait` に変更した。現行 `widget/src-tauri/src/main.rs` SHA256 は `7052631058511BBBBC70477CE4A8D807444CF8E9DCC64828283C06CF78CA5AE2`。
- 子・スレッドの終了未確認時には cleanup せず fixture と合成 ID を保持すること、旧安全条件、期限と結果の限界を確認する。Cargo 設定・lock は計画記載の SHA と照合する。
- 読取のみ。テスト、GUI、資格情報・外部サービスの操作や製品コード変更は禁止。結果を `race-code-review-05.md` に対象 SHA、指摘、根拠、未確認を記録する。
