# G01 調査結果の独立レビュー依頼 01

- run_id / task_id / request_revision: 20261118-160129-mvp-restart / G01 / 01
- レビュー担当 ID / role: /root/plan_review / verification_reviewer。作者 /root/parent_issue_analysis とは別。
- 共有 run の絶対パス: D:\Projects\GithubIssueWidget\docs\agent-runs\20261118-160129-mvp-restart
- 依頼: tasks/G01/request-03.md SHA256 CBC5E324B5E2E9FBDD261C535531550502236C36E707553B35DF089DADFE4C2E。
- 対象結果: tasks/G01/result-01.md SHA256 BFDB22179081951FD8F7365F9391ED82BD7304F281C42CB3C17B768E6C8376E5。review 実施前後に実体と SHA を照合し、結果版に限定して判定する。
- 基準: main HEAD a4ac97286785a2a5ba0d070a909b58003fc49ff1。plan.md revision 02 SHA256 A6CB37DC369D69B334564235CD00C3AF284250109D4A56BE63C6CC7B104FE990 および tasks/plan/review-02.md。ユーザー原文と最新の条件は tasks/G01/request-03.md と tasks/G01/management-02.md を参照。
- 一次資料: GitHub #1〜#7 の本文を独立に取得。AGENTS.md、docs/agent-workflow.md、docs/issue-workflow.md、環境結果 tasks/environment/result-01.md SHA256 7232E15B14A883FD63BED6A12054263D64662883AE0B620D887377E66A8E5885 と tasks/E01/management-01.md を必要に応じ照合。
- 完了条件: G01 依頼の全未決事項について事実・仮説・提案・未確認の分離、Issue 要件と整合、検証可能性、#2 着手に対する根拠と不明の扱い、共有チャット・環境の未確認の正確性を確認。結果の取り違え、要求の緩和、分類規則や権限制約の誤読も探す。承認は実装開始や外部操作の認可を意味しない。
- 証跡: 依頼・対象・元資料の版や日時、ハッシュ、作業ディレクトリ、コマンドと終了コード、未実施を記載。blocking と non-blocking 指摘、指摘ごとの根拠と是正条件、結果版限定の判定を示す。
- 所有範囲: tasks/G01/review-01.md と必要な自分の質問ファイルのみ。親ディレクトリと宛先不在を確認し .tmp から確定する。結果、計画、製品、GitHub、Git は変更禁止。書込み失敗時は依存操作を停止する。
- 停止条件: ユーザー追加指示「失敗を3回繰り返すまでに緩和します」。初回を含め同種の失敗が今回3回生じたら即中止し親へ報告。完了後は結果ファイルのパスを通知する。