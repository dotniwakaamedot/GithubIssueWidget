# 移動競合の限定実行 preflight 案

担当 `/root/existing_work_review`。`movement-rust-fixture-handoff-decision-01.md` と `pagination-fixture-handoff-01.md` に従い、前担当の直接プロセス終了と書込み停止を確認。OS の全子孫の終了は未確認として継承する。作成した実行器の独立レビューと管理の開始通知前に fixture の入力差替え・実行をしない。

## 現在と予定する差替え

- `pagination-test-execution-01/project/src/main.rs` SHA `AEFF1E65AD93EFA04773CEBBB9736A6BDC5B9938DB9FAF937DD88B3D9CBFE86F` は現在の製品 main と一致。`pagination_tests.rs` `BF690E818EB182FE16395E0AE3946B043D41DADD2B75235EFEACDC4873F4DE3D`、`Cargo.lock` `2486C9E87F8511A2CBB3C09177E278139409C68A308FD48642327EDDF1D88E7F` は引継ぎと一致。fixture の movement は製品と同じ `ED49F4BA52C6887BCE427E4A6664D335092A1C3311E1B1ED61B4CE322791ABA5`。
- 差替え対象だけを新規の `movement-rust-test-execution-01/original-movement.rs` に排他的に保存し、元と SHA を照合した後、レビュー済み隔離 `movement-rust-isolated-02/movement.rs` SHA `88BF21BB45C077DFDA592D906C753D168C5641E35EB12324D4ACF1652374695B` を fixture の `project/src/movement.rs` に反映する案。直前にすべての SHA・停止・所有を再確認し、異なれば止める。pagination のソース・ログを変更しない。target とキャッシュは専用 fixture のものを再利用し、ビルド時に更新され得ることを明記。旧生成物の個々の不変性は保証しない。
- 既存のログ4件はそのまま保護し、新しい出力は所有する別ディレクトリの新規ファイルで `CreateNew`。現在どちらも不存在。親・別担当の同時利用があれば停止する。

## 新しい実行器の固定版

- `movement-rust-test-execution-01/run-movement.ps1`: SHA256 `AF6C6C39AC166180D0B7D7B750562F5366CFA4797A0D65E907C03791314E07D3`。
- `movement-rust-test-execution-01/run-movement.cmd`: SHA256 `321976316709C7B3AC2742D203F4E508179E739A44D9C73E9CE261719179B19E`。
- 以前にレビューされた pagination 実行器の環境クリア、専用 CARGO_HOME/target/TEMP/profile、stdio の同時排出、直接 cmd の終了コード取得と900秒超の保持観測を踏襲。コマンドは `cargo test --locked --offline --bin github-issue-widget movement::tests::post_write_competing_ -- --test-threads=1`。前方一致で3件を1回実行し、件数と完全名・各成功、直接終了コード、stdout/stderr の終端を実行後に確認。0件や欠落は成功でない。
- 他の全テストは繰り返さない。各 loopback mock は受付5秒・読み書き/要求3秒、6応答固定。実サービスや資格情報は使わない。コンパイラ・build script・子孫の終了を direct cmd の結果だけで保証しない。900秒超過時もハンドルを保持し観測し、不明な PID を終了しない。ログ排出待ち・未知の OS 異常は未保証。

## レビュー依頼

実行器の対象ハッシュ・接頭辞が3件のみを選ぶこと、対象版、保存・差替えの所有と旧証拠保護、環境・ログ・有限性・不明時の扱いを独立確認してください。承認と親の開始通知前は実行・差替えしない。依存/target の実行後状態はハンドオフで完全ハッシュがなく、未知の他作業や子孫も未確認である。実 GitHub、GUI、実資格情報と全 Issue の検証は範囲外。

上記ハッシュ・ディレクトリ一覧・ログ不存在を読み取り確認。新規実行器以外の変更・テスト実行はしていない。`Get-FileHash` 等は終了0。新たな失敗は認識していない。
