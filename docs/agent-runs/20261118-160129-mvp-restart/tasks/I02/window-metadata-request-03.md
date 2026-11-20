# I02 ウィンドウ OS メタデータ調査依頼 03

担当 /root/drag_design。対象は本依頼の固定参照版。共有 run に新規結果・質問を記録する範囲を所有し、既存プロセス・成果物を保護する。

画像と AX の不一致は2回で、画像自体・取得範囲は未保存、原因未確定。所有セッションと対象プロセスの生存、PID・開始時刻・実行パス・HWND の対応を読み取り確認する。公開 OS API の安全な読み取り方法が明確な範囲で、ウィンドウの最小化・可視属性・DWM cloak・session・desktop・DPI・必要な座標メタデータを確認し、根拠・時刻・値・失敗・不明を記録する。取得不能を推測で補わず、対象同定や副作用が不明なら停止して質問。可視属性は画素の可視性や非遮蔽の証明ではない。機密や他窓の内容は取得しない。

CUA 画面再取得、getState、ブラウザ inventory、クリック、ドラッグ、フォーカス・ウィンドウ変更、プロセス終了・再起動、製品変更、外部送信は禁止。画面入力は未承認。

結果は tasks/I02/window-metadata-result-01.md、質問は questions/I02-window-metadata-question-01.md。事実・仮説・未確認、対象版 SHA、手法、観測と残課題を区別する。既存成果を上書きせず固定参照の承認済み公開方式を用い、確定後にパスのみ通知。

同種2回で該当操作を中断し別担当が客観調査、同種3回で全作業中止。画像不一致・browser inventory エラー・直接公開逸脱はそれぞれ2回で独立調査済み。依頼形式訂正の検査停止も2回で独立調査済み。同種3回目は直ちに全停止。旧 request-01 は保持。

## 管理データ

```json
{
    "assignee":  "/root/drag_design",
    "references":  [
                       {
                           "path":  "D:\\Projects\\GithubIssueWidget\\docs\\agent-runs\\20261118-160129-mvp-restart\\tasks\\I02\\marker-execution-result-01.md",
                           "sha256":  "0443CCB85EC2F1537C83AC1981775DEDA01EB8BD2A942BE81EF3ECFD2F405F71"
                       },
                       {
                           "path":  "D:\\Projects\\GithubIssueWidget\\docs\\agent-runs\\20261118-160129-mvp-restart\\questions\\I02-screenshot-execution-question-02.md",
                           "sha256":  "9CF8EE470585DA27A186130B4714BB4E1C6F083E52196C4E2771E09FFD0419BD"
                       },
                       {
                           "path":  "D:\\Projects\\GithubIssueWidget\\docs\\agent-runs\\20261118-160129-mvp-restart\\questions\\I02-screenshot-execution-question-03.md",
                           "sha256":  "F6E54E8A1A44B3522723756192CCA6999AAACE4B327E90CEB812B7CA78183B55"
                       },
                       {
                           "path":  "D:\\Projects\\GithubIssueWidget\\docs\\agent-runs\\20261118-160129-mvp-restart\\process\\evidence-publication-protocol-01.md",
                           "sha256":  "65E36A266503EB12E20B4045BC7D2A346C8F72B97BA3ED3EF73A3B80B36B3382"
                       }
                   ],
    "screenshot_mismatch_count":  2,
    "browser_inventory_failure_count":  2,
    "direct_publication_violation_count":  2,
    "format_correction_check_failures":  2,
    "investigate_at":  2,
    "stop_all_at":  3
}
```
