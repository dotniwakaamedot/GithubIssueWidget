# I02 実装途中成果の独立レビュー依頼 01

- run / task / request_revision: 20261118-160129-mvp-restart / I02 / 01。レビュー担当 ID `/root/plan_review`、role verification_reviewer。作者 `/root/foundation` と別。
- ユーザー原文: 「Issueを確認し、プロジェクトを進行してください。あなたはオーケストレーションに徹してください。同じ間違いや失敗を２度繰り返したら中止してください。」追加指示原文: 「失敗を3回繰り返すまでに緩和します」。初回を含め同種の失敗が今回3回発生した時点で中止し、親へ直ちに報告する。
- 共有 run: D:\Projects\GithubIssueWidget\docs\agent-runs\20261118-160129-mvp-restart。対象 worktree: D:\Projects\GithubIssueWidget-issue-2-20261118-160129、branch `task/issue-2-foundation-20261118-160129`、HEAD a4ac97286785a2a5ba0d070a909b58003fc49ff1。対象は未コミットの README.md と未追跡 widget/ の全17ファイル。
- 対象依頼 `tasks/I02/request-02.md` SHA256 CA64B05910EA68BB2731D38A0B108640A535EBCFFDB7353B1B536DB2B9B8AC84。対象結果 `tasks/I02/result-01.md` SHA256 F0AEAD322062EED3054B1A51B0A44DC05EB85E257A3A2EE59026F99C9112BE03。承認済み計画 `plan-02.md` SHA256 A6CB37DC369D69B334564235CD00C3AF284250109D4A56BE63C6CC7B104FE990。
- 管理が対象結果の17ファイルの実体 SHA256 と未追跡を含む status 全17件を照合し一致。生成された release exe は SHA256 6D322C27151564F706482E0EAE5870EB30B535B39E3623D5742B34206033D149 で一致。レビュー担当も実体・結果・差分・削除・対象 HEAD を独立に照合し、対象版が変われば旧承認を使用しない。
- 一次資料: 最新の https://github.com/dotniwakaamedot/GithubIssueWidget/issues/2 と親 #1、依頼、結果、適用先 AGENTS.md・docs/agent-workflow.md・docs/issue-workflow.md。目的・完了条件ごとに対象版と証跡を照合し、事実・推測・未検証を区別する。
- 特に、作者はウィンドウ移動の自動操作で2回位置変化なし（原因未確定）、Python パイプで日本語破損2回を報告。依頼参照・対象の整合不備も2件発生している。それぞれの同種3回目で中止。**同じドラッグ操作や文字破損経路を再試行せず**、既存コード・操作手順・ログ・環境・設定を根拠に原因候補と区別方法を静的に分析する。未確認なら原因未確定とし、リスクのない別の検証方法を提案する。作業を変えて同じ失敗を誘発しない。
- 移動の未達、最小サイズ・中央・OS 最前面・8方向リサイズ・拒否 URL の未検証、外部リンク確認の限界を blocking / 未達として扱う。未確認を合格としない。依頼にある Windows 起動・リリースビルド、権限最小化、Opener の許可制限、秘密の取扱い、不要権限、仕様・回帰も確認。テスト結果は確認可能な証拠に限定し、実測していない項目を断定しない。
- 所有範囲: 共有 run の `tasks/I02/review-01.md` と必要な自担当の質問ファイルのみ。製品ファイル、作者の結果、他担当作業、GitHub、git 状態の変更禁止。検証用出力は製品と分離。事前に親ディレクトリと宛先不在を確認し、書込み失敗時は依存操作を止め、同じディレクトリの .tmp から確定。
- レビューには実ファイルと結果のハッシュ、Issue 原文・基準、指摘 ID・根拠・再現/検証の安全な方針・是正条件、コマンド・cwd・終了コード、未確認と対象版限定の判定を記載。レビュー結果公開後はその確定パスのみ通知。push / PR / merge / 外部書込みは禁止。