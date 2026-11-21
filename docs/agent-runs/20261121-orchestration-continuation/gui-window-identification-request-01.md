# 生存中 release GUI の窓識別方法の独立調査依頼

- 担当 `/root/existing_work_review`。起動担当 `/root/gui_failure_investigation` とは別。共有 run `D:/Projects/GithubIssueWidget/docs/agent-runs/20261121-orchestration-continuation/`、結果 `gui-window-identification-01.md` のみ所有。
- `gui-release-observation-status-01.md` は直接 PID 6004・SessionId 2・保持ハンドル照合 true、`MainWindowHandle=39655226` と、CUA `listWindows()` に対応 HWND が現れないことを記録。GUI の表示・終了や原因は未確定。起動担当が同じライブセッション 83178 とハンドルを保持しているので、中断・入力・閉鎖・新規起動を行わない。
- 利用可能な Windows と CUA の読み取り手段・対話セッション・PID/HWND の関係を調べ、同一の対象ウィンドウを安全に特定・可視性確認できる方法と、できない場合の具体的未確認を提示する。必要な対応 skill とツール指示を適用。個人画面・ウィンドウ内容を無差別に保存/公開しない。PID や名前だけで同一性を断定せず、他のウィンドウ/プロセスには操作しない。
- 読取り・方法検討のみ。終了、kill、設定変更、再起動はせず、観測時点・コマンド・結果・不足・安全な次手を記録し通知する。起動担当は同一ハンドルの維持と既存セッションの観測を継続する。
