# 移動失敗後の一覧失敗 UI 回帰テスト依頼

- 担当 `/root/project_survey`。共有 run `D:/Projects/GithubIssueWidget/docs/agent-runs/20261121-orchestration-continuation/`。新規 `movement-ui-isolated-01/` と結果 `movement-ui-regression-result-01.md` のみ所有。製品・他担当の変更を戻さない。
- `movement-regression-plan-01.md` と承認 `movement-regression-plan-review-01.md` を参照。製品 main SHA `6FCF2A34145450C5029B7FF80DDB3EC1DA5E27173AAB84E57F16D6D54C0E6A3E`、test `66E118A5A7062854C23BE405D2E9578C974FA283BEA332F2A7C51DCA1BFFD13C` を確認し、異なれば中止。新規隔離コピーで、移動結果不明の失敗後に一覧の2回目も失敗する **新規1ケース**を追加し、古いカードが消え、移動と一覧の両エラーが画面に伝わることを検証する。現在の実装と異なれば失敗を保持し管理へ報告。
- 今回はコードの作成と静的確認。固定版の独立レビュー・限定実行依頼後、先に成功済みの39件を理由なく全再実行せず新ケースを選択して実行する。専用領域・ログ、前回成功した通常の隔離 Vitest 方式を再利用可能か記録。実 GUI・GitHub・資格情報は使わない。
- 隔離ファイルと変更前後の SHA・差分・実施コマンド/終了・未実施・未確認を結果に記録し、確定後書込みを止めて通知。製品本体への適用は別途判断。
