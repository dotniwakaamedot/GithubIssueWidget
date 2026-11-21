# 未完了の実画面要件の安全な確認手順調査

- 担当予定 `/root/runtime_gap_procedure`。親 `/root` が ID を確認して起動。共有 run `D:/Projects/GithubIssueWidget/docs/agent-runs/20261121-app-completion-restart/`、作業場所 `D:/Projects/GithubIssueWidget`。共有環境の他者変更を戻さない。
- `completion-audit-01.md`、Issue #1・#2・#6・#7 原文、ユーザー報告 `user-observation-07.md`〜`10.md`、`issue-11-state-04.md`、同期確認の追記、現在のソースと実行ファイル・README を照合。既存証拠がどの版・要件を満たすかを特定し、本人に既に試してもらった操作を不要に再依頼しない。
- 実画面の初期・最小寸法、中央・最前面、最新 EXE のウィンドウ移動・リサイズ・許可 URL、許可しない URL の実挙動、非 status ラベル保持などについて、不足を安全かつ最小の本人操作または独立検証で確認する具体的手順を提案。実画面・GitHub 書込みが必要な場合は既存承諾の #11 以外を操作しない方針、秘密の非出力、ラベル追加や復元の具体的副作用、停止条件と証拠の限界を明示。API 履歴や静的設定から実 UI を推測しない。
- 読取りのみ。アプリ、Issue、設定、資格情報、既存文書を変更しない。新規 `runtime-gap-procedure-01.md` のみ所有し、対象版・根拠・未確認・次の安全な作業を記載。UTF-8 読戻し後に一時ファイルから確定しパスのみ通知。同種失敗2回は停止・報告。
