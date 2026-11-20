# I02 専用コピー検証方式の独立レビュー依頼 03

以下の構造化フィールドを依頼の正本とする。著者とは別のレビュアーが対象設計と元の Issue #1/#2、参照資料を独立に照合する。担当と作者、出力先、履歴、閾値、参照ファイル・SHA は同一データから生成した。実担当と参照版・出力先・競合は起動直前に再照合する。これらは排他ロックではなく、変更や所有不明を検出したら停止する。
```json
{
    "run_id":  "20261118-160129-mvp-restart",
    "task_id":  "I02-copy-design-review",
    "reviewer":  "/root/plan_review",
    "author":  "/root/verification_design",
    "output_relative_path":  "tasks/I02/verification-design-review-03.md",
    "user_original":  "Issueを確認し、プロジェクトを進行してください。あなたはオーケストレーションに徹してください。同じ間違いや失敗を２度繰り返したら中止してください。",
    "user_additions":  [
                           "失敗を3回繰り返すまでに緩和します",
                           "2回同じ問題にあったら別のエージェントに客観的に調査してもらってください"
                       ],
    "thresholds":  {
                       "independent_investigation":  2,
                       "stop_all":  3
                   },
    "current_history":  {
                            "content_validation_failures":  2,
                            "path_base_failure_after_restart":  1,
                            "structured_generation_type_error":  1
                        },
    "references":  [
                       {
                           "base":  "D:\\Projects\\GithubIssueWidget\\docs\\agent-runs\\20261118-160129-mvp-restart",
                           "relative_path":  "tasks/I02/verification-design-request-04.md",
                           "sha256":  "A3A4DC5CFE8F178E5EB9480A35EBF90F73708BB71E75C007D278B5B37784F3C6"
                       },
                       {
                           "base":  "D:\\Projects\\GithubIssueWidget\\docs\\agent-runs\\20261118-160129-mvp-restart",
                           "relative_path":  "tasks/I02/verification-design-result-03.md",
                           "sha256":  "B2B552301D897717CC80404A64297C460BADED4C8AE4B0F130B1557F7B08DAED"
                       },
                       {
                           "base":  "D:\\Projects\\GithubIssueWidget\\docs\\agent-runs\\20261118-160129-mvp-restart",
                           "relative_path":  "tasks/I02/isolation-inventory-result-01.md",
                           "sha256":  "D73C3CE1114775361C4BF045E774F6F352C2F86796073DBD5842B6EE92D90555"
                       },
                       {
                           "base":  "D:\\Projects\\GithubIssueWidget\\docs\\agent-runs\\20261118-160129-mvp-restart",
                           "relative_path":  "tasks/I02/verification-design-result-02.md",
                           "sha256":  "971D1BE06A8D735AB75C57B5398F2E667D0F288D991B34F473D15080140A2926"
                       },
                       {
                           "base":  "D:\\Projects\\GithubIssueWidget\\docs\\agent-runs\\20261118-160129-mvp-restart",
                           "relative_path":  "tasks/I02/verification-design-review-02.md",
                           "sha256":  "0BF94DA7F7FF203326DA5E7133EADADE6A48E6BE1ADA4CCC5DF9F2BF67869FA5"
                       },
                       {
                           "base":  "D:\\Projects\\GithubIssueWidget\\docs\\agent-runs\\20261118-160129-mvp-restart",
                           "relative_path":  "tasks/I02/combined-review-01.md",
                           "sha256":  "F6DF723A067C988F144087B6A4833EDD306AAB56071068894424D359F70004D3"
                       },
                       {
                           "base":  "D:\\Projects\\GithubIssueWidget\\docs\\agent-runs\\20261118-160129-mvp-restart",
                           "relative_path":  "tasks/I02/result-02.md",
                           "sha256":  "520044FE6138609B99595C5FAF1D3973DD2D62EB82F27FE90392568C9599ED13"
                       },
                       {
                           "base":  "D:\\Projects\\GithubIssueWidget\\docs\\agent-runs\\20261118-160129-mvp-restart",
                           "relative_path":  "tasks/I02/management-04.md",
                           "sha256":  "0D569B792156CE706ACF0BF58295EBBFEE10D7FFE66434D09E36975B419F0239"
                       },
                       {
                           "base":  "D:\\Projects\\GithubIssueWidget\\docs\\agent-runs\\20261118-160129-mvp-restart",
                           "relative_path":  "process/reference-check-design-02.md",
                           "sha256":  "3CDCB637F739F9A9DBE240D0901557BC2BB23732ABF7CCC528F98A514C06C06E"
                       }
                   ]
}
```

対象は同一 Windows の専用コピー方式。元 worktree の未コミット17ファイル・ignored 生成物、既存資産と秘密の保護、コピー版と元版・ログ・実行バイナリの対応、dist・target・gen・cache・temp・profile・registry・子プロセス・通信・UI への副作用、所有・保持・復旧、排他と並行性、B02〜B04 の証拠と未確認を評価する。VM や OS 全体の完全隔離を当然視せず、通常の副作用と保護が必要な資産・残リスクを区別する。B01 の移動は原因未確定で未達。

実機操作・ドラッグ・入力・アプリ起動・build・通信を伴う試験・製品・既存証拠・Git・GitHub の変更は禁止。存在する資料と公式一次資料の静的読取のみ。パスを推測せず基点・相対パス・注記を分け、全件の存在確認後にハッシュを照合する。履歴と閾値は上記フィールドを参照し、同種2回目は該当操作を停止して別担当が客観調査、同種3回目は全作業を中止する。過去のドラッグ・文字破損経路は再試行しない。

共有 run の指定結果と新規質問だけを所有し、親ディレクトリと確定先・tmp の不存在を確認し UTF-8 往復後に確定する。結果には各要求・対象版・実測 SHA・根拠・未確認・blocking / non-blocking 指摘・是正条件・cwd・コマンドと終了コード、版限定の判定を記載。設計承認は実行許可や製品完成を意味しない。確定パスだけを通知する。