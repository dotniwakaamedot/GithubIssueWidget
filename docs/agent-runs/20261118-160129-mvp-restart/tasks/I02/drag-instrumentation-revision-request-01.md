# I02 診断版の記録取得・安全領域の修正依頼 01

構造化フィールドを依頼の正本とする。担当と前回レビュアー、対象 scratch、四つの変更、出力、ユーザー指示、失敗履歴・閾値、固定参照と SHA は同一データから生成した。起動直前に実担当・対象版・出力先・競合を再照合し、変更や所有不明なら停止する。照合は排他ロックではない。
```json
{
    "run_id":  "20261118-160129-mvp-restart",
    "task_id":  "I02-drag-instrumentation-revision",
    "owner":  "/root/drag_design",
    "reviewer":  "/root/plan_review",
    "scratch":  "D:\\Projects\\GithubIssueWidget-drag-diagnostic-20261118-160129",
    "original_files":  17,
    "unchanged_original":  14,
    "modified_original":  3,
    "new_diagnostic_files":  1,
    "port":  1435,
    "output_relative_path":  "tasks/I02/drag-instrumentation-revision-result-01.md",
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
                           "relative_path":  "tasks/I02/drag-instrumentation-review-01.md",
                           "sha256":  "93D043E8D9DC5F13AA9C27005B7FC05838DA01C05348C5FC7A31D5D5AC6CFAAF"
                       },
                       {
                           "base":  "D:\\Projects\\GithubIssueWidget\\docs\\agent-runs\\20261118-160129-mvp-restart",
                           "relative_path":  "tasks/I02/drag-count-correction-01.md",
                           "sha256":  "71637C9E9ADF66DD9094A9E9C53A1AEE01761F5EA9917530C29D9EBC57FFBC86"
                       },
                       {
                           "base":  "D:\\Projects\\GithubIssueWidget\\docs\\agent-runs\\20261118-160129-mvp-restart",
                           "relative_path":  "tasks/I02/drag-click-request-01.md",
                           "sha256":  "83D63BA78D6CEEC7E07BDF9834583D0274BCADCEFC5DB5BCAB6B25377880CB6C"
                       },
                       {
                           "base":  "D:\\Projects\\GithubIssueWidget\\docs\\agent-runs\\20261118-160129-mvp-restart",
                           "relative_path":  "questions/I02-drag-click-question-03.md",
                           "sha256":  "06B57BE45C4A756BB2C240E9260B5F744996EADE5D04D795DD07E6695818D981"
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

前回独立レビューの B01/B02 は blocking、診断版の起動や入力は承認されていない。元のドラッグは2回位置不変で原因未確定であり再試行しない。前回の質問03は元17ファイル中の未変更数を誤記し、訂正記録に14件不変・3件変更・1件新規と記載した。旧質問・レビューは書き換えない。

B01: 対象 main WebView・PID・window ID・exe・対象版を一意に識別し、メモリ内の非機密イベント配列を具体的に取得し専用 evidence に保存する方法を設計する。取得手段の実際の利用可能性、異なる WebView や不要なデータへのアクセス、時刻と操作・版の対応、失敗・不明時の停止を明記。取得可能性が不明なまま入力しない。技術的な方法検証が必要なら同じクリックの反復をせず安全な別方法を提案し、実行前に別途レビューと親判断を受ける。

B02: 対象 HWND/PID・DPI と尺度・header、各 button、周囲 resize 帯の境界・安全余白を操作前に確認し、button と resize 帯を除いた非対話領域を一意に選択する具体的手順を設計。対象が不明・狭い・他者操作と競合する場合は入力せず停止。単一押下・解放以外、特にドラッグ、移動、リサイズを禁止。診断版での成功は製品版の移動や過去の動作の証明ではない。

診断 scratch の構造化参照で示す4ファイルに限り設計に必要な修正を許可。元 worktree・既存証跡・Git・GitHub と他者の作業は変更しない。修正の必要がなければ文章で具体的方法を提示。元17件・前版4変更・改訂版の差分と全 SHA、収集項目・上限・秘密情報を含まない条件、1435と分離された資産・プロセスを記録。動作確認を装って起動、build、通信、UI表示、クリックや入力注入を行わない。B03 の結果と既存 scratch を保護し再利用・停止しない。

共有 run の指定結果と新規質問、および上記診断 scratch の4対象のみ所有。親ディレクトリと確定先・tmp 不在を確認し UTF-8 往復後に結果を確定。前レビュー各指摘への対応、対象版・差分・実測 SHA、取得・対象選択方法と安全性、未確認、失敗時停止・現実的副作用、cwd・読取コマンドと終了コードを記録。修正版は別担当がレビューし親が判断するまで起動・入力禁止。パスのみ通知。

構造化フィールドの失敗履歴を保持し、同種2回で該当操作中断・別担当の客観調査、同種3回で全停止。以前のドラッグ・文字破損・lockfile 失敗経路や同じ誤りを繰り返さない。