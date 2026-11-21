# 同期修正の隔離準備依頼

- 担当 `/root/project_survey`。共有 run `D:/Projects/GithubIssueWidget/docs/agent-runs/20261121-orchestration-continuation/`。作業場所も同 run 内の新規 `sync-isolated-01/` に限定。現在 checkout と旧 worktree の製品ファイル・Git 状態・既存キャッシュは変更しない。他担当も作業中であり他者の変更を戻さない。
- 対象は承認済み `sync-fix-plan-02.md` SHA256 `F647C117C3748DA4C1704E2FF89D4015D1422802D3D922F6AFAEF227DBE258CD`、レビュー `sync-fix-plan-review-02.md` と親判断 `sync-isolated-proposal-01.md`。実装方針・要件・回帰条件は計画を参照。
- 依存: `sync-ownership-result-01.md` が旧担当停止・PR 網羅を未確認とするため、現 checkout の製品変更と統合は保留。隔離準備のみ着手。出典の `widget/src/main.tsx` SHA256 `01A7E8A6961BE2DC4F00D96F9369098321EF5137F6959650931E36ABFB357658` と `widget/src/main.test.tsx` SHA256 `6D1F45AFDBFC3BD66071C81FD7BA0025A186BD6A1CC0CE89022D06FB19B7D1E1` を作成直前に照合。不一致なら中断し管理に通知。新規領域が既存なら上書きしない。
- 所有範囲は隔離ディレクトリの2つの複写ファイルと `sync-isolated-result-01.md` のみ。コピーの開始時ハッシュを確認し、同期の実装と回帰テストを準備し、画面文言も修正。README は読取りで確認し、修正が必要な点だけ結果へ報告する。元ファイルへ適用しない。
- GUI・実 GitHub・資格情報・合成子・テストは実行しない。テスト方式は別途承認を待つ。完了後は出典と隔離の SHA256、変更内容、未検証、削除対象なし、実行したコマンドと結果を記録し、書込みを停止して確定結果のパスのみ通知する。成果は別担当のレビューと後続統合を要する。
