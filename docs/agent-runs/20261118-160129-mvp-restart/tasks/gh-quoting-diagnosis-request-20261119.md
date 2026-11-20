# GitHub CLI 引用失敗の独立調査依頼

担当 `/root/gh_quoting_diagnosis`。ユーザー原文: 「同じ失敗を2回繰り返したら別のエージェントに客観的な視点で調査してもらってください。その後失敗を2回繰り返したら停止してください。」

1件目は `tasks/test-repository-20261119.md` に記録した、PowerShell から `gh label list --jq` へ渡した文字列の引用が失われ jq の解析が失敗した事象。2件目は `tasks/issue-05-review-20261119-03.md` のレビュー担当が記録した、`gh issue view` の整形用引用が PowerShell で意図どおり解釈されず失敗した事象。両方とも後続では JSON を整形指定なしに読んでいる。元の具体的なコマンドが一部不明な場合は推測で埋めず、確認できる事実と共通点・差、再発防止・未確認を整理する。失敗したコマンドの再実行や GitHub・製品・UI の変更は禁止。結果を `tasks/gh-quoting-diagnosis-20261119.md` に UTF-8 一時ファイル経由で確定し SHA256 を通知する。

共有 run `D:/Projects/GithubIssueWidget/docs/agent-runs/20261118-160129-mvp-restart` と指定の正確な相対パスを使用し、実在確認後に読む。PathNotFound は独立調査後1回発生済みで、もう1回発生した場合はユーザー指定の全停止条件に達する。他者の成果を保護する。
