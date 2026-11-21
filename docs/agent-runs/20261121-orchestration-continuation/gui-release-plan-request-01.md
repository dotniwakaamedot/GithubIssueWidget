# 現行版 release GUI 起動の具体方式の検討依頼

- 担当 `/root/existing_work_review`、共有 run `D:/Projects/GithubIssueWidget/docs/agent-runs/20261121-orchestration-continuation/`。結果は `gui-release-plan-01.md` のみ。独立した実行方式の検討であり、起動しない。
- `gui-f078-next-investigation-01.md`、F-078 と旧合成子の未確認、現行版・通常の利用可能な対話環境と実行ツールの性質を参照。release の現行ビルドから空設定の直接起動・ウィンドウの観察・通常終了・直接子の終了を実現する具体的な版・環境・専用領域・秘密の非出力・期限と失敗時の扱いを提案。空設定で credential/network に到達しないかは並行の `gui-empty-startup-audit-01.md` を待つ条件とする。
- 未追跡子孫・旧プロセスの不明、WebView2・Windows セッション・OS credential store の共有範囲を明確にし、無関係なプロセスを操作せず、実測可能な保証と残余リスクを区別する。独自監督器完成や未確認 VM を一律に前提にしない。開発モードは別のプロセス・検証として扱う。
- 読取り・計画のみ。ビルド・GUI・実資格情報・GitHub 書込みはしない。対象と所有・必要なレビュー・操作手順と終了/失敗判定、条件未達時の独立作業を記録しパスを通知。
