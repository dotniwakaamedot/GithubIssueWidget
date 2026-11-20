# I02 専用診断 marker 調整の独立レビュー依頼 01

構造化フィールドを依頼の正本とする。担当と作者、旧新 scratch・対象件数、出力、ユーザー指示、失敗履歴と閾値、固定参照・SHA は同一データから生成。開始直前に実担当・参照版・所有者・出力先・競合を再照合し、変更や所有不明なら停止する。照合は排他ロックではない。
```json
{
    "run_id":  "20261118-160129-mvp-restart",
    "task_id":  "I02-marker-adjustment-review",
    "reviewer":  "/root/plan_review",
    "author":  "/root/drag_design",
    "source_scratch":  "D:\\Projects\\GithubIssueWidget-geometry-diagnostic-20261118-160129",
    "new_scratch":  "D:\\Projects\\GithubIssueWidget-marker-diagnostic-20261118-160129",
    "prior_diagnostic_files":  18,
    "expected_unchanged":  17,
    "expected_changed":  1,
    "observed_gap_css_px":  11,
    "required_gap_css_px":  12,
    "output_relative_path":  "tasks/I02/marker-adjustment-review-01.md",
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
                    "direct_final_publication_violations":  2,
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
                           "relative_path":  "tasks/I02/marker-adjustment-request-01.md",
                           "sha256":  "2127961B3C0AAF363BAF43261D16944B3F3099592BF5D4E93C207FE1F3AC6007"
                       },
                       {
                           "base":  "D:\\Projects\\GithubIssueWidget\\docs\\agent-runs\\20261118-160129-mvp-restart",
                           "relative_path":  "tasks/I02/marker-adjustment-result-01.md",
                           "sha256":  "BBCC775114D265FD76EEDEF09335B7EAC2180EFC557FEE4B6EF601F8D7C18F58"
                       },
                       {
                           "base":  "D:\\Projects\\GithubIssueWidget\\docs\\agent-runs\\20261118-160129-mvp-restart",
                           "relative_path":  "tasks/I02/marker-adjustment-plan-01.md",
                           "sha256":  "154C238B447F23BA31B4B1855D5FE1432AC8741B25AEB39B58F8C40FE5890BEC"
                       },
                       {
                           "base":  "D:\\Projects\\GithubIssueWidget\\docs\\agent-runs\\20261118-160129-mvp-restart",
                           "relative_path":  "process/publication-marker-decision-01.md",
                           "sha256":  "8397B4C7F0797BE0FE09AC5069FECDC42CF97C7145504D219CDCED6AFDEEECC3"
                       },
                       {
                           "base":  "D:\\Projects\\GithubIssueWidget\\docs\\agent-runs\\20261118-160129-mvp-restart",
                           "relative_path":  "process/evidence-publication-protocol-01.md",
                           "sha256":  "65E36A266503EB12E20B4045BC7D2A346C8F72B97BA3ED3EF73A3B80B36B3382"
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
                           "relative_path":  "tasks/I02/evidence-geometry-01/observation-01.json",
                           "sha256":  "679FEAD536EB88B7540BA31930BCFDE31556A59816991974681E00AA764F8621"
                       },
                       {
                           "base":  "D:\\Projects\\GithubIssueWidget\\docs\\agent-runs\\20261118-160129-mvp-restart",
                           "relative_path":  "tasks/I02/result-02.md",
                           "sha256":  "520044FE6138609B99595C5FAF1D3973DD2D62EB82F27FE90392568C9599ED13"
                       },
                       {
                           "base":  "D:\\Projects\\GithubIssueWidget-marker-diagnostic-20261118-160129",
                           "relative_path":  "copy-manifest.json",
                           "sha256":  "CA678FB46C1CF89C34C44C5EF7A22E900E391AA13D72E961430AE32CE2FA916D"
                       },
                       {
                           "base":  "D:\\Projects\\GithubIssueWidget-marker-diagnostic-20261118-160129",
                           "relative_path":  "widget/src/main.tsx",
                           "sha256":  "DE50084BEEA52147D7B3482AC84ADEFE70AA2C5E8F211BDF1930A8AED7C1D1DD"
                       },
                       {
                           "base":  "D:\\Projects\\GithubIssueWidget-marker-diagnostic-20261118-160129",
                           "relative_path":  "widget/src/diagnostic.ts",
                           "sha256":  "840D7F69349AEED52C0A5D6FA6319317CCE432390A74463F8E824069F26E412D"
                       },
                       {
                           "base":  "D:\\Projects\\GithubIssueWidget-marker-diagnostic-20261118-160129",
                           "relative_path":  "widget/vite.config.ts",
                           "sha256":  "194D17A739D0F626DDAF94B5D2B6F9335DBF50BB571AB4F7739DA727ED4F772A"
                       },
                       {
                           "base":  "D:\\Projects\\GithubIssueWidget-marker-diagnostic-20261118-160129",
                           "relative_path":  "widget/src-tauri/tauri.conf.json",
                           "sha256":  "EDE0EA3E1EE5C6FE067A5E127EEAAE02A19110E8CD8E057C8B7FCB2B826A55D5"
                       },
                       {
                           "base":  "D:\\Projects\\GithubIssueWidget-geometry-diagnostic-20261118-160129",
                           "relative_path":  "widget/src/main.tsx",
                           "sha256":  "820F164EF8947DA6B87F6ACE8B74D787E09ED9AF7A6610E2C07678DFA2C4AE5C"
                       }
                   ]
}
```

対象は前回1サンプルで最小余白11 CSS pxだった marker に対し、診断コピーの位置だけを調整した結果。実体の旧新18ファイルをそれぞれ正しい基点で全件存在確認後に hash を照合し、17件不変・main.tsx 1件だけが変更され、新旧元の manifest と固定版・作者報告が一致するか確認。コピーや元 worktree、他者の成果物、稼働中の旧診断 app / server に変更を加えない。

静的差分が marker の top 16 → 20 CSS px のみか、通常 drag が無効のままか、非機密の event / geometry 出力と12 CSS px 判定、残る button / resize / capability に想定外の変化がないかを確認。旧サンプルの算術と、viewport / responsive / DPI / 画面・AX 対応 / target / 遮蔽が変われば余白・安全性は未証明であることを明示。必要な実画面・全矩形・対象 HWND/PID/exe・非競合・空出力の実測が未実施なら、クリックや製品版移動の成功としない。

前回の直接確定2件は客観調査済みで、同種3件目は全中止。指定の公開方式に従って結果を確定し、元の逸脱ファイルは保護する。画像不一致1回・browser inventory failure2回・元ドラッグ2回の履歴を保持し、同種2回は該当操作中断と別担当の客観調査、3回で全停止。失敗した capture / getState / browser inventory やドラッグを再試行しない。

実際の build・依存導入・アプリ起動・UI 表示・画面取得・クリック・ドラッグ・通信試験、元/旧新 scratch・process・既存証跡・Git・GitHub の変更は禁止。静的な読み取り・必要な公式一次資料だけを利用。対象版の変更後は旧承認を流用しない。

所有範囲は共有 run の指定結果と自担当の新規質問のみ。親ディレクトリと確定先・tmp 不在を確認し、一時ファイル新規作成・write / flush / close・UTF-8 と内容を照合後、既存物を置換しない rename により確定。版・実測 SHA・旧新差分・各要求・前提・未確認・安全ゲート・通常副作用、blocking / non-blocking 指摘と是正条件、cwd・コマンド・終了コード、版限定の判定を記録する。承認は実際の起動・画面取得・入力・製品完成を自動許可しない。結果パスのみ通知する。