# Windows 資格情報の合成ライフサイクル検証依頼

- 担当予定 `/root/windows_credential_test`。親 `/root` が ID を照合して起動。共有 run `D:/Projects/GithubIssueWidget/docs/agent-runs/20261121-app-completion-restart/`、作業場所 `D:/Projects/GithubIssueWidget`。
- 背景: Issue #3 は Windows Credential Manager の保存・更新・削除を要求。対象 `widget/src-tauri/src/main.rs` SHA256 `AEFF1E65AD93EFA04773CEBBB9736A6BDC5B9938DB9FAF937DD88B3D9CBFE86F` の `isolated_windows_credential_lifecycle` は一意な UUID 名の合成 service、固定の合成値を使い、保存・読取・更新・削除後の不存在を確認する ignored test。該当コードと対象版を自身で確認し、過去の現行版成功証拠を照合する。ユーザーの本番 PAT や設定は取得・表示・変更しない。
- 実行前に安全性を確認: 実キーの service がアプリの `github-task-widget` と異なる一意名であり、固有の合成エントリだけを作成・削除すること、同じ test 名を狭く指定できること、失敗・中断時に実エントリが残る可能性と確認方法、監督子プロセス・終了条件とログを把握する。条件が成立しない、削除先を識別できない、別試験が走る等なら実行せず報告。実資格情報・既存 service・実アプリ・GitHub・既存設定を触らない。
- 条件が成立すれば新規 `windows-credential-test-sandbox-01/` の専用入力・target・ログで `--locked --offline` を使い該当 ignored test のみ実行。対象版・コマンド・出力・終了状態・生成した合成 service の識別と削除の証拠を記録し、秘匿情報は保存しない。異常終了・削除不明時は推測で他資格情報を操作せず停止し、具体的な識別情報と再開条件を報告する。自動的な再実行をしない。合成エントリによる実 OS API 確認と実アプリの PAT 保存・復元や WebView の検証を区別する。
- 新規 `windows-credential-test-result-01.md` と専用領域のみ所有。他者の変更を戻さない。対象 SHA、既存証拠、事前安全確認、結果・終了コード・未確認・残る物を記録。UTF-8 実データを読戻し一時ファイルから確定し、パスのみ通知。同種失敗2回は当該作業を停止し親へ報告。結果は独立レビューが必要。
