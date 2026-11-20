# 基盤ウィンドウの表示矩形と中央位置

- 観測日時 `2026-11-19T00:40:41.6494847Z`。対象 `C:/Users/semis/orca/workspaces/GithubIssueWidget/issue-widget-mvp-resume/widget/src-tauri/target/release/github-issue-widget.exe` SHA256 `472DEED04C601BE378C8C8B471F332F98C4043C82A0174FAA5F246D3D5898DC7`、基準 HEAD `59578f8a4a775b4bd66c6acdde318b2e934abc08`。
- `powershell.exe -NoProfile -File D:/Projects/GithubIssueWidget/docs/agent-runs/20261118-160129-mvp-restart/tasks/I02/window-center-20261119.ps1` exit 0。スクリプト SHA256 `AA5A73C8968A9F234AB140B9511EF55ACCA3E1F17263CD4B1CABD06EEED943DA`。対象プロセスを起動し、対象ウィンドウの `ClientToScreen`、`GetClientRect`、DWM 拡張フレーム矩形と同一モニターの作業領域を読み、対象プロセスのみ停止した。入力・移動・リサイズは行っていない。
- 実測: クライアント左上 `(1478,797)`・大きさ `900×520`、DWM 拡張フレーム `(1477,796)-(2379,1318)`、作業領域 `(0,0)-(3840,2112)`。作業領域中心 `(1920,1056)` に対し、表示フレーム中心 `(1928,1057)` は右へ8px・下へ1px、クライアント領域中心 `(1928,1057)` も右へ8px・下へ1pxずれた。
- 確認された事実はこの環境・一時点・対象 exe の差。原因（不可視枠、Tauri/Windows の中央計算や DPI の影響など）は未特定。Issue #2 の「中央表示」が許容する差の明示はなく、完全一致を確認できたとは扱わない。他 DPI やモニター、起動時の変化・繰返しは未確認。原出力は会話のツール結果であり独立した生ログはない。
- 実験中の新たなコマンド失敗なし。旧作業・成果は変更していない。
