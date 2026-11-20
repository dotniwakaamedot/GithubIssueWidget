# I02 検証方法設計の独立レビュー 01

- run_id / task_id / review_revision / 回数: 20261118-160129-mvp-restart / I02-verification-design-review / 01 / 1
- owner_role / owner_agent_id: verification_reviewer / /root/plan_review。作者 /root/verification_design とは別担当。
- 対象 `tasks/I02/verification-design-result-01.md` SHA256 E821627AF74FF335D29433BF47F8682D36C2BF970B18036365495769F0153B4D は依頼と実体が一致し、レビュー終盤にも再照合して一致。
- 参照実体の SHA256 は、依頼 `verification-design-request-02.md` DBB1F63472016EB0AC1B14CD9E401A342C7B8A45C20C61A2B3F65D12A6991C71、`combined-review-01.md` F6DF723A067C988F144087B6A4833EDD306AAB56071068894424D359F70004D3、`result-02.md` 520044FE6138609B99595C5FAF1D3973DD2D62EB82F27FE90392568C9599ED13、`drag-result-01.md` B34245F943AD4B9E69F5DBC8020C762499055A2ABDE878F57446131038C5CCB6、`management-04.md` 0D569B792156CE706ACF0BF58295EBBFEE10D7FFE66434D09E36975B419F0239、`plan-02.md` A6CB37DC369D69B334564235CD00C3AF284250109D4A56BE63C6CC7B104FE990 と一致。
- 対象 worktree `D:\Projects\GithubIssueWidget-issue-2-20261118-160129` の branch `task/issue-2-foundation-20261118-160129`、HEAD a4ac97286785a2a5ba0d070a909b58003fc49ff1 は依頼と一致。対象の製品17パスは基点・相対パスを分離し全件存在確認後に SHA256 を照合して一致。status は README 変更と widget 未追跡16件のみ、削除なし。
- 判定: **要修正**。対象設計版は承認しない。実機操作、製品、外部更新の許可を意味しない。

## 要求ごとの確認

| 要求 | 設計と根拠 | 判定・未確認 |
| --- | --- | --- |
| B02 実寸・中央・最前面・リサイズ | HWND/PID、画面矩形・可視枠・クライアント・DPI・モニター、topmost の OS 状態と UI 表示を対応付け、8方向・最小制約を別途安全審査後に測る。Tauri のサイズ契約、座標・DPI の未確認は合格にしない | 検証案として妥当。ハーネスの実行可能性、具体的入力が位置変更試行や他作業に干渉しないことは未確認 |
| B03 対象版・ログ・終了コード・開発起動・build | stdout/stderr と実プロセス ExitCode、環境、対象17 hash の前後照合、exe hash、専有・直列化を設計 | **B01 blocking**。共有 `target` の分離だけでは既存の他の生成物の保護を保証しない |
| B04 実 IPC・許可拒否・既定ブラウザ | main WebView の識別、非 mock IPC、許可・拒否、既定関連付け、ブラウザの副作用・隔離、未知の前提は未確認とする | 検証案として妥当。接続可能性・実際の実行安全性、ACL エラー由来の識別、ブラウザのプロセス識別は未実証 |
| 並行性・失敗・対象版 | 同じ window・port・target・profile を専有、競合は直列。hash 不一致・副作用不明を中断、同種2回で操作停止・独立調査、3回で全停止 | 適合。各担当 ID・専有領域は実施依頼時に確定する必要がある |
| B01 移動 | 原因確定前の同じタイトル移動再試行を禁止、診断は別途とする | 適合。元の Issue の移動動作は依然未達 |

## 指摘

| ID | 種別 | 根拠・影響 | 是正条件 |
| --- | --- | --- | --- |
| B01 | blocking | 共通条件は既存 ignored 生成物を変更・削除しないと定める一方、B03 は対象 `widget/` で build を行い「別の検証対象領域または作業用の新しいビルド出力先」を選ぶ案を残す。対象 `src-tauri/tauri.conf.json` の `beforeBuildCommand` は `npm run build`、`package.json` は `vite build`、`frontendDist` は `../dist`。現在の `vite.config.ts` に出力先変更はなく、Cargo の target だけを別指定しても既存 `widget/dist` が書き換わり得る。その他の生成先や依存キャッシュも同様に実測・確認されていない。 | 検証 run の全書込み先（少なくとも frontend dist、Cargo target、Tauri 生成物、ログ、一時物と依存取得先）を事前に特定し、既存保護物と重ならない不変ソースの専用コピー等で build/dev を行う具体案を明記する。コピー元との全 hash 対応、依存・設定差、実行バイナリと元の対象版の対応、所有・保持も記録し、隔離を確認できなければ未実施とする。 |

## 実施した確認・限界

- cwd `D:\Projects\GithubIssueWidget`。まず共有 run 絶対基点を用いて依頼の7参照の実在を確認し、各 SHA256 を実体から計算。対象設計・依頼・管理記録を `Get-Content -Encoding UTF8` で確認し、設計 hash を最後に再計算。各コマンド群終了コード 0。
- 対象 worktree の17ファイルは `result-02.md` の製品欄のみを解析し、全件存在確認後 `Get-FileHash -Algorithm SHA256`。`git -C <worktree> branch --show-current`、`rev-parse HEAD`、`status --short --untracked-files=all` の結果を確認。各終了コード 0。
- Issue #1/#2 の最新本文を `gh issue view` で取得（各終了コード 0）。対象の package・vite・Tauri 設定を読取。[Tauri WebDriver](https://v2.tauri.app/develop/tests/webdriver/) と [GetWindowRect](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getwindowrect) の一次資料を参照。Windows 対応は文書上の事実でありこの環境での接続成功ではない。
- UI・デスクトップ・入力・ドラッグ・通信を伴う試験、build、対象アプリ起動、既存生成物の変更は行っていない。設計の各ハーネス・WebDriver・既定ブラウザ照会の実行可能性は未検証。今回のレビュー担当の失敗なし。
- 親ディレクトリ存在・確定先と tmp の不存在を確認し、UTF-8 往復と代表文字の一致後に確定。設計の修正版は新しい SHA256 を独立にレビューする。