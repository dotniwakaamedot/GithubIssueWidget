# I02 診断版 DOM 境界観測の独立レビュー依頼 01

構造化フィールドを依頼の正本とする。担当と作者、旧新 scratch、対象ファイルと件数、出力、ユーザー指示、失敗履歴・閾値、固定参照と SHA は同一データから生成。起動直前に実担当・対象版・出力先・競合を再照合し、変化や所有不明なら停止する。照合は排他ロックではない。
```json
{
    "run_id":  "20261118-160129-mvp-restart",
    "task_id":  "I02-geometry-instrumentation-review",
    "reviewer":  "/root/plan_review",
    "author":  "/root/drag_design",
    "source_scratch":  "D:\\Projects\\GithubIssueWidget-drag-diagnostic-20261118-160129",
    "geometry_scratch":  "D:\\Projects\\GithubIssueWidget-geometry-diagnostic-20261118-160129",
    "prior_diagnostic_files":  18,
    "expected_unchanged":  17,
    "expected_changed":  1,
    "output_relative_path":  "tasks/I02/geometry-instrumentation-review-01.md",
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
                           "relative_path":  "tasks/I02/geometry-instrumentation-request-01.md",
                           "sha256":  "FE760D81FD0C614E2BAA547C139D39596AB9A4DFCD22AF490FEDECDE43C8CE4E"
                       },
                       {
                           "base":  "D:\\Projects\\GithubIssueWidget\\docs\\agent-runs\\20261118-160129-mvp-restart",
                           "relative_path":  "tasks/I02/geometry-instrumentation-result-01.md",
                           "sha256":  "31A5ECC4D7699F27B00E6B6717FF69D40CAB857BDF463BF671D1A7EA0DF41B52"
                       },
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
                           "relative_path":  "tasks/I02/drag-instrumentation-review-02.md",
                           "sha256":  "191A8B66C672DF8C1794FB08312B390BC55402561B4FE189B6CD5ECAAB2ABBF4"
                       },
                       {
                           "base":  "D:\\Projects\\GithubIssueWidget\\docs\\agent-runs\\20261118-160129-mvp-restart",
                           "relative_path":  "tasks/I02/drag-instrumentation-revision-result-01.md",
                           "sha256":  "43AE521C9934642B3CA71553BACAA7C7E0129C4DD8421D59B4713A84000DCEE1"
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
                       },
                       {
                           "base":  "D:\\Projects\\GithubIssueWidget-drag-diagnostic-20261118-160129",
                           "relative_path":  "widget/src/diagnostic.ts",
                           "sha256":  "56A62632E8D63AEE5DD0AB636C954E30FBB76562C90C88CD9C0D53AB497B6D37"
                       }
                   ]
}
```

対象は同一時点の marker / header / 全 button / resize 帯の CSS viewport 矩形を非機密の output に示す診断コード案と専用コピー。screenshot 観測設計レビューの B01 是正条件を、元 Issue #2、元17ファイルと旧診断版・新診断版、対象コード・結果と固定 SHA に照らして独立確認。旧18ファイル中17不変・diagnostic.ts 1変更という結果を両基点の全件実在確認後に再計算し、元製品・旧診断 scratch と既存生成物・保護中 process を変更しない。

同一同期 sample と開始・終了の単調時刻、header / marker / button 3件 / resize 8件、座標・viewport・DPR・scroll・DOM hit target の計測と必要な失敗表示を精査。各矩形間の最小余白の計算が必要な「非重複・各境界から12 CSS px」を実際に保証するか、矩形の重なり・空や負の大きさ・位置変化・要素増減・祖先の transform / viewport との不整合・数値異常を検討し、未確認は未確認とする。DOM・画像・Win32・操作 API の座標系の対応、OS窓による遮蔽や時間差は静的コードだけで証明しない。

出力は指定された固定分類・幾何情報・時刻だけか、DOM text・URL・キー入力・Issue 内容・個人情報を取得しないか確認。header の drag 無効化、残る button / resize / capability、診断の副作用、同じ画面内で正確な output を取得し保存できる条件を評価。geometry と実際の対象窓・PID・exe・画面・DPI が対応しない場合、取得欠落やレイアウト変化・ユーザー操作と競合がある場合に入力しないゲートを確認。新旧 scratch、old process / port / profile と B03 の所有・非競合、保持・復元と通常の副作用も評価する。

実装・型検査・build・起動・UI 表示・画面取得・クリック・ドラッグ・移動・通信試験、製品・旧新 scratch・既存証跡・Git・GitHub の変更は禁止。静的読取と必要な公式一次資料の調査のみ。対象の画像不一致は1回、同種2回で該当観測を止め別担当へ客観調査、3回で全停止。browser inventory は2回失敗済みのため getState / 同じ browser inventory を再実行せず、同種3回で全停止。構造化フィールドの他の失敗履歴を保持する。

共有 run の指定結果と自担当の新規質問のみ所有。親ディレクトリと確定先・tmp 不在を確認し UTF-8 往復後に確定。対象旧新18件・変更分・参照の実測 SHA、コードの具体的根拠・余白式・安全性・版・未確認、blocking / non-blocking 指摘と是正条件、cwd・コマンド・終了コード、版限定の判定を記録。設計承認は実際の起動・画面取得・入力、製品完成を自動許可しない。結果パスのみ通知。