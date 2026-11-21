# Windows IPC 隔離コピーの安全境界修正依頼 02

- 担当 `/root/windows_ipc_adapter`（元作者を親が再開）。共有 run `D:/Projects/GithubIssueWidget/docs/agent-runs/20261121-app-completion-restart/`、作業場所 `D:/Projects/GithubIssueWidget`、基準 HEAD `4961b25bec53a93439acdf5230de77f959908af0`。他者も作業中であり変更を戻さない。
- 根拠 `windows-ipc-adapter-review-01.md` SHA256 `1A70B564FDE8C4353A8E2860174D78F0B209B3960937F11F26E2EABBCEC603E5` は blocking: 親 `windows_ipc_guard` のみの禁止ゲートを直接起動可能な子 `windows_ipc_guard_child` が通らず、実 credential に到達し得る。元成果の `windows-ipc-adapter-01/main.rs` SHA256 `F10F86CCC567801D197B92222CE478F0D98F8CC0C4575A9DA0E9CFA8D4DED61E`、`test_adapter.py` `A3682FD85B4A59DFE6FEF24F00712D63C0A758B46B37D2F8277CF00E9FB8DCA5`、`adapter.py` `0A350F40863D3E80FA621195EAB8143E404026C594BF802CB382971EA6F421B0`。開始前に実体の SHA を照合し異なる場合は変更せず質問。
- 所有は `windows-ipc-adapter-01/main.rs` と `test_adapter.py`、同領域の新しい試験ログ・必要な新規補助ファイル、および新規 `windows-ipc-adapter-fix-result-02.md` のみ。確定済みの元 `windows-ipc-adapter-result-01.md`・旧ログは変更しない。`adapter.py` と本番、他者成果、資格情報、Git、GitHub は変更しない。作業後は所有ファイルへの書込みを止める。
- 修正: 親・子それぞれの直接起動を含み、実 Credential Manager の読取り／書込みに至る全入口が fail-closed となる明示的ゲートを共通に適用する。元の親ゲートだけを検査する静的試験を改善し、親・子を別々に直接指定しても実 credential API に到達しないことを **実 OS 資格情報に触れない安全な合成経路** で検証する。実行可能な模擬テストが成立しない場合は範囲と不足を明示し、実施していない動的検証を主張しない。Rust コンパイルや接続を追加する場合は依頼の所有範囲を超えない新規専用素材で、実 OS 禁止が確立してから行う。
- 実 OS 経路は無効のまま。実 PAT、既存 service、実アプリ・GitHub を操作しない。変更前後 SHA、具体的入口と検証、コマンド・終了コード・安全なログ、削除対象と未確認を記録。UTF-8 実データを読戻して一時ファイルから確定し、パスのみ通知。失敗2回は当該作業を停止し親に報告。
- 後続は作者以外による対象新版の独立再レビュー。既存 `windows-ipc-adapter-connect-request-01.md` はレビュー01が要修正のため開始しない。実 OS の解禁は本依頼に含まない。
