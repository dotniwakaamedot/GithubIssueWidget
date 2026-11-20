# I02 検証方法設計の修正依頼 03

- run / task / request: 20261118-160129-mvp-restart / I02-verification-design / 03。担当 `/root/verification_design`、作業担当。前版作者と同一担当、進行管理 `/root/restart_planner`。旧担当は終了し書込み停止を確認。対象版の独立レビュー担当は `/root/plan_review`。
- ユーザー原文: 「Issueを確認し、プロジェクトを進行してください。あなたはオーケストレーションに徹してください。同じ間違いや失敗を２度繰り返したら中止してください。」追加指示「失敗を3回繰り返すまでに緩和します」「2回同じ問題にあったら別のエージェントに客観的に調査してもらってください」。今回、初回を含む同種2回で該当操作を中断し別担当へ独立調査、同種3回で全作業を中止し親へ報告。
- 共有 run: D:\Projects\GithubIssueWidget\docs\agent-runs\20261118-160129-mvp-restart。対象 worktree: D:\Projects\GithubIssueWidget-issue-2-20261118-160129、branch `task/issue-2-foundation-20261118-160129`。前版と HEAD・未コミット17ファイルの版を照合し、変化・所有不明なら停止。
- 確定参照と実体から計算した SHA256:
- tasks/I02/verification-design-request-02.md SHA256 DBB1F63472016EB0AC1B14CD9E401A342C7B8A45C20C61A2B3F65D12A6991C71
- tasks/I02/verification-design-result-01.md SHA256 E821627AF74FF335D29433BF47F8682D36C2BF970B18036365495769F0153B4D
- tasks/I02/verification-design-review-01.md SHA256 829758054D5DF5B0D8D920CF56F46D8EE58725D07200B95C269CABE486A6D6EE
- tasks/I02/management-04.md SHA256 0D569B792156CE706ACF0BF58295EBBFEE10D7FFE66434D09E36975B419F0239
- tasks/I02/result-02.md SHA256 520044FE6138609B99595C5FAF1D3973DD2D62EB82F27FE90392568C9599ED13
- tasks/I02/combined-review-01.md SHA256 F6DF723A067C988F144087B6A4833EDD306AAB56071068894424D359F70004D3
- plan-02.md SHA256 A6CB37DC369D69B334564235CD00C3AF284250109D4A56BE63C6CC7B104FE990
- レビュー `verification-design-review-01.md` の B01 は blocking、前版の設計は未承認。B03 では `CARGO_TARGET_DIR` の隔離だけでは Vite の frontend dist 等を守れない。Issue の機能 B01（移動）とは別の設計レビュー指摘 ID である。
- 是正条件: 対象検証 run の全書込み先（少なくとも frontend dist、Cargo target、Tauri 生成物、ログ、一時物、依存取得先）を事前に特定。既存保護物と重ならない不変ソース専用コピー等で build/dev を行う具体的な方法、コピー元・コピーの全対象 hash の対応、依存・設定差、実行バイナリと対象版の関係、所有・保持・停止条件を設計。隔離を確認できなければ未実施とする。前版の B02/B04、並行性、全 Issue 条件を維持し、元の基準を弱めない。
- 前回は非実在パス3件で停止。再開後、前版担当が cwd と共有 run の基点を取り違えて PathNotFound を1回報告。同種2回目は該当操作を中断し別担当へ独立調査、同種3回目は全停止。パスは推測せず実在を確認し、複数基点の一覧は基点・相対パス・64桁 SHA・注記を分け、全件存在確認後に照合。旧レビューで再び同じ意味の指摘が出れば適用手順に従い停止する。
- 過去のドラッグ試行は位置変化なし2回、文字破損2回。原因が確定するまでドラッグ・移動、入力注入、以前の文字破損経路を実行しない。本担当は設計のみであり、build・アプリ起動・OS 操作・通信を伴う検証・製品や既存生成物、Git、GitHub の変更は禁止。
- 所有範囲: 共有 run の `tasks/I02/verification-design-result-02.md` と自担当の新規質問のみ。製品・他担当の証拠・既存結果は変更しない。宛先の親ディレクトリと確定先・tmp の不存在を確認し、UTF-8 往復と代表文字を確認して確定。書込み失敗時は依存操作を中断。
- 成果: 前版の各項目との対応と B01 是正、対象版・必要な全書込み先・隔離・所有・安全条件・検証証跡・並行性・未確認、読取資料・cwd・コマンド・終了コードを記録。修正版は別の確認担当がレビューする。実行指示や完了承認とみなさず、確定パスのみ通知。