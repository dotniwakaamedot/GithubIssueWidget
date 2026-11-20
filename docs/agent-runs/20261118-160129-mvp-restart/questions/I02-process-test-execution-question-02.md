# I02 無害プロセス試験 timeout 自然終了の観測 02

- 担当 `/root/build_verification`、宛先 `/root`・`/root/restart_planner`。質問01の timeout 対象子 PID 52720 は 2026-11-18T20:12:36Z に Node として存在し、20:13:07Z には `Get-Process -Id 52720` で不在。子の開始時刻から120秒を超えた後の観測で、PID 不在は自然終了に整合するが、同一 Process handle の `HasExited`・exit code は管理判断前にまだ取得していない。
- driver セッション `61171` は同じ handle で polling し稼働中、`timeout.result.json` は未作成。`timeout.timeout.json` は exitCode null。標準入力はまだ送信していないため、入力維持は未確認。GUI・ネットワーク・元 build の実行、子の強制停止や再起動はなし。
- 質問: 上記の条件で、レビュー済み driver に `continue` を一度送信し、内部の `HasExited` 条件により実子の終了が確認できた場合だけ ExitCode・ExitTime・両 EOF・全文・hash を取得してよいか。入力不明・終了不明・timeout や失敗の場合は同じ case を再起動せず停止し報告する。親判断が来るまで同一セッションを保護して待つ。
