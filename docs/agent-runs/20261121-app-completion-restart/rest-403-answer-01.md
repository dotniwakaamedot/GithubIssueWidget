# REST 403 に関する担当者回答

- 1回目: #11 本体 `GET /repos/dotniwakaamedot/GithubIssueWidget/issues/11`、`2026-11-21 06:41 UTC` 頃。Python 終了1、`HTTP Error 403: rate limit exceeded` が例外表示に含まれた。`X-RateLimit-Remaining`、`X-RateLimit-Reset`、`Retry-After` は取得しておらず未確認。専用ログは保存せず、ツール出力と `issue-11-sync-state-01.md` に記録。
- 2回目: #11 イベント `GET /repos/dotniwakaamedot/GithubIssueWidget/issues/11/events?per_page=100&page=1`、`2026-11-21 06:48 UTC` 頃。Python 終了1、例外を抑止して HTTP 403 のみ表示。応答ヘッダー・本文は取得しておらず、この回の rate limit は推測であり原因未確認。専用ログは保存せず、ツール出力と `issue-11-sync-state-02.md` に記録。
- 正確な発生秒はいずれも未保存。どちらも匿名 REST、GET。再実行はしていない。資格情報や本文は本回答に記載しない。
