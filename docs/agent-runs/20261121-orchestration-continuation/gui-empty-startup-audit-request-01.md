# 現行版の空設定起動経路の調査依頼

- 担当 `/root/gui_failure_investigation`、共有 run `D:/Projects/GithubIssueWidget/docs/agent-runs/20261121-orchestration-continuation/`。所有は `gui-empty-startup-audit-01.md` のみ。他担当の製品・旧 fixture・変更を保護。
- `gui-f078-next-investigation-01.md` と現行フロントエンド・Rust の起動処理を固定 SHA で照合し、新規空 APPDATA/LOCALAPPDATA と未設定時に `connection_info`・`pending_creation`・設定復旧・Credential Manager・ネットワークへ到達する条件と副作用を正確に追う。空設定維持や同一ユーザー競合の前提、環境変数だけでは credential store を隔離できない制約を明記。
- 読取りのみ。GUI・合成子・資格情報・GitHub を起動/操作せず、実データを出力しない。対象版・根拠、実行結果、確定/推測/未確認と安全な次の観測条件を記録して通知。glob は既存 F-012 の教訓を適用し、実在ディレクトリと `rg -g` を使う。
