# Issue #2 実装依頼 01

- run_id / task_id / request_revision: 20261118-160129-mvp-restart / I02 / 01
- 担当 ID / role: /root/foundation / task_worker。進行管理 /root/restart_planner、親 /root。別の担当者も作業しているため、他者の変更を取り消さず保護すること。
- 共有 run（別 worktree からも同じ場所）: D:\Projects\GithubIssueWidget\docs\agent-runs\20261118-160129-mvp-restart
- 作業場所: D:\Projects\GithubIssueWidget-issue-2-20261118-160129、branch `task/issue-2-foundation-20261118-160129`、基準 HEAD `a4ac97286785a2a5ba0d070a909b58003fc49ff1`。開始前に HEAD、branch、git status と Issue の現状を確認。主 checkout を切替・変更しない。
- ユーザー原文: 「Issueを確認し、プロジェクトを進行してください。あなたはオーケストレーションに徹してください。同じ間違いや失敗を２度繰り返したら中止してください。」追加指示原文: 「失敗を3回繰り返すまでに緩和します」。初回を含め同種の失敗が今回3回発生した時点で全作業を中止し、管理・親に直ちに通知する。依頼原文にある2回条件より最新指示を優先。
- 一次資料: https://github.com/dotniwakaamedot/GithubIssueWidget/issues/2 の最新本文（原文の既存条件と追加条件の全て）と親 https://github.com/dotniwakaamedot/GithubIssueWidget/issues/1、作業場所の AGENTS.md、docs/agent-workflow.md、docs/issue-workflow.md。G01 の親未決事項調査とレビュー `tasks/G01/result-01.md` SHA256 BFDB22179081951FD8F7365F9391ED82BD7304F281C42CB3C17B768E6C8376E5、`tasks/G01/review-01.md` を参照。Tauri 等の現在の実装仕様が必要なら公式一次資料で確認。
- 計画: 共有 `plan.md` revision 02 SHA256 A6CB37DC369D69B334564235CD00C3AF284250109D4A56BE63C6CC7B104FE990 は `tasks/plan/review-02.md` で承認。親の G01 判断と最新停止条件は `tasks/G01/management-02.md` および依頼を参照。計画承認は外部送信、push、PR、merge の承認を意味しない。
- 開始条件: #2 の本文・状態・開始コメントが書込まれ読戻し済み。最新の担当、状態、全コメント、関連 PR、worktree と競合作業を開始前に再照合し、新たな競合・不明なら着手せず質問する。予定 branch 作成済み。親 #1 に対する統合候補 `integration/issue-1` は同じ基準からローカル作成、PR base と公開は未確定。
- 目的・完了条件: Tauri 2、React、TypeScript、Rust で Windows から実行できる基盤を作る。必要環境・開発起動・リリースビルド手順を文書化し Windows で実行・ビルド。装飾なしダークテーマ、初期 900×520・最小 600×300、中央・初期最前面の独立ウィンドウ、移動・リサイズ・最前面切替を提供。Opener で外部 URL を開き許可先を制限し Tauri capability は必要最小限。起動・操作・ビルドと未確認環境を記録。各 Issue 原文の完了条件は全て維持し、実機で未確認なら未確認と報告し成功扱いしない。
- 検証: Windows 上の開発起動・release build、初期／最小サイズ、中央・最前面・切替、移動・リサイズ、許可 URL と拒否 URL の挙動、必要な静的チェック／テスト。対象版、作業ディレクトリ、正確なコマンド、各終了コード、証拠、未実施と理由を記載。テスト用の外部 GitHub 書込みは行わない。ビルド環境は `tasks/environment/result-01.md` を参考にし、実動作・依存互換は未検証であることに注意。
- 所有範囲: この task worktree 内の新規 `widget/` 以下（アプリ、設定、必要なテストと証跡を除く）、ルート `README.md` のアプリ起動手順への必要な追記、必要ならルート `.gitignore` のみ。共有 run への結果 `tasks/I02/result-01.md` および自担当の質問ファイルのみ。他の既存ファイル、共有 plan、他担当成果、主 checkout は変更禁止。範囲の追加が必要なら確定質問ファイルで管理に確認し、勝手に拡張しない。
- Git: タスク worktree からのローカル作業・必要なローカル commit は可。push、PR、merge、Issue 更新、削除、他者作業の巻戻しは禁止。資格情報や private Issue をログ・設定・結果へ出力しない。依存導入・ビルドの副作用は確認して記録。
- 引渡し: `docs/agent-workflow/templates/work-result.md` に準じ、依頼版・対象 HEAD、全変更（未追跡・削除も含む）の SHA256、検証コマンドと結果、要求ごとの実装・未実施、リスク・仮定・環境を `tasks/I02/result-01.md` に確定。自己承認せず、提出後はレビュー対象を変更しない。書込み前に共有先の親ディレクトリ・宛先不在を確認し、`.tmp` を確定名にする。失敗時は依存操作を止める。