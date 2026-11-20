# I02 診断版コードの独立レビュー依頼 02

構造化フィールドを依頼の正本とする。担当と作者、対象 scratch、修正版4ファイル、出力、ユーザー指示、履歴・閾値、固定参照と SHA は同一データから生成。実担当・対象版・出力先・競合は起動直前に再照合し、変化や所有不明なら停止。照合は排他ロックではない。
```json
{
    "run_id":  "20261118-160129-mvp-restart",
    "task_id":  "I02-drag-instrumentation-review",
    "review_round":  2,
    "reviewer":  "/root/plan_review",
    "author":  "/root/drag_design",
    "scratch":  "D:\\Projects\\GithubIssueWidget-drag-diagnostic-20261118-160129",
    "original_files":  17,
    "unchanged_original":  14,
    "modified_original":  3,
    "new_diagnostic_files":  1,
    "port":  1435,
    "output_relative_path":  "tasks/I02/drag-instrumentation-review-02.md",
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
                           "relative_path":  "tasks/I02/drag-instrumentation-revision-request-01.md",
                           "sha256":  "78213D01E56433DE9F31E47E3B9A4410E20B6CD1F158DC986064CC6C2533C217"
                       },
                       {
                           "base":  "D:\\Projects\\GithubIssueWidget\\docs\\agent-runs\\20261118-160129-mvp-restart",
                           "relative_path":  "tasks/I02/drag-instrumentation-revision-result-01.md",
                           "sha256":  "43AE521C9934642B3CA71553BACAA7C7E0129C4DD8421D59B4713A84000DCEE1"
                       },
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
                           "sha256":  "820F164EF8947DA6B87F6ACE8B74D787E09ED9AF7A6610E2C07678DFA2C4AE5C"
                       },
                       {
                           "base":  "D:\\Projects\\GithubIssueWidget-drag-diagnostic-20261118-160129",
                           "relative_path":  "widget/src/diagnostic.ts",
                           "sha256":  "56A62632E8D63AEE5DD0AB636C954E30FBB76562C90C88CD9C0D53AB497B6D37"
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

前回レビュー B01 は入力後の配列取得方法と対象 main WebView の識別が未確定、B02 は button / resize 帯を除く一意な入力領域が未確定で blocking。独立して元版・前版・修正版と実体 SHA を照合し、作者結果の提案がそれぞれの是正条件を満たすか評価する。前回質問のファイル件数は管理の訂正記録を参照し、元17件のうち14件不変・3件変更・別に1件新規という実数を独立に確認。2回目の同じ意味の指摘は適用手順に従い停止する。

対象 main WebView・HWND/PID・exe・バージョンの識別、空の diagnostic-ready を入力前に同一対象でアクセシビリティ又は画像として取得できる前提、単一入力後の取得・非機密保存・時刻とハッシュの対応、失敗や不明時に入力や再試行を停止する条件を評価。output の表示可能性・型・OS/UI の実動作は未実施として扱い、静的コードだけで成功としない。

header・marker・各 button・resize 帯の実際の境界、対象 PID・DPI・座標変換・12 CSS px の安全余白と唯一の候補、操作領域が不明・不足・重なり時に入力しないゲートを評価。コード上 header の `startDragging` が無効化されているか、別の drag 経路・resize 機能・capability が残ること、イベントの収集項目と秘密・他 UI への副作用を確認。診断版は元の過去のドラッグ到達や製品移動の成功を証明しない。

B03 scratch・元 worktree・既存証拠・同一 desktop・port・profile の所有と非競合、build・依存の書込み先と通常の副作用、対象版凍結と親判断を確認。実際のクリック、ドラッグ、UI操作、アプリ起動、build、通信試験、製品・scratch・既存証拠・Git・GitHub の変更は禁止。パスを推測せず基点・相対パス・注記を分け、全件存在確認後に hash を照合。構造化フィールドの履歴を保持し、同種2回で該当操作を中断し別担当が客観調査、3回で全停止。過去のドラッグ・文字破損・lockfile の失敗経路を再試行しない。

共有 run の指定結果と新規質問のみ所有。親ディレクトリと確定先・tmp 不在を確認し UTF-8 往復後に確定。対象版の四変更と元17件の実測 SHA・前版との差分・各指摘の是正判定・根拠・未確認、blocking / non-blocking と是正条件、cwd・コマンド・終了コード、版限定の結論を記載。承認は実際の起動・入力や製品完成を自動許可しない。結果パスだけを通知。