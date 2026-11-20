# I02 ドラッグ追加切り分け方法の独立レビュー依頼 01

構造化フィールドを依頼の正本とする。担当と作者、出力、ユーザー原文・追加指示、失敗履歴と閾値、参照版・SHA は同一データから生成した。実担当・参照版・出力先・競合は起動直前に管理が再照合する。照合は排他ロックではなく、変更や所有不明があれば停止する。
```json
{
    "run_id":  "20261118-160129-mvp-restart",
    "task_id":  "I02-drag-investigation-review",
    "reviewer":  "/root/plan_review",
    "author":  "/root/drag_design",
    "output_relative_path":  "tasks/I02/drag-investigation-review-01.md",
    "user_original":  "Issueを確認し、プロジェクトを進行してください。あなたはオーケストレーションに徹してください。同じ間違いや失敗を２度繰り返したら中止してください。",
    "user_additions":  [
                           "失敗を3回繰り返すまでに緩和します",
                           "2回同じ問題にあったら別のエージェントに客観的に調査してもらってください"
                       ],
    "thresholds":  {
                       "independent_investigation":  2,
                       "stop_all":  3
                   },
    "history":  {
                    "drag_no_movement_before_current_restart":  2,
                    "lockfile_read_failures":  2,
                    "content_validation_failures":  2,
                    "path_base_failure_after_restart":  1,
                    "structured_generation_type_error":  1,
                    "recent_process_read_syntax_error":  1,
                    "recent_output_garbled":  1,
                    "official_page_open_internal_error":  1
                },
    "references":  [
                       {
                           "base":  "D:\\Projects\\GithubIssueWidget\\docs\\agent-runs\\20261118-160129-mvp-restart",
                           "relative_path":  "tasks/I02/drag-investigation-design-request-01.md",
                           "sha256":  "4E1CA571E5A9E05BCFD77F32570A4D8FB7CAF693CC1D69ADD254DD313A2CFABE"
                       },
                       {
                           "base":  "D:\\Projects\\GithubIssueWidget\\docs\\agent-runs\\20261118-160129-mvp-restart",
                           "relative_path":  "tasks/I02/drag-investigation-design-result-01.md",
                           "sha256":  "FA635B310797FBCB5C50F6B0F5967E87B0B3131CDB33F8F899332EF1E54A7A08"
                       },
                       {
                           "base":  "D:\\Projects\\GithubIssueWidget\\docs\\agent-runs\\20261118-160129-mvp-restart",
                           "relative_path":  "tasks/I02/drag-result-01.md",
                           "sha256":  "B34245F943AD4B9E69F5DBC8020C762499055A2ABDE878F57446131038C5CCB6"
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
                           "relative_path":  "questions/I02-build-question-10.md",
                           "sha256":  "C2B20065574387F6DA4C52EDB518913CBC72EC29E9EE659F8C07C2BBC7ECEB4F"
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
                       },
                       {
                           "base":  "D:\\Projects\\GithubIssueWidget\\docs\\agent-runs\\20261118-160129-mvp-restart",
                           "relative_path":  "process/reference-check-failure-01.md",
                           "sha256":  "78AAD8B6C3329E29472FCF7BC144237C25F6B2BF2E8CF71284FB9890D4683957"
                       }
                   ]
}
```

対象はドラッグの追加切り分け設計。Issue #1/#2 の原文と過去の事後記録、対象版のコード・capability、固定参照を独立に照合し、事実と仮説・未確認、イベント送達・押下保持・IPC 権限と OS 応答の区別、段階的な証拠、設計が成功した場合に元の Issue の移動要件を実証できるかを確認する。診断版の挙動を製品版の成功と混同せず、元事象の原因確定と修正後の回帰確認・実機証明は別に扱う。

専用コピー・対象版・所有・同時進行 build と画面・process・port の競合、入力・環境・秘密・副作用・復元、観測ログの正確性と限界、各段階の独立レビュー・安全な中断条件を評価。実行可能性を未確認の前提は未確認として記録し、設計承認を実行許可や製品完成とみなさない。B03 のセッションは参照時点で live・終了不明であり、実際の状態を確認せず再起動・停止しない。

実機操作・ドラッグ・入力・アプリ起動・build・通信を伴う試験・製品・既存証拠・Git・GitHub の変更は禁止。存在する資料と公式一次資料の静的読取のみ。パスを推測せず基点・相対パス・注記を分け、全件の存在確認後にハッシュを照合する。構造化フィールドの失敗履歴を保持し、同種問題が初回を含む2回目なら該当操作を停止し別担当が客観調査、3回目なら全作業を中止する。失敗した公式ページ取得をそのまま繰り返さず、一次資料の他の安全な参照が必要か判断する。

共有 run の指定結果と新規質問だけを所有し、親ディレクトリと確定先・tmp 不在を確認し UTF-8 往復後に確定する。各要求、対象版、実測 SHA、根拠、未確認、blocking / non-blocking 指摘、是正条件、cwd、コマンド・終了コード、版限定の判定を記載。結果パスだけを通知する。