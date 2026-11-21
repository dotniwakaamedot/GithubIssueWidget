# 同期修正案の修正依頼 02

- 担当 `/root/project_survey`、作業場所 `D:/Projects/GithubIssueWidget/docs/agent-runs/20261121-orchestration-continuation/sync-isolated-01/`、共有 run `D:/Projects/GithubIssueWidget/docs/agent-runs/20261121-orchestration-continuation/`。前担当は同一で、旧対象への書込みが停止していることを確認してから再開する。他担当・既存変更を保護する。
- 対象は `sync-isolated-review-01.md` の B1。隔離 `main.tsx` SHA256 `02EF79F6671D9F3F416B4D8B57AEBE4D52A0040932C8B2348CB14BF7E22BFD37`、`main.test.tsx` SHA256 `C0E248CF39C71C34AE9A8485CB29BD8AE9FAB82EE7636D7BE8E8D22C41A54D35`。開始前に照合し、不一致なら変更しない。
- 実時間の経過計測を単調時計へ変更し、壁時計の前後変更で同期周期が変わらない回帰試験を追加する。タイマーを使うテストの偽時計の扱いに注意し、60秒周期・長時間処理・旧応答遮断を維持する。OS スリープ時の挙動は推測で保証しない。
- 対象ファイルと新しい結果 `sync-isolated-result-02.md` のみ所有。旧確定結果は編集しない。テスト・GUI・実 GitHub・資格情報・合成子・製品ファイルへの適用は行わず、テスト未実施とする。出典・変更後 SHA、差分と未確認を新結果に記録し、完了後は書込みを停止してパスのみ通知する。独立した新版レビューを受ける。
