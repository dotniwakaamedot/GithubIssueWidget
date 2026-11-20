# I02 旧 geometry 終了と marker 診断版の実行依頼 01

構造化フィールドを依頼の正本とする。担当・レビュアー・旧新 scratch、過去の handle / PID、port、出力、ユーザー指示、失敗履歴と閾値、固定参照・SHA は同一データから生成した。過去の handle / PID は現在の同一性を保証しない。開始直前に担当・参照版・所有・対象・競合を再照合し、不明なら停止する。照合は排他ロックではない。
```json
{
    "run_id":  "20261118-160129-mvp-restart",
    "task_id":  "I02-marker-execution",
    "owner":  "/root/drag_design",
    "reviewer":  "/root/plan_review",
    "old_scratch":  "D:\\Projects\\GithubIssueWidget-geometry-diagnostic-20261118-160129",
    "marker_scratch":  "D:\\Projects\\GithubIssueWidget-marker-diagnostic-20261118-160129",
    "old_dev_session":  68650,
    "old_app_session":  4498,
    "old_app_pid":  9796,
    "port":  1435,
    "evidence_relative_path":  "tasks/I02/evidence-marker-execution-01",
    "output_relative_path":  "tasks/I02/marker-execution-result-01.md",
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
                           "relative_path":  "tasks/I02/marker-adjustment-result-01.md",
                           "sha256":  "BBCC775114D265FD76EEDEF09335B7EAC2180EFC557FEE4B6EF601F8D7C18F58"
                       },
                       {
                           "base":  "D:\\Projects\\GithubIssueWidget\\docs\\agent-runs\\20261118-160129-mvp-restart",
                           "relative_path":  "tasks/I02/marker-adjustment-review-01.md",
                           "sha256":  "9A5A522FDF3B5F2A6E13AFE7EC3B1D26A9C1157400F13EC9C622E340DA999104"
                       },
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
                           "relative_path":  "tasks/I02/geometry-instrumentation-review-01.md",
                           "sha256":  "5EAC109FC6AE4FE665EE83982385E9EA01FC258FCBE438D415ABB2E4469F9B68"
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
                           "relative_path":  "process/reference-check-design-02.md",
                           "sha256":  "3CDCB637F739F9A9DBE240D0901557BC2BB23732ABF7CCC528F98A514C06C06E"
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
                       }
                   ]
}
```

親は独立レビュー済み marker 診断版について、実施前条件を確認し、所有者が旧 geometry app / server のみを識別して停止、terminal / port を確認した後、新版の依存準備・build・起動・観測を進めると判断した。これは実画面・余白・製品版ドラッグの成功や Issue 完了を証明しない。

旧版の操作は所有者本人のみ。各 session handle を poll し現在の live / terminal / 不明を区別し、PID・exe path・版・実行コマンド・開始時刻・port・所有を再照合。対象不明・再利用・別 process や他者の process への影響が不明なら止めて親に相談する。対象と安全な影響範囲を確認できる場合、旧 app、次に旧 server だけを穏当に終了し、それぞれの handle の terminal・終了コードと port の状態を記録。timeout や終了不明を成功扱いせず、盲目的に繰り返さない。通常終了ができず明示的な追加判断が必要なら保留し、子孫の一括停止、他アプリへの信号や推測 UI 操作を行わない。旧版の scratch・ログ・証拠・元製品は保持し、削除しない。

旧版 terminal と対象 port / process / desktop の非競合を確認後、marker scratch の対象18件を固定参照・manifest と独立レビュー版に照合し、元版との差分・SHA を記録。旧版の process / window と混同しない。scratch・parent・正規化先・リンク・容量・所有、元 worktree・B03・旧版資産との分離を確認。専用 target / cache / temp / profile / evidence を使い、元生成物・既存の秘密や資格情報を読み出し・コピーしない。依存、public registry、lifecycle scripts と通常の OS 副作用を確認し、重要な保存先・権限・秘密へのアクセスが不明なら該当操作を停止。

条件が揃えば新版の依存準備・型検査 / build・起動・対象表示を実施。コマンド・cwd・環境・UTC・個別 exit code、PID・exe path / SHA・window ID・port・対象版、ログと副作用を記録。失敗や結果不明を成功扱いしない。対象以外の process は触れず、所有 handle を保持。getState / browser inventory は2回失敗済みで再使用せず同種3回で全停止。以前の同一 screenshot 観測も反復しない。

入力前に対象 main 窓・PID / exe SHA / version を一意に照合し、同窓から空の event output と geometry output の完全な読取り・保存可能性を確認。各 marker / header / button / resize の有限・正寸法、同一時点の CSS 座標、DPI・client・screen・操作 API 座標の対応、全境界の12 CSS px 以上の実余白、画像と AX の対象一致・非遮蔽、ユーザー操作の非競合を確認。どれか不足・不明・対象喪失なら**入力せず停止**。同じ失敗を繰り返す画面取得は禁止であり、必要な画像対応が得られない場合は未実施として根拠を記録し管理に報告する。上記がすべて確認でき、依頼と親の判断に一致する場合だけ指定対象への単一押下・解放1回を行い、失敗・不達・不明時に繰り返さない。ドラッグ・移動・リサイズ、他の button 操作は禁止。診断成功を元製品の移動や Issue 完了としない。

証拠・結果は構造化フィールドの専用領域に限定し、既存の直接確定逸脱2件を保持。親ディレクトリと確定先・tmp 不在を確認し、同ディレクトリの新規一時ファイルへの write / flush / close、UTF-8 と形式の検査、非上書き rename、内容・SHA 読戻しを行う。直接確定をしない。同種の直接公開3件目で全作業中止。結果には旧 process の所有・終了、対象版、操作、各 exit、観測値・画像 / AX の可否、実施・未実施、証拠 SHA、副作用・未確認を記録。元製品、Git、GitHub の変更、push / PR / merge、既存資産の削除は禁止。

同種問題は初回を含む2回で該当操作を中断し別担当が客観調査、3回で全停止。構造化フィールドの失敗履歴を維持し、発生時は種別・回数・停止理由を親へ通知する。結果パスのみ通知。