# 担当 ID 不一致の訂正

- `/root/issue_inventory` と `/root/output_failure` の旧 followup は interrupt が前状態 running を返し、両担当のそのターンを中断した。該当タスクの `result-02.md` と `failure-record-review-01.md` は訂正時点で存在しない。以後この2名に同じ範囲を依頼しない。
- 正式な新担当と依頼: `/root/issue_completion_audit` → `tasks/01/request-03.md` SHA256 `565BF8C8F995D52B884743DBAA2EBEA87E4719CF802AD11CFDD12D2AD184B0D5`、`/root/failure_review` → `failure-record-review-request-02.md` SHA256 `C71F5F7208D22184747A73FA543AD44536A3ACAFEF8930521A7F3B96E54B4743`。どちらも既に親が起動しており、確定依頼の絶対パスを該当担当へ通知した。
- 以後の担当起動は親が行う。管理は親と担当 ID を先に確定し、起動前に依頼書を確定・パスを親に通知する。既存担当からの移管は停止と成果を先に確認する。
