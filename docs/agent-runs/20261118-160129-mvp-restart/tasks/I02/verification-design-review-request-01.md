# I02 検証方法設計の独立レビュー依頼 01

- run / task / request: 20261118-160129-mvp-restart / I02-verification-design-review / 01。担当 `/root/plan_review`、verification_reviewer。作者 `/root/verification_design` とは別担当。管理は実際の担当 ID が一致することを起動前に確認する。
- ユーザー原文: 「Issueを確認し、プロジェクトを進行してください。あなたはオーケストレーションに徹してください。同じ間違いや失敗を２度繰り返したら中止してください。」追加指示「失敗を3回繰り返すまでに緩和します」「2回同じ問題にあったら別のエージェントに客観的に調査してもらってください」。今回初回を含む同種2回で該当操作を中断し別担当へ独立調査、同種3回で全作業を中止して親に報告する。
- 共有 run: D:\Projects\GithubIssueWidget\docs\agent-runs\20261118-160129-mvp-restart。対象 worktree: D:\Projects\GithubIssueWidget-issue-2-20261118-160129、branch `task/issue-2-foundation-20261118-160129`。結果に列挙された製品17ファイルの実体・SHA256・未追跡を含む状態を照合し、変更・不明なら停止。
- 入力の確定ファイルと実体から計算した SHA256:
- tasks/I02/verification-design-request-02.md SHA256 DBB1F63472016EB0AC1B14CD9E401A342C7B8A45C20C61A2B3F65D12A6991C71
- tasks/I02/verification-design-result-01.md SHA256 E821627AF74FF335D29433BF47F8682D36C2BF970B18036365495769F0153B4D
- tasks/I02/combined-review-01.md SHA256 F6DF723A067C988F144087B6A4833EDD306AAB56071068894424D359F70004D3
- tasks/I02/result-02.md SHA256 520044FE6138609B99595C5FAF1D3973DD2D62EB82F27FE90392568C9599ED13
- tasks/I02/drag-result-01.md SHA256 B34245F943AD4B9E69F5DBC8020C762499055A2ABDE878F57446131038C5CCB6
- tasks/I02/management-04.md SHA256 0D569B792156CE706ACF0BF58295EBBFEE10D7FFE66434D09E36975B419F0239
- plan-02.md SHA256 A6CB37DC369D69B334564235CD00C3AF284250109D4A56BE63C6CC7B104FE990
- 作者の結果にある依頼 SHA と作者 ID は管理が実体と照合済み。レビュー担当も対象設計版と実体を前後に再照合し、変更後の旧承認を使わない。
- 前回は非実在パス参照3件で停止。今回の再開後、設計担当が共有 run と cwd の基点の取違えによる PathNotFound を1回報告した。同種2回目は該当操作を中断し別担当へ独立調査、同種3回目は全停止。パスを推測せず存在を確認し、複数基点の一覧は基点・相対パス・64桁 SHA・注記を分けて全件存在確認後に照合する。過去のドラッグ2回と文字破損2回も記録し、同経路を再試行しない。
- 一次資料: Issue #1/#2 最新本文、対象 worktree の AGENTS.md、docs/agent-workflow.md、上記依頼・結果・独立レビュー。B02 の Windows OS 最前面、寸法、中央、最小制約、各方向リサイズ、B03 の対象版・実行ログ・終了コード・ビルドと dev 起動、B04 の実 Tauri IPC ACL における許可・拒否、既定ブラウザと実際の遷移について、設計が元の完了条件を証明できるか独立に確認する。
- 重点: B02 の HWND/PID・DPI・座標系・枠・モニター・制約と UI 操作、B03 の不変ソース版・環境・プロセス終了コード・生成物とログの同時対応、B04 の IPC 対象 main WebView・非 mock・誤許可時の外部副作用・ACL と URL 正規化の識別、既定ブラウザの照合。未確認の技術前提、実行安全性、秘密情報・他アプリ影響、復元、対象版の競合、所有権、並行可否・直列化、停止・独立確認を根拠付きで評価する。提案の実現可能性と証拠十分性を区別する。
- 実際のデスクトップ・入力操作、ドラッグ、対象アプリ起動、ビルド、通信を伴う試験、製品・既存証拠・Git・GitHub の変更は禁止。コード・文書の静的読取と公式一次資料の調査のみ。安全・権限が不明な操作は行わず未確認にする。
- 所有範囲: 共有 run の `tasks/I02/verification-design-review-01.md` と自担当の新規質問のみ。親ディレクトリと宛先・tmp 不在を確認し UTF-8 往復と代表文字を検証して確定。書込み失敗時は依存操作を中断。
- 結果: 対象版・実測 SHA・作者・読取元・cwd・コマンド・終了コード、要求ごとの判定、blocking / non-blocking 指摘と根拠・是正条件、未確認、対象版限定の承認可否を記載。実行の許可や製品の完了を意味しない。確定パスのみ通知。