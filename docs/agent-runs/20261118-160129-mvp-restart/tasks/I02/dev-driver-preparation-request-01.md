# I02 開発起動検証用ドライバー準備依頼 01

担当 /root/build_verification。独立レビュー済み dev-verification-design-01.md の対象版に限り、実行前の具体 driver・overlay・証拠計画を準備する。準備成果を固定し作者以外の独立レビューと親判断を受けるまで dev・build・依存導入・ネットワーク取得・GUI・対象 process の起動停止を行わない。元製品・既存 release/marker scratch・他者の診断セッションを保護する。

新規専用候補 D:\Projects\GithubIssueWidget-dev-20261119-01 の親・正規化先・所有・リンク・不存在・競合を確認。元 worktree の未コミット17ファイルを manifest と基点内の実体 SHA により固定し、必要な専用コピーは通常バイトで作成して元・コピー全件の path/hash を照合。HEAD だけで版を識別せず、旧資産と専用の source/target/dist/cache/temp/profile/ログが重複しないことを確かめる。

独立承認計画の overlay と dev driver を新規版で用意し、対象17件・overlay の唯一の指定差・toolchain・driver と実行コマンドの SHA/版を固定。Windows の既定 WebView profile の共有可能性、appDirectoriesOverride・WEBVIEW2_USER_DATA_FOLDER・registry policy の優先と実際の profile 分離の確認方法を具体化。設定が実効か判別できなければ成功扱いせず停止。overlay 付き debug 起動は既定設定や release exe の検証を意味しない。

Node/npm、Vite、対象 app の PID・実行パス・SHA・開始時刻・親子関係とウィンドウ所有を区別して識別する手順、同一 handle による app の正常 close・exit、wrapper の exit・EOF、独立した Vite exit の取得可否を具体化。close 要求の戻り値のみで正常終了としない。秘密を含み得るログは保護し別担当の限定確認・固定 hash receipt・親判断前に公開しない。起動不明、timeout、fault、入力途絶、対象不一致や所有不明時は同じ操作を繰り返さず handle・証拠を保護し親へ質問。

既存 app/server と port 1435 を保護し、対象 port 1420 の IPv4/IPv6・他者操作・デスクトップ・CPU/メモリ/ディスク・ネットワーク・通常 OS 副作用を直前に再確認する条件を用意。画像不一致・browser inventory・JSON 件数問題は各2回で調査済み、同じ画像取得・getState/browser inventory・失敗した配列化構文は再実行しない。過去に拒否された別 window metadata review 公開操作は再試行・回避しない。

結果は tasks/I02/dev-driver-preparation-result-01.md、質問は questions/I02-dev-driver-preparation-question-01.md から空き番号。作成物・SHA、静的検査、根拠、未実施・実行ゲートと残リスクを公開プロトコルで確定しパスのみ通知。独立レビュー後に親が実行可否を判断する。

同種2回で該当操作を中断し別担当が客観調査、初回含む同種3回で全作業停止。直接公開逸脱・依頼形式訂正検査停止も各2回で調査済み、各同種3回目は全停止。

## 管理データ

```json
{
    "assignee":  "/root/build_verification",
    "reviewer":  "/root/process_test_review",
    "references":  [
                       {
                           "path":  "D:\\Projects\\GithubIssueWidget\\docs\\agent-runs\\20261118-160129-mvp-restart\\tasks\\I02\\dev-verification-design-01.md",
                           "sha256":  "B8563D57E131E8D70CD6307270BAF394084B4C1B297BA1D29A19EF694190F037"
                       },
                       {
                           "path":  "D:\\Projects\\GithubIssueWidget\\docs\\agent-runs\\20261118-160129-mvp-restart\\tasks\\I02\\dev-verification-design-review-01.md",
                           "sha256":  "0FC588CD5E671C3616388F4AE1199AEF9859FF694DA72092185C9B133A3718D7"
                       },
                       {
                           "path":  "D:\\Projects\\GithubIssueWidget\\docs\\agent-runs\\20261118-160129-mvp-restart\\tasks\\I02\\dev-verification-design-request-01.md",
                           "sha256":  "6267BAA463643AB6C80D279844CD2AAAE818F8504D8A68F13134BABE359B38F1"
                       },
                       {
                           "path":  "D:\\Projects\\GithubIssueWidget\\docs\\agent-runs\\20261118-160129-mvp-restart\\tasks\\I02\\real-build-result-review-01.md",
                           "sha256":  "2CD96248A50CE4D0A76D4F10688A1CFEA73844C58C0A56BD7027150A8DA97F51"
                       },
                       {
                           "path":  "D:\\Projects\\GithubIssueWidget\\docs\\agent-runs\\20261118-160129-mvp-restart\\tasks\\I02\\result-02.md",
                           "sha256":  "520044FE6138609B99595C5FAF1D3973DD2D62EB82F27FE90392568C9599ED13"
                       },
                       {
                           "path":  "D:\\Projects\\GithubIssueWidget\\docs\\agent-runs\\20261118-160129-mvp-restart\\questions\\I02-window-metadata-question-02.md",
                           "sha256":  "AC284337BFC0AA4718674B973D3BE129679800B3410CE987836B90AA43E394FE"
                       },
                       {
                           "path":  "D:\\Projects\\GithubIssueWidget\\docs\\agent-runs\\20261118-160129-mvp-restart\\process\\evidence-publication-protocol-01.md",
                           "sha256":  "65E36A266503EB12E20B4045BC7D2A346C8F72B97BA3ED3EF73A3B80B36B3382"
                       },
                       {
                           "path":  "D:\\Projects\\GithubIssueWidget-build-20261119-02\\source-manifest.json",
                           "sha256":  "BB5253F1D02C83E183C86A6981CA3768A01892E3D1B40EF3AC4AA6A014DE0033"
                       }
                   ],
    "screenshot_mismatch_count":  2,
    "browser_inventory_failure_count":  2,
    "json_count_failure_count":  2,
    "direct_publication_violation_count":  2,
    "format_correction_check_failures":  2,
    "investigate_at":  2,
    "stop_all_at":  3
}
```
