# I02 実ビルド用ドライバー修正依頼 01

担当 /root/build_verification。独立レビュー real-build-driver-review-01.md の対象設計01と driver-02 は B01/B02 のため未承認。固定参照の実体 SHA と所有・対象を照合し、既存版・元製品・コピー・旧証拠・他担当のプロセスを保護。修正は専用 scratch 内の新規 driver 版と共有 run の新規設計・質問のみ。既存版は上書きしない。

B01: 両ストリームの一時ログを保護領域に保持し、必要な秘密混入確認、検出の限界、不明時の停止と所有者への報告を、実コードまたは確実な実行ゲートとして具体化。確認前の確定名への公開、内容の不要な表示や外部送信を防ぐ。秘密らしい内容を質問・ログへ再掲しない。

B02: 個々の非同期タスクの正常 EOF、fault、cancel を区別し、稼働中も監視。fault・ログ書込み失敗・起動成否不明・待機例外の際に所有プロセスと handle、部分ログ・証跡を保護し、管理に状態と限界を報告して同一 stage を自動再起動しない。wait/continue は fault を正常 EOF と扱わず、VS bootstrap のタスクも同様に扱う。追跡・保持できない場合を明示して安全な停止判断につなげる。

前依頼の17件固定、専用環境・通信・資源競合・exit/EOF/timeout・対象版条件を維持し、修正版コードと設計の SHA、検証の範囲、未確認、実行前条件を記録。静的検査は対象を明示し、実 build・依存導入・ネットワーク取得・GUI・他者プロセス操作は禁止。対象版の独立再レビューと親判断まで新 driver を実行しない。

結果は tasks/I02/real-build-driver-design-02.md、質問は questions/I02-real-build-driver-revision-question-01.md から空き番号。公開プロトコルで確定しパスのみ通知。拒否された window metadata review の公開操作は再試行・回避禁止。

同種2回で該当操作を中断し別担当が客観調査、同種3回で全作業停止。画像不一致・browser inventory エラー・直接公開逸脱・依頼形式訂正検査停止は各2回で調査済み、各同種3回目は全停止。

## 管理データ

```json
{
    "assignee":  "/root/build_verification",
    "reviewer":  "/root/process_test_review",
    "references":  [
                       {
                           "path":  "D:\\Projects\\GithubIssueWidget\\docs\\agent-runs\\20261118-160129-mvp-restart\\tasks\\I02\\real-build-driver-review-01.md",
                           "sha256":  "1738E469B6F0D3D974F27CD4BA1E5784896AD911BAA171A8BB8F636AEBF8E2DF"
                       },
                       {
                           "path":  "D:\\Projects\\GithubIssueWidget\\docs\\agent-runs\\20261118-160129-mvp-restart\\tasks\\I02\\real-build-driver-design-01.md",
                           "sha256":  "F4D173AAEF4964FB8C1B46B2A9BD4D71E16B4DDCD7EDAFB45651B79D6BA9E50D"
                       },
                       {
                           "path":  "D:\\Projects\\GithubIssueWidget\\docs\\agent-runs\\20261118-160129-mvp-restart\\tasks\\I02\\real-build-driver-request-01.md",
                           "sha256":  "EECFAD527EEA925C001B9A64BA859869648CE7248DAEF30A4E241FAFCC11E99E"
                       },
                       {
                           "path":  "D:\\Projects\\GithubIssueWidget\\docs\\agent-runs\\20261118-160129-mvp-restart\\tasks\\I02\\process-test-result-review-01.md",
                           "sha256":  "3B2BA6D87D2D40CCCDE6DED595F6C29A6A5624C69CF7A614EFFA5AB5FDC3652C"
                       },
                       {
                           "path":  "D:\\Projects\\GithubIssueWidget\\docs\\agent-runs\\20261118-160129-mvp-restart\\process\\evidence-publication-protocol-01.md",
                           "sha256":  "65E36A266503EB12E20B4045BC7D2A346C8F72B97BA3ED3EF73A3B80B36B3382"
                       },
                       {
                           "path":  "D:\\Projects\\GithubIssueWidget-build-20261119-02\\build-driver-02.ps1",
                           "sha256":  "6B0E8CEB7C86C52E06982A8FB83A024E34EEE18C226B285319777F68608EC078"
                       },
                       {
                           "path":  "D:\\Projects\\GithubIssueWidget-build-20261119-02\\source-manifest.json",
                           "sha256":  "BB5253F1D02C83E183C86A6981CA3768A01892E3D1B40EF3AC4AA6A014DE0033"
                       },
                       {
                           "path":  "D:\\Projects\\GithubIssueWidget-build-20261119-02\\copy-manifest.json",
                           "sha256":  "BCE67077C4CB323586F52E527318B6584B3D48BBC956E879B75774ED6F5482E0"
                       }
                   ],
    "investigate_at":  2,
    "stop_all_at":  3,
    "direct_publication_violation_count":  2,
    "format_correction_check_failures":  2
}
```
