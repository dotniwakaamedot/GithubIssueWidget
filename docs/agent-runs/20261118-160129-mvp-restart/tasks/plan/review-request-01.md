# 計画の独立レビュー依頼 01

- run_id / task_id / request_revision: 20261118-160129-mvp-restart / P01 / 01
- 作者: /root/restart_planner。レビュー担当: 親が指定する作者以外の担当 ID。
- 共有 run: D:\Projects\GithubIssueWidget\docs\agent-runs\20261118-160129-mvp-restart
- 対象: plan.md revision 01、SHA256 E1C053A0A545C8F005E6C8EEB602F265868128202AA930C859769D310FBFFD69
- 基準候補: a4ac97286785a2a5ba0d070a909b58003fc49ff1、main。変更内容は対象版全体。
- 一次資料: ユーザー原文は plan.md、Issue #1〜#7 は GitHub 原文、AGENTS.md、docs/agent-workflow.md、docs/issue-workflow.md、docs/adapters/git-github.md。一次資料と現在の Git / GitHub 状態を独立照合。
- レビュー: ユーザー原文の脱落、Issue 条件、依存・所有競合、権限と着手前照合、対象版と実行可能性、Windows 実機・セキュリティ・失敗時検証、未確認の扱いを確認。blocking と非 blocking を分離し、承認は該当 SHA256 限定。
- 所有範囲: tasks/plan/review-01.md のみ。計画、アプリ、GitHub 状態、他担当の成果は変更禁止。書きかけは同じディレクトリの .tmp から確定する。
- 検証: 対象ハッシュを計算して一致を確認し、取得時刻、コマンド、証拠、未確認を結果へ記載。結論、指摘 ID、根拠、是正条件を明示する。
- 制約: 同種の失敗が2回発生したら即時中止・親へ報告。認可されない外部書込み、push、PR、merge、削除は禁止。質問は共有 run の質問ファイルで行う。
- 結果: D:\Projects\GithubIssueWidget\docs\agent-runs\20261118-160129-mvp-restart\tasks\plan\review-01.md