# I02 診断窓の画像対応と安全領域の観測方法設計 01

構造化フィールドを依頼の正本とする。担当・process 所有者、過去に観測した window ID、出力、ユーザー原文・追加指示、失敗履歴と閾値、固定参照・SHA は同一データから生成。ID の現存や対象一致は過去の観測であり、自動的に現在の対象とみなさない。担当・参照版・出力先・競合は開始直前に再照合し、変更や所有不明なら停止する。
```json
{
    "run_id":  "20261118-160129-mvp-restart",
    "task_id":  "I02-screenshot-observation-design",
    "owner":  "/root/screenshot_design",
    "process_owner":  "/root/drag_design",
    "window_id":  267184,
    "output_relative_path":  "tasks/I02/screenshot-observation-design-result-01.md",
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
                           "relative_path":  "questions/I02-drag-click-question-07.md",
                           "sha256":  "C82C3B3361FB9C1BB2517CEFD14E73B9A23575E23E3E6B40CFA8DF8E9BB7C2B3"
                       },
                       {
                           "base":  "D:\\Projects\\GithubIssueWidget\\docs\\agent-runs\\20261118-160129-mvp-restart",
                           "relative_path":  "questions/I02-drag-click-question-08.md",
                           "sha256":  "E45AD6C76F7AB0C6DFA2FFD404E39594E54222033D0D17B36245D741362E3BA8"
                       },
                       {
                           "base":  "D:\\Projects\\GithubIssueWidget\\docs\\agent-runs\\20261118-160129-mvp-restart",
                           "relative_path":  "questions/I02-drag-click-answer-08.md",
                           "sha256":  "48F367DB57BE93F852E7C0950757BA020BD74BA145D867AD791A9D258DC06716"
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
                           "relative_path":  "tasks/I02/drag-investigation-design-result-01.md",
                           "sha256":  "FA635B310797FBCB5C50F6B0F5967E87B0B3131CDB33F8F899332EF1E54A7A08"
                       },
                       {
                           "base":  "D:\\Projects\\GithubIssueWidget\\docs\\agent-runs\\20261118-160129-mvp-restart",
                           "relative_path":  "tasks/I02/build-verification-result-01.md",
                           "sha256":  "7A8AF3B60D32D2AC94888801067C73031DB8A84974DA95E6373D03C03E68A098"
                       },
                       {
                           "base":  "D:\\Projects\\GithubIssueWidget\\docs\\agent-runs\\20261118-160129-mvp-restart",
                           "relative_path":  "process/reference-check-design-02.md",
                           "sha256":  "3CDCB637F739F9A9DBE240D0901557BC2BB23732ABF7CCC528F98A514C06C06E"
                       }
                   ]
}
```

診断窓の accessibility tree と capture 画像が一致しないという前回1件の観測を扱う。画像は保存されず、遮蔽・描画・capture・desktop 等は未確定の仮説。既存資料と必要な CUA / Windows の一次ドキュメントを読み、個別 window ID・PID・exe SHA・時刻・origin / DPI・座標系・ターゲット識別をどのように対応付けるか、対象窓の画像に一致することを保証できる別手法の可否と限界を設計する。期待した window が写らない事象を説明できない場合は未確認のままとし、原因を推測で確定しない。

候補ごとに読み取りか副作用のある処理か、対象取り違え・他ウィンドウ／個人情報の写込み・画面操作への干渉・DPI・スケール・座標変換、証拠の保存範囲と SHA、失敗・不一致・対象消失時の停止、所有・独立レビューと親判断を定める。診断版の空 output、marker / header / 全 button / resize 帯の実境界・12 CSS px の安全余白を Issue の要求と結び付け、正確な対象窓・対象版と安全領域を確認できないとき入力しない。元のドラッグ成功や過去の挙動は診断版の観測のみで証明しない。

本担当は文書ベースの設計のみ。CUA や OS の新たな画面取得・UI 操作・クリック・ドラッグ・移動・process の poll / 停止・再起動、アプリ起動・build・通信試験、製品・scratch・既存証拠・Git・GitHub の変更は禁止。process handle 22902 と 17031 の再 poll は所有者だけが別回答に基づき実施する。API を推測して呼ばず、資料の場所・存在と版を確かめてから読む。通常の副作用・観測できない事項を明記する。

前回の画像不一致は1回、同種2回目で該当観測を中止して別担当の客観調査、3回目で全停止。browser inventory エラー2回は独立調査済みで、同じ getState / browser inventory は再実行せず同種3回目は全停止。構造化フィールドの他の失敗履歴も保持する。診断入力や別手法の実行は、本設計の独立レビューと親判断、対象版・所有・安全条件の再照合まで禁止。

所有範囲は指定の共有 run 結果と新規質問のみ。親ディレクトリと確定先・tmp 不在を確認し UTF-8 往復後に確定する。候補ごとの一次資料・対象版・判断根拠・未確認、実行手順の安全性と停止条件、証拠の紐付け、cwd・コマンドと終了コードを記載。結果パスだけを通知する。