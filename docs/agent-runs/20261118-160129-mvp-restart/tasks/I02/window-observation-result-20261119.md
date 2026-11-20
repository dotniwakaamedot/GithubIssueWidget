# 基盤ウィンドウ初期状態の OS 観測

- 観測日時: 2026-11-19T00:37:40.6667900Z。対象: `C:/Users/semis/orca/workspaces/GithubIssueWidget/issue-widget-mvp-resume/widget/src-tauri/target/release/github-issue-widget.exe`、SHA256 `472DEED04C601BE378C8C8B471F332F98C4043C82A0174FAA5F246D3D5898DC7`、PID 53868。基準 HEAD `59578f8a4a775b4bd66c6acdde318b2e934abc08`。
- 観測手段: `powershell.exe -NoProfile -File D:/Projects/GithubIssueWidget/docs/agent-runs/20261118-160129-mvp-restart/tasks/I02/window-observation-20261119.ps1`。スクリプト SHA256 `50E63F9B26ED38EE72FCE39CAB9526946854C02019D6A6E40B5747DF9CFF6DBD`。終了コード0。起動した対象プロセスだけを観測後に停止。Windows API のウィンドウ・クライアント矩形、DPI、モニター矩形、最前面フラグを読み取った。入力・移動・リサイズは行っていない。
- 実測: DPI 96。クライアント矩形 `(0,0)-(900,520)`。外枠矩形 `(1470,796)-(2386,1325)`。モニター矩形 `(0,0)-(3840,2160)`、作業領域 `(0,0)-(3840,2112)`。`WS_EX_TOPMOST` は true。ツール出力を事後転記したもので、別ファイルへの生ログ保存は行っていない。
- 判断: 初期クライアント領域 900×520 と観測時の最前面フラグは確認できた。外枠には不可視枠等が含まれ得るため、外枠中心と作業領域中心の差だけではアプリの中心配置の仕様適合を確定しない。表示中の他ウィンドウとの重なり、実際の可視性、最小サイズ、リサイズ、ドラッグ、最前面切替と URL 拒否は未検証。プロセスを強制停止したため閉じる操作の成功も証明しない。
- 今回の観測処理で新たな失敗はない。旧作業場所・既存証拠は変更していない。
