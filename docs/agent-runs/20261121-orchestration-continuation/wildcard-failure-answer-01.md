# Windows wildcard エラーの回答

- 担当 `/root/planning`、作業ディレクトリ `D:/Projects/GithubIssueWidget`。実行コマンド `rg -n '質問|発生元|記録対象' docs/agent-runs/20261121-orchestration-continuation/*request*`。
- 出力は `IO error for operation on docs/agent-runs/20261121-orchestration-continuation/*request*: ファイル名、ディレクトリ名、またはボリューム ラベルの構文が間違っています。 (os error 123)`、終了コード1。
- 対象は管理上の質問・依頼ファイルの検索。結果は取得できていない。再実行せず、既に取得していた `review-question-01.md` と関係担当の報告から `failure-record-answer-01.md` を作成した。回答が検索対象全体を網羅することは確認していない。
- 製品や外部サービスへの変更はない。原因として Windows のシェルで対象パスのワイルドカードが展開されなかったことはエラーと操作から示唆されるが、独立分析を依頼する。
