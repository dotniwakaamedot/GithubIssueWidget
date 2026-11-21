# 移動後一覧失敗 UI 回帰案レビュー

- 確認 `/root/plan_review`、作者 `/root/project_survey` と別担当。依頼 `movement-ui-regression-review-request-01.md`。
- 結果 SHA `BC338BAD99BE8C32D2CB30605CDF74C8829A472928CD3139E9D1E9A095E3D91E`、隔離 test SHA `3123D7B77EA1E283082FDFF198BA882ACA1E0AAF216E9D7D567F20B55D615B63` は開始・終了時に依頼値と一致。製品 main `6FCF2A34145450C5029B7FF80DDB3EC1DA5E27173AAB84E57F16D6D54C0E6A3E`、test `66E118A5A7062854C23BE405D2E9578C974FA283BEA332F2A7C51DCA1BFFD13C` は基準と一致。削除対象なし。
- 判定: **固定版の試験案を承認**。blocking なし。1件の限定実行と結果の確認が必要であり、試験成功・実 UI・実 GitHub・製品変更の承認ではない。

| 条件 | 静的確認 |
| --- | --- |
| Issue #6 と差分 | 製品 test との差分は1ケースの追加のみ。移動結果不明後、2回目の一覧取得も失敗し、古いカードが消え、移動と一覧の両エラーが alert として残ることを検査する。送信先・revision・対象・状態も確認する。 |
| 重複しない目的 | 既存の `refreshes after an uncertain move and keeps the error visible` は再取得が成功する経路。今回の失敗経路は異なり、既存39件を無理由に再実行する必要はない。 |
| 実行・保護 | 現行の mock/UI と矛盾しない期待値だが、実際に成功するかは未実行。専用の出力・正確な対象ケース・固定された入力・実行方法と直接終了を記録し、既存 fixture のログや token を上書きしない。限定1件が成功したら、追加の変更や具体的懸念なしに繰り返さない。 |
| 限界 | mock の1ケースであり、実 OS ウィンドウ・GitHub・本物の競合、他の Issue 要件を証明しない。結果を製品テストに取り込む場合は所有と基準を別途確認する。 |

`D:/Projects/GithubIssueWidget` で依頼・結果・計画・前回取得した Issue #6・現行 main/test と隔離差分を照合。`git diff --no-index` は差分あり終了1、SHA は開始・終了とも一致。試験・GUI・外部書込み・製品編集は未実施。対象が変われば新版を確認する。
