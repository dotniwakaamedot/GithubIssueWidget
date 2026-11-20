# I02 無害プロセス試験設計の独立レビュー依頼 01

担当 /root/plan_review、作者 /root/build_verification と独立。固定版の設計・実体コード・Node と依頼・独立調査を参照して SHA を確認し、実行前の妥当性と安全性をレビュー。すべて読み取り限定で試験は起動しない。

正常0・意図的非0・両ストリーム同時大量出力・timeout の期待値と実コードを照合。Process handle・引数・環境・stdout/stderr 同時消費・EOF と exit code・時間・各証跡の対応、出力順序とバイト数、予期しない終了や失敗時の扱い、所有・停止範囲、timeout 中の同一 handle と入力維持が要件を満たすか評価。確認できない実行環境は未検証と記録。

専用 scratch と元製品・旧証拠・既存プロセスの保護、出力の非上書き・公開手順、秘密・環境・ネットワーク・GUI・資源消費、既存画面作業との競合を確認。判定は対象 SHA・根拠・blocking と解消条件を明記。結果は tasks/I02/process-test-design-review-01.md に公開プロトコルを守り確定しパスのみ通知。

試験・ビルド・process停止・画像取得・CUA getState/browser inventory・入力・製品変更・外部送信は不可。拒否されたウィンドウメタデータレビューの公開操作を再試行・回避しない。

同種2回で該当操作を中断し別担当の客観調査、同種3回で全作業停止。画像不一致・browser inventory エラー・直接公開逸脱・依頼形式訂正検査停止は各2回で調査済み、各同種3回目は全停止。

## 管理データ

```json
{
    "assignee":  "/root/plan_review",
    "author":  "/root/build_verification",
    "references":  [
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
                           "path":  "D:\\Projects\\GithubIssueWidget\\docs\\agent-runs\\20261118-160129-mvp-restart\\process\\evidence-publication-protocol-01.md",
                           "sha256":  "65E36A266503EB12E20B4045BC7D2A346C8F72B97BA3ED3EF73A3B80B36B3382"
                       },
                       {
                           "path":  "D:\\Projects\\GithubIssueWidget-process-test-20261119-01\\child.cjs",
                           "sha256":  "55FFB888FAA4F5E2B93D0F806DF6892834258B15756535895BC68937C621276C"
                       },
                       {
                           "path":  "D:\\Projects\\GithubIssueWidget-process-test-20261119-01\\driver-03.ps1",
                           "sha256":  "D00C204A2DE136EC411366D540E80AD72BFC84338BE78DEB4A9B59F84876AD31"
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
