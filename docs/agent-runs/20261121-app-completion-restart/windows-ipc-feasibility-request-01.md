# Windows 合成資格情報・IPC 試験の実行可否確認依頼

- 担当予定 `/root/windows_ipc_feasibility`。親 `/root` が ID を照合して起動。共有 run `D:/Projects/GithubIssueWidget/docs/agent-runs/20261121-app-completion-restart/`、作業場所 `D:/Projects/GithubIssueWidget`、基準 HEAD `4961b25bec53a93439acdf5230de77f959908af0`。
- 根拠: 承認計画 `plan-approved-03.md` SHA256 `355CD3D07C9CEAD72576CACC02D31B27484F7C165AEAB967E2FBCC548DD175C1`、Issue #3、`windows-credential-test-result-01.md`、`credential-safety-design-01.md`・`credential-safety-review-01.md`。対象 `widget/src-tauri/src/main.rs` SHA256 `AEFF1E65AD93EFA04773CEBBB9736A6BDC5B9938DB9FAF937DD88B3D9CBFE86F`。開始時に一致を確認し相違時は停止。
- 別担当の報告: 既存 ignored `windows_ipc_guard` は合成 service 識別子を書込み前に stdout flush し、分離 APPDATA と Windows Credential Manager の保存・更新・削除、および mock IPC を扱う可能性がある。報告を事実とせず、正確なテスト名・生成経路・ログ・service と account の全識別子・排他・権限・前提・終了・例外と未確認をコードから独立確認。既存の対象版成功証拠も確認し、再実行が必要か判断する。
- 実行可否の判断では、書込み前の完全識別子の耐久記録と読戻し、既存 service と競合しない所有権、書込み途中・異常終了・監督者停止・ログ保存失敗での再開・残留照合、対象以外を触らないこと、親子孫の識別・終了と秘密非出力を評価する。条件付き作成不能時の隔離や API の性質も推測しない。必要条件が証明できない場合の具体的な停止・再開条件を示す。
- **本依頼は読取り調査のみ**。実資格情報、実 PAT、既存設定、実アプリ・GitHub の書込み・テスト実行はしない。新規 `windows-ipc-feasibility-result-01.md` のみ所有し、対象版・根拠・事実・推測・未検証・後続手順を記録。共有 checkout で他者の変更を戻さない。UTF-8 実データを読戻し一時ファイルから確定後パスのみ通知。出力を限定し同種失敗2回で停止・報告。独立レビューが必要。
