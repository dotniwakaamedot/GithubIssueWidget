# I02 ドラッグ失敗の追加切り分け方法 設計依頼 01

構造化フィールドを依頼の正本とする。担当、対象結果の作者、所有、ユーザー原文・追加指示、失敗履歴と閾値、固定参照と SHA は同じデータから生成した。実担当・参照版・出力先・競合は開始直前に再照合し、変更や所有不明なら停止する。照合は排他ロックではない。
```json
{
    "run_id":  "20261118-160129-mvp-restart",
    "task_id":  "I02-drag-investigation-design",
    "owner":  "/root/drag_design",
    "source_author":  "/root/drag_diagnosis",
    "output_relative_path":  "tasks/I02/drag-investigation-design-result-01.md",
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
                    "recent_output_garbled":  1
                },
    "references":  [
                       {
                           "base":  "D:\\Projects\\GithubIssueWidget\\docs\\agent-runs\\20261118-160129-mvp-restart",
                           "relative_path":  "tasks/I02/drag-result-01.md",
                           "sha256":  "B34245F943AD4B9E69F5DBC8020C762499055A2ABDE878F57446131038C5CCB6"
                       },
                       {
                           "base":  "D:\\Projects\\GithubIssueWidget\\docs\\agent-runs\\20261118-160129-mvp-restart",
                           "relative_path":  "tasks/I02/combined-review-01.md",
                           "sha256":  "F6DF723A067C988F144087B6A4833EDD306AAB56071068894424D359F70004D3"
                       },
                       {
                           "base":  "D:\\Projects\\GithubIssueWidget\\docs\\agent-runs\\20261118-160129-mvp-restart",
                           "relative_path":  "tasks/I02/drag-plan-01.md",
                           "sha256":  "C6AC531B2AB8FB2BAAC401BFBEE09135356E7CA2E411A09D4C27198611A1B70A"
                       },
                       {
                           "base":  "D:\\Projects\\GithubIssueWidget\\docs\\agent-runs\\20261118-160129-mvp-restart",
                           "relative_path":  "tasks/I02/drag-plan-review-01.md",
                           "sha256":  "A1577D6DC6F51CE788C1F68DC97FDEB3FD2D75EC7DB7F4D2065A36E3DF3E7FA3"
                       },
                       {
                           "base":  "D:\\Projects\\GithubIssueWidget\\docs\\agent-runs\\20261118-160129-mvp-restart",
                           "relative_path":  "tasks/I02/result-02.md",
                           "sha256":  "520044FE6138609B99595C5FAF1D3973DD2D62EB82F27FE90392568C9599ED13"
                       },
                       {
                           "base":  "D:\\Projects\\GithubIssueWidget\\docs\\agent-runs\\20261118-160129-mvp-restart",
                           "relative_path":  "questions/I02-build-question-10.md",
                           "sha256":  "C2B20065574387F6DA4C52EDB518913CBC72EC29E9EE659F8C07C2BBC7ECEB4F"
                       },
                       {
                           "base":  "D:\\Projects\\GithubIssueWidget\\docs\\agent-runs\\20261118-160129-mvp-restart",
                           "relative_path":  "questions/I02-build-question-09.md",
                           "sha256":  "04E1931D735747621F89E83341A3D50C23A4CE95DDFE01BA346CD2F321539337"
                       },
                       {
                           "base":  "D:\\Projects\\GithubIssueWidget\\docs\\agent-runs\\20261118-160129-mvp-restart",
                           "relative_path":  "process/reference-check-design-02.md",
                           "sha256":  "3CDCB637F739F9A9DBE240D0901557BC2BB23732ABF7CCC528F98A514C06C06E"
                       },
                       {
                           "base":  "D:\\Projects\\GithubIssueWidget\\docs\\agent-runs\\20261118-160129-mvp-restart",
                           "relative_path":  "process/reference-check-failure-01.md",
                           "sha256":  "78AAD8B6C3329E29472FCF7BC144237C25F6B2BF2E8CF71284FB9890D4683957"
                       }
                   ]
}
```

Issue #1/#2 と固定参照を読み、B01 の既往2回の位置変化なし、実際の DOM target・入力保持・IPC 権限判断・Windows 応答が未観測という事実を保持する。原因は未確定であり、元のドラッグを再試行しない。

入力イベント・対象要素・マウスボタンの保持、Tauri の invoke と権限、Windows 側のメッセージ・ウィンドウ状態を区別できる、実行可能な独立検証方法を設計する。実際に記録できる事実、仮説、未確認、各候補を区別し、必要な計測手順、対象版・版変更の影響、非機密の記録内容・証拠 SHA、操作前提・安全性・復元・失敗時停止条件・独立レビューと次の判断を明確にする。通常の手順や副作用と保護対象を区別し、実行不能な仮定や完全な証明を前提にしない。既存 API / tool 仕様は公式一次資料を優先し、推測パスを開かず先に存在を確認する。

本担当は設計と読み取りのみ。製品・既存証跡・Git・GitHub を変更しない。アプリ起動、ドラッグ・入力注入・マウス操作・画面操作・OS操作・build・通信試験は行わない。現在 B03 build のセッションは生存・終了不明であり、起動・停止・再実行せず、対象プロセスや scratch・evidence を操作しない。読み取り調査と並行できない対象があれば停止して報告する。

所有範囲は指定の共有 run 結果と新規質問のみ。親ディレクトリ・確定先・tmp 不存在を確認して UTF-8 往復で確定する。結果に対象ファイル・版・実測 SHA、資料と取得時刻、各読み取りの cwd・コマンド・終了コード、次の計測案と操作の副作用・残リスク・未確認を記録。設計の独立レビュー前に実行せず、結果パスのみ通知する。

同種問題は初回を含め2回で該当操作を中断し別担当へ客観調査、3回で全作業を中止する。構造化フィールドの履歴を保持し、先行するドラッグ・文字破損・lockfile 失敗を同じ方法で繰り返さない。