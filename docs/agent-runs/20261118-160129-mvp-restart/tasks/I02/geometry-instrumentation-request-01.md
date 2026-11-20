# I02 診断版の DOM 境界観測の設計・修正依頼 01

構造化フィールドを依頼の正本とする。担当・レビュアー・旧新 scratch・出力、ユーザー指示、失敗履歴・閾値、固定参照と SHA は同一データから生成。開始直前に担当 ID・対象版・出力先・所有者・競合を再照合する。照合は排他ロックではない。
```json
{
    "run_id":  "20261118-160129-mvp-restart",
    "task_id":  "I02-geometry-instrumentation",
    "owner":  "/root/drag_design",
    "reviewer":  "/root/plan_review",
    "source_scratch":  "D:\\Projects\\GithubIssueWidget-drag-diagnostic-20261118-160129",
    "new_scratch":  "D:\\Projects\\GithubIssueWidget-geometry-diagnostic-20261118-160129",
    "output_relative_path":  "tasks/I02/geometry-instrumentation-result-01.md",
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
                           "relative_path":  "tasks/I02/screenshot-observation-review-01.md",
                           "sha256":  "B8851AFC77E21D6B48A6007BF15BBFF5C74CBFADF0834445AAEAD13317ECCB5F"
                       },
                       {
                           "base":  "D:\\Projects\\GithubIssueWidget\\docs\\agent-runs\\20261118-160129-mvp-restart",
                           "relative_path":  "tasks/I02/screenshot-observation-design-result-01.md",
                           "sha256":  "4D33B22968BCF4FBB6930C095B56480A87C47E1F2A55406AD9D97485E0539F79"
                       },
                       {
                           "base":  "D:\\Projects\\GithubIssueWidget\\docs\\agent-runs\\20261118-160129-mvp-restart",
                           "relative_path":  "tasks/I02/drag-instrumentation-revision-result-01.md",
                           "sha256":  "43AE521C9934642B3CA71553BACAA7C7E0129C4DD8421D59B4713A84000DCEE1"
                       },
                       {
                           "base":  "D:\\Projects\\GithubIssueWidget\\docs\\agent-runs\\20261118-160129-mvp-restart",
                           "relative_path":  "tasks/I02/drag-instrumentation-review-02.md",
                           "sha256":  "191A8B66C672DF8C1794FB08312B390BC55402561B4FE189B6CD5ECAAB2ABBF4"
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
                           "relative_path":  "questions/I02-drag-click-question-08.md",
                           "sha256":  "E45AD6C76F7AB0C6DFA2FFD404E39594E54222033D0D17B36245D741362E3BA8"
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

screenshot 観測設計の独立レビュー B01 は、marker・各 button・resize 帯を同時点の同一 CSS 座標系で測り、12 CSS px の余白を確認する具体的入口が不足すると指摘した。Issue の移動機能 B01 とは別の指摘である。過去の診断窓画像は未保存・原因未確定で、同じ画像取得やドラッグは再試行しない。元のウィンドウ移動は未達のまま。

元17件と旧診断版の4変更の実体・SHA を固定参照と照合し、old scratch と所有中の dev server / app の現存・所有を確認。旧 process・port・scratch と証跡は古い版として保護し、止めたり再起動したり変更しない。新 scratch は独立した未存在の専用領域で、親・正規化先・所有・reparse / hardlink・容量と他の worktree / B03 資産との非重複を確認。元と旧診断の対象ファイルだけを通常バイトコピーし、コピー元・先のサイズ・SHA と差分を記録。既存生成物や node_modules はコピーしない。所有者・対象版が確認不能なら停止する。

新しい診断版について、同一タイミング・同一 CSS viewport 座標系で marker・header・全 button・各 resize 帯の getBoundingClientRect と viewport サイズ、devicePixelRatio、非機密の固定要素区分・時刻を取得し、既存 output からアクセスできる具体案を設計し必要なコードだけを新 scratch 内で修正。DOM text、URL、キー入力、Issue 内容や個人情報を読まない。取得・表示自体の副作用、描画の遅延・layout shift、要素欠落、スクロール、DPI、CSS と対象アプリ・画面座標の対応、12 CSS px 余白の計算方法と限界を示す。矩形・対象識別・観測時刻・実座標が取れない、境界が重なる、対象変更や競合がある場合は入力しない。矩形の値を推測したり、AX tree とスクリーンショットの不一致を成功としない。

起動中の旧版と新コードの対象版・process・window を混同しない。新しい診断版のファイルと変更点・SHA、許可する観測項目と出力仕様・安全領域の判定・各条件が揃わない場合の停止方法を確定記録に示し、作者以外のレビューと親判断を得る。実際の取得・起動・build・UI 表示・画面 capture・クリック・ドラッグ・移動・外部通信は行わない。元ドラッグの2回の位置不変は原因未確定、診断版の観測も製品版成功を証明しない。

所有範囲は構造化フィールドの新 scratch と指定の共有 run 結果・新規質問のみ。旧 scratch と実行中プロセス、元 worktree・既存証跡・Git・GitHub の変更・削除は禁止。結果には作業ディレクトリ・入力と出力の version/hash・コピー/差分・設計、読み取りコマンドと終了コード、実装・未確認・リスクを記載。親ディレクトリと確定先・tmp 不在を確認し UTF-8 往復後に確定し、パスだけ通知する。

構造化フィールドの失敗履歴を維持。同種問題の初回を含む2回で該当操作を中断し別担当の客観調査、3回で全停止。画像不一致は既往1回、browser inventory error は既往2回。getState / browser inventory・前回の screenshot・ドラッグ・文字破損経路を再試行しない。