# I02 対象ウィンドウ OS メタデータ調査結果 01

- run / task / 担当: `20261118-160129-mvp-restart` / `I02-window-metadata` / `/root/drag_design`。依頼 `tasks/I02/window-metadata-request-03.md` SHA256 `E92A1B51CD06AD09EBFBA8A7B0DD2FAC7D157158045CA20E662ED0A1DEE03890`。依頼の構造化参照4件は絶対パスで全件実在確認後に実体 SHA256 を照合し、全件一致。
- 結論: **今回の対象窓は Windows の可視属性が true、最小化されておらず、DWM cloak は 0 だった。** これは画素が見えている、他窓やロック画面に遮られていない、過去画像と AX が一致する、入力が安全であることを証明しない。過去画像との不一致の原因は未特定。
- 製品対象 worktree: `D:\Projects\GithubIssueWidget-issue-2-20261118-160129`、branch `task/issue-2-foundation-20261118-160129`、HEAD `a4ac97286785a2a5ba0d070a909b58003fc49ff1`。対象実装は未コミット/未追跡17件であり、実体 SHA を固定した `tasks/I02/result-02.md` および診断版を記録した `tasks/I02/marker-execution-result-01.md` を参照。今回、対象製品ファイルを変更せず、17件全体の現状態は再照合していない。削除対象なし。

## 対象と所有 handle

- 2026-11-18T19:40:17Z 前後、所有 app session `70494` と dev server session `19143` の各空 poll は5秒後に同じ session ID を返し、観測時点で live。exit code は未取得。取得 timeout や空出力を terminal と扱わない。別担当は所有 handle の引継ぎを前提にしない。
- PowerShell の `Get-Process -Id 52960` と対象 path filter、`Get-FileHash -Algorithm SHA256`（cwd `D:\Projects\GithubIssueWidget`、exit 0）: PID `52960`、session `2`、開始 UTC `2026-11-18T19:25:33.6036834Z`、MainWindowHandle `6559504`、対象 exe path `D:\Projects\GithubIssueWidget-marker-diagnostic-20261118-160129\target\debug\github-issue-widget.exe` 一致、実体 SHA256 `54DBCECFD920F6F920942A190C81B28BD7742C6052ABB58AB9C89E1F71C37AA8`。時点以後の PID / HWND 再利用や非競合は保証しない。

## 公開 Win32 API の読み取り値

2026-11-18T19:40:56Z、cwd `D:\Projects\GithubIssueWidget`、PowerShell でローカルの ASCII C# 型を `Add-Type` し、**対象 HWND 6559504 のみ**を read-only で照会（終了コード 0）。`GetWindowThreadProcessId` は PID 52960 / thread 53852 と一致。`IsWindowVisible=true`、`IsIconic=false`、`GetDpiForWindow=96`。`DwmGetWindowAttribute(DWMWA_CLOAKED=14)` HRESULT 0・cloak 0。`GetWindowRect` 成功で left 1470 / top 796 / right 2386 / bottom 1325、`GetClientRect` 成功で `[0,0,900,520]`、`ClientToScreen(0,0)` 成功で `[1478,797]`。

`GetThreadDesktop` と `GetUserObjectInformation(UOI_NAME)` は対象 thread の desktop 名 `Default`、`OpenInputDesktop` と同名照会は input desktop 名 `Default` を返し、開いた input desktop handle は `CloseDesktop` 成功。これらは**照会時点**の情報であり、取得時点の対象窓と input desktop の名前一致のみを示す。過去の画像が背景だった時点の desktop、画面ロック、ユーザー操作、セッションの所有や遮蔽状態を遡って証明しない。`GetWindowRect` の DPI 仮想化・非表示枠、`GetDpiForWindow` の awareness 依存を考慮し、上記座標を CSS 座標や screenshot・入力座標と無条件に同一視しない。

## 過去の最初の画像についての補足

前回 `questions/I02-drag-click-question-08.md` の `cua.getApp({windowId:267184})` は旧診断 exe の window を AX tree 上で識別し、その後 `getAXStateAndScreenshot()` の tool 表示に対象 AX と背景画像が同時に示された。呼出し結果を変数に代入・画像保存しておらず、ツール応答から screenshot の origin、screenshot ID、原寸メタデータや画像データを独立に再照合できる証拠は残っていない。画像を再取得せず、画面がロックされていたか、別 desktop・描画・遮蔽・capture のいずれかを確定しない。後の新診断の画像不一致は別担当の記録を参照し、推測で補わない。

## 実施・未実施・停止条件

- 依頼文と参照4件は `Get-Content -Encoding UTF8`・`Test-Path`・`Get-FileHash -Algorithm SHA256` で確認（exit 0）。旧記録 `marker-execution-result-01.md` は診断版18対象・実装版を列挙。元・診断の実体全ファイル SHA 再計算や独立レビューは本調査範囲外。
- Win32 の `GetWindowThreadProcessId`、`IsWindowVisible`、`IsIconic`、`GetWindowRect`、`GetClientRect`、`ClientToScreen`、`GetDpiForWindow`、`DwmGetWindowAttribute`、`GetThreadDesktop`、`OpenInputDesktop`、`GetUserObjectInformation`、`CloseDesktop` を対象確認と read-only 情報のため使用。input desktop の handle 作成・close はローカル一時リソースで、窓、focus、desktop や既存プロセスの変更ではない。
- CUA 画面再取得・`getState` / browser inventory・画像保存・クリック・ドラッグ・フォーカスや位置の変更・build・プロセス停止/再起動・外部送信は行っていない。本担当が今回変更したのはこの新規結果のみで、旧共有記録・証拠・scratch・Git・GitHub は変更しない。観測の不一致やデスクトップ状態の原因は未確定、実画面・非遮蔽・ユーザーの操作状況・座標整合と安全な入力方法は未検証。独立レビューと親の判断を待つ。
- 今回の実行失敗なし。画像不一致・browser inventory エラー・直接公開逸脱は各2回で独立調査済み。同種2回で該当操作を止め別担当の客観調査、3回目で全作業を直ちに中止。結果は所有ディレクトリの親存在・確定先と `.tmp` 不在を確認し、`.tmp` を UTF-8 内容検査後に非上書き rename して確定。実体 SHA は確定後に親が独立計算する。
