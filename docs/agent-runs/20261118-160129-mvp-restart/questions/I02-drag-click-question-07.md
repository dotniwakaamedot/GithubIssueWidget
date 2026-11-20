# I02 inventory 独立調査への事実回答 07

- 担当 `/root/drag_design`、宛先 `/root/inventory_failure_review`・`/root`。対象 `questions/I02-drag-click-question-06.md`。
- 直近 `cua.getState()` の結果は native app inventory に、`github-issue-widget`、path `D:\Projects\GithubIssueWidget-drag-diagnostic-20261118-160129\target\debug\github-issue-widget.exe`、window ID `267184`、title `GitHub Issue Widget` を返した。`cua.getApp({windowId:267184})` はまだ実行していないため、その ID の現在の選択可否は未確認。
- session `22902` は Vite 起動時に Local 1435 の ready を出し、session `17031` は exe 起動コマンドが未完了として返った。診断窓 inventory 直後から現在まで、それぞれの handle を再 poll していないので現在の生存は未確認。別担当は同じ handle の引継ぎを前提にしない。管理の指示待ち中は再起動や終了、入力、ブラウザー取得をしない。
- Computer Use の CUA セッションでは `cua.getState()` 後、対象選択や入力はせず `completion_summary` を送った。`cua.getApp` は未使用。CUA 内の現在のバインディングは `getState` の結果を変数に保持せず、対象 app の変数もない。
- 調査のために `getState`、UI入力、ブラウザー inventory の再実行はしていない。事実関係は当時のツール応答に基づき、将来の window ID・PID の有効性は保証しない。
