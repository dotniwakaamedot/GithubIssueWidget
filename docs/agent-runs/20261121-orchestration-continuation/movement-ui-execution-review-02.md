# 移動 UI 限定試験結果の独立レビュー 02

- 確認 `/root/plan_review`、実行者 `/root/project_survey` と別担当。依頼 `movement-ui-execution-review-request-01.md` と担当・結果名訂正 `movement-ui-execution-review-request-02.md`。
- 期待・開始/終了 SHA256: 結果 `11EC361A3510CBAE2667EAA55BC9862D1FF620B4BA3761EC6884C16E6974C91D`、stdout `72587E94D0E8A98A652982EB1330DF8D724163C974FAAD50A34939A93F1A8CF0`、隔離 test `3123D7B77EA1E283082FDFF198BA882ACA1E0AAF216E9D7D567F20B55D615B63`、製品 main `6FCF2A34145450C5029B7FF80DDB3EC1DA5E27173AAB84E57F16D6D54C0E6A3E`。実行 fixture の test も同 SHA。削除対象なし。
- 判定: **固定版の新規1ケースの限定試験結果を承認**。blocking なし。製品へのテスト適用は対象ファイルの現在版・所有を確認したうえで可能。実 UI、GitHub、アプリ全体の完了承認ではない。

| 項目 | 根拠・限界 |
| --- | --- |
| 件数 | stdout に Vitest 5.1.1、1ファイル成功、1 passed・39 skipped と終了行。未実行39件を今回の成功に含めない。選択テスト名と直接 PowerShell 終了0は実行者の確定記録に依存。 |
| 対象版 | 隔離案 test と実行 fixture test は同じ SHA、製品 main は依頼版と一致。新規ケースが移動不明後の一覧失敗、カード消去、両エラーを検査することは `movement-ui-regression-review-01.md` で確認済み。 |
| 製品 test への適用 | 製品 test の基準 `66E118A5A7062854C23BE405D2E9578C974FA283BEA332F2A7C51DCA1BFFD13C` と隔離版の差は新規1件のみ。着手直前に基準・所有を再確認し、変更があれば保護・再評価する。適用後に SHA が試験版と一致することを検証し、別担当が差分を確認する。追加変更や懸念がなければ同じ1件の再実行を一律に要求しない。 |
| 未確認 | 終了コード・選択コマンド・環境は実行者報告に依存。子孫終了、実 OS ウィンドウ・認証・GitHub・実競合は未確認。 |

作業場所 `D:/Projects/GithubIssueWidget`。依頼・結果・stdout と固定版の SHA を照合し、対象ファイル・既存試験・隔離案との差分と以前の静的レビューを確認。テスト再実行、GUI、実 GitHub・製品編集は行っていない。対象版が変われば再評価する。
