# 大量ページ試験 fixture の引き継ぎ

2026-11-21、担当 `/root/issue_survey`。`pagination-fixture-handoff-request-01.md` に基づく。`pagination-test-execution-01/` の書込みを停止し、親の指示に従って後任へ所有を引き継ぐ。元の製品・Git・旧 fixture は変更しない。

## 終了状態

- 修正版の2件の直接 `cmd` はそれぞれ終了コード0、期限超過なし。第1ケースの exec セッション `95672` は完了結果を受領、第2ケースは同一呼出しで完了。各 stdout に `running 1 test`・指定ケース `ok`・`1 passed` と完了行を確認した。詳細と全ログ SHA は `pagination-regression-execution-result-01.md`。
- 初版の第1試行は直接終了1、ログなし。原因とテストに到達したかは不明。その実行セッションは結果を受領し、同じ方式の再実行はしていない。キャッシュの複写・ハッシュ照合セッションも終了コード0を受領。現在、自分が把握する同一実行セッションに実行中のものはない。
- 直接プロセス終了とパイプ排出完了を確認したが、**全子孫の OS レベルの終了は追跡していない**。残存なし・他プロセスと非干渉と断定しない。旧不明プロセスを含む他のプロセスを停止していない。

## 現行版と保持対象

- 専用 `project/src/main.rs` SHA256 `AEFF1E65AD93EFA04773CEBBB9736A6BDC5B9938DB9FAF937DD88B3D9CBFE86F`、`pagination_tests.rs` `BF690E818EB182FE16395E0AE3946B043D41DADD2B75235EFEACDC4873F4DE3D`、`project/Cargo.lock` `2486C9E87F8511A2CBB3C09177E278139409C68A308FD48642327EDDF1D88E7F`。
- 初版 `run-test.ps1` `CD8705690A28D9F607BA596037307F92607B395BCD9687FEFD779639D58E1063`。修正版 `run-test-02.ps1` `733FF4BC3D12ED823BCA206BEB39E8DD4302DF3C2D8785C0CF4E9BFDCBA21F30`、`run-test-02.cmd` `75329EA73C642862A9AB8D049564659F0B2063D0F4848D2C3598C7108CB16161`。この固定版は大量ページ2ケース専用であり、別テスト実行を自動的に許可・保証しない。
- 専用 `target/`、`cargo-home/`、`temp/`、`user/`、`project/` と `logs/` は既存の実行証拠・生成物として保持。ログや入力を上書き・削除しない。実行前の registry 全ファイル一致は結果記録に記載したが、実行後の target や cache 全体の固定ハッシュは未取得。必要な再利用範囲、変更、出力先、残存リスクを後任が実行前に確認し、別の固定版レビューと親の判断を受ける。
- 実行用入力・実行器の上記 SHA は本引き継ぎ時に再計測した。別担当・別作業が並行しうるため、再利用直前の状態も後任が確認する。
