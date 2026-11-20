# I02 実ビルド用ドライバー設計の独立レビュー依頼 01

担当 /root/process_test_review、作者 /root/build_verification と独立。固定版設計、driver-02、manifest、前提レビュー・無害試験を参照して SHA と実体を確認。元と新規コピーの未追跡を含む対象17ファイルを、各基点・相対パス・期待 hash を分離し、全件存在確認後に実体と照合。空集合を成功としない。旧ドライバーを実行しない。

実コードと設計を照合し、rustup・npm・release と VS bootstrap の直接起動対象・子孫・シェル・exit code 伝播の限界、専用の source・target・dist・cache・temp・環境・通信先と資格情報/他資産保護を検査。公開依存と install/build script、OS 通常副作用、CPU・メモリ・ディスク・画面診断との競合および各 stage の前提条件を評価。

同一 Process handle、stdout/stderr の別非同期消費と EOF・エラー、実 exit code・時刻・PID、ログ・機密混入時の扱い・exe SHA と対象版の紐付けを照合。timeout・EOF 待機・fault・入力途絶・予期しない書込み・起動成否不明の際、handle と所有セッションを追跡・保持して親判断を待てるか評価。ログ上限は実停止でない点も確認し、未検証の実行性を区別。

対象 SHA と根拠、blocking・解消条件・未確認・実行前の条件を記載し、tasks/I02/real-build-driver-review-01.md に公開プロトコルで確定後パスのみ通知。実装レビューは実 build の成功や実行許可ではない。読み取り限定で build、依存導入、通信、UI、入力、process 停止/起動、製品変更は禁止。既存成果・他者の作業を保護。拒否された window metadata review 公開操作を再試行・回避しない。

同種2回で該当操作を中断し別担当が客観調査、同種3回で全作業停止。画像不一致・browser inventory エラー・直接公開逸脱・依頼形式訂正検査停止は各2回で調査済み、各同種3回目は全停止。

## 管理データ

```json
{
    "assignee":  "/root/process_test_review",
    "author":  "/root/build_verification",
    "references":  [
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
                           "path":  "D:\\Projects\\GithubIssueWidget\\docs\\agent-runs\\20261118-160129-mvp-restart\\tasks\\I02\\process-test-result-01.md",
                           "sha256":  "E0E7FABA107E515CEF1DC0B65A839DBDD832748DB2DD20D0F30249174B2DF175"
                       },
                       {
                           "path":  "D:\\Projects\\GithubIssueWidget\\docs\\agent-runs\\20261118-160129-mvp-restart\\tasks\\I02\\result-02.md",
                           "sha256":  "520044FE6138609B99595C5FAF1D3973DD2D62EB82F27FE90392568C9599ED13"
                       },
                       {
                           "path":  "D:\\Projects\\GithubIssueWidget\\docs\\agent-runs\\20261118-160129-mvp-restart\\tasks\\I02\\verification-design-review-03.md",
                           "sha256":  "37899D597AAC6DA8F94E2775818A8DDC76796C0EBAA730E9404BCE14EEE20C25"
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
