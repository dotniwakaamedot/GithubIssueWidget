# 進行レビュー依頼 01

- 対象期間: 前回の文字化けによる中止から、再開後の計画 revision 01 発行まで。
- 共有 run: D:\Projects\GithubIssueWidget\docs\agent-runs\20261118-160129-mvp-restart
- 対象資料: plan.md（SHA256 E1C053A0A545C8F005E6C8EEB602F265868128202AA930C859769D310FBFFD69）、tasks/plan/review-request-01.md、ユーザー原文、AGENTS.md、docs/agent-workflow.md。前回の経緯は会話を参照する。
- 目的: 反復失敗の停止条件、再開時の旧担当停止、文字コード処理、依頼の共有 run 化、計画の独立レビューと Issue 着手前の権限・照合の遵守を確認。事実と原因仮説を分ける。
- 進行上の問題があれば根拠、優先度、具体案、期待効果、費用・リスク、次担当、検証方法を提示。Skill 提案時は既存 Skill を調べ、提案は導入権限を意味しない。
- 提案がある場合、各提案の識別キーを `20261118-160129-mvp-restart:review-01.md:<提案ID>` とし、docs/agent-workflow/templates/issue-proposal.md に従い process/issues/ に別ファイルの Issue 本文案を保存する。提案なしなら理由を明記する。
- 所有範囲: process/review-01.md、必要に応じ process/issues/ の提案ファイル。対象外の書込み、アプリ変更、GitHub 変更、push、PR、merge、削除は禁止。各ファイルは .tmp から確定する。
- 同種の失敗を2回繰り返したら中止し親へ直ちに通知する。結果は確定ファイルパスだけを通知する。