# I02 開発起動・正常終了の検証計画依頼 01

担当 /root/build_verification。独立レビュー済み release は対象17ファイルの専用コピーと直接子 exit 0 に限る。B03 は dev 起動・正常終了が未達。今回の範囲は具体的な計画と必要な読み取り調査に限定し、起動・build・入力・process 停止・外部変更を行わない。対象版と既存成果・他者の作業を保護。

対象17ファイルの元と専用コピー、証跡、exe と SHA、コマンド・ツール・環境を識別し、dev 起動から実際の exe/PID・ウィンドウとの対応、正常な終了と exit/ログを検証する実行可能な方法を設計。サーバーと GUI/子孫を区別し、ユーザー操作やウィンドウ内容・秘密を収集せずに示せることと未証明を明示。終了方法は所有対象・副作用・正常終了の意味を定め、原因不明や所有不一致時の停止と証拠保護を具体化。

別の専用コピー・target/dist/cache/temp・port を設計し、既存診断 app/server とその所有セッション、既存 scratch・元製品への書込みを避ける。対象版・各出力とその親、競合、空き資源、ネットワークと通常 OS 副作用、既存 user アプリの保護、停止前後の所有同定と予期しない出力・子孫・timeout 時の同一 handle の保持を明記。ウィンドウが必要な場合、実行前に対象・他担当・現デスクトップの競合を実際に確認し、不明なら実行しない。

画像と AX の不一致2回、browser inventory エラー2回、JSON 件数問題2回は各独立調査済み。該当する画像取得・getState/browser inventory・失敗した配列化構文を再実行せず、他方式も具体的根拠と停止条件を示して独立レビューと親判断を受ける。CUA 禁止操作、以前に自動ポリシー拒否された window metadata review の公開操作の再試行・回避は禁止。同種3回目は全停止。

計画の所有・作業場所・固定対象 SHA・作業手順・出力先・証跡・条件・失敗時対応・外部副作用・残リスク・検証限界を tasks/I02/dev-verification-design-01.md に記録。質問は questions/I02-dev-verification-question-01.md から空き番号。公開手順で非上書き確定しパスのみ通知。独立レビューと親判断まで実行しない。

同種2回で該当操作を中断し別担当が客観調査、初回含む同種3回で全作業停止。直接公開逸脱・依頼形式訂正検査停止も各2回で独立調査済み、各同種3回目は全停止。

## 管理データ

```json
{
    "assignee":  "/root/build_verification",
    "references":  [
                       {
                           "path":  "D:\\Projects\\GithubIssueWidget\\docs\\agent-runs\\20261118-160129-mvp-restart\\tasks\\I02\\real-build-result-review-01.md",
                           "sha256":  "2CD96248A50CE4D0A76D4F10688A1CFEA73844C58C0A56BD7027150A8DA97F51"
                       },
                       {
                           "path":  "D:\\Projects\\GithubIssueWidget\\docs\\agent-runs\\20261118-160129-mvp-restart\\tasks\\I02\\real-build-result-01.md",
                           "sha256":  "D4056E537530E340B9556C82503D55FE40236FFE49471D429E1F3D2575E837D3"
                       },
                       {
                           "path":  "D:\\Projects\\GithubIssueWidget\\docs\\agent-runs\\20261118-160129-mvp-restart\\tasks\\I02\\result-02.md",
                           "sha256":  "520044FE6138609B99595C5FAF1D3973DD2D62EB82F27FE90392568C9599ED13"
                       },
                       {
                           "path":  "D:\\Projects\\GithubIssueWidget\\docs\\agent-runs\\20261118-160129-mvp-restart\\tasks\\I02\\marker-execution-result-01.md",
                           "sha256":  "0443CCB85EC2F1537C83AC1981775DEDA01EB8BD2A942BE81EF3ECFD2F405F71"
                       },
                       {
                           "path":  "D:\\Projects\\GithubIssueWidget\\docs\\agent-runs\\20261118-160129-mvp-restart\\questions\\I02-window-metadata-question-02.md",
                           "sha256":  "AC284337BFC0AA4718674B973D3BE129679800B3410CE987836B90AA43E394FE"
                       },
                       {
                           "path":  "D:\\Projects\\GithubIssueWidget\\docs\\agent-runs\\20261118-160129-mvp-restart\\tasks\\I02\\verification-design-result-03.md",
                           "sha256":  "B2B552301D897717CC80404A64297C460BADED4C8AE4B0F130B1557F7B08DAED"
                       },
                       {
                           "path":  "D:\\Projects\\GithubIssueWidget\\docs\\agent-runs\\20261118-160129-mvp-restart\\process\\evidence-publication-protocol-01.md",
                           "sha256":  "65E36A266503EB12E20B4045BC7D2A346C8F72B97BA3ED3EF73A3B80B36B3382"
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
