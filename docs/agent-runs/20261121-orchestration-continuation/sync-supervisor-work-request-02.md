# テスト監督器の backend 接続依頼 02

- 担当 `/root/process_safety`、作業場所 `D:/Projects/GithubIssueWidget/docs/agent-runs/20261121-orchestration-continuation/test-supervisor-01/`、共有 run `D:/Projects/GithubIssueWidget/docs/agent-runs/20261121-orchestration-continuation/`。旧版は `supervisor-backend-answer-01.md` と `sync-supervisor-backend-investigation-01.md` の6ファイルの SHA。作業前に停止と版を照合し、不一致なら変更しない。他担当の作業・元製品・旧 fixture を保護する。
- 独立調査が指摘した未接続部品を実際に結び、専用合成子と限定 Vitest の監督に必要な永続予約、生成時 Job 所属、保持ハンドルによる識別・所属確認、再開、期限、ログ排出、直接子と Job の終了確認、終了不明時の予約保持を実装する。構造体と API の型・エラーを明確にし、無関係な PID や旧プロセスを操作しない。
- 正常・期限超過・生成/所属/再開/ログ/終了の失敗、監督者停止等の合成障害テストも具体的に実装し、各ケースがどの境界に達したか・何をもって成功/不明とするかを記録できるようにする。秘密のない専用入力に限定し、外部・loopback や Credential Manager を呼ばない。OS 障害・breakaway の無条件保証を主張しない。
- 現時点は実装と静的検査まで。実プロセスを生成する合成テスト・Vitest・GUI・実 GitHub 操作は、版別レビューで実行条件を確認し正式な実行依頼が出るまで行わない。一般の構文・型等の読み取り検査は可能なら実施し、環境に依存する未実行を隠さない。未完成でも動作すると報告しない。
- `test-supervisor-01/` の必要なファイルと新版結果 `test-supervisor-result-02.md` だけを所有。未追跡を含む全変更ファイルの SHA、削除対象、実行コマンド・終了コード、実行可能な次の合成試験・未確認を記す。作業完了後に書込みを停止しパスを通知。別担当が固定版をレビューする。
