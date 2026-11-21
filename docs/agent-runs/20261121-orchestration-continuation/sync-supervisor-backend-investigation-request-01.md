# テスト監督器の Windows backend 調査依頼

- 担当 `/root/gui_failure_investigation`。作者 `/root/process_safety` と別担当。共有 run `D:/Projects/GithubIssueWidget/docs/agent-runs/20261121-orchestration-continuation/`、作業場所 `D:/Projects/GithubIssueWidget`。結果は固有 `sync-supervisor-backend-investigation-01.md` と質問のみ。
- `sync-supervisor-work-request-01.md` と `sync-test-runtime-review-01.md` B1、先行 `../20261121-issue-completion/gui-supervisor-plan-02.md` とレビュー、実装担当の現在の版・不確実性を照合。未完成の書込み中ファイルは勝手に編集・レビュー対象として固定しない。担当に質問ファイルを出し、固定された成果・懸念・SHA の回答を受け取る。
- Windows で限定 Node/Vitest と子孫を監督する現実的な方式を独立調査し、生成時所属の方法、必要 API・引数、環境・ハンドル、失敗時・タイムアウト・監督者停止と終了証拠、実装可能な境界、合成試験を具体的に整理する。既存不明プロセスを停止・推測同定しない。OS 障害や breakaway の無条件保証を主張しない。
- 読取りと設計のみ。新たなプロセス生成、テスト、GUI、ネットワーク、権限変更、実資格情報や既存製品への変更は行わない。対象版 SHA、根拠と仮説・未確認・実行結果・失敗を明示。確定後はパスのみ通知。実装・実行の許可とはしない。
