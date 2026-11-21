# 移動後一覧失敗の UI 試験の製品への追加依頼

- 担当 `/root/project_survey`。作業場所 `D:/Projects/GithubIssueWidget`、共有 run `D:/Projects/GithubIssueWidget/docs/agent-runs/20261121-orchestration-continuation/`。所有は製品 `widget/src/main.test.tsx` と新結果 `movement-ui-integration-result-01.md` のみ。他担当も作業中であり、既存未コミット変更・main.tsx・Rust・他成果を戻さない。
- `movement-ui-regression-review-01.md` と `movement-ui-execution-review-02.md` の承認を確認。実行済みの隔離 test SHA `3123D7B77EA1E283082FDFF198BA882ACA1E0AAF216E9D7D567F20B55D615B63`、現製品 test SHA `66E118A5A7062854C23BE405D2E9578C974FA283BEA332F2A7C51DCA1BFFD13C`、製品 main SHA `6FCF2A34145450C5029B7FF80DDB3EC1DA5E27173AAB84E57F16D6D54C0E6A3E` と current status・他担当の対象所有を直前確認。不一致・他者編集・競合があれば上書きせず中止する。
- 承認・実行済みの **新規1ケースのみ**を現 test に追加し、適用前後の差分と対象 SHA を読戻す。適用後が上記隔離試験版と一致することを確認し、既存同期テストを維持する。別ファイル・Git・旧 worktree は変更しない。
- 同一版で限定1件は既に成功し、追加の変更・失敗がない限り同じ試験を再実行しない。版・差分・確認コマンドと終了・過去の試験証拠・未確認を結果へ記録し、書込み停止後にパスを通知。別担当の最終版レビューを受ける。
