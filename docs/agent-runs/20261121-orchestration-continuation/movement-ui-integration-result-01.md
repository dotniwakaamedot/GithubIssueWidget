# 移動後一覧失敗 UI テストの統合結果

担当 `/root/project_survey`。`movement-ui-integration-request-01.md` に従い、固定版の独立レビューと1ケース成功を確認し、製品 `widget/src/main.test.tsx` に新規1ケースを追加した。既存の同期テストと他担当の変更は保持。

- 適用前の製品 test SHA `66E118A5A7062854C23BE405D2E9578C974FA283BEA332F2A7C51DCA1BFFD13C`、製品 main `6FCF2A34145450C5029B7FF80DDB3EC1DA5E27173AAB84E57F16D6D54C0E6A3E`、隔離 test `3123D7B77EA1E283082FDFF198BA882ACA1E0AAF216E9D7D567F20B55D615B63` を照合。適用前の `git diff --no-index` は新規1ケースのみ、差分あり終了1。
- 適用後 test SHA は `3123D7B77EA1E283082FDFF198BA882ACA1E0AAF216E9D7D567F20B55D615B63` で試験版と一致し、適用後の no-index 比較は終了0。main と既存未コミット `main.rs` SHA `61595B68317E15FDD8F2BED88B920A468D5516740E3D3124710384709C52FC24` は不変。SHA と適用コマンドは終了0。
- 当該版の隔離ケースは `movement-ui-execution-result-01.md` で1件成功、39件 skipped・直接終了0。対象版・結果は `movement-ui-execution-review-02.md` で独立承認。指示どおり再実行せず、実製品での新たな実行結果はない。

実 GUI、実 GitHub、実競合、子孫終了は未確認。製品機能コード・旧 worktree・Git 状態・他者資産は変更せず、削除対象なし。今回失敗なし。固定版と差分の独立最終レビューが必要。
