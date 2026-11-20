# I02 実ビルド用ドライバー修正版の独立レビュー依頼 02

担当 /root/process_test_review、作者 /root/build_verification と独立。固定版 design-02 と driver-03、旧版・B01/B02・manifest・参照の実体と SHA を照合。17対象の元と専用コピーは基点・相対パス・期待 hash を分離し、全存在確認後に照合。対象版が変われば旧承認を流用しない。

B01: 保護ディレクトリの ACL・所有・リンク、両一時ログ、秘密パターンの走査と限界、不明/検出時の保護、独立した内容確認、receipt の担当・hash・承認と公開前ゲートをコード実体と照合。機密をレビュー結果・画面に出さない。パターン一致なしを秘密不存在と断定しない。

B02: 各非同期 task の pending・正常 EOF・fault・cancel、実行中/timeout/EOF 待機と VS bootstrap、例外・起動成否不明・ログ書込み失敗・入力途絶を照合。実プロセス handle・task・一時ログの保持と追跡、再起動防止、continue の受理条件を具体的に検証。保持できない場合を成功や EOF にしない。

実 build の利用可能性として各 stage・依存・シェルと子孫・exit 伝播、環境と所有、ポート・画面・CPU/メモリ/容量の競合、通信・通常 OS 副作用、実行前条件、exe/ログ/17件版の結合を評価。実行されていない API・保護・例外経路や他環境の非競合は未検証とする。

対象 SHA、事実、blocking と解消条件、未確認を tasks/I02/real-build-driver-review-02.md に公開プロトコルで確定しパスのみ通知。実行承認は親が別途判断。読み取りのみ、driver/build・依存導入・ネットワーク取得・GUI・入力・process 停止/起動・製品変更は禁止。既存成果を保護。過去に拒否された window metadata review の公開操作は再試行・回避しない。

同種2回で該当操作を中断し別担当が客観調査、同種3回で全作業停止。画像不一致・browser inventory エラー・直接公開逸脱・依頼形式訂正検査停止は各2回で調査済み、各同種3回目は全停止。

## 管理データ

```json
{
    "assignee":  "/root/process_test_review",
    "author":  "/root/build_verification",
    "references":  [
                       {
                           "path":  "D:\\Projects\\GithubIssueWidget\\docs\\agent-runs\\20261118-160129-mvp-restart\\tasks\\I02\\real-build-driver-design-02.md",
                           "sha256":  "0D6F42F22537F7DE5C943BC0E53AD98795227EE5719A20FDD7476EB505B17E21"
                       },
                       {
                           "path":  "D:\\Projects\\GithubIssueWidget\\docs\\agent-runs\\20261118-160129-mvp-restart\\tasks\\I02\\real-build-driver-design-01.md",
                           "sha256":  "F4D173AAEF4964FB8C1B46B2A9BD4D71E16B4DDCD7EDAFB45651B79D6BA9E50D"
                       },
                       {
                           "path":  "D:\\Projects\\GithubIssueWidget\\docs\\agent-runs\\20261118-160129-mvp-restart\\tasks\\I02\\real-build-driver-review-01.md",
                           "sha256":  "1738E469B6F0D3D974F27CD4BA1E5784896AD911BAA171A8BB8F636AEBF8E2DF"
                       },
                       {
                           "path":  "D:\\Projects\\GithubIssueWidget\\docs\\agent-runs\\20261118-160129-mvp-restart\\tasks\\I02\\real-build-driver-revision-request-01.md",
                           "sha256":  "B95B676EF8051623B4A41F72B4639E7FD51C0BD73C69ABC660BA352F93863B64"
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
                           "path":  "D:\\Projects\\GithubIssueWidget-build-20261119-02\\build-driver-02.ps1",
                           "sha256":  "6B0E8CEB7C86C52E06982A8FB83A024E34EEE18C226B285319777F68608EC078"
                       },
                       {
                           "path":  "D:\\Projects\\GithubIssueWidget-build-20261119-02\\source-manifest.json",
                           "sha256":  "BB5253F1D02C83E183C86A6981CA3768A01892E3D1B40EF3AC4AA6A014DE0033"
                       },
                       {
                           "path":  "D:\\Projects\\GithubIssueWidget-build-20261119-02\\copy-manifest.json",
                           "sha256":  "BCE67077C4CB323586F52E527318B6584B3D48BBC956E879B75774ED6F5482E0"
                       }
                   ],
    "investigate_at":  2,
    "stop_all_at":  3,
    "direct_publication_violation_count":  2,
    "format_correction_check_failures":  2
}
```
