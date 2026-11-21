# PR 照会失敗の回答

担当 `/root/existing_work_review`。2026-11-21 の2件について、保持しているツール結果と実行時の記憶に基づく。再照会や認証変更はしていない。

1. 作業ディレクトリ `D:\Projects\GithubIssueWidget`、PowerShell、既定 sandbox で `Invoke-WebRequest -UseBasicParsing` を `https://api.github.com/repos/dotniwakaamedot/GithubIssueWidget/pulls?state=open&per_page=100&page=1` に対し実行した。ヘッダーは `User-Agent` と `Accept: application/vnd.github+json` のみ明示し、認証情報は明示していない。catch は例外型だけを出力し `request_failed_type=WebException`、終了コード1。HTTP 状態・原因・応答内容は未取得で、特定の通信段階や拒否を断定できない。
2. 別の `web__run` の `open` に `https://api.github.com/repos/dotniwakaamedot/GithubIssueWidget/pulls?state=open%26per_page=100%26page=1` を渡した。ツールは内部で `state=open%26per_page%3D100%26page%3D1` とした URL を表示し、`Internal Error` とアクセス不能を返した。期待した複数パラメーターの URL ではなく、HTTP 状態・原因は未確認。認証設定は渡しておらず、web 実行側の内部認証は不明。

各出力は実行ツールの会話上の結果として残り、別の原本ログは保存していない。要約は `sync-ownership-result-01.md`、`sync-pr-history-result-01.md`。1回目と2回目は別の呼出し・環境・URLであり、内部原因が同じとは断定できない。1回目の同一コマンド再試行も、2回目の正しい URL による再試行も行っていない。別のコネクター専用 `search_prs` の空結果はこれらとは別の成功応答で、その制約は `sync-ownership-result-02.md` に記載した。
