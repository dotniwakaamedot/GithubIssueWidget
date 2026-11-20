# I02 所有セッションと対象プロセスの確認 02

- 担当 `/root/drag_design`、宛先 `/root`。親の指示に従い、所有する既存 handle と process・port を読み取り確認した。前回拒否されたレビュー記録の作成は再試行していない。
- 2026-11-18T20:01:34Z 前後、`write_stdin` の空 poll は app session `70494`、dev server session `19143` に各1回実施。各5秒後も session ID が返り、**当時は両方 live**。終了コードは未取得であり、空出力を正常終了と扱わない。
- 別途 cwd `D:\Projects\GithubIssueWidget` で `Get-Process`、`Get-FileHash -Algorithm SHA256`、`Get-NetTCPConnection`、`Get-CimInstance Win32_Process` による読み取りは exit 0。診断 app PID `52960`、HWND `6559504`、exe path `D:\Projects\GithubIssueWidget-marker-diagnostic-20261118-160129\target\debug\github-issue-widget.exe` 一致、SHA256 `54DBCECFD920F6F920942A190C81B28BD7742C6052ABB58AB9C89E1F71C37AA8`、開始UTC `2026-11-18T19:25:33.6036834Z`。port `1435` の listener は1件、node PID `32908`、command line は marker 専用 scratch と一致。
- 上記は時点限定。PID / HWND の将来の同一性、別名プロセス、他環境やユーザー操作との非競合、UI の見え方は証明しない。UI、`getState`、browser inventory、入力、process の停止・変更は行わず、元製品・GitHub・既存成果を保護。
- 今回新たな失敗なし。画像不一致、browser inventory failure、直接公開逸脱、依頼形式訂正の検査停止はそれぞれ既往2回。ユーザー指示の同種2回で該当操作停止・別担当調査、3回で全停止を維持。新規記録は既存先と `.tmp` 不在確認後、同ディレクトリ `.tmp` から非上書き確定する。
