# I02 追加検証・ドラッグ診断の独立レビュー依頼 01

- run / task / request: 20261118-160129-mvp-restart / I02-combined-review / 01。独立レビュー担当 `/root/plan_review`。追加検証作者 `/root/foundation`、診断作者 `/root/drag_diagnosis` と別担当。
- 共有 run: D:\Projects\GithubIssueWidget\docs\agent-runs\20261118-160129-mvp-restart。対象 worktree: D:\Projects\GithubIssueWidget-issue-2-20261118-160129、branch `task/issue-2-foundation-20261118-160129`。対象 HEAD は前版結果に記録された値を実体で確認する。
- 入力の確定ファイルと実体から計算した SHA256:
- tasks/I02/request-05.md SHA256 F01FFAE95DFD63405313F7C3BE3A0D3165A01FAEEA2E800994AB3F6036932EC9
- tasks/I02/result-02.md SHA256 520044FE6138609B99595C5FAF1D3973DD2D62EB82F27FE90392568C9599ED13
- tasks/I02/drag-request-03.md SHA256 46C1451583A57D0D71EC2A607F409C92CE0F7FA7D1E81790CA2FBA83CE5E7BEE
- tasks/I02/drag-result-01.md SHA256 B34245F943AD4B9E69F5DBC8020C762499055A2ABDE878F57446131038C5CCB6
- tasks/I02/result-01.md SHA256 F0AEAD322062EED3054B1A51B0A44DC05EB85E257A3A2EE59026F99C9112BE03
- tasks/I02/review-01.md SHA256 7DB95B62CF8D23894AAEB4FCFAB51DD4A4634CE42C1C330421E4689E4FE6DEF9
- tasks/I02/evidence-03/drag-history-01.md SHA256 49233DFDBC3A610E1FE99670607463C26E917F496022D9756EAC3A07E74B7B02
- tasks/I02/management-03.md SHA256 3A59B412A078E8D22D0EDC269A5257F12ABBCE397E7BF9DFA4F603EC9E0ABF77
- plan-02.md SHA256 A6CB37DC369D69B334564235CD00C3AF284250109D4A56BE63C6CC7B104FE990
- 2つの結果と各依頼・事後記録のハッシュは管理が連結照合した。実装・ログ・既存証拠は結果に記載された実ファイルの SHA256 をそれぞれ再計算し、未追跡を含む全差分・削除・HEAD と一致を確認する。版が異なればレビューを中止し報告する。
- 一次資料: Issue #1 と #2 の最新本文、対象 worktree の AGENTS.md、docs/agent-workflow.md、上記計画・依頼・結果・レビュー。Issue #2 の各条件、review-01.md の B01〜B04 ごとに、実施・証拠・未実施と限界、現時点の解消状況を判定する。ドラッグ診断について一次資料・過去操作の事後記録・API 契約から事実と仮説の区別、残る不明、計測案の安全性と証拠の限界を独立照合する。
- 前回の参照整合不備3件で中止し、今回の再開後に依頼文の古い件数表現が同種2件発生した。今回の同種3件目で全作業を中止する。過去の移動失敗2回、文字破損2回も記録し、同じ危険な経路を再試行しない。最新ユーザー指示「失敗を3回繰り返すまでに緩和します」に従い、初回を含む同種3回で中止する。
- **ドラッグ・ウィンドウ移動・新規入力注入・過去の文字破損経路は実行しない。** 対象製品、既存証拠・生成物、Git、GitHub を変更しない。再現・検証に外部の書込みや疑わしい操作が必要な場合は行わず未確認と記す。根拠不足の事項を合格とせず、解決済みか実証済みかを結果の報告と区別する。
- 所有範囲: 共有 run の `tasks/I02/combined-review-01.md` と必要な自担当の新規質問のみ。親ディレクトリと宛先・tmp 不在を事前確認し、UTF-8 往復・代表文字を確認して確定。書込み失敗時は依存操作を停止する。
- 結果: 対象版・実測ハッシュ・作者・作業場所・各参照の版・実行コマンドと終了コード、B01〜B04の根拠・判定・未確認、診断の事実と仮説・計測案の限界、blocking 指摘と是正条件、全体の版限定の判定を記載。確定パスのみ通知。push / PR / merge / 外部書込みは禁止。