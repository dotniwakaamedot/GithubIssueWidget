# I02 B03 結果・診断クリック依頼の独立確認 01

構造化フィールドを依頼の正本とする。担当・各作者、出力、ユーザー原文・追加指示、失敗履歴と閾値、固定参照・SHA は同一データから生成。実担当・参照版・出力先・競合を起動直前に再照合する。照合は排他ロックではない。
```json
{
    "run_id":  "20261118-160129-mvp-restart",
    "task_id":  "I02-build-drag-preflight-review",
    "reviewer":  "/root/plan_review",
    "build_author":  "/root/build_verification",
    "request_author":  "/root/restart_planner",
    "drag_owner":  "/root/drag_design",
    "output_relative_path":  "tasks/I02/build-drag-preflight-review-01.md",
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
                    "dispatch_before_dependency_gate_confirmed":  1
                },
    "references":  [
                       {
                           "base":  "D:\\Projects\\GithubIssueWidget\\docs\\agent-runs\\20261118-160129-mvp-restart",
                           "relative_path":  "tasks/I02/build-verification-request-01.md",
                           "sha256":  "2EA23BAEAE1CA017EBDCA8296D3D720FCDCE00E79A20B58C4152915BD5F00711"
                       },
                       {
                           "base":  "D:\\Projects\\GithubIssueWidget\\docs\\agent-runs\\20261118-160129-mvp-restart",
                           "relative_path":  "tasks/I02/build-verification-result-01.md",
                           "sha256":  "7A8AF3B60D32D2AC94888801067C73031DB8A84974DA95E6373D03C03E68A098"
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
                           "relative_path":  "tasks/I02/drag-investigation-review-01.md",
                           "sha256":  "D9FABAE1C51E0B46864477933214102A53876DCBFC9AB54CF37644A98F9024A3"
                       },
                       {
                           "base":  "D:\\Projects\\GithubIssueWidget\\docs\\agent-runs\\20261118-160129-mvp-restart",
                           "relative_path":  "tasks/I02/result-02.md",
                           "sha256":  "520044FE6138609B99595C5FAF1D3973DD2D62EB82F27FE90392568C9599ED13"
                       },
                       {
                           "base":  "D:\\Projects\\GithubIssueWidget\\docs\\agent-runs\\20261118-160129-mvp-restart",
                           "relative_path":  "tasks/I02/combined-review-01.md",
                           "sha256":  "F6DF723A067C988F144087B6A4833EDD306AAB56071068894424D359F70004D3"
                       },
                       {
                           "base":  "D:\\Projects\\GithubIssueWidget\\docs\\agent-runs\\20261118-160129-mvp-restart",
                           "relative_path":  "questions/I02-build-question-10.md",
                           "sha256":  "C2B20065574387F6DA4C52EDB518913CBC72EC29E9EE659F8C07C2BBC7ECEB4F"
                       },
                       {
                           "base":  "D:\\Projects\\GithubIssueWidget\\docs\\agent-runs\\20261118-160129-mvp-restart",
                           "relative_path":  "process/reference-check-design-02.md",
                           "sha256":  "3CDCB637F739F9A9DBE240D0901557BC2BB23732ABF7CCC528F98A514C06C06E"
                       }
                   ]
}
```

独立して2対象を確認する。第一に、B03 作業結果について前版の製品17件と専用コピーの対象版、作業記録・各証拠の基点と実体 SHA、親判断による所有セッションの中断、ログ・出力バイナリ・終了コードの有無を照合する。ログ上の完了と npm/Cargo 自体の終了コード未取得は区別し、dev 未実施と併せて B03 を過大評価しない。結果に記載された実行・所有・副作用・未検証が証拠で裏付けられる範囲を判定し、実際の build を再実行しない。

第二に、ドラッグ無効化の診断コピーでの単一クリック依頼について、元の Issue・対象版・版変更と対象資産保護、専用 scratch / port / cache / profile・既存 B03 との非競合、UI 対象・許される入力・非機密ログ・副作用と停止条件を評価する。実際のコードは未作成であり、作成後の診断版ハッシュと `startDragging` 無効化を独立レビューし、親判断までクリックしないことを確認。原因不明の元のドラッグは再試行しない。単一クリックで元の移動要件や過去の入力到達を実証したとみなさない。

B03 の所有セッションは対象結果で terminal と報告されている。実際の確定記録・利用可能なプロセス／画面・port 状態から、診断 UI が B03 と干渉しない前提の確認手順を検討する。観測できない・競合・所有不明なら未確認とし、セッションを再起動・停止・削除しない。レビュー依頼と親確認より前に作業者を起動した運用上の不備は履歴に1件として記録し、次回は依存ゲートの完了確認後に起動する。

実機操作・クリック・ドラッグ・アプリ起動・build・通信を伴う試験、製品・既存証拠・Git・GitHub の変更は禁止。静的な読み取りと必要な公式一次資料の調査に限定。パスを推測せず基点・相対パス・注記を分け、全件の存在確認後に SHA を照合する。構造化フィールドの履歴を保持し、同種2回で該当操作中断・別担当が客観調査、3回で全作業を中止。過去のドラッグ・文字破損・lockfile 失敗経路を繰り返さない。

共有 run の指定結果と新規質問だけを所有。親ディレクトリ・確定先・tmp 不在を確認し UTF-8 往復して確定。両対象の版・実測 SHA・作者・根拠・未確認、要件ごとの判定、blocking / non-blocking 指摘と是正条件、cwd・コマンド・終了コード、対象版限定の判断を記載。設計の承認は診断入力・製品完成の承認ではない。確定パスだけを通知する。