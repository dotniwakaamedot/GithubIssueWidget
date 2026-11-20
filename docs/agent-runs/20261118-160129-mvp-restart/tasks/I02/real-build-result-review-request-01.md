# I02 専用コピー release build 結果の独立レビュー依頼 01

担当 /root/process_test_review、作者 /root/build_verification と独立。固定参照の結果・設計・driver・manifest・各 stage JSON・ログ・receipt・exe の実体 SHA と結果記載の期待値を比較。元と専用コピーの17ファイルは基点・相対パス・SHA を分け全件存在確認後に実体照合、各 stage 前後の記録・件数・hash と結び付ける。空集合や HEAD のみを対象版の証拠にしない。

rustup、npm、release と VS bootstrap の直接子 PID・時刻・exit・両 EOF、ログ hash、receipt・担当・stage、exe 実体 SHA、構造化結果の相互整合を確認。承認されたログを必要最小限に読む場合も秘密値・候補・断片・本文を出力・記録しない。不明・秘密候補は内容を伏せ保護し親へ報告。receipt と clear-patterns-only は秘密不存在の完全な保証ではない。

実際の exit は直接起動子のもので、npm→shell→Tauri→Cargo の各 exit や伝播、当時の handle 連続性、途中一時変更、完全な非競合、旧 build の exit は未証明として区別。release build の証拠が示す範囲と B03 の残条件、dev・ウィンドウ・Opener 等未実施を判定し、対象版・根拠・blocking・未確認を記録。

結果は tasks/I02/real-build-result-review-01.md。公開プロトコルで非上書き確定しパスのみ通知。読み取り限定で build・依存導入・通信・GUI・入力・process 停止/起動・製品変更は禁止。元製品・scratch・証拠を保護。拒否された別の window metadata review 公開操作は対象外、再試行・回避しない。

同種2回で該当操作を中断し別担当が客観調査、同種3回で全作業停止。画像不一致・browser inventory エラー・直接公開逸脱・依頼形式訂正検査停止は各2回で調査済み、各同種3回目は全停止。

## 管理データ

