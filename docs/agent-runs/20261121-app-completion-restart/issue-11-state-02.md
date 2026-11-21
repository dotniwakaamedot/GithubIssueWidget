# 検証 Issue #11 の状態確認 02

親から修正版で移動できたというユーザー報告を受け、公開リポジトリ `dotniwakaamedot/GithubIssueWidget` の Issue #11 を番号指定で読み取った。公開 REST 応答は HTTP 200、状態 `open`、ラベルは `status:todo` のみ、更新時刻 `2026-11-21T05:40:37Z`。この時点の状態に限り、操作順や全遷移を証明しない。

作業場所 `D:/Projects/GithubIssueWidget`。Python 標準ライブラリの HTTPS GET と JSON 抽出を実行し終了コード 0。Issue 本文・資格情報は出力せず、Issue やアプリ設定を変更していない。未確認はユーザーの操作順、取得後の更新、および非公開データ。今回認識した失敗なし。確定後の SHA256 は別途取得する。
