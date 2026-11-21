# 大量ページの限定試験実行判断

`pagination-runner-review-01.md` は `pagination-test-execution-01/run-test.ps1` SHA256 `CD8705690A28D9F607BA596037307F92607B395BCD9687FEFD779639D58E1063` を承認。`/root/issue_survey` は `pagination-regression-execution-request-01.md` の事前条件と入力・依存・ログの確認を完了した場合、指定の新規2ケースのみ各1回実行する。0件は成功にしない。終了コード・テスト名と件数・ログ・直接プロセスと子孫の未確認を分け、期限・失敗時は同じセッションを確認し未知のプロセスを終了しない。実データ・既合格の無関係な試験には触れない。
