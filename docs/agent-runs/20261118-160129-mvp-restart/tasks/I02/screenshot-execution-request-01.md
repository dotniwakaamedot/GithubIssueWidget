# I02 診断窓の画像・OS 座標の安全な確認依頼 01

構造化フィールドを依頼の正本とする。担当・process 所有者・レビュー担当、過去の窓 ID / PID、対象 scratch・port、出力、ユーザー指示、失敗履歴・閾値、固定参照・SHA は同一データから生成した。過去の ID / PID と handle の現存は保証されない。開始直前に担当・参照版・所有・対象・競合を再照合し、不明なら停止する。照合は排他ロックではない。
```json
{
    "run_id":  "20261118-160129-mvp-restart",
    "task_id":  "I02-screenshot-execution",
    "owner":  "/root/screenshot_design",
    "process_owner":  "/root/drag_design",
    "reviewer":  "/root/plan_review",
    "scratch":  "D:\\Projects\\GithubIssueWidget-marker-diagnostic-20261118-160129",
    "prior_window_id":  6559504,
    "prior_pid":  52960,
    "port":  1435,
    "evidence_relative_path":  "tasks/I02/evidence-screen-02",
    "output_relative_path":  "tasks/I02/screenshot-execution-result-01.md",
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
                           "relative_path":  "tasks/I02/marker-execution-result-01.md",
                           "sha256":  "0443CCB85EC2F1537C83AC1981775DEDA01EB8BD2A942BE81EF3ECFD2F405F71"
                       },
                       {
                           "base":  "D:\\Projects\\GithubIssueWidget\\docs\\agent-runs\\20261118-160129-mvp-restart",
                           "relative_path":  "tasks/I02/marker-adjustment-review-01.md",
                           "sha256":  "9A5A522FDF3B5F2A6E13AFE7EC3B1D26A9C1157400F13EC9C622E340DA999104"
                       },
                       {
                           "base":  "D:\\Projects\\GithubIssueWidget\\docs\\agent-runs\\20261118-160129-mvp-restart",
                           "relative_path":  "tasks/I02/screenshot-observation-design-result-01.md",
                           "sha256":  "4D33B22968BCF4FBB6930C095B56480A87C47E1F2A55406AD9D97485E0539F79"
                       },
                       {
                           "base":  "D:\\Projects\\GithubIssueWidget\\docs\\agent-runs\\20261118-160129-mvp-restart",
                           "relative_path":  "tasks/I02/screenshot-observation-review-01.md",
                           "sha256":  "B8851AFC77E21D6B48A6007BF15BBFF5C74CBFADF0834445AAEAD13317ECCB5F"
                       },
                       {
                           "base":  "D:\\Projects\\GithubIssueWidget\\docs\\agent-runs\\20261118-160129-mvp-restart",
                           "relative_path":  "questions/I02-drag-click-question-08.md",
                           "sha256":  "E45AD6C76F7AB0C6DFA2FFD404E39594E54222033D0D17B36245D741362E3BA8"
                       },
                       {
                           "base":  "D:\\Projects\\GithubIssueWidget\\docs\\agent-runs\\20261118-160129-mvp-restart",
                           "relative_path":  "tasks/I02/drag-click-request-01.md",
                           "sha256":  "83D63BA78D6CEEC7E07BDF9834583D0274BCADCEFC5DB5BCAB6B25377880CB6C"
                       },
                       {
                           "base":  "D:\\Projects\\GithubIssueWidget\\docs\\agent-runs\\20261118-160129-mvp-restart",
                           "relative_path":  "process/evidence-publication-protocol-01.md",
                           "sha256":  "65E36A266503EB12E20B4045BC7D2A346C8F72B97BA3ED3EF73A3B80B36B3382"
                       },
                       {
                           "base":  "D:\\Projects\\GithubIssueWidget\\docs\\agent-runs\\20261118-160129-mvp-restart",
                           "relative_path":  "process/reference-check-design-02.md",
                           "sha256":  "3CDCB637F739F9A9DBE240D0901557BC2BB23732ABF7CCC528F98A514C06C06E"
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

対象は稼働中として報告された marker 診断版。AX の1サンプルは空 event と最小15 CSS px を示したが、これは手動転記・一時点であり、実画像・OS 座標・対象との一致・非遮蔽・ユーザー操作との非競合は未確認。製品版の drag 成功、過去2回の原因解明、Issue 完了は未達。process handle は `/root/drag_design` の所有物であり、本担当は継承したと仮定せず、所有者に live / terminal を確認する。独自の停止・再起動や他 process の操作は禁止。

まず CUA の提供ドキュメント・適用 skill を読み、対象 PID / exe path / hash / window ID / port と所有者の状態を安全な読み取りで再照合する。前回の対象外画像の原因は未確定。全サーフェス `cua.getState()` / browser inventory は2回失敗しており再使用しない。前回失敗した画面取得を盲目的に反復せず、CUA 公式提供の個別窓を対象とする別の合理的な入口とその出力・副作用を先に検討。提供 API や返却された状態・文書で実際に利用可能と確認した操作だけを用い、提供されない操作を推測しない。target が消失・不一致・複数候補・アクセス不明なら停止する。

ユーザーが依頼した必要な UI 検証の範囲として、妥当な個別対象の読み取り・画像／OS 座標の観測を可能なら行う。実施時は対象窓・版・PID/exe・時刻を固定し、他アプリ・private data・秘密が写り込まないよう限定する。個別窓の選択や画像取得が user 操作と干渉し得ることは記録し、想定外の窓・明確なユーザー操作・保護情報・安全不明を見たら停止。画面ロック・ユーザー不在等を推測せず、観測できた範囲のみ記す。

画像の内容が対象 main WebView の marker・header・button / resize・空 event と一致するか、画面 origin・client / frame・DPI / coordinate と AX / DOM の対応を客観的に確認。画像 ID や窓名だけで一致・非遮蔽を証明せず、観測の時刻差・外枠・仮想化・複数 display と遮蔽の限界を明記。必要な正確な画像・座標が得られない場合は代替を推測せず未確認のまま止める。今回画像不一致1回があり、同種2回目は当該観測を中断し別担当の客観調査、3回で全中止。browser inventory と同一の error は既往2回、同種3回目で全停止。問題発生後に別 API を連続試行して閾値を回避しない。

**クリック・押下・ドラッグ・移動・リサイズ等の入力は未承認。** process の終了・再起動、既存アプリ・設定・scratch・証拠・Git / GitHub の変更は禁止。アプリが終了している場合は再起動せず結果を未確認として親へ報告。観測で所有窓以外の内容を保存しない。CUA ツールの初回呼出し・completion_summary 等、提供文書と適用 skill の作業終了規則を順守する。

指定の新規 evidence と結果、新規質問のみ所有。親ディレクトリと確定先・tmp 不在を確認し、同一ディレクトリに新規一時作成、write / flush / close、UTF-8 と構造を検査して既存物を置換しない rename で確定。直接確定逸脱は既往2件で、同種3件目は全停止。対象版・参照 SHA、使用 API と具体的入口、選択した理由・代替との違い、対象識別・OS 座標と画像の一致可否、時刻、証拠の範囲と SHA、外部副作用・不明・失敗と停止判断、cwd・コマンド・終了コードを記録。入力・製品完了を主張せず、結果パスのみ通知する。