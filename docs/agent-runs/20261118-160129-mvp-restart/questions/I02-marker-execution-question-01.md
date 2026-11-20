# I02 旧 geometry プロセスの終了方法確認 01

- 担当 `/root/drag_design`、宛先 `/root`・`/root/restart_planner`。依頼 `tasks/I02/marker-execution-request-01.md` の参照16件は全件実在・実体 SHA が一致。旧 app session `4498` と server session `68650` は各空 poll で session ID が返り live（観測時点）。
- read-only の `Get-CimInstance Win32_Process` / `Get-NetTCPConnection` / `Get-FileHash`（cwd `D:\Projects\GithubIssueWidget`、exit 0）により、旧 geometry app PID `9796`、親 PID `36580`、exe `D:\Projects\GithubIssueWidget-geometry-diagnostic-20261118-160129\target\debug\github-issue-widget.exe`、SHA256 `F10DC7E78705DC40927C95CBABC372FA5F64F1F2EE52B0B46A80E6BA56BCC2BD`、開始 2026-11-19 04:06 頃。port 1435 の listener は node PID `54736`、親 PID `52836`、command line は旧 geometry scratch と一致、開始 04:05 頃。app の子に WebView2 PID `59548`。観測は一時点に限る。
- 旧 session の `exec_command` 起動時には `tty` を指定しておらず、stdin への Ctrl+C が console signal として伝わるか、process group と第三者への影響範囲は未確認。前回同じ種類の事情で、前回の専用 PID のみ強制終了する明示的判断を受けたが、その判断は今回の PID 9796 / 54736 へ自動的に拡張しない。
- 対象 desktop の画像と AX の不一致は未解決。推測による画面入力や以前失敗した screenshot / getState は実施しない。依頼は通常終了できず追加判断が必要なら保留と規定し、所有 PID が不明・巻き込みの懸念がある場合は停止を求める。
- 質問: 今回の診断専用 app / server を終了する方法について親の具体的判断をお願いしたい。旧2 process の停止や Ctrl+C、UI 操作、別 PID への信号は行っていない。新 marker 版の依存準備・build・起動・入力も未実施。元 product・各 scratch・証拠・GitHub は保護。
- 今回の失敗なし。以前の画像不一致1回、browser inventory failure2回、直接公開2件を維持。同種2回で独立調査、3回で全停止。
