# Issue #4 同期修正の現行版への限定統合依頼

- 担当 `/root/project_survey`。作業場所 `D:/Projects/GithubIssueWidget`、共有 run `D:/Projects/GithubIssueWidget/docs/agent-runs/20261121-orchestration-continuation/`。所有は製品 `widget/src/main.tsx` と `widget/src/main.test.tsx`、結果 `sync-integration-result-01.md` のみ。他担当も存在するため、それ以外の既存変更・未追跡・旧 worktree を保護し、他者の変更を戻さない。
- 承認された `sync-integration-plan-02.md` SHA256 `0F502ADC83AC07E70385858763F133F08FE9149A12A55D951FB1E8D1D8C75BAE`、`sync-integration-plan-review-02.md`、`sync-practical-test-result-review-01.md` を読む。現 checkout の HEAD は `5e74e1642fd18fe630dce0e684afd853721276ae`。開始前に HEAD、git status、2ファイルの差分・所有、旧 worktree の利用可能な状態を確認。
- 製品 main SHA256 `01A7E8A6961BE2DC4F00D96F9369098321EF5137F6959650931E36ABFB357658`、test `6D1F45AFDBFC3BD66071C81FD7BA0025A186BD6A1CC0CE89022D06FB19B7D1E1`。隔離・試験済み main `6FCF2A34145450C5029B7FF80DDB3EC1DA5E27173AAB84E57F16D6D54C0E6A3E`、test `66E118A5A7062854C23BE405D2E9578C974FA283BEA332F2A7C51DCA1BFFD13C`。いずれか不一致、対象の他担当による編集・所有、予期しない競合を確認した場合は変更せず結果・質問を確定し管理に通知する。
- 条件を満たす場合のみ同期修正とそのテストを対象2ファイルに最小差分で適用。適用直前の現行を読み、適用後に全文 SHA と試験版の一致・差分の範囲・他ファイルを確認。旧 worktree・branch・PR や main.rs、Git 状態を操作しない。
- 統合後の実行は別途合意された安全な方式のみ。未実行であれば隔離版39件成功・対象ハッシュ一致をその範囲で記録し、製品全体・GUI・実 API の成功としない。結果に全変更対象 SHA・削除対象・確認コマンドと終了結果・未確認を記録し、確定後は書込みを止めパスのみ通知。別担当が最終版を独立レビューする。
