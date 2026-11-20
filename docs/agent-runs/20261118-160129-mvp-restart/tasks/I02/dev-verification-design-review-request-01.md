# I02 開発起動・正常終了計画の独立レビュー依頼 01

担当 /root/process_test_review、作者 /root/build_verification と独立。固定版 design-01、依頼、release 結果・独立確認、manifest と対象17ファイルの実体 SHA を照合し、対象と残条件を特定。専用 scratch・コピー・対象 debug exe と release exe の違い、生成物と既存資産・診断 session の所有を検討。

既定 identifier、Windows 既定 WebView profile の共有可能性と Tauri の appDirectoriesOverride・WEBVIEW2_USER_DATA_FOLDER・registry policy の実効条件、別設定が有効と確認できる方法を一次資料とコードで照合。設定 overlay を使う場合に検証できること・既定設定では未検証のこと・B03 完了条件との対応を明記し、観測できない項目を推測しない。

dev command・別 port の必要性と実際の接続先、プロセスと window の対象同定、終了可能性と CloseMainWindow の条件・戻り値・対象実子 exit/EOF と Node/Vite/子孫の独立した終了状態を具体的に検討。既存 1435 と対象候補 1420 の競合、画面・焦点・秘密・資源・ネットワークの副作用、事前確認と各失敗時の同一 handle 保持・停止範囲を評価し、実行方法が未具体なら blocking を示す。

画像不一致・browser inventory・JSON 件数問題は各2回で独立調査済み。失敗した画像取得・getState/browser inventory・配列化構文を再試行しない。拒否された別の window metadata review 公開操作は対象外で再試行・回避しない。読み取り以外の build・起動・UI・入力・process 停止・外部更新は禁止。

対象版 SHA・根拠・blocking と解消条件・未確認を tasks/I02/dev-verification-design-review-01.md に公開プロトコルで非上書き確定しパスのみ通知。承認は実行許可ではなく、具体的 driver と設定版のレビュー、親判断、実施直前の所有と競合確認が別途必要。既存成果・他者プロセスを保護。

同種2回で該当操作を中断し別担当が客観調査、初回含む同種3回で全作業停止。直接公開逸脱・依頼形式訂正検査停止も各2回で調査済み、各同種3回目は全停止。

## 管理データ

```json
{
    "assignee":  "/root/process_test_review",
    "author":  "/root/build_verification",
    "references":  [
                       {
                           "path":  "D:\\Projects\\GithubIssueWidget\\docs\\agent-runs\\20261118-160129-mvp-restart\\tasks\\I02\\dev-verification-design-01.md",
                           "sha256":  "B8563D57E131E8D70CD6307270BAF394084B4C1B297BA1D29A19EF694190F037"
                       },
                       {
                           "path":  "D:\\Projects\\GithubIssueWidget\\docs\\agent-runs\\20261118-160129-mvp-restart\\tasks\\I02\\dev-verification-design-request-01.md",
                           "sha256":  "6267BAA463643AB6C80D279844CD2AAAE818F8504D8A68F13134BABE359B38F1"
                       },
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
                           "path":  "D:\\Projects\\GithubIssueWidget\\docs\\agent-runs\\20261118-160129-mvp-restart\\questions\\I02-window-metadata-question-02.md",
                           "sha256":  "AC284337BFC0AA4718674B973D3BE129679800B3410CE987836B90AA43E394FE"
                       },
                       {
                           "path":  "D:\\Projects\\GithubIssueWidget\\docs\\agent-runs\\20261118-160129-mvp-restart\\process\\evidence-publication-protocol-01.md",
                           "sha256":  "65E36A266503EB12E20B4045BC7D2A346C8F72B97BA3ED3EF73A3B80B36B3382"
                       },
                       {
                           "path":  "D:\\Projects\\GithubIssueWidget-build-20261119-02\\source-manifest.json",
                           "sha256":  "BB5253F1D02C83E183C86A6981CA3768A01892E3D1B40EF3AC4AA6A014DE0033"
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
