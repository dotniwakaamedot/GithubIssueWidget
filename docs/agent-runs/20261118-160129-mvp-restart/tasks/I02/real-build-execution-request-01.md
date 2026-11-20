# I02 実ビルド実行依頼 01

担当 /root/build_verification。親は実行前条件が満たされる場合に限り、独立レビュー02で承認された design-02 と driver-03 による専用領域の実ビルドを指示。対象・所有・条件を確認できなければ開始せず質問する。元 product worktree・旧 scratch・他者プロセス・証拠を保護。

開始直前および各 stage 前に固定参照・driver と toolchain executable の絶対パス・SHA、manifest の17件について元とコピーの全存在・実体 SHA、基点内・reparse/hardlink・権限・既存出力/一時名、専用 target/dist/cache/temp の所有と非重複を確認。前後 hash は途中変更の不存在を保証しない。開始条件・所有が不明なら停止。

公開取得先・実際の設定と必要な依存、資格情報が意図せず渡されないこと、ディスク・メモリ・CPU・既存 app/server と画面担当との資源競合を確認。環境変数は完全な秘密保護境界ではなく OS 共通資源やネットワークの副作用がある。競合や不明点があれば停止し親へ質問。GUI・dev 起動・元 worktree の書込みはしない。

rustup、npm、release を設計順に別所有セッションで実行。各 stage は同一 Process handle で PID・時刻・実 exit・両 stream EOF と保護ログの hash、対象版と成果の証跡を結ぶ。前 stage の成功とログの独立確認・receipt・確定結果、対象17件の前後一致を確認するまで次に進まない。npm/Cargo 等の個別 exit は直接取得できなければ推定しない。

両ログを保護領域に保持。自動検査が clear でも秘密不存在の保証ではないため、独立担当による必要最小限の確認・親の判断・対象 stage/driver/log hash を結ぶ receipt と同一 session への publish 指示まで確定名へ公開しない。値やログ断片を共有 run・会話に出さない。検出・読取不明・hash 不一致は保持し親へ質問。

timeout、EOF 待ち、fault/cancel、起動成否不明、入力途絶、予期しない書込み・ログ保護失敗では同じ stage を再実行せず、取得できた同一 handle・所有セッション・部分証拠を保持して親に報告。親判断前に continue/publish/停止しない。100 MiB は観測閾値で厳密な上限ではないため稼働中の容量状態も確認。

質問は questions/I02-real-build-execution-question-01.md から空き番号、最終結果は tasks/I02/real-build-result-01.md。必要な証拠を秘匿しながら対象版・コマンド・終了状態・EOF・hash・未確認を記録し、公開プロトコルで確定後パスのみ通知。結果は別担当が独立レビューする。拒否された window metadata review の公開操作は再試行・回避禁止。

同種2回で該当操作を中断し別担当が客観調査、同種3回で全作業停止。画像不一致・browser inventory エラー・直接公開逸脱・依頼形式訂正検査停止は各2回で調査済み、各同種3回目は全停止。

## 管理データ

```json
{
    "assignee":  "/root/build_verification",
    "reviewer":  "/root/process_test_review",
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
                           "path":  "D:\\Projects\\GithubIssueWidget\\docs\\agent-runs\\20261118-160129-mvp-restart\\tasks\\I02\\real-build-driver-review-02.md",
                           "sha256":  "708128F1D84F95059A505BEAB38C8E0A70E575AD845F3D6A993C6E4242B7EDAD"
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
