# I02 終了コード・出力の無害試験依頼 01

担当 /root/build_verification。所有範囲は新規専用 scratch D:\Projects\GithubIssueWidget-process-test-20261119-01 と本依頼に指定した新規結果・質問のみ。開始前に親・正規化先・リンク・確定先の不存在と他作業非競合を確認。元製品・旧 build scratch・既存証跡・他者プロセスは変更・削除しない。

固定参照の独立調査を読み、利用可能な .NET API を確認し、無害な子プログラムと Process ドライバーの具体コード・コマンド・期待値・証跡・資源と時間上限、各ストリームの EOF と実プロセス終了コードの独立記録・停止判断を設計する。独立調査の条件により、具体コードと対象・保存先は実行前に独立レビューを受け親が判断する。設計とコードの固定版を準備して質問し、レビュー・親判断まで起動しない。

承認後のみ、独立したケースとして正常終了0、意図した非0、stdout/stderr の同時大量出力、timeout を無害な短時間プロセスで試験。外部通信・ポート・GUI を使わず、専用領域内のログと通常 OS 記録以外の副作用を発生させない。実行対象・PID・開始終了時刻・各ストリーム終端/量・終了後のみの実 exit code・期待値を結び付け、結果と対象スクリプト SHA を保存。

時間切れ時は失敗/未完了を記録し、同一の実行 handle を保持、他の同一試験を起動しない。所有・子孫・停止範囲を確認し管理に質問し、別途判断まで終了・強制停止しない。ハンドル断絶や起動成否不明時も再実行せず質問する。試験の成功は元ビルドの成功を意味しない。本ビルド・既存方式の再実行、製品変更、外部送信、デスクトップ操作は禁止。

試験計画は tasks/I02/process-test-design-01.md、質問は questions/I02-process-test-question-01.md、承認後の結果は tasks/I02/process-test-result-01.md。固定参照・担当・SHA を確認し、承認済み公開方法で新規に確定、パスのみ通知。結果は別担当が独立レビューする。

同種2回で該当操作を止め別担当が客観調査、同種3回で全作業停止。画像不一致・browser inventory エラー・直接公開逸脱・依頼形式訂正検査停止は各2回で独立調査済み、それぞれ同種3回目は全停止。拒否されたレビュー記録の公開操作は再試行・回避しない。

## 管理データ

```json
{
    "assignee":  "/root/build_verification",
    "references":  [
                       {
                           "path":  "D:\\Projects\\GithubIssueWidget\\docs\\agent-runs\\20261118-160129-mvp-restart\\tasks\\I02\\build-wait-diagnosis-result-01.md",
                           "sha256":  "B538DC70E8C0CAC5036E13C983A1BC2222AA9A3B7B5943C91AECF5E8451859D9"
                       },
                       {
                           "path":  "D:\\Projects\\GithubIssueWidget\\docs\\agent-runs\\20261118-160129-mvp-restart\\tasks\\I02\\build-verification-result-01.md",
                           "sha256":  "7A8AF3B60D32D2AC94888801067C73031DB8A84974DA95E6373D03C03E68A098"
                       },
                       {
                           "path":  "D:\\Projects\\GithubIssueWidget\\docs\\agent-runs\\20261118-160129-mvp-restart\\tasks\\I02\\verification-design-review-03.md",
                           "sha256":  "37899D597AAC6DA8F94E2775818A8DDC76796C0EBAA730E9404BCE14EEE20C25"
                       },
                       {
                           "path":  "D:\\Projects\\GithubIssueWidget\\docs\\agent-runs\\20261118-160129-mvp-restart\\process\\evidence-publication-protocol-01.md",
                           "sha256":  "65E36A266503EB12E20B4045BC7D2A346C8F72B97BA3ED3EF73A3B80B36B3382"
                       }
                   ],
    "investigate_at":  2,
    "stop_all_at":  3,
    "direct_publication_violation_count":  2,
    "format_correction_check_failures":  2
}
```
