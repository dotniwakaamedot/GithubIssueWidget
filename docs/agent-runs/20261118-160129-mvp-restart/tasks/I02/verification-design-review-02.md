# I02 検証方法設計の独立レビュー 02

- run_id / task_id / review_revision / 回数: 20261118-160129-mvp-restart / I02-verification-design-review / 02 / 2
- owner_role / owner_agent_id: verification_reviewer / /root/plan_review。作者 /root/verification_design とは別担当。
- 対象 `tasks/I02/verification-design-result-02.md` SHA256 971D1BE06A8D735AB75C57B5398F2E667D0F288D991B34F473D15080140A2926 は依頼と実体が一致。開始後・終了直前に再照合した。
- 参照実体 SHA256: `verification-design-request-03.md` 7CB095F5217F840136A668AE9F8726457259DEEC40D3A26F2FAB06719C38D6AE、`verification-design-result-01.md` E821627AF74FF335D29433BF47F8682D36C2BF970B18036365495769F0153B4D、`verification-design-review-01.md` 829758054D5DF5B0D8D920CF56F46D8EE58725D07200B95C269CABE486A6D6EE、`combined-review-01.md` F6DF723A067C988F144087B6A4833EDD306AAB56071068894424D359F70004D3、`result-02.md` 520044FE6138609B99595C5FAF1D3973DD2D62EB82F27FE90392568C9599ED13、`management-04.md` 0D569B792156CE706ACF0BF58295EBBFEE10D7FFE66434D09E36975B419F0239、`plan-02.md` A6CB37DC369D69B334564235CD00C3AF284250109D4A56BE63C6CC7B104FE990。依頼値と全件一致。
- 対象 worktree `D:\Projects\GithubIssueWidget-issue-2-20261118-160129` の branch `task/issue-2-foundation-20261118-160129`、HEAD a4ac97286785a2a5ba0d070a909b58003fc49ff1 は依頼と一致。前版製品一覧から17パスを基点・相対パスに分離し全件の存在確認後に SHA256 を照合、全件一致。status は README 変更と widget 未追跡16件のみ、削除対象なし。
- 判定: **対象 SHA256 の設計案に限り承認**。実行許可、実行可能性の実証、製品完了の承認ではない。blocking 指摘なし。

## 要求ごとの確認

| 要求 | 独立確認 | 判断・未確認 |
| --- | --- | --- |
| 前版設計 B01: 全書込み先隔離 | 元 worktree での build/dev を禁止し、既存資産に書き込めない専用 Windows 検証環境、VM 内のコピーと frontend dist、target、Tauri gen、cache、temp、evidence、OS/profile/registry を分離。リンク・共有 mount・ネットワーク・予定外書込みに対する停止条件を指定 | **設計上は是正**。前版と同じ指摘の再発なし。隔離、VM、オフライン依存、監視・制限アカウントの利用可能性は未確認で、不明なら実行しない |
| 元版・コピー・生成物の対応 | 元とコピー各17件、転送物・依存・lock・ツール・コマンド・exit code・exe hash を manifest で対応。元 exe と同一バイナリと主張せず、前後 hash と並行書込み禁止・ACL/監視を併用 | 適合。依存・設定差、監視方式、再現性は検証前提であり実績ではない |
| B02 実寸・中央・最前面・リサイズ | 前版 B02 の HWND/PID・矩形・DPI・モニター・各方向・OS 状態と安全審査を維持。VM 上の画面測定を Windows 11 実機の成功と扱わない | 適合。具体的測定・実機の安全性は未実施 |
| B03 開発起動・release build | 隔離コピーで別 run とし、環境・実コマンド・stdout/stderr・実プロセス exit code・起動ウィンドウ・正常終了を元版の manifest に対応 | 適合。offline 依存と VM 準備、実ビルドは未実施。失敗・強制終了を成功扱いしない |
| B04 実 IPC・既定ブラウザ・拒否 | 前版の非 mock IPC、実際の許可・拒否、既定ブラウザ照合、隔離・副作用評価を維持。ネットワーク遮断下の拒否だけでは許可 URL の実遷移を証明しない | 適合。WebDriver・OS 関連付け・実ネットワーク環境の実行可能性は未確認 |
| B01 移動・停止・並行 | 原因未確定時の同じ移動禁止、同種2回で該当操作停止・別担当調査、3回で全停止を継続。共有 VM・port・画面・ブラウザ・生成物は直列 | 適合。移動機能は未達、具体担当と保管先は実施前に割当が必要 |

## 指摘と注意

blocking および新規 non-blocking 指摘なし。前版の指摘は設計上解消されたが、検証環境や監視の存在を証明したものではない。設計の実行は個々の前提・所有・認可を管理が確認した後に判断し、前提が不明なら未実施を維持する。Issue #2 の完了条件 B01〜B04 は本レビューでは解決していない。

## 実施した確認・限界

- cwd `D:\Projects\GithubIssueWidget`。依頼、対象、前版、レビューの7参照と対象について、共有 run の絶対基点を用いて実在確認し `Get-FileHash -Algorithm SHA256` で依頼値と照合。`Get-Content -Encoding UTF8` で修正版・依頼を読取。各コマンド群終了コード 0。
- 前版レビュー時に対象 `package.json`、`vite.config.ts`、`src-tauri/tauri.conf.json` を確認し、Vite の dist が Cargo target と別である指摘の根拠を確認済み。今回の修正版がこの点を含む全書込み先を候補とし、未確認を中止条件にすることを確認。
- 対象 worktree に対し結果一覧の製品欄のみを解析、全17パスの `Test-Path` 成功後に hash を照合。`git -C <worktree> branch --show-current`、`rev-parse HEAD`、`status --short --untracked-files=all` で変更・削除を確認。各終了コード 0。
- Issue #1/#2 は直前の設計レビュー時に GitHub 本文を取得して照合。今回実機操作・アプリ起動・通信を伴う試験・build・VM・依存準備・既存証拠や製品の変更・GitHub 更新は行っていない。
- 親ディレクトリ存在・確定先と tmp の不存在を確認し、UTF-8 往復・代表文字確認後に確定。本レビュー担当の失敗なし。変更後の版に旧承認は使わない。