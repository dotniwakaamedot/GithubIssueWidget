# release GUI 空設定観測の独立レビュー依頼

- 担当 `/root/plan_review`、作者 `/root/gui_failure_investigation` と別担当。共有 run `D:/Projects/GithubIssueWidget/docs/agent-runs/20261121-orchestration-continuation/`。
- `gui-release-observation-preflight-01.md` SHA `8F17A28BDA7B98121F63F03C3B9835BE7B150EE850EF03400C5ADD020876C20E`、`gui-release-observation-01/observe.py` SHA `F82AD7CADEC4F94B1E7832A32A701C0CE59CEE9E713AAD1833834FC839F48CF7`、対象 EXE `23C383EB2E46CB205E052A9D4A24EE73B968EB2C776F7150826350685419B306` を開始・終了時に照合。
- 空設定の前提、固定 EXE・直接子の識別、起動失敗/識別不能・閉鎖と終了、秘密・既存状態・不明な旧プロセスの保護を具体的に確認。GUI を起動せず、実測可能な保証と未知の子孫や WebView2・OS credential 共有の未確認を区別。実行を妨げる問題があれば最小の具体修正を示す。
- `gui-release-observation-review-01.md` に blocking と未確認を確定し通知。実際の表示・製品完成は承認しない。
