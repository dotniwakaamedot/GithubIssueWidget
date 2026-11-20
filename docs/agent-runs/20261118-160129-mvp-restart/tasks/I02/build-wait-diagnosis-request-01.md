# I02 B03 終了待機の独立調査依頼 01

担当 /root/build_wait_diagnosis。既存結果とその固定証跡・スクリプトを読み取り、外側 Start-Process -Wait が戻らなかった事実と、実際の npm/Cargo 終了コードが未取得で B03 未完了であることを照合する。実行・停止済みセッション、既存 scratch や成果、診断 app/server を保護する。対象17ファイルの版は HEAD のみでなく参照 manifest と実体 SHA を区別する。

プロセスの終了待ち、標準出力・標準エラーの消費、子孫プロセス、シェルと終了コードの伝播などについて保存済み証拠から言える事実と仮説、未保存の証拠を区別する。元方式を再実行せず、.NET Process の handle と stdout/stderr 同時非同期消費を含む別方式を検討する。最初に無害な専用小規模試験で終了・出力・終了コード・タイムアウトを検証する具体案、その後の本ビルド適用条件、証跡と失敗停止条件を提示する。今回は試験・ビルド・新規プロセス起動を行わない。

新しい専用 scratch・target・dist・cache・temp、元資産・秘密・既存証拠の保護、通常 OS 副作用、ネットワーク接続、CPU・メモリ・ディスク、ポート・画面への影響を示し、画面診断と並行できる条件と未確認を評価。操作権限や所有が不明なら該当操作を保留。画像取得・入力・CUA getState/browser inventory・画面/プロセス操作・製品変更・外部送信は禁止。

結果は tasks/I02/build-wait-diagnosis-result-01.md。承認済み公開方式で新規一時名から非上書き確定し、版・実体 SHA・限界を記しパスだけ通知。既存レビュー結果の公開操作はポリシーで拒否され未確定であり、その再試行・回避は行わない。

同種2回で該当操作を止め別担当が客観調査、同種3回で全作業停止。画像不一致・browser inventory エラー・直接公開逸脱・依頼形式訂正検査停止は各2回で調査済み、それぞれ同種3回目は全停止。既存失敗操作を盲目的に再試行しない。

## 管理データ

```json
{
    "assignee":  "/root/build_wait_diagnosis",
    "references":  [
                       {
                           "path":  "D:\\Projects\\GithubIssueWidget\\docs\\agent-runs\\20261118-160129-mvp-restart\\tasks\\I02\\build-verification-result-01.md",
                           "sha256":  "7A8AF3B60D32D2AC94888801067C73031DB8A84974DA95E6373D03C03E68A098"
                       },
                       {
                           "path":  "D:\\Projects\\GithubIssueWidget\\docs\\agent-runs\\20261118-160129-mvp-restart\\tasks\\I02\\verification-design-result-03.md",
                           "sha256":  "B2B552301D897717CC80404A64297C460BADED4C8AE4B0F130B1557F7B08DAED"
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
