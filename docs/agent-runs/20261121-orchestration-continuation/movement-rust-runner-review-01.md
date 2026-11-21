# 移動競合3件の実行器レビュー

2026-11-21、確認者 `/root/issue_survey`、実装者 `/root/existing_work_review` と別担当。`movement-rust-runner-review-request-01.md` と担当訂正 `movement-rust-runner-review-request-02.md`、`movement-rust-execution-preflight-01.md` を照合。実行・fixture 差替え・既存ファイル変更はしていない。

## 判定

**限定された実行器と preflight 案を、以下の実行前条件付きで承認。** これはコードの読取りレビューであり、3件の実行結果、実 GitHub・GUI・資格情報や全子孫終了の承認ではない。

| 観点 | 確認と条件 |
| --- | --- |
| 所有・保護 | `pagination-fixture-handoff-01.md` と `movement-rust-fixture-handoff-decision-01.md` に前担当の書込み停止と移管あり。前担当が確認した直接セッションは終了、OS レベルの全子孫は未確認。実行前に未知の同時使用、入力 SHA と親の開始通知を再確認する。 |
| 元の movement の保護 | preflight は専用の `original-movement.rs` を排他的に作成し元と SHA を照合してから、レビュー済みの隔離 movement を fixture のみに反映する案。現時点で backup と新 stdout/stderr は不存在。これは手順の提案であり runner が自動検証するわけではない。実行担当は直前の版・不存在・保存後の一致を確認し、異常なら差替えず停止する。 |
| pagination 証拠 | `main.rs`、`pagination_tests.rs`、`Cargo.lock` は引継ぎ版と一致。既存 pagination ソースと4ログを上書きしない。Cargo は専用 target/cache を更新しうるので既存生成物の不変は主張しない。新ログ2件は `CreateNew`、再利用・上書きはしない。 |
| 3件の選択 | cmd は `cargo test --locked --offline --bin github-issue-widget movement::tests::post_write_competing_ -- --test-threads=1`。隔離 movement に一致する3つの `#[test]` を確認し、他の固定 src 3ファイルに同じ文字列はなかった。ただしフィルタは前方文字列一致であって `--exact` ではない。実行後に名前3件・収集数・各成功・想定外の混入なしをログで確認する。成功済み pagination を繰り返さない。 |
| 限定環境・終了 | script は子環境を消去し明示変数、専用 Cargo home/target/TEMP/profile、VS と固定 Rust を使う。stdout/stderr は新規ファイルへ同時排出。直接 cmd の終了コードを取得し、900秒経過は報告して同一ハンドルを監視する。ログ待ち・未知の子孫や OS 障害は未保証。終了不明を成功にせず無関係な PID を停止しない。環境 allowlist だけで OS 資格情報・全依存の通信が隔離されたとは証明できない。 |
| ケース範囲 | 各テストは loopback の合成6応答、更新後の競合に部分反映の警告が含まれることを検証。実サーバーの同時更新を証明しない。件数0・終了不明・ログ欠落は失敗または未完了と区別する。 |

## 対象版と確認

- preflight SHA256 `D9D7B8CB1905E150CE3252ADD63B6DA3B04C608D666E8B1B26C024FE881099D9`。
- `run-movement.ps1` `AF6C6C39AC166180D0B7D7B750562F5366CFA4797A0D65E907C03791314E07D3`、`run-movement.cmd` `321976316709C7B3AC2742D203F4E508179E739A44D9C73E9CE261719179B19E`。
- 隔離候補 movement `88BF21BB45C077DFDA592D906C753D168C5641E35EB12324D4ACF1652374695B`、差替え前 fixture movement `ED49F4BA52C6887BCE427E4A6664D335092A1C3311E1B1ED61B4CE322791ABA5`。以上の SHA はレビュー開始・終了時に一致。
- fixture main `AEFF1E65AD93EFA04773CEBBB9736A6BDC5B9938DB9FAF937DD88B3D9CBFE86F`、pagination test `BF690E818EB182FE16395E0AE3946B043D41DADD2B75235EFEACDC4873F4DE3D`、Cargo.lock `2486C9E87F8511A2CBB3C09177E278139409C68A308FD48642327EDDF1D88E7F` は開始時に引継ぎ版と一致。`Get-Content`、限定 `rg`、`Get-FileHash`、`Test-Path` は終了0。操作失敗は認識していない。
