# I02 旧診断終了と新版準備・起動の依頼 01

構造化フィールドを依頼の正本とする。担当・レビュアー・旧新 scratch、過去の handle / PID、port、出力、ユーザー指示、失敗履歴と閾値、固定参照・SHA は同一データから生成した。過去の handle / PID は現在の同一性を保証しない。開始直前に担当・参照版・所有・対象・競合を再照合し、不明なら停止する。照合は排他ロックではない。
```json
{
    "run_id":  "20261118-160129-mvp-restart",
    "task_id":  "I02-geometry-execution",
    "owner":  "/root/drag_design",
    "reviewer":  "/root/plan_review",
    "old_scratch":  "D:\\Projects\\GithubIssueWidget-drag-diagnostic-20261118-160129",
    "geometry_scratch":  "D:\\Projects\\GithubIssueWidget-geometry-diagnostic-20261118-160129",
    "old_dev_session":  22902,
    "old_app_session":  17031,
    "old_app_pid":  21312,
    "port":  1435,
    "evidence_relative_path":  "tasks/I02/evidence-geometry-01",
    "output_relative_path":  "tasks/I02/geometry-execution-result-01.md",
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
                    "screenshot_mismatch":  1,
                    "browser_inventory_failure":  2,
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
                           "relative_path":  "tasks/I02/geometry-instrumentation-result-01.md",
                           "sha256":  "31A5ECC4D7699F27B00E6B6717FF69D40CAB857BDF463BF671D1A7EA0DF41B52"
                       },
                       {
                           "base":  "D:\\Projects\\GithubIssueWidget\\docs\\agent-runs\\20261118-160129-mvp-restart",
                           "relative_path":  "tasks/I02/geometry-instrumentation-review-01.md",
                           "sha256":  "5EAC109FC6AE4FE665EE83982385E9EA01FC258FCBE438D415ABB2E4469F9B68"
                       },
                       {
                           "base":  "D:\\Projects\\GithubIssueWidget\\docs\\agent-runs\\20261118-160129-mvp-restart",
                           "relative_path":  "tasks/I02/geometry-instrumentation-request-01.md",
                           "sha256":  "FE760D81FD0C614E2BAA547C139D39596AB9A4DFCD22AF490FEDECDE43C8CE4E"
                       },
                       {
                           "base":  "D:\\Projects\\GithubIssueWidget\\docs\\agent-runs\\20261118-160129-mvp-restart",
                           "relative_path":  "tasks/I02/drag-click-request-01.md",
                           "sha256":  "83D63BA78D6CEEC7E07BDF9834583D0274BCADCEFC5DB5BCAB6B25377880CB6C"
                       },
                       {
                           "base":  "D:\\Projects\\GithubIssueWidget\\docs\\agent-runs\\20261118-160129-mvp-restart",
                           "relative_path":  "tasks/I02/verification-design-result-03.md",
                           "sha256":  "B2B552301D897717CC80404A64297C460BADED4C8AE4B0F130B1557F7B08DAED"
                       },
                       {
                           "base":  "D:\\Projects\\GithubIssueWidget\\docs\\agent-runs\\20261118-160129-mvp-restart",
                           "relative_path":  "tasks/I02/verification-design-review-03.md",
                           "sha256":  "37899D597AAC6DA8F94E2775818A8DDC76796C0EBAA730E9404BCE14EEE20C25"
                       },
                       {
                           "base":  "D:\\Projects\\GithubIssueWidget\\docs\\agent-runs\\20261118-160129-mvp-restart",
                           "relative_path":  "tasks/I02/result-02.md",
                           "sha256":  "520044FE6138609B99595C5FAF1D3973DD2D62EB82F27FE90392568C9599ED13"
                       },
                       {
                           "base":  "D:\\Projects\\GithubIssueWidget\\docs\\agent-runs\\20261118-160129-mvp-restart",
                           "relative_path":  "questions/I02-drag-click-question-09.md",
                           "sha256":  "6C5E76248C84C943EA1858D60FA2C918F745E759C17A4FEB03830BCB0CF3EA9C"
                       },
                       {
                           "base":  "D:\\Projects\\GithubIssueWidget\\docs\\agent-runs\\20261118-160129-mvp-restart",
                           "relative_path":  "process/reference-check-design-02.md",
                           "sha256":  "3CDCB637F739F9A9DBE240D0901557BC2BB23732ABF7CCC528F98A514C06C06E"
                       },
                       {
                           "base":  "D:\\Projects\\GithubIssueWidget-geometry-diagnostic-20261118-160129",
                           "relative_path":  "copy-manifest.json",
                           "sha256":  "F75B6663167FADC32277BE610DAA67C1C3B96B7FC74BA1AEB6890C9EF3C56DC1"
                       },
                       {
                           "base":  "D:\\Projects\\GithubIssueWidget-geometry-diagnostic-20261118-160129",
                           "relative_path":  "widget/src/main.tsx",
                           "sha256":  "820F164EF8947DA6B87F6ACE8B74D787E09ED9AF7A6610E2C07678DFA2C4AE5C"
                       },
                       {
                           "base":  "D:\\Projects\\GithubIssueWidget-geometry-diagnostic-20261118-160129",
                           "relative_path":  "widget/src/diagnostic.ts",
                           "sha256":  "840D7F69349AEED52C0A5D6FA6319317CCE432390A74463F8E824069F26E412D"
                       },
                       {
                           "base":  "D:\\Projects\\GithubIssueWidget-geometry-diagnostic-20261118-160129",
                           "relative_path":  "widget/vite.config.ts",
                           "sha256":  "194D17A739D0F626DDAF94B5D2B6F9335DBF50BB571AB4F7739DA727ED4F772A"
                       },
                       {
                           "base":  "D:\\Projects\\GithubIssueWidget-geometry-diagnostic-20261118-160129",
                           "relative_path":  "widget/src-tauri/tauri.conf.json",
                           "sha256":  "EDE0EA3E1EE5C6FE067A5E127EEAAE02A19110E8CD8E057C8B7FCB2B826A55D5"
                       }
                   ]
}
```