```json
{
    "assignee":  "/root/process_test_review",
    "author":  "/root/build_verification",
    "references":  [
                       {
                           "path":  "D:\\Projects\\GithubIssueWidget\\docs\\agent-runs\\20261118-160129-mvp-restart\\tasks\\I02\\real-build-result-01.md",
                           "sha256":  "D4056E537530E340B9556C82503D55FE40236FFE49471D429E1F3D2575E837D3"
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
                           "path":  "D:\\Projects\\GithubIssueWidget\\docs\\agent-runs\\20261118-160129-mvp-restart\\tasks\\I02\\result-02.md",
                           "sha256":  "520044FE6138609B99595C5FAF1D3973DD2D62EB82F27FE90392568C9599ED13"
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
                           "path":  "D:\\Projects\\GithubIssueWidget-build-20261119-02\\source-manifest.json",
                           "sha256":  "BB5253F1D02C83E183C86A6981CA3768A01892E3D1B40EF3AC4AA6A014DE0033"
                       },
                       {
                           "path":  "D:\\Projects\\GithubIssueWidget-build-20261119-02\\copy-manifest.json",
                           "sha256":  "BCE67077C4CB323586F52E527318B6584B3D48BBC956E879B75774ED6F5482E0"
                       },
                       {
                           "path":  "D:\\Projects\\GithubIssueWidget-build-20261119-02\\cargo-target\\release\\github-issue-widget.exe",
                           "sha256":  "D5FD13D80B6FE7F72ACD0BA6CEA3E0E19F261E417F8F5331E81F94D09A379D44"
                       },
                       {
                           "path":  "D:\\Projects\\GithubIssueWidget-build-20261119-02\\rustup.result.json",
                           "sha256":  "58E4EFDCB6E212E0BD2F640D2CB015EA7DC896CB055C38C25B290E9FC94FAEBE"
                       },
                       {
                           "path":  "D:\\Projects\\GithubIssueWidget-build-20261119-02\\rustup.before.json",
                           "sha256":  "D747AA1C2D1BF9A2ACAC9FD0CE55712062B58B714EEFCE4B6F3E796D56CBEDF8"
                       },
                       {
                           "path":  "D:\\Projects\\GithubIssueWidget-build-20261119-02\\rustup.after.json",
                           "sha256":  "D747AA1C2D1BF9A2ACAC9FD0CE55712062B58B714EEFCE4B6F3E796D56CBEDF8"
                       },
                       {
                           "path":  "D:\\Projects\\GithubIssueWidget-build-20261119-02\\rustup.log-review-needed.json",
                           "sha256":  "9F7954D45DCA84415B7DD62430F3B2D67D8BE63706ECE3E672E790401CE7B8C1"
                       },
                       {
                           "path":  "D:\\Projects\\GithubIssueWidget-build-20261119-02\\rustup.protected-logs\\stdout.log",
                           "sha256":  "BBBB3EB31124866016C34BAFEF844DE0A7F20F5B1545B2B29102F90F5C565E08"
                       },
                       {
                           "path":  "D:\\Projects\\GithubIssueWidget-build-20261119-02\\rustup.protected-logs\\stderr.log",
                           "sha256":  "B221EB4122453884168CD67BEB24241647E8B8653986BA3162D3163A1E53256C"
                       },
                       {
                           "path":  "D:\\Projects\\GithubIssueWidget\\docs\\agent-runs\\20261118-160129-mvp-restart\\tasks\\I02\\real-build-rustup-log-approval-01.json",
                           "sha256":  "0A008E59EFC56D5FD2B69A8D295ABB9F479FCB2EFA5B622E491880D2FA7F238E"
                       },
                       {
                           "path":  "D:\\Projects\\GithubIssueWidget-build-20261119-02\\npm.result.json",
                           "sha256":  "6F138DBD599471CA977587C9D2E19ED7CFF8DE9051513620702BAF0E782814C5"
                       },
                       {
                           "path":  "D:\\Projects\\GithubIssueWidget-build-20261119-02\\npm.before.json",
                           "sha256":  "D747AA1C2D1BF9A2ACAC9FD0CE55712062B58B714EEFCE4B6F3E796D56CBEDF8"
                       },
                       {
                           "path":  "D:\\Projects\\GithubIssueWidget-build-20261119-02\\npm.after.json",
                           "sha256":  "D747AA1C2D1BF9A2ACAC9FD0CE55712062B58B714EEFCE4B6F3E796D56CBEDF8"
                       },
                       {
                           "path":  "D:\\Projects\\GithubIssueWidget-build-20261119-02\\npm.log-review-needed.json",
                           "sha256":  "127B0859CBFD1C1AC1BB67E31242D0DC6340034D05ED0AC5EEFCE11DF90C7CAB"
                       },
                       {
                           "path":  "D:\\Projects\\GithubIssueWidget-build-20261119-02\\npm.protected-logs\\stdout.log",
                           "sha256":  "CCD84407B9A07765F902DDC23F3517F6952D23D6B41B36D4B3F9D2D1339288FF"
                       },
                       {
                           "path":  "D:\\Projects\\GithubIssueWidget-build-20261119-02\\npm.protected-logs\\stderr.log",
                           "sha256":  "960379AB2B6E16C97F4063BCD816D9DF4CC6C32CBCC2D926FAC49ACA2DC92829"
                       },
                       {
                           "path":  "D:\\Projects\\GithubIssueWidget\\docs\\agent-runs\\20261118-160129-mvp-restart\\tasks\\I02\\real-build-npm-log-approval-01.json",
                           "sha256":  "759AA996CE4FC9DC88573077F8C20C57432662F8C4DB6C28235A671C7559CE8D"
                       },
                       {
                           "path":  "D:\\Projects\\GithubIssueWidget-build-20261119-02\\release.result.json",
                           "sha256":  "2F339B7395902D13A3FCF42C23D3FE88E0A65D279714E0A9CF2AE3C684F0307E"
                       },
                       {
                           "path":  "D:\\Projects\\GithubIssueWidget-build-20261119-02\\release.before.json",
                           "sha256":  "D747AA1C2D1BF9A2ACAC9FD0CE55712062B58B714EEFCE4B6F3E796D56CBEDF8"
                       },
                       {
                           "path":  "D:\\Projects\\GithubIssueWidget-build-20261119-02\\release.after.json",
                           "sha256":  "D747AA1C2D1BF9A2ACAC9FD0CE55712062B58B714EEFCE4B6F3E796D56CBEDF8"
                       },
                       {
                           "path":  "D:\\Projects\\GithubIssueWidget-build-20261119-02\\release.log-review-needed.json",
                           "sha256":  "61CAC53E6701BE13697379C7EA6F6C7B80B78216E5B511A88CF7C2ABC9A7FB45"
                       },
                       {
                           "path":  "D:\\Projects\\GithubIssueWidget-build-20261119-02\\release.protected-logs\\stdout.log",
                           "sha256":  "8C615277D28A2E9C3A2B7CB3272E7C37B660F7B8678659C4EE19A00D7B88AD87"
                       },
                       {
                           "path":  "D:\\Projects\\GithubIssueWidget-build-20261119-02\\release.protected-logs\\stderr.log",
                           "sha256":  "35D143952405B01086E538741F7142D833882CBA626219E2160CC19D96647F71"
                       },
                       {
                           "path":  "D:\\Projects\\GithubIssueWidget\\docs\\agent-runs\\20261118-160129-mvp-restart\\tasks\\I02\\real-build-release-log-approval-01.json",
                           "sha256":  "DF528B1EBE829F9133C0687A5CB4D68C37310FA5754D5D337605A45063AF09CA"
                       }
                   ],
    "investigate_at":  2,
    "stop_all_at":  3,
    "direct_publication_violation_count":  2,
    "format_correction_check_failures":  2
}
```
