# I02 B03 専用コピーでの build・dev 検証依頼 01

構造化フィールドを依頼の正本とする。担当 ID、対象・証跡の所有、ユーザー原文・追加指示、失敗履歴と閾値、参照版と SHA は同じデータから生成した。実担当・対象版・出力先・競合の状態は起動直前に管理が再照合する。ファイル照合は排他ロックではない。
```json
{
    "run_id":  "20261118-160129-mvp-restart",
    "task_id":  "I02-build-verification",
    "owner":  "/root/build_verification",
    "design_author":  "/root/verification_design",
    "design_reviewer":  "/root/plan_review",
    "scratch":  "D:\\Projects\\GithubIssueWidget-build-20261118-160129",
    "evidence_relative_path":  "tasks/I02/evidence-build-01",
    "output_relative_path":  "tasks/I02/build-verification-result-01.md",
    "user_original":  "Issueを確認し、プロジェクトを進行してください。あなたはオーケストレーションに徹してください。同じ間違いや失敗を２度繰り返したら中止してください。",
    "user_additions":  [
                           "失敗を3回繰り返すまでに緩和します",
                           "2回同じ問題にあったら別のエージェントに客観的に調査してもらってください"
                       ],
    "thresholds":  {
                       "independent_investigation":  2,
                       "stop_all":  3
                   },
    "current_history":  {
                            "content_validation_failures":  2,
                            "path_base_failure_after_restart":  1,
                            "structured_generation_type_error":  1
                        },
    "references":  [
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
                           "relative_path":  "tasks/I02/isolation-inventory-result-01.md",
                           "sha256":  "D73C3CE1114775361C4BF045E774F6F352C2F86796073DBD5842B6EE92D90555"
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
                           "relative_path":  "tasks/I02/management-04.md",
                           "sha256":  "0D569B792156CE706ACF0BF58295EBBFEE10D7FFE66434D09E36975B419F0239"
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

対象 worktree は D:\Projects\GithubIssueWidget-issue-2-20261118-160129 の task/issue-2-foundation-20261118-160129。Issue #1/#2、承認済みの専用コピー方式と独立レビュー、前版結果を読む。B03 は対象版と dev / release の実行・終了コード・出力の対応を示す作業であり、B01 の移動、B02/B04 や Issue 全体の完了を主張しない。

開始前に GitHub Issue の担当・状態・全コメント・PR、実行中担当・process・port・作業場所を再照合し、競合・対象版変化・権限や保護対象が不明なら実施しない。前版結果の製品欄だけから元17ファイルについて基点・相対パス・注記・SHA を分離し、全件の正規化先・実在・通常ファイル・再解析ポイントとハードリンクを確認してから hash と Git 状態を照合。既存の ignored node_modules・dist・target・gen、共有 run と他者の証跡を保護する。

構造化フィールドの scratch は未存在の専有予定地である。親の存在・正規化先・所有・再解析ポイント・ディスク空きと書込み権限を確認してから新規作成し、製品17ファイルだけを通常バイトコピーする。元とコピーのパス・サイズ・SHA、リンク不使用、作業前後の元の状態を manifest に記録。既存ファイルや所有不明を上書きせず、削除しない。

承認済み方式に従いコピー内 dist・node_modules・gen、専用 CARGO_TARGET_DIR・npm cache・CARGO_HOME・RUSTUP_HOME・TEMP/TMP・APPDATA/LOCALAPPDATA・証拠領域を分離。必要なツール・依存の出所・版・lockfile・設定・lifecycle scripts と実書込み範囲を、資格情報や private data を表示・コピーせず事前確認。専用領域で offline 依存が足りないとき、公開 registry 取得が安全か対象・想定通信・副作用を判断し、不明なら停止して管理へ報告する。env は子プロセスに限り、ユーザーの永続設定・Credential Manager・既存 profile を変更しない。通常の OS cache やシステムログの可能性は記録し、全面隔離を主張しない。重要な予期しない出力・秘密へのアクセス・他者作業との競合があれば停止する。

条件を満たせばコピー側で lockfile に基づく依存準備後、承認済み方式の build --no-bundle と dev を実行。cwd・ツール・コマンド・開始終了 UTC・stdout/stderr・実プロセス終了コード、コピーと元の前後 SHA、lock、生成 exe SHA、PID・ウィンドウを記録し、dev の正常終了を証拠化する。作業対象アプリの必要な起動・閉じる操作だけとし、ドラッグ・ウィンドウ移動・リサイズ・外部リンクやブラウザ操作は行わない。プロセス起動・ログ取得・正常終了に不明や失敗があれば成功扱いしない。環境・依存差や通常の副作用を明記する。

所有範囲は構造化フィールドの専用 scratch、共有 run の専用 evidence と結果、新規質問のみ。親ディレクトリと確定先・tmp 不存在を確認し UTF-8 往復して確定する。各証拠と変更・生成物を対象版・SHA と対応付け、未実施・失敗・リスクを区別し独立レビューへ渡す。成果物を削除せず、製品・元 worktree・既存証跡・GitHub を変更しない。push / PR / merge は未許可。

同種の問題が初回を含め2回なら該当操作を停止し別担当に客観調査を依頼、3回なら全作業を中止する。構造化フィールドの履歴も遵守し、過去のドラッグと文字破損の経路を再試行しない。確定後は結果のパスだけを通知する。