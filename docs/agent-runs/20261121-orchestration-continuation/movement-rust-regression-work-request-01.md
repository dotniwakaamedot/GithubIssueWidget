# 移動競合 Rust 回帰テストの隔離作成依頼

- 担当 `/root/process_safety`。共有 run `D:/Projects/GithubIssueWidget/docs/agent-runs/20261121-orchestration-continuation/`。新規 `movement-rust-isolated-01/` と結果 `movement-rust-regression-result-01.md` のみ所有。製品・未コミット main.rs・他担当の作業を保護。
- 承認済み `movement-regression-plan-01.md`、`movement-audit-review-01.md` と Issue #6 を参照。製品 `movement.rs` SHA `ED49F4BA52C6887BCE427E4A6664D335092A1C3311E1B1ED61B4CE322791ABA5`・main.rs `61595B68317E15FDD8F2BED88B920A468D5516740E3D3124710384709C52FC24` を確認。不一致なら中止。
- 既存 `final_status_conflict_is_not_reported_as_success` 等と重複しない、初回 GET 後の他者によるステータス/状態変更と最終結果の不一致を合成する最小ケースを設計し、独立期待値と有限な loopback 終了を実装。既存ケースが既に同等なら追加せず根拠を示す。実 GitHub/credential/UI は使わない。
- まず隔離案と静的確認を記録。実行は具体的な対象名・プロセス/出力の条件を固定しレビュー後に、新規ケースのみ行う。元ファイルと隔離版の SHA、差分、未確認・コマンド結果を記録して書込み停止後に通知。製品変更は別途判断。
