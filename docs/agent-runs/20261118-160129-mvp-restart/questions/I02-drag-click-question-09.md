# I02 所有セッションの読み取り確認 09

- 担当 `/root/drag_design`、宛先 `/root`・`/root/restart_planner`。`questions/I02-drag-click-answer-08.md` に従い所有 handle を一度ずつ poll し、対象プロセスを読み取り。
- 2026-11-18T18:42:30Z 前後、`write_stdin` による空 poll は dev server `22902`・診断アプリ `17031` の両方で session ID が返り、各5秒で終了していない。観測時点で live、exit code は未取得。出力なしは正常終了の証明ではない。
- cwd `D:\Projects\GithubIssueWidget`、PowerShell の `Get-Process -Name github-issue-widget` と exe path フィルター、`Get-FileHash -Algorithm SHA256` は exit 0。対象 `D:\Projects\GithubIssueWidget-drag-diagnostic-20261118-160129\target\debug\github-issue-widget.exe` に一致する process は1件、PID `21312`、MainWindowHandle `267184`、exe SHA256 `2E7A6582B8F74E3E005788E4A0C5B311BCECCF956119726F4C8760477DCC23C4`。これは対象版依頼のコード SHA と併せて評価する。origin / DPI / 安全領域は今回観測しておらず未確認。
- 同じ画像取得、`cua.getState()`、ブラウザー inventory、画面操作、クリック、ドラッグ、プロセス停止・再起動はいずれも行っていない。アプリと dev server は観測時点で稼働し保持中。別担当の画面観測設計・独立レビュー・親判断を待つ。
- 今回の操作失敗はなし。前回の画像不一致1回、ブラウザー inventory エラー2回を維持。同種2回で該当操作中断・別担当調査、同種3回で全作業停止。
