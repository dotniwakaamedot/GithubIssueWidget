# 移動競合3件の新規 target による再ビルド依頼

- 担当 `/root/existing_work_review`。共有 run `D:/Projects/GithubIssueWidget/docs/agent-runs/20261121-orchestration-continuation/`。既存 `pagination-test-execution-01/` の専用 fixture の移管範囲を所有し、他担当・製品・既存 target と既存ログを保護。競合があれば作業しない。
- `movement-rust-execution-result-02.md` は直接終了0でも `running 0 tests`、79 filtered out で対象3件は**未実行**。親から別担当調査の結果、新テスト名が既存バイナリに含まれず fixture の更新時刻がバイナリより古いこと、新規専用 target による確実な再ビルドが推奨された。依存関係の原因は固定版の調査結果に基づき、推測を確定しない。
- 既存製品/fixture の main SHA `AEFF1E65AD93EFA04773CEBBB9736A6BDC5B9938DB9FAF937DD88B3D9CBFE86F`、fixture の承認 movement SHA `88BF21BB45C077DFDA592D906C753D168C5641E35EB12324D4ACF1652374695B`、製品 movement・保存済み元版 SHA `ED49F4BA52C6887BCE427E4A6664D335092A1C3311E1B1ED61B4CE322791ABA5` と対象所有・旧ログを確認し、変更があれば中止する。
- 新規かつ固有の Cargo target と新規ログを用い、既存 target・registry・他の証跡を削除・上書きせず、`--locked --offline --bin github-issue-widget` で `movement::tests::post_write_competing_` の新規3件だけを選ぶ。環境・出力・direct プロセスの観測は承認済み `movement-rust-runner-review-01.md` と同じ範囲に保ち、ランナー変更は target と新ログ先の最小範囲に絞って内容・SHA を固定する。変更版の別担当確認後に管理が開始を通知する。実 GitHub・credential・GUI は使わない。
- 実行時は3件の正確な名前・各成否・終了コード・ログ完全性を確認。0件・不明を成功としない。同じ失敗2回なら別担当調査、既存成功試験は無理由に再実行しない。固定版・コマンド・結果・未確認を固有結果へ記録しパスを通知。
