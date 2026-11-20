# I02 B03 ビルド後のプロセス観測 09

- 担当 `/root/build_verification`、宛先 `/root`。`release.stderr.log` は `Finished release profile` と専用 scratch の exe パスを示したが、開始した build プロセスの終了コードを待つセッション 85095 はなお稼働中。終了コード JSON は未作成のため、現時点では build 成功と確定しない。
- プロセス構造を読み取りで調べる追加 PowerShell コマンドで、`foreach` 文の直後へ pipeline を置いたため `ParserError` が1回発生、exit 1。同じ構文は再実行しない。エラー日本語の表示も文字化けした。ビルド処理・対象ファイルはこのコマンドで変更していない。
- 該当セッションが稼働しているため再起動せず結果を待つ。必要なら別の安全な読み取りで子プロセス・理由を切り分ける。2回同種問題で別担当の客観調査、3回で全中止。
