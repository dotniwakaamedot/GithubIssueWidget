# 修正版のユーザー実機観測 07

- 親 `/root` からの報告: ユーザーは selection-build 修正版で「半透明の文字が表示されなくなりドラッグ移動も成功」と報告した。対象は親が提示した `D:/Projects/GithubIssueWidget/docs/agent-runs/20261121-app-completion-restart/selection-build-sandbox-01/cargo-target/release/github-issue-widget.exe`。管理は現時点の実ファイル SHA256 `0D8AB47088B7A19233EF0D7D08CF033B681B20CA0E729FEAF73EAB74DCC98F55` を `Get-FileHash` 終了0で確認した。本人報告と EXE の現在実体の照合であり、管理自身が画面を観測したわけではない。
- 先行して親が確認した通り、実際には Issue は1件だけ移動し、複数選択に見えた現象は文字の半透明表示である。ユーザー報告は今回の操作における現象改善を示すが、全 Windows 環境や他の操作全般の保証ではない。
- 親が #11 の全6方向と最終 Done / closed の実機試験を依頼中。結果待ちであり成功とみなさない。入力・リンク・キーボードや実資格情報のライフサイクルの回帰は別途の証拠が必要。秘密や非公開 Issue 内容は記録していない。
