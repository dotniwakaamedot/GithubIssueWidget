# I02 診断 marker の余白不足に対する計画 01

次は実施案であり、作者以外の独立レビューと親の判断前にコピー・コード修正・起動・画像取得・入力を行わない。対象版・観測・保存先・参照 SHA・履歴を次の構造化データに固定する。
```json
{
    "run_id":  "20261118-160129-mvp-restart",
    "task_id":  "I02-marker-adjustment-plan",
    "owner":  "/root/restart_planner",
    "source_scratch":  "D:\\Projects\\GithubIssueWidget-geometry-diagnostic-20261118-160129",
    "candidate_new_scratch":  "D:\\Projects\\GithubIssueWidget-marker-diagnostic-20261118-160129",
    "observed_minimum_gap_css_px":  11,
    "required_minimum_gap_css_px":  12,
    "history":  {
                    "direct_final_publication_violations":  2,
                    "screenshot_mismatch":  1,
                    "browser_inventory_failure":  2,
                    "drag_no_movement_before_current_restart":  2
                },
    "thresholds":  {
                       "independent_investigation":  2,
                       "stop_all":  3
                   },
    "references":  [
                       {
                           "base":  "D:\\Projects\\GithubIssueWidget\\docs\\agent-runs\\20261118-160129-mvp-restart",
                           "relative_path":  "questions/I02-geometry-execution-question-04.md",
                           "sha256":  "B980D2002CB0BE7EE55F322C770EBCA49896B82F5B2F85BF505E316CFD4F9092"
                       },
                       {
                           "base":  "D:\\Projects\\GithubIssueWidget\\docs\\agent-runs\\20261118-160129-mvp-restart",
                           "relative_path":  "tasks/I02/evidence-geometry-01/observation-01.json",
                           "sha256":  "679FEAD536EB88B7540BA31930BCFDE31556A59816991974681E00AA764F8621"
                       },
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
                           "relative_path":  "tasks/I02/screenshot-observation-review-01.md",
                           "sha256":  "B8851AFC77E21D6B48A6007BF15BBFF5C74CBFADF0834445AAEAD13317ECCB5F"
                       },
                       {
                           "base":  "D:\\Projects\\GithubIssueWidget\\docs\\agent-runs\\20261118-160129-mvp-restart",
                           "relative_path":  "process/evidence-publication-protocol-01.md",
                           "sha256":  "65E36A266503EB12E20B4045BC7D2A346C8F72B97BA3ED3EF73A3B80B36B3382"
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

- 事実: 同一 CSS viewport の1サンプルの手動転記で marker top 16、上側 resize bottom 5、最小余白11 CSS px と報告。求めた12 CSS px を満たさないため入力は未実施。手動転記・一時点・対象画像未保存であり、実際の画面座標・遮蔽・DPI・ユーザー操作の非競合は未証明。
- 独立して計画・対象版をレビューした後、専用の新たな未使用 scratch に対象ファイルを通常バイトコピーし、元版と旧診断の全対象をハッシュ対応付ける。新 scratch の親・正規化先・再解析ポイント・hardlink・所有・容量・元/旧 run や生成物との非重複を確認。現に動いている旧 geometry app / server と scratch は所有者が管理し、当該計画では停止・変更・再起動・削除しない。
- 作者以外が確認可能な最小差分として、診断版の marker の配置だけを見直し、既報の上側 resize 帯から必要な余白を確保できる候補を示す。例: 現サンプルの top を20 CSS px にすると上側の差は15 CSS px となるが、これは提案値であり viewport・他 button / 帯・header の全境界、重なり、DPI、hit target、responsive layout を保証しない。実体に合う配置を設計し、元製品の配置・drag 機能・他者成果物は変更しない。
- 診断版の変更・元と旧版との差分、収集対象が非機密に限られること、対象全ファイルと診断差分の SHA、版・所有・影響、12px 判定の全対象と未確認を新しい確定結果へ記録。新たな type/build・起動・画面観測・UI 入力は本計画の承認だけでは行わない。別の独立レビューで変更版・安全条件・実施方法を確認し、親の判断を得る。
- 実際の画面・AX・対象 PID / exe / window、座標系・DPI・全 button / resize 帯・12 CSS px 以上の余白・非遮蔽と記録取得が客観的に確認できない場合はクリックしない。旧画像と AX の不一致を解消済みとみなさず、失敗済み screenshot / browser inventory や元版ドラッグを同じ方法で繰り返さない。診断版の単一入力は製品版移動や過去の因果の証明ではない。
- 共有 run 直接確定の手順逸脱は2件で独立調査済み。両記録を保持し、提案中の安全な公開方法は独立レビュー・親判断まで新規実行の根拠にしない。同種3件目で全作業を中止する。その他も同種2回で該当操作中断・別担当の客観調査、3回で全停止。新旧 scratch・所有プロセス・ログは削除しない。