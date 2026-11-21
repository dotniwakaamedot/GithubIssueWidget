# 同期統合後の型検査・ビルド実行依頼

- 担当 `/root/project_survey`。共有 run `D:/Projects/GithubIssueWidget/docs/agent-runs/20261121-orchestration-continuation/`。所有は `sync-test-fixture-01/` の明示された新規 HTML・ログ・ビルド出力と `sync-post-integration-check-result-01.md`。製品、共有依存、Git、旧 worktree や他担当の変更を戻さない・変更しない。
- 承認済み `sync-post-integration-check-plan-01.md` SHA256 `99E859B8E4D27A706ACA58E1D29A02D93FB5E77935169BE4EF2BE58623C14C76` と `sync-post-integration-check-review-01.md` の開始条件に従う。現製品・fixture の main/test SHA、全対象入力、実行体、既存 manifest と既知生成物、環境、空き容量・出力先を事前に確認。不一致・予期しない他者変更・不明な書込み先があれば実行せず管理に通知。
- `widget/index.html` を実在確認し SHA 固定の上、fixture に同名ファイルがない場合だけ通常コピーし、前後の一致を確認。既存 secret token は内容を読まず保護。独立 fixture で固定 Node 経由 `node_modules/typescript/bin/tsc --noEmit`、成功時のみ `node_modules/vite/bin/vite.js build` を各一回実行。前回承認した非機密の最小環境と専用ログを使用。各直接終了コードを直ちに捕捉し、出力と生成物を確認する。
- セッションが継続中なら同じ ID を待ち、観測期限を終了とみなさない。GUI、実 GitHub、資格情報、旧プロセスの停止はしない。子孫の終了が不明なら直接の成功とは別に記録。失敗・未知の出力は保持し、同じ失敗2回時は別担当に調査を依頼する。
- 結果に実行場所・安全なコマンド・環境変数名・各入力と実行体 SHA・終了コード・出力の完全性・ビルド生成物・未確認を記録し、確定後書込みを停止してパスを通知。別担当が対象版・証拠をレビューする。
