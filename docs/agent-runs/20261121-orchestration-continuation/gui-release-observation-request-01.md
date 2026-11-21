# 現行 release の空設定 GUI 観測の準備依頼

- 担当 `/root/gui_failure_investigation`。共有 run `D:/Projects/GithubIssueWidget/docs/agent-runs/20261121-orchestration-continuation/`。`gui-release-plan-01.md`・`gui-empty-startup-audit-01.md` と各レビュー、現行ビルド結果を基礎にする。既存製品、credential、旧 fixture と他担当の作業を保護。
- 実行済みビルド EXE の実体と SHA `23C383EB2E46CB205E052A9D4A24EE73B968EB2C776F7150826350685419B306`、ソースとの対応とアクセスを確認。新規空の専用 APPDATA/LOCALAPPDATA・TEMP と対象 exe の実行場所、最小非機密環境、直接子の保持・期限・同一ウィンドウ識別・通常の閉じる操作・終了コードを固定する。対象ユーザー・対話セッション・WebView2 の確認可能な事実と未確認を区別する。
- 空設定と非操作の前提で、ログを無保護に晒さず直接表示・終了を観察する具体的な実行器/手順を作り SHA とともに記録。旧不明対象・識別不能なプロセスを停止せず、直接子と未知の子孫の未確認を区別。OS credential store の分離を主張しない。正常表示以外の結果を成功と扱わない。
- まず対象と手順を固有の確定結果へ提出し、作者以外の確認を受け管理の開始通知が来るまで起動しない。不要な試験の再実行はしない。実 PAT・実 GitHub・既存設定の操作は対象外。起動後の結果や障害を推測しない。
