# HTML ドラッグ設定修正の独立レビュー依頼

- 担当予定 `/root/drag_fix_review`。親 `/root` が起動。修正作者 `/root/drag_fix` とは別担当。
- 対象 `widget/src-tauri/tauri.conf.json` SHA256 `0132FCC92345F5D60DE84FD25197FCE853044AD63DAE28757AB4592CC72B991E`、基準 SHA `5C1F3A0E2E487DD4345A2DD335FE4AACA483FEEEA26C11AAB99B374278EC1364`、作者の `drag-fix-result-01.md`、調査 `drag-investigation-01.md`。版・差分・Tauri schema とネイティブファイルドロップへの影響範囲を照合する。
- 設定 false は HTML5 drag の候補対策として妥当か、既存設定・要求を破壊しないか確認。実 Windows の解決を静的確認だけで主張しない。ビルド・実機検証は別担当。
- 既存ファイル読取りのみ。結果 `D:/Projects/GithubIssueWidget/docs/agent-runs/20261121-app-completion-restart/drag-fix-review-01.md` を所有し、対象 SHA・blocking・未確認を記載して確定後パスのみ通知。他者の変更を戻さない。
