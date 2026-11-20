# 進行レビュー依頼 02

- 進行レビュー担当 `/root/progress_review`。共有 run D:\Projects\GithubIssueWidget\docs\agent-runs\20261118-160129-mvp-restart。
- 対象期間: 進行レビュー01以降、G01 再開、#2 記録・作業場所作成、I02 依頼・実装途中結果とレビュー依頼まで。
- 一次資料: plan-02.md SHA256 A6CB37DC369D69B334564235CD00C3AF284250109D4A56BE63C6CC7B104FE990、plan.md の現状、process/review-01.md、process/issue-registration-02.md、tasks/G01/request-01.md〜request-03.md、questions/G01-question-01.md、tasks/I02/request-01.md と request-02.md、tasks/I02/management-01.md と management-02.md、tasks/I02/result-01.md SHA256 F0AEAD322062EED3054B1A51B0A44DC05EB85E257A3A2EE59026F99C9112BE03、tasks/I02/review-request-01.md、tasks/issue-02/workspace-01.md。
- 目的: 依頼の参照・対象の整合不備2件、文字破損2回、画面移動自動操作2回の対応と3回目停止の運用、成果物所有・対象版・証拠の固定、Issue 記録・重複防止、未完了状態を独立に点検。事実と原因仮説を区別し、同じ危険な操作を試さない。レビューはコードの技術的合否を代行しない。
- 提案がある場合は各提案に識別キー `20261118-160129-mvp-restart:review-02.md:<提案ID>` をつけ、docs/agent-workflow/templates/issue-proposal.md に従い process/issues/ に未使用の別名で1提案1ファイルを作成。前回提案 P01 の重複と Issue #8 を照合し、同一内容の新規提案にしない。採否・Issue 登録は管理が判断する。
- 所有範囲: `process/review-02.md` と必要な `process/issues/` の新規ファイル・質問のみ。親ディレクトリと宛先不在を先に確認し .tmp から確定。書込み失敗時は後続操作停止。製品・他担当の成果・GitHub 更新は禁止。成果ファイルのパスを通知する。
- 追加指示「失敗を3回繰り返すまでに緩和します」に従い、初回を含め同種3回発生で中止・親に報告。すでに上記3種類は各2回であるため再発に特に注意。