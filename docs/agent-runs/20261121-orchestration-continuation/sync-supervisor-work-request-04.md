# テスト監督器の担当移管・実装依頼 04

- 担当 `/root/gui_failure_investigation`。前担当 `/root/project_survey` は `test-supervisor-result-02.md` で部分実装と8ファイルの固定 SHA を報告し、稼働一覧上は終了している。共有 run `D:/Projects/GithubIssueWidget/docs/agent-runs/20261121-orchestration-continuation/`、作業場所 `test-supervisor-01/`。開始時に結果記載の全8ファイル・未追跡と SHA を照合し、不一致または所有が不明なら変更しない。
- `sync-supervisor-work-request-02.md` と `sync-supervisor-backend-investigation-01.md`、`test-supervisor-result-02.md` の未完了を参照。予約・Job・生成の backend 接続、入出力・期限・終了と復旧、失敗境界の合成試験を実装し、安全な限定テストへの前進を優先する。実装不可能または未確認の点は具体的な根拠と次の手順を記録し、動作済みと主張しない。
- 所有は `test-supervisor-01/` と新結果 `test-supervisor-result-03.md`。既存製品・依存・旧 fixture や他者の変更を戻さない。実プロセスを生成する合成試験・Vitest・GUI・外部通信は、対象版と実行方式を別担当がレビューし、開始条件を満たすまで行わない。静的構文等は可能な範囲で実施。
- 結果に未追跡を含む変更ファイルの SHA・削除・検証コマンドと終了コード・残課題を記し、作業終了時に書込みを止めてパスを通知。対象版変更後に旧承認を流用しない。
