# I02 ウィンドウ OS メタデータ調査依頼 01

担当: /root/drag_design。共有 run の tasks/I02 内の本依頼対象。作業範囲は読み取り調査と新規結果・質問ファイルのみ。他担当のファイル・プロセス・診断資産を保護する。ユーザー原文と条件は固定参照の既存依頼を参照。

目的: 画像と AX の不一致２回について、所有中の診断プロセスと対象 HWND を再識別し、画像取得なしで OS メタデータを確認する。画像自体と取得範囲は保存されておらず、原因は未確定。独立調査は HWND 所有、最小化、可視、DWM cloak、session、desktop、DPI 等の読み取りによる切り分けを提案。可視属性は実画面での表示・非遮蔽を証明しない。

実施: 固定参照の版と自身の所有セッションをまず照合し、プロセスの生存・PID・開始時刻・実行パス・HWND の関係を読み取りで確認。適用可能な公開 OS API について根拠と成功・失敗・不明を記録し、HWND の最小化・表示属性・DWM cloak・session・desktop・DPI および必要な座標メタデータを取得できる範囲で記録。異なるセッションや取得失敗は推測で補完しない。環境に使える明確な読み取り手段が不明なら停止して質問する。機密情報や他窓の内容を取得・記録しない。

禁止: CUA による画面再取得、getState、ブラウザ inventory、クリック、ドラッグ、ウィンドウ変更、フォーカス変更、プロセス終了・再起動、製品変更、外部送信。対象同定・権限・API の副作用が不明なら該当操作を停止。入力の安全性は本調査では承認されない。

結果: 同ディレクトリの window-metadata-result-01.md、質問は共有 run の questions/I02-window-metadata-question-01.md に保存。事実・仮説・未確認、使用した手法と根拠、時刻、対象版 SHA、得られた値と失敗、残課題を区別。画像不一致の原因と画素の可視性は未確認のまま扱う。確定後は結果パスのみ通知。既存成果は上書きしない。保存は参照した承認済み公開方式を使用し、書込みや検証の不明時は後続操作を停止。

失敗管理: 同種２回で該当操作を中断して別担当が客観調査、同種３回で全作業停止し親へ連絡。画像不一致２回・browser inventory エラー２回・共有 run 直接公開逸脱２回は客観調査済みで、各々の同種３回目は全停止。再取得等で確認しない。依頼、参照版、担当不一致も判明すれば停止し報告。

## 固定参照と管理データ
`json
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
    "investigate_at":  2,
    "stop_all_at":  3
}
``r