目的は、所有者が旧診断 app と server を対象確認後に穏当に終了し、証拠を保持して、新たに独立レビューされた geometry 診断版を専用領域で準備・起動・表示すること。これは画像・安全領域・単一クリック・製品版ドラッグの成功や Issue 完了を意味しない。

旧版の操作は所有担当本人のみ。各 session handle を poll して現在の live / terminal / 不明を区別し、PID・exe path・版・port と対象所有を照合する。対象不明・別 process・不測の状況なら停止し親に質問する。所有中の旧 app、次に旧 server だけを通常の穏当な終了方法で終了し、各 handle の terminal と終了コード・時刻・関連 port の状態を記録。timeout を terminal と決めつけず、他アプリ・B03 process を触らない。強制終了や所有外への操作が必要なら実施せず親に相談する。旧ログ・旧 scratch と生成物は削除せず保持する。

対象の新 scratch と元 worktree の版を固定参照・manifest に照らして全件存在・SHA を確認。対象版の4変更と geometry 1変更の実体を独立レビュー版と対応付け、旧版の process / window と混同しない。ポート1435を含む process・desktop・ユーザー操作の競合を読み取りで確認し、不明なら新規起動しない。親と旧新 scratch・元製品・B03 資産の非重複、所有、リンク・再解析ポイントと空き容量を確認。別の専用 target・cache・temp・profile・evidence を使用し既存資産・資格情報を保護する。依存と public registry の必要性・接続先・lifecycle scripts・通常副作用を事前確認し、不明なら停止。ユーザー設定や Credential Manager を変更しない。

条件が揃えば新診断版を準備し、型検査 / build と実際の起動・表示を実施。コマンド、cwd、環境、開始終了 UTC、個別 exit code、PID・exe 絶対パス・SHA・window ID・対象版を記録。build / 起動の失敗・結果不明は成功とせず、同種失敗を数える。所有した process の handle を保持し、観測中の終了・再実行は親判断を得る。dev port の所有と実体を確認し、対象以外の process を変更しない。

入力は禁止。対象 main 窓・exe の個別識別ができる場合に限り、文書化された提供 API による安全な読み取りで event output / geometry output の公開可否と対象 ID を確認することはできる。ただし同じ全サーフェス getState / browser inventory は既に2回失敗し、前回の対象外画像となった同じ screenshot 観測も1回失敗しているため、これらを再実行しない。代替画面取得や UI 入力を要する場合は実施せず、別途承認された方式を待つ。対象画像・AX・矩形・有限正寸法・DPI / 座標・全 button / resize との余白が不足・不明なら単一クリックは行わない。ドラッグ・移動・リサイズは常に禁止。

実行状況と旧 process の所有・終了記録、対象版・変更の SHA、操作・ログ・終了コード・新 process / port / window の対応、通常の副作用・未確認と各失敗を指定 evidence と結果へ記載。親ディレクトリ・確定先・tmp 不在を確認し UTF-8 往復後に確定し、結果パスのみ通知。scratch・旧新生成物・証拠の削除、元製品・Git・GitHub 変更、push / PR / merge は禁止。

同種問題は初回を含む2回で該当操作を中断し別担当が客観調査、3回で全停止。構造化フィールドの失敗履歴を保持し、同じ失敗経路を盲目的に再試行しない。