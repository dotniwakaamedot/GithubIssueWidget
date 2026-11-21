# 修正版ウィンドウ操作の本人報告

- 担当: `/root/record_user_check`。作業場所: `D:/Projects/GithubIssueWidget`。共有 run: `D:/Projects/GithubIssueWidget/docs/agent-runs/20261121-app-completion-restart/`。
- 対象として会話で案内した実行ファイル: `D:/Projects/GithubIssueWidget/widget/release-20261121-selection/github-issue-widget.exe`。記録時点で実体の SHA256 は `0D8AB47088B7A19233EF0D7D08CF033B681B20CA0E729FEAF73EAB74DCC98F55`。`git rev-parse HEAD` は `4961b25bec53a93439acdf5230de77f959908af0`。HEAD は未追跡・未コミットの内容を識別しない。
- 親が依頼した操作は、タイトルバーによるウィンドウ移動、端によるサイズ変更、最前面固定の切替と他ウィンドウとの前後関係、Issue カードのクリックによる GitHub 該当ページ表示の４点。ユーザーは修正版のパスを尋ね、親が上記パスを示した後、「すべて動きました」と報告した。これは本人報告であり、担当者による画面の独立観察ではない。
- 初期サイズ・最小サイズの実測、起動直後の中央位置・最前面状態、許可外 URL の実際の拒否、別 PC や表示環境での動作は、この報告からは確認できない。GitHub ページの内容や秘密情報は収集していない。
- 確認: 作業ディレクトリで `Get-FileHash -Algorithm SHA256 -LiteralPath` と `git rev-parse HEAD` は終了 0。対象 EXE の SHA は上記と一致。対象ファイルがこの時点で存在することとハッシュを確認したものであり、実際に起動されたプロセスのハッシュを独立測定したものではない。
- 変更は本記録のみ。製品・既存設定・資格情報・GitHub は変更していない。削除対象なし。認識した失敗なし。最終ファイルの SHA256 は自己参照を避けて確定後に管理側が取得する。
