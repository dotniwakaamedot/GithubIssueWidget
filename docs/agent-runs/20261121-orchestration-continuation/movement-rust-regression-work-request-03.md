# 競合時の部分反映通知とテスト修正依頼

- 担当 `/root/existing_work_review`。共有 run `D:/Projects/GithubIssueWidget/docs/agent-runs/20261121-orchestration-continuation/`、所有は既存 `movement-rust-isolated-02/movement.rs` と新版結果 `movement-rust-regression-result-03.md` のみ。製品 `movement.rs` や他担当の作業を変更しない。
- `movement-rust-requirement-review-01.md` は Issue #6 に部分反映の語の明文指定はない一方、競合通知を明確にする計画の具体条件に現状が不足すると指摘。隔離版 SHA `CB3C9F5CBB2EE0A37AEB35E9850B1DDEC74E28CC9D52FB9B981031450F78BD9C` と製品 SHA `ED49F4BA52C6887BCE427E4A6664D335092A1C3311E1B1ED61B4CE322791ABA5` を開始前に確認、不一致なら中断。
- 最終不一致時の既存の不一致・最新状態確認を保ちつつ、先行書込みが反映済みの**可能性**を誤解なく通知する小規模修正を隔離版に実装。書込み済みと断定しない。新規3テストは単独文字 `一部` でなく実際に採用した意味を表す明確な警告文句を検証し、最終不一致・要求順も保つ。既存と重なる事例を明示し、追加テストの過度な反復を避ける。
- テスト実行は実行条件・固定版の独立確認後。製品・既存 target・実 GitHub・credential・GUI を変更/操作しない。元と成果の SHA、差分・静的検査と未実行・未確認を結果に記録し、書込み停止後パスを通知する。
