# I02 単一クリックによる入力到達の診断依頼 01

構造化フィールドを依頼の正本とする。担当・対象版・専用領域・出力先・ユーザー指示・履歴・閾値・固定参照は同一データから生成。作業前に担当 ID、対象版、領域、参照と競合を再照合する。照合は排他ロックではない。
```json
{
    "run_id":  "20261118-160129-mvp-restart",
    "task_id":  "I02-drag-click-diagnostic",
    "owner":  "/root/drag_design",
    "design_reviewer":  "/root/plan_review",
    "source_worktree":  "D:\\Projects\\GithubIssueWidget-issue-2-20261118-160129",
    "source_branch":  "task/issue-2-foundation-20261118-160129",
    "source_head":  "a4ac97286785a2a5ba0d070a909b58003fc49ff1",
    "scratch":  "D:\\Projects\\GithubIssueWidget-drag-diagnostic-20261118-160129",
    "isolated_dev_port":  1435,
    "evidence_relative_path":  "tasks/I02/evidence-drag-click-01",
    "output_relative_path":  "tasks/I02/drag-click-result-01.md",
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
                    "official_page_open_internal_error":  1
                },
    "references":  [
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
                           "relative_path":  "tasks/I02/drag-result-01.md",
                           "sha256":  "B34245F943AD4B9E69F5DBC8020C762499055A2ABDE878F57446131038C5CCB6"
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
                           "relative_path":  "questions/I02-build-question-10.md",
                           "sha256":  "C2B20065574387F6DA4C52EDB518913CBC72EC29E9EE659F8C07C2BBC7ECEB4F"
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

目的はドラッグを無効にした別の診断版で、単一押下・解放が対象 header に到達するかを非機密イベントで観測すること。これは過去のドラッグの到達、製品版の移動成功、IPC 権限や OS の応答、Issue 完了を証明しない。元の移動操作の再試行はしない。

**実行条件:** この依頼の具体的な対象版・操作領域・方式・安全条件を作者以外が独立レビューし、親が対象版を確認するまでコピー作成・診断コード作成・入力を実行しない。B03 は参照時点で終了不明のセッションを所有者が中断・証拠記録中であり、その担当から確定記録を受領し、当該セッションの terminal と対象 UI・プロセス・port の非競合を確認するまで、画面入力・診断アプリ起動を行わない。停止や削除を本担当が代行しない。対象 ID、所有、ユーザーの操作状況や開始条件が不明なら停止する。

承認後、source worktree の対象17ファイルを前版結果と全件照合し、実体コピーのみを指定の未存在 scratch へ作る。親・正規化先・再解析ポイント・ハードリンク・ACL・容量・元資産との非重複を先に確認し、元とコピーの path・size・SHA と元状態を記録。既存生成物・共有 run・B03 scratch を読み書き上書きしない。診断用の差分は指定 scratch のコピーにだけ作り、元版と変更点・ファイル SHA・依存・ツール・生成 exe を別版として固定する。ポートは構造化フィールドの専用値を事前に空き確認し、診断コピーの dev URL 変更が必要なら差分に明記。共有 port、target、cache、temp、profile、既存プロセスを使わず、承認済みの専用コピー方式の書込み先・資格情報保護・通常副作用を適用する。不明な lifecycle script、保存先、権限、ログへの秘密混入は実行せず停止。

診断版では該当 header の通常 `startDragging` を無効化し、event type、button、buttons、isTrusted、target の区分（header・非 button 子・button）、粗い位置、単調時刻のみを専用バッファに記録。URL・テキスト・キー・Issue 内容は取得しない。コードと構成の差分・ハッシュについて入力前に独立レビューを受け、診断版が移動開始を呼ばないことを確認。対象 PID・exe SHA・window ID・位置・対象領域・desktop の競合を識別し、親の対象版判断後に header 内で単一の押下・解放のみ行う。ドラッグ、移動、リサイズ、別ウィンドウの操作は行わない。対象が曖昧、イベント不達、操作・記録が失敗または不明なら繰り返さず止め、証拠と理由を親へ報告する。

証拠は専用 evidence に対象版と診断差分・元/コピー17 SHA、環境・cwd・コマンド・終了コード・PID/ID・観測時刻・単一入力の結果・許可されたイベントのみに限定し、個人情報を含めない。対象版を含む証拠の SHA と未確認を結果に記録。処理完了後は所有した診断プロセスのみ安全に終了し、既存アプリ・B03 のプロセスを触らず、scratch と証拠は保持し削除しない。生成物や通常の OS cache 等の副作用・環境差は記録。現状や条件により実施不能なら未実施の理由を明示する。

所有範囲は構造化フィールドの専用 scratch、専用 evidence と結果、新規質問のみ。元製品・既存証跡・Git・GitHub 更新、push / PR / merge / 削除は禁止。親ディレクトリと確定先・tmp 不在を確認し UTF-8 往復後に確定する。履歴のある操作を再試行せず、同種2回で該当操作中断と別担当の客観調査、3回で全作業を中止する。依頼と診断版のレビュー完了前に実行を開始しない。結果パスのみ通知する。