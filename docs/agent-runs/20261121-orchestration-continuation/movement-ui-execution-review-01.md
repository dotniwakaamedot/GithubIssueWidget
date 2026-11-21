# 移動 UI 限定試験結果レビュー 01

- 確認者 `/root/planning/plan_review`、実行者 `/root/project_survey` と別担当。開始・終了 SHA256 は結果 `11EC361A3510CBAE2667EAA55BC9862D1FF620B4BA3761EC6884C16E6974C91D`、stdout `72587E94D0E8A98A652982EB1330DF8D724163C974FAAD50A34939A93F1A8CF0`、実行 test `3123D7B77EA1E283082FDFF198BA882ACA1E0AAF216E9D7D567F20B55D615B63`、製品 main `6FCF2A34145450C5029B7FF80DDB3EC1DA5E27173AAB84E57F16D6D54C0E6A3E` と依頼値が一致。実行 fixture main も同 SHA。削除対象なし。
- 判定: **固定版の限定試験結果を承認**。blocking なし。これを UI 全体、実 GUI・GitHub、子孫終了や Issue #6 完了の承認とはしない。

## 根拠と限界

- stdout は対象 `src/main.test.tsx` の 40 件中 1 passed・39 skipped、ファイル1件成功と完了行を示す。個別ケース名とコマンドは stdout にないため、指定フィルターと直接 PowerShell 終了コード0は実行者の確定結果に依存する。出力された件数と結果は報告と一致。
- 既存の静的レビュー `movement-ui-regression-review-01.md` は固定 test 版が「移動不明後に再取得も失敗した場合、古いカードを消し両エラーを表示する」ケースであることを確認。今回のログを特定ケースの実行に結びつける証拠は実行者のコマンド記録であり、独立した完全なプロセス記録はない。
- 入力コピー・依存・環境変数・実行時の前後一致は実行者記録に依存し、再実行や manifest 全件の独立照合は行っていない。stderr は0 bytesとの記録。Vitest worker・esbuild の子孫終了と依存全体の外部通信、実画面・credential・実競合は未確認。
- 39件は今回未実施。適用する際は実際の製品 test の所有・基準と差分を再確認し、製品テストに取り込んだ版を別途レビューする。

作業場所 `D:/Projects/GithubIssueWidget`。依頼、結果、標準出力、先行レビュー、対象ハッシュを読取り照合。試験再実行・GUI・外部操作・製品変更はしていない。
