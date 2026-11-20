# PowerShell 構文エラーの独立調査依頼

担当 `/root/powershell_parser_diagnosis`。ユーザー原文: 「同じ失敗を2回繰り返したら別のエージェントに客観的な視点で調査してもらってください。その後失敗を2回繰り返したら停止してください。」

共有 run `D:/Projects/GithubIssueWidget/docs/agent-runs/20261118-160129-mvp-restart` の `tasks/plan/resume-review-20261119.md` は補助集計中の PowerShell パイプ構文 `ParserError` を1回記録している。`questions/gh-quoting-parser-warning-20261119.md` は別担当が `foreach(...) { ... } | Format-Table` の直接記述で `ParserError: EmptyPipeElement` を1回発生させたと報告。前者の元コマンドやパーサー識別子は未確認。両記録の共通点と差、確定事実・推測・安全な対処を読み取り専用で整理する。失敗した構文の再実行は行わず、実在確認後に指定資料を読む。製品・GitHub・UI・既存証拠を変更しない。

結果を `tasks/powershell-parser-diagnosis-20261119.md` に UTF-8 一時ファイル経由で確定し SHA256 を通知する。PathNotFound は独立調査後1回発生済みで、さらに1回発生したらユーザー指定の全停止条件に達するため、指定の正確なパスと基点を使い、実在確認してから参照する。他者の成果を保護する。
