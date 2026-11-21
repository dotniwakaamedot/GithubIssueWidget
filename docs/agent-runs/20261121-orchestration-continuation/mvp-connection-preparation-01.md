# MVP 接続と実操作の準備

- 起動確認後、本人がアプリの設定画面に `owner`・`repository` と対象リポジトリに限定した fine-grained PAT を直接入力する。必要な権限は `Metadata: Read` と `Issues: Read and write`。トークンの値を会話・共有ファイル・ログに書かない。接続確認成功は書込み権限までの証明ではない。根拠 `README.md`。
- 実操作の既承された承諾は公開 `dotniwakaamedot/GithubIssueWidget` で検証用 Issue 1件を作成し、列間移動して最後にクローズする範囲。既存 Issue・PR の変更は含まれない。`status:todo`・`status:in-progress`・`status:done` は過去に確認済みだが、現在の存在・認証・対象を操作前に再確認する。根拠 `docs/handoff-20261121.md`。
- 送信結果が不明なら POST を自動再送せず、対象と既存の検証マーカーを確認する。失敗や競合時は GitHub から再取得し、途中反映の可能性と再取得失敗を区別。画面表示、Credential Manager の実保存/再起動後取得、作成・移動はまだ未検証。ユーザーの GUI 起動結果待ちの間に実 credential や実データに触れない。
