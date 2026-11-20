# I02 専用診断 marker の余白調整依頼 01

構造化フィールドを依頼の正本とする。担当・レビュー担当、旧新 scratch、対象、出力、ユーザー指示、失敗履歴と閾値、固定参照と SHA は同一データから生成。起動直前に実担当・参照版・所有者・出力先・競合を再照合し、変化や所有不明なら停止する。照合は排他ロックではない。
```json
{
    "run_id":  "20261118-160129-mvp-restart",
    "task_id":  "I02-marker-adjustment",
    "owner":  "/root/drag_design",
    "reviewer":  "/root/plan_review",
    "source_scratch":  "D:\\Projects\\GithubIssueWidget-geometry-diagnostic-20261118-160129",
    "new_scratch":  "D:\\Projects\\GithubIssueWidget-marker-diagnostic-20261118-160129",
    "output_relative_path":  "tasks/I02/marker-adjustment-result-01.md",
    "prior_sample_gap_css_px":  11,
    "required_gap_css_px":  12,
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
                           "relative_path":  "tasks/I02/marker-adjustment-plan-01.md",
                           "sha256":  "154C238B447F23BA31B4B1855D5FE1432AC8741B25AEB39B58F8C40FE5890BEC"
                       },
                       {
                           "base":  "D:\\Projects\\GithubIssueWidget\\docs\\agent-runs\\20261118-160129-mvp-restart",
                           "relative_path":  "process/evidence-publication-protocol-01.md",
                           "sha256":  "65E36A266503EB12E20B4045BC7D2A346C8F72B97BA3ED3EF73A3B80B36B3382"
                       },
                       {
                           "base":  "D:\\Projects\\GithubIssueWidget\\docs\\agent-runs\\20261118-160129-mvp-restart",
                           "relative_path":  "process/publication-marker-decision-01.md",
                           "sha256":  "8397B4C7F0797BE0FE09AC5069FECDC42CF97C7145504D219CDCED6AFDEEECC3"
                       },
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
                       }
                   ]
}
```

固定された marker 調整計画と安全な公開方式は、親が作者以外による独立確認の結果を受け採用した。旧 geometry 診断版の一時点の手動転記は最小余白11 CSS px で、要求12 CSS px を満たさない。これは対象画像や実画面・DPI の成功を証明しない。元版ドラッグは2回位置不変・原因不明で未達。

現在の旧画面・process・server は所有者が維持し、停止・変更・再起動しない。構造化フィールドの新 scratch は未使用の候補。親・正規化先・所有・reparse / hardlink・容量と元製品・既存 scratch / run / 他人の領域の非重複を確認してから通常バイトコピーを作成。固定参照と前版の対象一覧に基づいて必要な18ファイルだけをコピーし、元/コピー各ファイルの path・size・SHA を記録。既存 node_modules、cache、実行生成物や他者の証拠はコピーしない。対象不明・不一致・競合なら停止する。

変更は新 scratch の診断版 marker 配置に必要な最小範囲に限定し、元製品と旧診断版のコード・設定・結果を保護する。静的には上側 resize 帯との余白を改善できる候補を選び、全 button / resize 帯、header、viewport、responsive・DPI・hit target と他領域への影響は実測前提とする。例示値の採用自体を安全性の証明とみなさない。drag 無効化と非機密イベント・矩形収集条件を維持し、差分・前後 SHA・想定と未確認を明記。

この依頼ではコードの新規版準備と静的検討のみ許可。build / 依存導入 / 起動 / UI 表示 / 画像取得 / クリック / ドラッグ / リサイズ / 通信試験は禁止。新診断版のコード・構成・manifest・差分、動作前提・安全条件を別担当が独立レビューし親が判断するまで実行を開始しない。余白の根拠となる同一対象画面・AX・DPI・全境界・非遮蔽が不明なら入力しない。

所有範囲は新 scratch の必要な診断 marker 差分と指定結果・新規質問のみ。親ディレクトリ存在、確定先と tmp 不在を確認し、同ディレクトリの一時ファイルを新規作成、write / flush / close 成功後、UTF-8 と JSON の構文・構造・件数・内容を適切に検査してから既存物を置換しない rename で公開。確定後に内容・実体 SHA を再照合。誤って確定名へ直接書かない。失敗時は後続操作を止め、現存物を保護し報告する。直接公開は既往2件で客観調査済み、同種3件目で全停止。

結果には対象版・コピー元と新 scratch の全対象 SHA、変更点・静的根拠・残る安全ゲート、依存・出力・リスク、cwd・コマンド・終了コード、未実施を記録し、指定パスだけ通知。共有 run / 他者の証拠・既存の process・製品・Git・GitHub の変更、push / PR / merge / 削除は禁止。同種問題2回で該当操作を中断し別担当が客観調査、3回で全停止。過去の画像・browser inventory・ドラッグ・文字破損経路を再試行しない。