# I02 ウィンドウ移動の切り分け計画 独立レビュー依頼 01

- run / task / request revision: 20261118-160129-mvp-restart / I02-drag-plan / 01。レビュアー `/root/plan_review`、verification_reviewer。計画作者 `/root/restart_planner`、実装作者 `/root/foundation` と別担当。
- 共有 run: D:\Projects\GithubIssueWidget\docs\agent-runs\20261118-160129-mvp-restart。対象 `tasks/I02/drag-plan-01.md` SHA256 C6AC531B2AB8FB2BAAC401BFBEE09135356E7CA2E411A09D4C27198611A1B70A。
- 参照: `tasks/I02/result-01.md` SHA256 F0AEAD322062EED3054B1A51B0A44DC05EB85E257A3A2EE59026F99C9112BE03、`tasks/I02/review-01.md` SHA256 7DB95B62CF8D23894AAEB4FCFAB51DD4A4634CE42C1C330421E4689E4FE6DEF9、GitHub Issue #2 原文、AGENTS.md と docs/agent-workflow.md。対象 worktree D:\Projects\GithubIssueWidget-issue-2-20261118-160129、HEAD a4ac97286785a2a5ba0d070a909b58003fc49ff1 と前版結果記載の未コミット17ファイル。対象 hash が異なる場合は不一致を記録し止める。
- 判定範囲: 因果不明の2回の位置変化なしと不足証拠を正確に扱っているか、ツールの押下／保持契約と座標・イベント・Tauri・OS の区別、証拠の追跡性・元資料、操作前の安全条件、独立性、版変更時の扱い、停止条件が十分かを確認。事実・仮説・未確認を区別し、実機移動が未証明であることを維持する。後続操作を承認する判断は親が行う。
- 最新ユーザー指示原文: 「失敗を3回繰り返すまでに緩和します」。初回を含め同種失敗3回で作業を中止し親へ報告。タイトル部移動は位置変化なし2回、文字破損2回、依頼参照不整合2件の各3回目で中止。**ドラッグ・ウィンドウ移動の再試行と過去の文字破損経路の再使用は禁止**。レビュー中は実入力・アプリ変更を行わず文書・コード・既存証拠を読む。
- 所有範囲: `tasks/I02/drag-plan-review-01.md` と必要な質問の新規ファイルのみ。製品・対象計画・結果・GitHub・Git は変更禁止。親ディレクトリと確定先・tmp の不存在を確認し、書込み失敗後は依存する操作を止め、.tmp から確定する。
- 結果: 対象版の実測 SHA256、参照元、担当 ID、cwd・確認コマンド・終了コード・未確認、blocking / non-blocking 指摘と根拠・是正条件、版限定の判定を記載。確定後はファイルパスだけを通知。push / PR / merge / 外部書込みは禁止。