# Issue #8〜#10 調査時の出力失敗の独立分析依頼

- 担当予定 `/root/workflow_output_analysis`。親 `/root` が起動。共有 run `D:/Projects/GithubIssueWidget/docs/agent-runs/20261121-app-completion-restart/`、作業場所 `D:/Projects/GithubIssueWidget`。
- 対象: `/root/workflow_audit` の `questions/09-question-01.md` に報告された出力切り詰め2回。ユーザーの同種2回時の別担当調査の指示により、元操作の再実行はせず、実行コマンド・終了結果・記録・既存教訓を照合して原因、未確認、具体的な回避策と未検証の効果を記録する。
- 所有: 新規 `workflow-output-analysis-01.md` のみ。製品・共有 plan・失敗ログ・元の担当結果や他者の変更を変更せず、欠落した出力の復元を主張しない。記録更新は管理へ引き継ぐ。対象と出力量を絞り、同種失敗2回なら停止し親へ報告。結果の UTF-8 実データを読戻し、確定後パスのみ通知。
