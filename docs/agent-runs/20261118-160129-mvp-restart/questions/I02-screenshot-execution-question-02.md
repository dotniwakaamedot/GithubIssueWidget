# I02 窓観測前の所有 handle 確認結果 02

- 担当 `/root/drag_design`、宛先 `/root`・`/root/screenshot_design`・`/root/restart_planner`。対象 `questions/I02-screenshot-execution-question-01.md`。
- 2026-11-18T19:31:20Z 前後に、担当が所有する app session `70494`、server session `19143` をそれぞれ `write_stdin` の空入力で1回 poll。いずれも session ID が返り、各5秒待機後も終了していない。観測時点では live、exit code 未取得。観測後の存続は保証しない。
- cwd `D:\Projects\GithubIssueWidget`、`Get-Process` / `Get-FileHash -Algorithm SHA256` / `Get-NetTCPConnection` / `Get-CimInstance Win32_Process` の読み取りは exit 0。対象 exe `D:\Projects\GithubIssueWidget-marker-diagnostic-20261118-160129\target\debug\github-issue-widget.exe` の SHA256 は `54DBCECFD920F6F920942A190C81B28BD7742C6052ABB58AB9C89E1F71C37AA8`、PID `52960`、MainWindowHandle `6559504`、exe path 一致。port `1435` listener は1件、node PID `32908` の command line に同じ marker scratch path が含まれる。
- process・port・handle の整合はその時点の観測であり、PID/ID 再利用、別名プロセスやデスクトップの非競合を完全には保証しない。別担当は session handle を継承したとみなさず、自ら必要な現時点の対象確認を行う。UI、getState、browser inventory、入力、停止は行っていない。
- 今回新たな失敗なし。元の画像不一致1回、browser inventory failure2回、直接確定逸脱2件を保持。同種2回で客観調査、3回で全停止。新規質問は同ディレクトリの `.tmp` から非上書きで確定する。
