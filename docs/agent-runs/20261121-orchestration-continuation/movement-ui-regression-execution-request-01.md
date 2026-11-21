# 移動後一覧失敗の UI 1ケース実行依頼

- 担当 `/root/project_survey`、共有 run `D:/Projects/GithubIssueWidget/docs/agent-runs/20261121-orchestration-continuation/`。新規 `movement-ui-test-execution-01/` と `movement-ui-execution-result-01.md` を所有し、既存 fixture・製品・Git・他担当の変更を保護。新規領域が既に存在すれば上書きしない。
- `movement-ui-regression-review-01.md` の固定版承認を確認し、製品 main `6FCF2A34145450C5029B7FF80DDB3EC1DA5E27173AAB84E57F16D6D54C0E6A3E`、製品 test `66E118A5A7062854C23BE405D2E9578C974FA283BEA332F2A7C51DCA1BFFD13C`、隔離 test `3123D7B77EA1E283082FDFF198BA882ACA1E0AAF216E9D7D567F20B55D615B63` を開始前に照合。不一致・所有不明は中断。
- 前回承認された `sync-practical-test-plan-01.md` と同種の独立プロジェクト、依存の通常コピー、秘密を渡さない専用環境・TEMP・ログを用意。以前の fixture、ログ、token は上書き・削除せず、共有依存を書換えない。新規 test と固定 main・設定・依存の入力版とコピーを確認。
- Vitest の対象ファイルと `clears stale cards and shows both errors when refresh after an uncertain move fails` に限定し1ケースのみ一度実行。0件、複数件、失敗、直接終了コード、ログ・子孫終了の未確認を区別し、予期しない条件を成功としない。前回成功の39件を無理由に再実行しない。実 GUI・GitHub・credential・旧プロセスを操作しない。
- 実行手順・コマンド・入力 SHA・環境変数名・各結果・ログ・失敗と未確認を固有結果に記録し、確定後は書込み停止してパスを通知。期待と異なればテストを緩めず報告し、別担当レビューを受ける。
