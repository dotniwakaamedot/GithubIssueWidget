# I02 診断窓の画像対応・観測方法の独立レビュー依頼 01

構造化フィールドを依頼の正本とする。担当と作者、process 所有者、出力、ユーザー原文・追加指示、失敗履歴と閾値、固定参照・SHA は同一データから生成。実担当・参照版・出力先・競合は起動直前に再照合する。照合は排他ロックではなく、変更や所有不明なら停止する。
```json
{
    "run_id":  "20261118-160129-mvp-restart",
    "task_id":  "I02-screenshot-observation-review",
    "reviewer":  "/root/plan_review",
    "author":  "/root/screenshot_design",
    "process_owner":  "/root/drag_design",
    "output_relative_path":  "tasks/I02/screenshot-observation-review-01.md",
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
                    "web_search_js_syntax_error":  1,
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
                           "relative_path":  "tasks/I02/screenshot-observation-design-request-01.md",
                           "sha256":  "175BFB142F627939CCAAAB74BDA0A5B14518368638E0A4C21E326EAC0F6F78C5"
                       },
                       {
                           "base":  "D:\\Projects\\GithubIssueWidget\\docs\\agent-runs\\20261118-160129-mvp-restart",
                           "relative_path":  "tasks/I02/screenshot-observation-design-result-01.md",
                           "sha256":  "4D33B22968BCF4FBB6930C095B56480A87C47E1F2A55406AD9D97485E0539F79"
                       },
                       {
                           "base":  "D:\\Projects\\GithubIssueWidget\\docs\\agent-runs\\20261118-160129-mvp-restart",
                           "relative_path":  "questions/I02-drag-click-question-08.md",
                           "sha256":  "E45AD6C76F7AB0C6DFA2FFD404E39594E54222033D0D17B36245D741362E3BA8"
                       },
                       {
                           "base":  "D:\\Projects\\GithubIssueWidget\\docs\\agent-runs\\20261118-160129-mvp-restart",
                           "relative_path":  "questions/I02-drag-click-question-09.md",
                           "sha256":  "6C5E76248C84C943EA1858D60FA2C918F745E759C17A4FEB03830BCB0CF3EA9C"
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
                           "relative_path":  "process/reference-check-design-02.md",
                           "sha256":  "3CDCB637F739F9A9DBE240D0901557BC2BB23732ABF7CCC528F98A514C06C06E"
                       }
                   ]
}
```

対象設計は、個別窓の AX と画像が一致しない1回の観測を受け、対象窓の画像・座標を安全に確認する代替方法の案である。元画像は未保存で原因は未確定。確定した対象・過去の handle / ID / PID / exe の時点を照合し、現在の値として推測しない。元のドラッグは2回不成功で未達、診断版の入力も未実施。

候補の API がこの環境の実際に利用可能なツール・適用 skill に対応し、個別窓に到達できる具体的入口を示すか、別プロバイダ間の ID を混同しないか、対象画像・origin・DPI・AX・window/client rect の対応を実用的に検証できるかを重点確認する。対象の一意性、他ウィンドウ・個人情報への影響、画像の保存・扱い、ユーザーの実際の操作との競合、客観的な余白・境界、非対話領域、記録の SHA・時刻・限界、失敗時停止を評価。必要な安全条件を維持し、根拠のない不要な追加条件で実行可能な検証を妨げない。資料や操作上の未確認を成功として扱わない。

生存中の2 session は元の `/root/drag_design` が所有し、別担当は handle の継承を仮定せず停止・再起動しない。今回の担当は文書・コードの静的読取と必要な公式一次資料の調査のみ。実際の画面取得・click・drag・UI / OS 操作・アプリ起動・build・通信試験、製品・scratch・Git・GitHub の変更は禁止。getState / browser inventory と失敗した web 検索スクリプトは再実行しない。browser inventory 失敗は既往2回、同じ失敗3回目で全停止。画像不一致は1回で、同種2回で該当観測を止め別担当が客観調査、3回で全停止。構造化フィールドの他の失敗履歴も保持する。

共有 run の指定結果と新規質問のみ所有。親ディレクトリと確定先・tmp 不在を確認し UTF-8 往復後に確定。対象設計・参照の SHA、候補ごとの具体的入口・成否の確認方法・限界、権限・安全条件・外部副作用、実施可能性と実施順、blocking / non-blocking 指摘・根拠・是正条件、cwd・コマンドと終了コードを記録。レビューはその版に限定し、実際の capture・入力・製品完成を自動許可しない。結果パスのみ通知する。