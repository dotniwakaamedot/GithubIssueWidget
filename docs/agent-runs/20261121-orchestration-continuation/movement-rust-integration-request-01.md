# GitHub 競合時の部分反映通知の限定統合

- 担当 `/root/existing_work_review`。共有 run `D:/Projects/GithubIssueWidget/docs/agent-runs/20261121-orchestration-continuation/`、作業場所 `D:/Projects/GithubIssueWidget`。所有は製品 `widget/src-tauri/src/movement.rs` と結果 `movement-rust-integration-result-01.md` のみ。他担当と既存未コミット変更・新規 pagination テスト等を保護する。
- `movement-rust-review-02.md` の静的承認と `movement-rust-rebuild-result-review-01.md` の固定版テスト承認を開始前に確認する。製品 movement SHA `ED49F4BA52C6887BCE427E4A6664D335092A1C3311E1B1ED61B4CE322791ABA5`、試験済み隔離 movement `88BF21BB45C077DFDA592D906C753D168C5641E35EB12324D4ACF1652374695B`、現製品 main `AEFF1E65AD93EFA04773CEBBB9736A6BDC5B9938DB9FAF937DD88B3D9CBFE86F` と対象所有・現状を照合。不一致や他担当の書込みがあれば変更しない。
- 最終 GET が期待と異なる際、既存の不一致・再取得案内を保ち「更新の一部が反映された可能性があります」と断定せず通知する1箇所の修正と、承認・実行済みの3ケースのみを追加。以前の既存差分・他者の編集を保護し、変更後の実体が試験版 SHA と一致することを照合する。競合を完全に防止できると主張せず、GitHub の再取得・自動再試行しない現行方針を保持する。
- 3件は同一版で成功済みであり無理由に再実行しない。前後 SHA・差分・確認コマンド/結果、未確認を `movement-rust-integration-result-01.md` に記録し書込み停止後にパスを通知。別担当の固定版レビューを要する。実 GitHub・credential・GUI は操作しない。
