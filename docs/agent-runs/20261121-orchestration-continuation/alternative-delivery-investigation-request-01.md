# 製品完成の代替経路を独立検討する依頼

- 担当 `/root/issue_survey`。同期案作者 `/root/project_survey` と監督器作者 `/root/process_safety`・`/root/gui_failure_investigation` とは別担当。共有 run `D:/Projects/GithubIssueWidget/docs/agent-runs/20261121-orchestration-continuation/`。結果は固有 `alternative-delivery-investigation-01.md` のみ。
- `alternative-delivery-plan-01.md` SHA256 `0C10F18232A68CC092E100EC4AD19F53A7F0DF0318657A0621B16FAF7A8026CA` と独立レビューで blocking がないことを開始条件とする。`test-supervisor-result-03.md`、`sync-test-runtime-review-01.md`、`sync-test-environment-options-01.md`、同期案の結果と Issue の要件・プロジェクト指示を参照する。
- 監督器を完成させることが目的化している状況を是正し、製品 Issue の完成に向け通常の独立環境・限定テスト・現行コードへの実装・統合を進める具体策を独立に検討する。利用可能な実行環境、対象のモック・固定入力・既存の変更、権限と実際のデータ・プロセス共有を根拠に、実際に必要な保護と合理的な期限・終了/失敗判定、未知の範囲を記録する。あり得るあらゆる副作用の完全否定や存在未確認の VM を不要に前提としない。
- 各選択肢の具体的手順・適用できる要件・実行可能性・必要な隔離・実行後に分かることと未確認を比較し、最も現実的な次の一手を提案。既存ユーザー変更、旧 worktree、秘密と非公開情報を保護し、外部書込みや実資格情報は個別に確認する。GUI や実 GitHub の未検証を明確に残す。
- 読取り・調査のみ。テスト、プロセス起動、権限変更、GUI、実 GitHub・資格情報操作、製品・既存 fixture の変更をしない。対象版 SHA・実施操作・終了結果・失敗・未確認を記録し、確定後書込みを停止してパスを通知。別担当による独立レビューを要する。
