# 移動競合3件の限定試験結果

担当 `/root/existing_work_review`。`movement-rust-rebuild-review-01.md` の固定版承認と `movement-rust-rebuild-decision-01.md` に基づき、新 target で追加3ケースだけを1回実行した。前回の0件収集を成功扱いせず、別担当の調査と新しい target の方式レビューを経て実行した。

## 結果

- 直接実行は `powershell.exe -NoProfile -ExecutionPolicy Bypass -File D:\Projects\GithubIssueWidget\docs\agent-runs\20261121-orchestration-continuation\movement-rust-test-execution-02\run-movement.ps1`。ランナーの direct cmd `exit=0`、`deadline_exceeded=False`。`exec_command` session `87382` は完了結果を受領し、stdout/stderr の排出待ちもランナーの終了に至った。子孫全体の OS レベルの終了は追跡していない。
- stdout は `running 3 tests`、`movement::tests::post_write_competing_state_change_reports_partial`、`...status_addition_reports_partial`、`...target_removal_reports_partial` の各 `ok`、`3 passed; 0 failed; 0 ignored; 79 filtered out` を記録。他の成功済み試験は再実行していない。
- stderr は新 target でのコンパイルと `Finished test profile`、test binary の起動を記録。stdout 356 bytes SHA256 `CB0CB8A4903EB8DB2FD37D2216BB393A69F4C92020923EE22E6D7DC9CDC8C26B`、stderr 8961 bytes SHA256 `210DC978063192C06327A497462101F7F1013B8E23BDF32A5C9F469BCA58F440`。stderr は末尾のみ表示し、全内容の独立レビューは未実施。

## 対象と保護

開始前・終了後の fixture movement SHA `88BF21BB45C077DFDA592D906C753D168C5641E35EB12324D4ACF1652374695B`、fixture main `AEFF1E65AD93EFA04773CEBBB9736A6BDC5B9938DB9FAF937DD88B3D9CBFE86F` は一致。製品 movement は `ED49F4BA52C6887BCE427E4A6664D335092A1C3311E1B1ED61B4CE322791ABA5` のまま、製品 main は `AEFF1E65AD93EFA04773CEBBB9736A6BDC5B9938DB9FAF937DD88B3D9CBFE86F`。レビューされた runner SHA `0583FA81C1F27753E4BDC6A8BD0DCF59714FE387FD9CB0266CA0186B3429E1BA`、cmd `321976316709C7B3AC2742D203F4E508179E739A44D9C73E9CE261719179B19E` を開始前に照合した。

新専用 `movement-rust-test-execution-02/target` と同ディレクトリの新ログを使用。既存 pagination target・ログと製品を上書きせず、既存専用 Cargo home は再利用し Cargo による書込み可能性がある。子プロセスの環境を限定し合成 loopback のみを試験対象としたが、OS credential store と全依存の非通信を証明するものではない。実 GitHub・GUI・実資格情報は操作していない。

この3ケースは模擬された最終不一致と部分反映の警告を検証する。実 GitHub の同時更新・実 UI、Issue #6 の全要件、製品への統合は未確認。製品に反映する場合は元 SHA・他担当の変更を再確認し、固定版の結果を別担当がレビューした後に最小差分を統合する。新しい操作失敗は認識していない。実行領域への書込みを停止し、結果・ログ・target は保持する。
