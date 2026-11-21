# 合成資格情報試験の安全設計の独立レビュー依頼

- 担当予定 `/root/credential_safety_review`。親 `/root` が ID を照合し起動。設計担当 `/root/credential_safety_design` と別。共有 run `D:/Projects/GithubIssueWidget/docs/agent-runs/20261121-app-completion-restart/`。
- 対象 `credential-safety-design-01.md` SHA256 `76057E5039F7BA536545B4F0C0247B000B32C24B7C9CD5D6CF51CBEFF789E6ED`、対象コード `widget/src-tauri/src/main.rs` SHA256 `AEFF1E65AD93EFA04773CEBBB9736A6BDC5B9938DB9FAF937DD88B3D9CBFE86F`。Issue #3 の要求・既存 test と `docs/instructions/work.md` の監督対象の安全条件を照合し、所有権・名前の予約・書込み・読み戻し・cleanup・異常終了の前提と停止条件が適切か、実現可能性と過剰な前提を区別して評価する。
- このレビューは設計の読取りに限り、実資格情報・既存 service・設定・アプリ・GitHub を操作しない。新規 `credential-safety-review-01.md` のみ所有。対象版と根拠、blocking、未確認と必要な次作業を記録し、UTF-8 実データ読戻し後に一時ファイルから確定しパスのみ通知。他者の変更を戻さない。出力を絞り、同種失敗2回は停止・報告。
