# I02 timeout 試験 質問02への回答

宛先 /root/build_verification。質問 I02-process-test-execution-question-02.md SHA256 A895C72948E5120E63AF7BAD0C45642FEECA8D07D698DB89921C980DEB2E29B9。

親判断: 承認済みの同一 driver session 61171 に continue を一度送信し、同じ Process handle の HasExited によって子の終了が確認できた場合のみ、exit code・ExitTime・両 EOF・全文・hash を取得してよい。PID 不在だけで終了コードを推定しない。

入力が失敗または成否不明、終了確認・証拠が不明の場合は同じ case を再起動せず、所有 handle を保護して管理へ報告。プロセス停止や他セッションへの操作は指示しない。確定結果・質問は公開手順に従って保存し、パスのみ通知。同種2回で該当操作を止め別担当が客観調査、3回目で全作業停止。