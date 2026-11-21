# Windows 開発起動の安全な確認依頼

- 担当予定 `/root/dev_start_check`。親 `/root` が ID を照合し起動。共有 run `D:/Projects/GithubIssueWidget/docs/agent-runs/20261121-app-completion-restart/`、作業場所 `D:/Projects/GithubIssueWidget`。Issue #2・#7 と承認計画を直接確認する。
- 目的: 現行版で未証明の Windows の開発起動・正常終了について、`widget/package.json`、Tauri 設定、アプリ起動時の設定・Credential Manager 読取と既存の稼働アプリ・ポート・プロセス・失敗記録を確認し、ユーザーの画面・データを妨げない方法と対象版の証拠を特定する。起動手段と実体、子プロセス、作業場所、監督・終了・ログを事前に明確化し、静的確認や Web 開発サーバーのみを Tauri 実起動と扱わない。
- 実行は既存アプリや資格情報を触らず、専用のコピー・出力・ポート・設定・サービスを安全に隔離でき、既存プロセスを止めず識別・終了できることを確認できた場合のみ。実 PAT や Credential Manager の既存 service を取得・変更しない。安全な分離が証明できない、ユーザー画面を奪う、状態不明などの場合は起動せず理由と具体的な再開条件を記録する。Windows の `Start-Process` は非表示、ユーザーに操作してもらう場合は親が明示的に案内する。
- `dev-start-check-result-01.md` と必要な新規 `dev-start-check-sandbox-01/` のみ所有。他者のソース・実行ファイル・既存ログを変更・削除せず、実 GitHub 書込み・設定・資格情報を操作しない。対象版・前提・コマンド・監督終了状態と未確認、同種失敗2回の停止条件を記録。UTF-8 実データ読戻し後に一時ファイルから確定しパスのみ通知する。独立レビューが必要。
