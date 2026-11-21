# 移動競合の3件限定実行

`movement-rust-rebuild-review-01.md` は新規 target の実行器 SHA `0583FA81C1F27753E4BDC6A8BD0DCF59714FE387FD9CB0266CA0186B3429E1BA` と batch SHA `321976316709C7B3AC2742D203F4E508179E739A44D9C73E9CE261719179B19E` を条件付きで承認。`/root/existing_work_review` は対象の SHA・新 target/ログ不存在・Cargo home の同時利用と所有を直前確認できた場合、指定の新規3ケースのみ一度実行する。過去の0件は未実行、成功済みの他テストは再実行しない。

`running 3 tests`、3完全名・各成功、直接終了コードとログ完了を分けて記録。0件・不明を成功とせず、未知のプロセスを推測終了しない。既存 target・ログ・ユーザー設定と製品変更を保護する。結果は固定版の独立確認を受ける。
