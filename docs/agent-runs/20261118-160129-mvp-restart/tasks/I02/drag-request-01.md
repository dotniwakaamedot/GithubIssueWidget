# I02 ウィンドウ移動の読み取り診断依頼 01

- run / task / request_revision: 20261118-160129-mvp-restart / I02-drag / 01。担当 `/root/drag_diagnosis`、role explorer。進行管理 `/root/restart_planner`、独立レビュー `/root/plan_review`。
- 共有 run: D:\Projects\GithubIssueWidget\docs\agent-runs\20261118-160129-mvp-restart。対象 worktree: D:\Projects\GithubIssueWidget-issue-2-20261118-160129、branch `task/issue-2-foundation-20261118-160129`、HEAD a4ac97286785a2a5ba0d070a909b58003fc49ff1。開始・終了時に全17対象ファイルの SHA256 と未追跡を含む状態を `tasks/I02/result-01.md` の一覧と照合。変更があれば停止して親に報告。
- 承認された読み取り計画 `tasks/I02/drag-plan-01.md` SHA256 C6AC531B2AB8FB2BAAC401BFBEE09135356E7CA2E411A09D4C27198611A1B70A、独立レビュー `tasks/I02/drag-plan-review-01.md` SHA256 A1577D6DC6F51CE788C1F68DC97FDEB3FD2D75EC7DB7F4D2065A36E3DF3E7FA3。前版結果 `tasks/I02/result-01.md` SHA256 F0AEAD322062EED3054B1A51B0A44DC05EB85E257A3A2EE59026F99C9112BE03、前版レビュー `tasks/I02/review-01.md` SHA256 7DB95B62CF8D23894AAEB4FCFAB51DD4A4634CE42C1C330421E4689E4FE6DEF9。開始前に各実体と SHA を照合。
- ユーザー原文: 「Issueを確認し、プロジェクトを進行してください。あなたはオーケストレーションに徹してください。同じ間違いや失敗を２度繰り返したら中止してください。」追加指示原文: 「失敗を3回繰り返すまでに緩和します」。初回を含め同種の失敗が3回発生した時点で全作業を中止し親に直ちに報告する。
- 前回ウィンドウ移動の自動試行2回は位置変化なし、原因未確定。文字破損2回、依頼参照不整合2件もそれぞれ同種3回目で中止。ドラッグ・ウィンドウ移動、入力注入、マウス移動、UI 操作の再試行、過去の文字破損経路は一切行わない。
- 目的: 承認済み drag-plan-01 の読み取り手順に沿い、過去の実行記録・ツールの一次ドキュメント・対象 Tauri 実装・既存 OS/アプリ情報を副作用なく照合。押下位置・要素・ボタン保持・API 呼出し・OS 応答の各層で、確定事実・仮説・必要証拠・安全な識別方法を整理する。利用できない過去記録は不足とし、暫定報告を確定事実にしない。
- 出力は静的な観測・計測手順案まで。実行中アプリへの入力、観測装置の起動、プロセス・ウィンドウの操作、製品や生成物・GitHub・Git の変更は禁止。read-only であることを各手順の前に確認する。安全性・秘密情報・副作用・版変化が不明なら実施せず親へ質問。
- 所有範囲: 共有 run の `tasks/I02/drag-result-01.md` と自担当の新規質問のみ。B02〜B04 の別担当は同じ worktree で証拠収集するが製品変更は禁止されている。実体が変われば停止し、版を再固定する。
- 成果: 利用した一次資料と版・URL・時刻、コマンド・cwd・終了コード、実際に確認できた証拠、候補ごとの未確認、失敗を起こさない計測方法・リスク・停止条件、次に必要な判断を記載。原因が確定しなければ未確定と明記。成果は親ディレクトリと宛先・tmp 不存在を確認し UTF-8 で .tmp から確定、代表文字を読戻してからパスのみ通知。