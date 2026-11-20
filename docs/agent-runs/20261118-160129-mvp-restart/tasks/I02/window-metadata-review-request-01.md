# I02 ウィンドウメタデータ独立レビュー依頼 01

担当 /root/plan_review。作者 /root/drag_design と独立。固定参照版を読み取り、対象版 SHA と対象 HWND/PID、実行方法・ハーネス、API・出力値・限界を根拠と照合する。ハーネスや生出力が保存されていなければ未確認と明記し、存在しない証拠を推測しない。事実・仮説・未検証、時点・PID再利用、DPI仮想化、属性と画素・非遮蔽・安全入力の違いを確認する。blocking と解消条件を列挙し、レビュー対象と SHA を明記。

別件 B03 は fixed build-verification-result-01.md に基づき、外側 Start-Process -Wait が戻らず npm/Cargo の終了コード未取得、原因未確定であることを維持。既存セッションやビルドを再実行せず、ログと既存スクリプトの読み取りで、終了コードの取得を可能にする異なる具体的な方式（プロセス所有・終了待ち・stdout/stderr 消費・タイムアウト・終了コードの紐付け・版/証跡固定）を比較。待機原因は仮説とし、事実と分ける。専用 scratch、CPU/メモリ/ディスク、環境、ポート、デスクトップとの干渉、秘密・既存資産保護と通常副作用を区別し、画面作業との並行可否を前提条件とともに評価。実行計画・リスクと不足証拠を提示し、実行自体はしない。

CUA 画面取得、getState、browser inventory、入力、ビルド、プロセス変更、製品変更、外部送信禁止。生存中 app/server は所有者のみ扱う。読み取り不能・版不一致・所有不明なら該当作業を保留。

結果は tasks/I02/window-metadata-review-01.md。参照先実体・担当と SHA を照合し、承認済み公開方式で一時名から非上書き確定、パスのみ通知。既存物を保護。

同種2回で該当操作を中断し別担当が客観調査、同種3回で全作業停止。画像不一致・browser inventory エラー・直接公開逸脱・依頼形式訂正の検査停止は各2回で調査済み。各々同種3回目は直ちに全停止。

## 管理データ

```json
{
    "assignee":  "/root/plan_review",
    "author":  "/root/drag_design",
    "references":  [
                       {
                           "path":  "D:\\Projects\\GithubIssueWidget\\docs\\agent-runs\\20261118-160129-mvp-restart\\tasks\\I02\\window-metadata-result-01.md",
                           "sha256":  "6C80CF283FE83F320A6968C3F0F9DED35DBE3BC5406040DDB7E2C772940B451C"
                       },
                       {
                           "path":  "D:\\Projects\\GithubIssueWidget\\docs\\agent-runs\\20261118-160129-mvp-restart\\tasks\\I02\\window-metadata-request-03.md",
                           "sha256":  "E92A1B51CD06AD09EBFBA8A7B0DD2FAC7D157158045CA20E662ED0A1DEE03890"
                       },
                       {
                           "path":  "D:\\Projects\\GithubIssueWidget\\docs\\agent-runs\\20261118-160129-mvp-restart\\tasks\\I02\\marker-execution-result-01.md",
                           "sha256":  "0443CCB85EC2F1537C83AC1981775DEDA01EB8BD2A942BE81EF3ECFD2F405F71"
                       },
                       {
                           "path":  "D:\\Projects\\GithubIssueWidget\\docs\\agent-runs\\20261118-160129-mvp-restart\\tasks\\I02\\build-verification-result-01.md",
                           "sha256":  "7A8AF3B60D32D2AC94888801067C73031DB8A84974DA95E6373D03C03E68A098"
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
