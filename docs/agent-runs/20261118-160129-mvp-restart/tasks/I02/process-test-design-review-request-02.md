# I02 無害プロセス試験改訂版の独立確認依頼 02

担当 /root/plan_review、作者 /root/build_verification と独立。固定参照の設計02・前版・driver-04・child・Node を照合し、SHA と実体を確認。前版の文字列全体と WAITING の順序検査不足が解消されているか、4ケースの内容、終了コードと EOF、タイムアウト時の同一 handle・入力維持・停止条件、出力・資産・秘密の保護、環境・競合を読み取りで独立確認する。実行能力は実行前には断定しない。

版ごとに対象 SHA、観察事実、blocking と解消条件、未確認・残る副作用を親へ会話で報告する。過去の特定の window metadata review 公開コマンドは自動ポリシー拒否されているため再試行・回避しない。レビューの確定ファイルは今回作成せず、正本未確定と明示する。

試験・ビルド・プロセス停止・画像取得・CUA getState/browser inventory・入力・製品変更・外部送信は禁止。既存の資産・プロセス・証跡を保護する。固定参照、担当、SHA または所有が不明な場合は停止し親へ通知。

同種2回で該当操作を中断して別担当が客観調査、同種3回で全作業停止。画像不一致・browser inventory エラー・直接公開逸脱・依頼形式訂正検査停止は各2回で調査済み、各同種3回目は全停止。

## 管理データ

```json
{
    "assignee":  "/root/plan_review",
    "author":  "/root/build_verification",
    "references":  [
                       {
                           "path":  "D:\\Projects\\GithubIssueWidget\\docs\\agent-runs\\20261118-160129-mvp-restart\\tasks\\I02\\process-test-design-02.md",
                           "sha256":  "31E6F1F35FD221DB21AFA35DFFD200F0D4E4E1DD979ABFA1A9DAE90D5B403894"
                       },
                       {
                           "path":  "D:\\Projects\\GithubIssueWidget\\docs\\agent-runs\\20261118-160129-mvp-restart\\tasks\\I02\\process-test-design-01.md",
                           "sha256":  "C39FE60DFEDA356AC5C39C86A1052BCD26E5C2DEADF3CB90F4F0922392E5DC48"
                       },
                       {
                           "path":  "D:\\Projects\\GithubIssueWidget\\docs\\agent-runs\\20261118-160129-mvp-restart\\tasks\\I02\\process-test-request-01.md",
                           "sha256":  "EAFAFD748888ADEE32FDFD0C5B6A238E2039EB446898052E16929733ADE6332C"
                       },
                       {
                           "path":  "D:\\Projects\\GithubIssueWidget\\docs\\agent-runs\\20261118-160129-mvp-restart\\tasks\\I02\\build-wait-diagnosis-result-01.md",
                           "sha256":  "B538DC70E8C0CAC5036E13C983A1BC2222AA9A3B7B5943C91AECF5E8451859D9"
                       },
                       {
                           "path":  "D:\\Projects\\GithubIssueWidget-process-test-20261119-01\\child.cjs",
                           "sha256":  "55FFB888FAA4F5E2B93D0F806DF6892834258B15756535895BC68937C621276C"
                       },
                       {
                           "path":  "D:\\Projects\\GithubIssueWidget-process-test-20261119-01\\driver-04.ps1",
                           "sha256":  "F24B68C56FA99A67685BFB864F9B39F768EBDBA0CC1D7B2E92A12CA4DF0C7FDD"
                       },
                       {
                           "path":  "C:\\Program Files\\nodejs\\node.exe",
                           "sha256":  "F5109DC868EC38703AD8F305E0ACEECE717CB60D4889873F434017AF4E96CCCB"
                       }
                   ],
    "investigate_at":  2,
    "stop_all_at":  3,
    "direct_publication_violation_count":  2,
    "format_correction_check_failures":  2
}
```
