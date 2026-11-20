# I02 release 保護ログの独立確認依頼 01

担当 /root/process_test_review、実行者 /root/build_verification と独立。固定参照・質問・metadata・driver・2ログの実体と SHA を確認し、ACL/所有・対象同一性・変更の有無を確かめる。内容の閲覧は秘密混入の確認に必要な最小限とし、実行やプロセス操作は行わない。

保護ログには credential 等が含まれ得る。秘密値、候補、断片、ログ本文をコンソール・共有 run・会話・外部に出力・転記しない。自動走査 clear-patterns-only は秘密不存在の保証ではない。検出・判定不能・アクセス不明・対象変化の場合は receipt を作らず内容を伏せて親に報告し、プロセス・ログを保持。

必要最小限の独立確認で公開可能と判断できた場合のみ、共有 run の tasks/I02/real-build-release-log-approval-01.json を公開プロトコルにより新規確定。JSON は stage=release、decision=approved-no-detected-sensitive、reviewer=/root/process_test_review、reviewedUtc=UTC ISO日時、driverSha256、stdoutSha256、stderrSha256 を実体の固定値で記録。内容・秘密の断片は書かない。親・tmp・確定先を確認し、対象 hash の前後再照合と JSON 構造の読戻しを行う。確定後パスのみ通知。親の判断前に publish 入力・ログ移動・次 stage を行わない。

receipt の承認は確認範囲に限り、秘密不存在の完全保証ではない。対象プロセスとセッションは作者が所有する。拒否された別の window metadata review 公開操作は再試行・回避しない。

同種2回で該当操作を中断し別担当が客観調査、同種3回で全作業停止。画像不一致・browser inventory エラー・直接公開逸脱・依頼形式訂正検査停止は各2回で調査済み、各同種3回目は全停止。

## 管理データ

```json
{
    "assignee":  "/root/process_test_review",
    "author":  "/root/build_verification",
    "stage":  "release",
    "references":  [
                       {
                           "path":  "D:\\Projects\\GithubIssueWidget\\docs\\agent-runs\\20261118-160129-mvp-restart\\questions\\I02-real-build-execution-question-05.md",
                           "sha256":  "4E5539726B87BB635ACE0090AF6769A1F2B236CB6E0096BE07FB8B67CE123A64"
                       },
                       {
                           "path":  "D:\\Projects\\GithubIssueWidget\\docs\\agent-runs\\20261118-160129-mvp-restart\\tasks\\I02\\real-build-execution-request-01.md",
                           "sha256":  "B3C0C3A65BE8A3980F79A10D8676C1971B24F26869612EE9FA7E92580249B898"
                       },
                       {
                           "path":  "D:\\Projects\\GithubIssueWidget\\docs\\agent-runs\\20261118-160129-mvp-restart\\tasks\\I02\\real-build-driver-design-02.md",
                           "sha256":  "0D6F42F22537F7DE5C943BC0E53AD98795227EE5719A20FDD7476EB505B17E21"
                       },
                       {
                           "path":  "D:\\Projects\\GithubIssueWidget\\docs\\agent-runs\\20261118-160129-mvp-restart\\tasks\\I02\\real-build-driver-review-02.md",
                           "sha256":  "708128F1D84F95059A505BEAB38C8E0A70E575AD845F3D6A993C6E4242B7EDAD"
                       },
                       {
                           "path":  "D:\\Projects\\GithubIssueWidget\\docs\\agent-runs\\20261118-160129-mvp-restart\\process\\evidence-publication-protocol-01.md",
                           "sha256":  "65E36A266503EB12E20B4045BC7D2A346C8F72B97BA3ED3EF73A3B80B36B3382"
                       },
                       {
                           "path":  "D:\\Projects\\GithubIssueWidget-build-20261119-02\\build-driver-03.ps1",
                           "sha256":  "BA0269227A5239F5E7B9CA0EF2802D3A5D5C7678821E67C141CC4A1958927FA9"
                       },
                       {
                           "path":  "D:\\Projects\\GithubIssueWidget-build-20261119-02\\release.log-review-needed.json",
                           "sha256":  "61CAC53E6701BE13697379C7EA6F6C7B80B78216E5B511A88CF7C2ABC9A7FB45"
                       },
                       {
                           "path":  "D:\\Projects\\GithubIssueWidget-build-20261119-02\\release.protected-logs\\stdout.log.tmp",
                           "sha256":  "8C615277D28A2E9C3A2B7CB3272E7C37B660F7B8678659C4EE19A00D7B88AD87"
                       },
                       {
                           "path":  "D:\\Projects\\GithubIssueWidget-build-20261119-02\\release.protected-logs\\stderr.log.tmp",
                           "sha256":  "35D143952405B01086E538741F7142D833882CBA626219E2160CC19D96647F71"
                       }
                   ],
    "investigate_at":  2,
    "stop_all_at":  3,
    "direct_publication_violation_count":  2,
    "format_correction_check_failures":  2
}
```
