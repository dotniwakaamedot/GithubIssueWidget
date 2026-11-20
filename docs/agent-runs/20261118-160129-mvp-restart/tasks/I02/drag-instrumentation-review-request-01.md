# I02 診断版コードの独立レビュー依頼 01

構造化フィールドを依頼の正本とする。担当と作者、対象 scratch、四つの診断変更、出力、ユーザー指示、失敗履歴・閾値、固定参照と SHA は同一データから生成。起動直前に実担当・対象版・出力先・競合を再照合し、変更や所有不明なら停止する。照合は排他ロックではない。
```json
{
    "run_id":  "20261118-160129-mvp-restart",
    "task_id":  "I02-drag-instrumentation-review",
    "reviewer":  "/root/plan_review",
    "author":  "/root/drag_design",
    "scratch":  "D:\\Projects\\GithubIssueWidget-drag-diagnostic-20261118-160129",
    "original_files":  17,
    "unchanged_original":  14,
    "modified_original":  3,
    "new_diagnostic_files":  1,
    "port":  1435,
    "output_relative_path":  "tasks/I02/drag-instrumentation-review-01.md",
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
                    "official_page_open_internal_error":  1,
                    "copy_section_extract_failure":  1,
                    "dispatch_before_dependency_gate_confirmed":  1,
                    "process_self_match_failure":  1
                },
    "references":  [
                       {
                           "base":  "D:\\Projects\\GithubIssueWidget\\docs\\agent-runs\\20261118-160129-mvp-restart",
                           "relative_path":  "questions/I02-drag-click-question-03.md",
                           "sha256":  "06B57BE45C4A756BB2C240E9260B5F744996EADE5D04D795DD07E6695818D981"
                       },
                       {
                           "base":  "D:\\Projects\\GithubIssueWidget\\docs\\agent-runs\\20261118-160129-mvp-restart",
                           "relative_path":  "questions/I02-drag-click-answer-02.md",
                           "sha256":  "0F17C0ACF8A8EC334C2AC60C1D6A2AD4D72017E0F42A1993EC3AED753D57368D"
                       },
                       {
                           "base":  "D:\\Projects\\GithubIssueWidget\\docs\\agent-runs\\20261118-160129-mvp-restart",
                           "relative_path":  "tasks/I02/drag-click-request-01.md",
                           "sha256":  "83D63BA78D6CEEC7E07BDF9834583D0274BCADCEFC5DB5BCAB6B25377880CB6C"
                       },
                       {
                           "base":  "D:\\Projects\\GithubIssueWidget\\docs\\agent-runs\\20261118-160129-mvp-restart",
                           "relative_path":  "tasks/I02/build-drag-preflight-review-01.md",
                           "sha256":  "EE06BEACEF4DFEF9E26204B7E3FC575827B5DAFC8682C0F9381E3E23485DEFF9"
                       },
                       {
                           "base":  "D:\\Projects\\GithubIssueWidget\\docs\\agent-runs\\20261118-160129-mvp-restart",
                           "relative_path":  "tasks/I02/drag-investigation-design-result-01.md",
                           "sha256":  "FA635B310797FBCB5C50F6B0F5967E87B0B3131CDB33F8F899332EF1E54A7A08"
                       },
                       {
                           "base":  "D:\\Projects\\GithubIssueWidget\\docs\\agent-runs\\20261118-160129-mvp-restart",
                           "relative_path":  "tasks/I02/drag-investigation-review-01.md",
                           "sha256":  "D9FABAE1C51E0B46864477933214102A53876DCBFC9AB54CF37644A98F9024A3"
                       },
                       {
                           "base":  "D:\\Projects\\GithubIssueWidget\\docs\\agent-runs\\20261118-160129-mvp-restart",
                           "relative_path":  "tasks/I02/evidence-drag-click-01/source-manifest.json",
                           "sha256":  "E5C0FA60CB40DF2E59F15FABEF884D545FDABC04076DBCD2C8FDD220367990DC"
                       },
                       {
                           "base":  "D:\\Projects\\GithubIssueWidget\\docs\\agent-runs\\20261118-160129-mvp-restart",
                           "relative_path":  "tasks/I02/result-02.md",
                           "sha256":  "520044FE6138609B99595C5FAF1D3973DD2D62EB82F27FE90392568C9599ED13"
                       },
                       {
                           "base":  "D:\\Projects\\GithubIssueWidget\\docs\\agent-runs\\20261118-160129-mvp-restart",
                           "relative_path":  "process/reference-check-design-02.md",
                           "sha256":  "3CDCB637F739F9A9DBE240D0901557BC2BB23732ABF7CCC528F98A514C06C06E"
                       },
                       {
                           "base":  "D:\\Projects\\GithubIssueWidget-drag-diagnostic-20261118-160129",
                           "relative_path":  "widget/src/main.tsx",
                           "sha256":  "F318C76FE54F081F55656B84C0C9680C1416560E272C003C480C6B7C03CA4FA2"
                       },
                       {
                           "base":  "D:\\Projects\\GithubIssueWidget-drag-diagnostic-20261118-160129",
                           "relative_path":  "widget/src/diagnostic.ts",
                           "sha256":  "76163B634209ABE4976F2A954E463EDFAEF7AB97B110E0B1D96858135682ADDF"
                       },
                       {
                           "base":  "D:\\Projects\\GithubIssueWidget-drag-diagnostic-20261118-160129",
                           "relative_path":  "widget/vite.config.ts",
                           "sha256":  "194D17A739D0F626DDAF94B5D2B6F9335DBF50BB571AB4F7739DA727ED4F772A"
                       },
                       {
                           "base":  "D:\\Projects\\GithubIssueWidget-drag-diagnostic-20261118-160129",
                           "relative_path":  "widget/src-tauri/tauri.conf.json",
                           "sha256":  "EDE0EA3E1EE5C6FE067A5E127EEAAE02A19110E8CD8E057C8B7FCB2B826A55D5"
                       }
                   ]
}
```

