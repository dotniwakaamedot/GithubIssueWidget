# I02 隔離環境・Windows 実機の読み取り調査依頼 01

- run / task / request: 20261118-160129-mvp-restart / I02-isolation-inventory / 01。担当予定 `/root/isolation_inventory`、作業担当。進行管理 `/root/restart_planner`。起動する担当 ID と一致することを管理が確認してから開始する。
- ユーザー原文: 「Issueを確認し、プロジェクトを進行してください。あなたはオーケストレーションに徹してください。同じ間違いや失敗を２度繰り返したら中止してください。」追加指示「失敗を3回繰り返すまでに緩和します」「2回同じ問題にあったら別のエージェントに客観的に調査してもらってください」。今回初回を含む同種2回で該当操作を中断し別担当へ独立調査、同種3回で全作業を中止して親へ報告する。
- 共有 run: D:\Projects\GithubIssueWidget\docs\agent-runs\20261118-160129-mvp-restart。対象 worktree: D:\Projects\GithubIssueWidget-issue-2-20261118-160129、branch `task/issue-2-foundation-20261118-160129`。前版の未コミット製品17ファイル、ignored 生成物、共有 run の証跡と他担当の作業を保護する。変更・所有不明・競合を発見すれば停止して報告。
- 読取専用の確定参照と実体から計算した SHA256:
- tasks/I02/verification-design-result-02.md SHA256 971D1BE06A8D735AB75C57B5398F2E667D0F288D991B34F473D15080140A2926
- tasks/I02/verification-design-review-02.md SHA256 0BF94DA7F7FF203326DA5E7133EADADE6A48E6BE1ADA4CCC5DF9F2BF67869FA5
- tasks/I02/verification-design-result-01.md SHA256 E821627AF74FF335D29433BF47F8682D36C2BF970B18036365495769F0153B4D
- tasks/I02/combined-review-01.md SHA256 F6DF723A067C988F144087B6A4833EDD306AAB56071068894424D359F70004D3
- tasks/I02/result-02.md SHA256 520044FE6138609B99595C5FAF1D3973DD2D62EB82F27FE90392568C9599ED13
- tasks/I02/management-04.md SHA256 0D569B792156CE706ACF0BF58295EBBFEE10D7FFE66434D09E36975B419F0239
- plan-02.md SHA256 A6CB37DC369D69B334564235CD00C3AF284250109D4A56BE63C6CC7B104FE990
- 目的: 承認済み検証設計の前提である Windows 専用 VM／隔離環境、offline 依存・toolchain、対話可能な Windows 11 実機の有無と利用条件を、現環境と利用権限の範囲で**読み取りのみ**調べる。存在、実際の起動・接続・権限、ライセンスや独立性は分け、検出だけで利用可能と断定しない。候補がなければ未確認とする。
- 検証設計は専用 VM を前提に元生成物・ユーザープロファイル・共有 cache への書込みを禁止している。VM が不在・未確認の場合、別ディレクトリと環境変数等を用いる代替案を、dist / target / gen / cache / temp / profile / registry / subprocess / network の全書込みと競合を隔離できるか静的に評価し、根拠・未確認・残る副作用・追加の独立レビューを列挙。代替案を自動採用せず、実装・実行に必要な条件を明示する。
- Windows 11 対話デスクトップ、OS バージョン、候補 VM の管理 CLI・既存構成、必要な Node・Rust・MSVC・WebView2 と検証用資料について、使用可能な読取手段を先に発見してから確認。コマンドが存在しない場合は実行せず未確認とする。CLI、VM、アプリの起動、インストール、設定変更、通信を伴う試験、ファイルの移動・削除・製品変更、GitHub 更新は禁止。情報へのアクセスが権限不足や影響不明なら試さず未確認として記す。
- 前回の非実在パス3件による停止と再開後の基点取違え1件を記録。パス・CLI を推測せず存在を確認し、一覧は基点・相対パス・64桁 SHA・注記に分離する。同種問題2回で中断・別担当へ独立調査、3回で全停止。過去のドラッグ不成功2回・文字破損2回の経路も再使用せず、UI 入力・ドラッグは一切しない。
- 所有範囲: 共有 run の `tasks/I02/isolation-inventory-result-01.md` と自担当の新規質問のみ。親ディレクトリ・宛先・tmp 不存在を確認し UTF-8 往復と代表文字を確認して確定。書込み失敗時は依存操作を止める。製品・既存証拠・Git・GitHub は変更禁止。
- 成果: 事実・根拠・未確認・提案を分け、参照元・cwd・コマンド・終了コード・対象版、VM の実在と未確認、Windows 実機の可否、代替案の競合・安全条件と未確認、次の判断を記載。独立レビューを受けるまで設計案を実行指示や Issue の完了として扱わず、確定パスのみ通知する。