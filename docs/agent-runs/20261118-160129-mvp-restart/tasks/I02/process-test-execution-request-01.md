# I02 無害プロセス試験の実行依頼 01

担当 /root/build_verification。独立承認は process-test-design-review-03.md の対象版に限る。親は無害試験の実行依頼を指示。固定版 design-02 と design-01、レビュー、child.cjs、driver-04.ps1、Node の参照実体 SHA を開始直前に確認し、相違なら実行せず停止。元製品・旧 scratch・既存証拠・他者のプロセスを保護する。

専用 scratch D:\Projects\GithubIssueWidget-process-test-20261119-01 と指定成果のみ所有。開始前に親と正規化先・リンク・所有・プロセス/資源の非競合を確認。normal、nonzero、flood、timeout 各モードの stdout.log、stderr.log、result.json、timeout.json と各 .tmp が全て不存在であることを直前に確認。既存物を上書き・削除しない。

design-02 と design-01 に従い、同版 driver-04.ps1 の4ケースを直列に実行。各ケース終了・全出力・期待 exit code・EOF・期待文字列全体・hash と証跡を確認し、失敗または不明があれば次に進まない。標準出力・標準エラー間の発生順序は証明しない。子・driver の起動成否、標準入力、結果の確定や内容が不明なら再実行せず管理へ質問。

timeout は未成功として記録し同一 Process handle と所有セッションを維持。自然終了の観測と保持可能な標準入力を確認して親へ質問し、判断前に continue・停止・再起動しない。ほかのケースも時間切れなら同様。管理判断後も子が終了していなければ続行しない。プロセス所有・子孫・停止範囲が不明なら停止操作はしない。

結果は tasks/I02/process-test-result-01.md、質問は questions/I02-process-test-execution-question-01.md から空き番号。終了コード・実時刻・PID・全出力の検証・対象 hash・証拠と未確認を記録、公開プロトコルに従い確定後パスのみ通知。独立レビューを受ける。元 build の実行・画像取得・GUI 操作・ポート・外部通信・製品変更は禁止。OS の通常記録等の副作用は残り得る。以前拒否された window metadata review の公開操作は再試行・回避しない。

同種2回で該当操作を中断し別担当が客観調査、同種3回で全作業停止。画像不一致・browser inventory エラー・直接公開逸脱・依頼形式訂正検査停止は各2回で調査済み、各同種3回目は全停止。

## 管理データ

```json
{
    "assignee":  "/root/build_verification",
    "reviewer":  "/root/process_test_review",
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
                           "path":  "D:\\Projects\\GithubIssueWidget\\docs\\agent-runs\\20261118-160129-mvp-restart\\tasks\\I02\\process-test-design-review-03.md",
                           "sha256":  "95F3DADB834846F1EF2DE9559611DCE8FE9D70994EAE5017B0C5C54211041D51"
                       },
                       {
                           "path":  "D:\\Projects\\GithubIssueWidget\\docs\\agent-runs\\20261118-160129-mvp-restart\\tasks\\I02\\process-test-request-01.md",
                           "sha256":  "EAFAFD748888ADEE32FDFD0C5B6A238E2039EB446898052E16929733ADE6332C"
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
