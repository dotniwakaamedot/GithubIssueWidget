# I02 診断窓の画像対応・観測方法の独立レビュー 01

- run_id / task_id / review_revision: 20261118-160129-mvp-restart / I02-screenshot-observation-review / 01
- owner_role / owner_agent_id: verification_reviewer / /root/plan_review。作者 /root/screenshot_design、process 所有者 /root/drag_design とは別担当。
- 対象 `tasks/I02/screenshot-observation-design-result-01.md` SHA256 4D33B22968BCF4FBB6930C095B56480A87C47E1F2A55406AD9D97485E0539F79。依頼の構造化参照9件について基点・相対パス・期待 SHA を分離し、全件存在確認後に実体 SHA を照合、全件一致。
- 判定: **要修正**。候補を観測・診断の案として示し安全条件を保っているが、元の安全余白ゲートを実際に評価できる入口が具体化されていない。実際の capture、入力、製品完成の承認ではない。

## 候補と要求の照合

| 対象 | 具体的入口・根拠 | 判定と限界 |
| --- | --- | --- |
| 過去の対象・所有 | 質問08は対象 exe SHA、PID、window ID と AX/画像不一致を、質問09は当時の2 handle の live を記録。元画像は未保存 | 適合。現在の process・window・handle は再検証しておらず、ID 再利用や同一性を仮定しない。別担当が所有 handle を引継いだとみなさない |
| A: 個別 CUA app 状態 | 提供される `mcp__cua_repl.js` の `cua.getApp({windowId})` は対象選択の入口。質問08には同じ app の AX が記録されている | 候補として適合。getState/browser inventory と同じ失敗経路を再使用せず、runtime が返す API のみを使う必要がある。今回新しい対象取得はしておらず、その方法が今利用可能か未実証 |
| B: `@oai/sky` 限定窓 | session の `mcp__node_repl__js` と Computer Use の skill に基づく `sky.list_windows()` / `list_apps()` → 返却 Window → `get_window_state(...)` という具体的入口がある。bundled `docs/api.md` は screenshot の origin・logical 寸法と最新 state の ID を定義 | 候補として適合。`Window.id` は opaque で PID は型に含まれず、別 provider の ID や Win32 HWND と同一視しない。対象一致を証明できない・同名が複数なら停止。browser inventory への実装上の影響と実際の capture は未実証 |
| C: OS 矩形 | `GetWindowRect` / `GetClientRect` / DWM bounds / DPI でウィンドウ全体を補助的に対応付ける案 | 候補として適合。ただし OS window 矩形だけでは DOM 要素や他窓の重なりは証明できない。実装・権限・座標系の照合は未検証 |
| marker と 12 CSS px | 設計のゲート4は marker、全 button、resize 帯の矩形を同じ版・時点・座標系で取得し距離を計算すると規定 | **B01 blocking**。どの提供 API とデータから各 DOM 境界・CSS px を取得するかは「別途確定」のまま。候補 A/B の提供型が示す AX tree は文字列、C は窓矩形であり、この案だけでは実行者が安全余白を客観的に算出できない |
| 停止・秘密・操作 | 画像不一致2回目の客観調査・3回目中止、browser inventory の既往2回を保持。画像と AX の不一致、対象不明、個人情報混入で入力を止める。撮影だけで UI 正常や Issue 完了としない | 適合。画像は未保存、遮蔽・別 desktop・描画等は未確定。実際の取得と対象一意性は現時点で未確認 |

## 指摘

| ID | 種別 | 根拠・影響 | 是正条件 |
| --- | --- | --- | --- |
| B01 | blocking | 安全余白ゲートが要求する marker・各 button・resize 帯の客観的な同一時点の矩形の取得手段と出力例が未定。`sky` の文書化された AX は tree 文字列、OS 窓 API は個別 DOM の境界を示さないため、提案をそのまま実行しても単一クリックの安全条件を判定できない。画面観察の失敗や安全性の問題を推測で埋められず、検証が停滞する。 | 利用可能な具体的入口と、marker・button・resize 帯について同一時点・同一 CSS 座標系の矩形、対象識別、尺度と観測時刻を得る方法を明示し、取得不能・競合時は入力を停止する。例として診断版で非機密の `getBoundingClientRect` 情報と距離のみを既存の表示領域に出す改訂案を検討できるが、DOM 変更が必要なら別版をハッシュ固定し既存の診断・安全条件を再レビューする。実際の方法を検証する前に同じクリックや対象画面取得を反復しない。 |

## 実施内容・未確認

- cwd `D:\Projects\GithubIssueWidget`。依頼の JSON 参照9件を基点・相対パス・SHA に分け、全件存在確認後に `Get-FileHash -Algorithm SHA256` で照合。対象設計、質問08・09、管理回答を UTF-8 で読取。各コマンド群終了コード 0。
- `C:\Users\semis\.codex\plugins\cache\openai-bundled\computer-use\26.1111.21308\skills\computer-use\SKILL.md` と bundled `docs/guidance.md`、`docs/api.md` の静的記述を確認。提供ツール一覧から `mcp__node_repl__js` の存在を確認したが、ツールの実行・capture・個別窓への到達は確認していない。
- `mcp__cua_repl` は現在の提供ツール宣言を参照した。`cua.getState()`、browser inventory、既往の失敗した web 検索や画像取得は再実行せず、実画面・入力・ドラッグ・build・アプリや所有 process の操作・通信試験・製品/scratch/既存証拠・Git/GitHub の変更も行っていない。
- 本レビュー担当の失敗はない。出力先の親ディレクトリ存在と確定先・tmp の不存在を確認し、UTF-8 往復と代表文字を検査して確定。対象版変更後は再レビューする。