# Issue #6 移動方式と回帰の独立レビュー依頼

- 担当 `/root/existing_work_review`、元監査 `/root/issue_survey` と別担当。共有 run `D:/Projects/GithubIssueWidget/docs/agent-runs/20261121-orchestration-continuation/`。所有は `movement-audit-review-01.md` のみ。既存製品と未コミット変更を保護する。
- `movement-audit-01.md` と Issue #6 原文、現在の `movement.rs`・関連する画面コードとテストを固定 SHA で照合。単一 PATCH の「可能な限り」という要件と、他者のラベルを守る個別操作の競合・部分反映のトレードオフを評価。既存の全6方向・欠落ラベル・失敗等の検証が何を実証し、何が未実行か明確にする。
- 競合回帰候補等をコード・要件に照らして具体的に検討し、確定欠陥、弱い証拠、実 GitHub 検証に依存する事項を分け、最小限の次の実行可能な作業を示す。読み取りのみでテスト・API・製品変更を行わない。
- 開始・終了時の対象版 SHA、終了結果・失敗と未確認を結果に記録。元監査版から UI が更新されている場合は旧承認を流用しない。確定後にパスを通知する。
