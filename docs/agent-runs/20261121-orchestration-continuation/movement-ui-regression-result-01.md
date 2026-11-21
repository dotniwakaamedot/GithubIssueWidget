# Issue #6 移動不明後の一覧失敗 UI ケース

担当 `/root/project_survey`。`movement-ui-regression-work-request-01.md` に従い、製品 test を新規 `movement-ui-isolated-01/main.test.tsx` に独立複写し、新規1ケースだけ追加した。製品・他担当の変更は編集していない。

- 開始前後で製品 `widget/src/main.tsx` SHA `6FCF2A34145450C5029B7FF80DDB3EC1DA5E27173AAB84E57F16D6D54C0E6A3E`、`widget/src/main.test.tsx` `66E118A5A7062854C23BE405D2E9578C974FA283BEA332F2A7C51DCA1BFFD13C`。コピー直後は後者と一致、変更後の隔離 test SHA `3123D7B77EA1E283082FDFF198BA882ACA1E0AAF216E9D7D567F20B55D615B63`。
- ケース名 `clears stale cards and shows both errors when refresh after an uncertain move fails`。初回一覧は旧カードを返し、Done 移動が結果不明で失敗、2回目の一覧も失敗するモック。表示から旧カードが消えること、移動と一覧の両エラーが `role=alert` に現れること、再取得回数2と対象・revision・updated_at を含む移動要求を検査する。
- `git diff --no-index` で差分はこの1ケースの追加のみ。終了1は差分ありを意味する。PowerShell のコピーと SHA 確認は終了0。新ケース・型検査・ビルド・実 GUI・GitHub・資格情報は**未実行**。実際に期待を満たすか未確認。

先行の隔離 Vitest 方式は参考になるが、旧 fixture はログ、dist、専用 TEMP cache と Vitest token を保有しており、上書きせずに新固定版を配置する手順・出力先・対象ケース選択と依存の不変性・直接終了を別途固定しレビューする必要がある。39件の再実行は依頼していない。生成された隔離ファイルは保持、削除対象なし。作業中の失敗なし。独立レビューと限定実行後、製品 test への適用を別途判断する。
