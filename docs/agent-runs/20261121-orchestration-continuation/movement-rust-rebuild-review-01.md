# 移動競合の新 target 実行器レビュー

2026-11-21、確認者 `/root/issue_survey`、作者 `/root/existing_work_review` と別担当。`movement-rust-rebuild-review-request-01.md`、preflight、0件調査と旧実行器レビューを読取り照合。実行・差替え・既存資産の変更はしていない。

**固定版の実行器を条件付きで承認。** 前回は古いバイナリの再利用が強く示唆されるが、Cargo の内部判断は未確定。空の新 `CARGO_TARGET_DIR` を使う方式は過去 fingerprint に依存せず、その問題を避ける具体策として妥当。実行前に次を満たす。

- 新 target と新 stdout/stderr が存在せず、所有範囲の同時利用がないことを直前に確認する。新 target は当該実行専用とし、既存 target・0件の exe/ログ・pagination 証拠を消去・上書きしない。共有する専用 Cargo home は利用競合を確認し、Cargo が更新しうることを認識する。現時点で新 target とログは不存在、空き容量の事前記録は preflight にある。
- 現行 fixture main SHA `AEFF1E65AD93EFA04773CEBBB9736A6BDC5B9938DB9FAF937DD88B3D9CBFE86F`、movement `88BF21BB45C077DFDA592D906C753D168C5641E35EB12324D4ACF1652374695B`、実行器・cmd と他の必要入力を直前に再確認し、差異・他担当の変更があれば停止する。元 movement と保存済み証拠を保護する。
- `--locked --offline --bin github-issue-widget movement::tests::post_write_competing_ -- --test-threads=1` は旧承認版と同じフィルタ。前回の静的確認では3件のみ一致したが接頭辞フィルタなので、実際の stdout で `running 3 tests`、3完全名の各 `ok`、他の混入なし、0 failed、直接終了コード、ログ終端を照合する。0件・途中切断・期限超過を成功扱いしない。成功済みの他テストを繰り返さない。
- 以前と同じ限定子環境・新規ログ・直接 cmd の終了/900秒超の観測を維持する。ログ排出が子孫に保持される場合や全子孫の終了は未保証。未知の PID・旧プロセスを推測で終了せず、不明なら対象を保持する。実 GitHub、実 Credential Manager、実 GUI の動作は対象外。

## 根拠

- `movement-rust-rebuild-preflight-01.md` SHA256 `A2B63AD2E2C273BFEC90CFB5F3186E9F510F5D5EF707A9C571144BE91E037AC3`、`run-movement.ps1` `0583FA81C1F27753E4BDC6A8BD0DCF59714FE387FD9CB0266CA0186B3429E1BA`、`run-movement.cmd` `321976316709C7B3AC2742D203F4E508179E739A44D9C73E9CE261719179B19E`。開始・終了時の3 SHA は一致。
- 旧・新 PowerShell の全文をメモリ内で比較し、`CARGO_TARGET_DIR` の `Join-Path $fixture 'target'` を `Join-Path $root 'target'` に置換した1箇所のみで完全一致。cmd は旧承認版と同じ SHA。比較・ハッシュ・存在確認は終了0。試験・サーバー・外部操作はしていない。操作失敗は認識していない。