質問03の報告と実体を独立に照合。元 manifest の17件は元とコピーを基点別に全件存在確認後に SHA を検証。管理の読み取りでは元17件のうち14件がコピーと一致、3件が変更され、別に diagnostic.ts 1件が新規で計4変更。質問03の「上記以外の15件」との記述は算術上の不一致があるため、実数と範囲を独立に特定し、是正や記録の補足が必要なら指摘する。上記の件数も検証を代行するものではない。

四つの対象ファイルと元版との差分を確認し、header の通常 `startDragging` が確実に呼ばれないか、別のドラッグ経路や Tauri 権限への影響がないか、mousedown / mouseup の capture / bubble、event type・button・buttons・isTrusted・対象区分・粗い位置・単調時刻以外の内容を記録しないかを確認。リサイズ処理は残存するため header 以外への入力が危険になり得る点を評価。診断版は製品版と異なり、クリック到達から過去のドラッグ成功や Issue 完了を推定しない。

port の構成と 1435 の専用性、target・cache・temp・profile・B03 scratch・UI／プロセスとの競合、環境・依存・起動による外部副作用、ログの取得方法・対象 HWND/PID/exe・入力先を特定できる条件を評価。依存準備・build・アプリ起動・UI 表示・入力はまだ行われていない。診断版の型・動作は静的確認だけでは未実証。必要な開始条件と親判断、失敗時停止・保持・非機密性を明記。

実際のクリック・ドラッグ・UI操作・アプリ起動・build・通信を伴う試験・製品／scratch／既存証拠／Git／GitHub の変更は禁止。静的な読取と必要な公式一次資料の調査のみ。パスを推測せず基点・相対パス・注記を分け、全件の存在確認後に hash を照合。構造化フィールドの履歴を保持し、同種2回で該当操作を中断し別担当が客観調査、3回で全停止。過去のドラッグ・文字破損・lockfile 失敗経路を再試行しない。

共有 run の指定結果と新規質問だけを所有。親ディレクトリと確定先・tmp 不在を確認し UTF-8 往復後に確定。対象17件と四変更の実測 SHA、差分、担当・版、各要求、根拠・未確認、blocking / non-blocking 指摘・是正条件、cwd・コマンド・終了コード、版限定の判定を記載。承認は実入力・製品完成を自動許可しない。結果パスのみ通知する。