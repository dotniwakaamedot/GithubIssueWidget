# I02 開発起動・正常終了の検証計画 01

- 担当 `/root/build_verification`。依頼 `tasks/I02/dev-verification-design-request-01.md` SHA256 `6267BAA463643AB6C80D279844CD2AAAE818F8504D8A68F13134BABE359B38F1`、固定参照7件は共有 run の絶対パスで全件存在・実体 SHA 一致。
- **本書は読み取りに基づく実行前の計画案。** dev の起動、依存導入、build、入力、process 停止、GUI・GitHub の変更は行っていない。具体コード・設定 overlay・所有者・実行前状態を固定し、別担当の独立レビューと親判断を経るまで実行しない。
- 元 worktree `D:\Projects\GithubIssueWidget-issue-2-20261118-160129`、branch `task/issue-2-foundation-20261118-160129`、HEAD `a4ac97286785a2a5ba0d070a909b58003fc49ff1`。README の変更と widget の未追跡16件を含む17ファイルは、release 時の専用コピーと共に個別 SHA を再照合し一致。元の17ファイルは `D:\Projects\GithubIssueWidget-build-20261119-02\source-manifest.json` SHA256 `BB5253F1D02C83E183C86A6981CA3768A01892E3D1B40EF3AC4AA6A014DE0033` で識別、HEAD のみには依存しない。
- 独立レビュー済み release exe `D:\Projects\GithubIssueWidget-build-20261119-02\cargo-target\release\github-issue-widget.exe` SHA256 `D5FD13D80B6FE7F72ACD0BA6CEA3E0E19F261E417F8F5331E81F94D09A379D44` と `real-build-result-01.md` / review-01 は release に限る。以下の dev は debug 実行ファイルと別設定・別依存環境であり、release exe と同一とは扱わない。

## 調査結果と競合

- 2026-11-18T21:11Z の読み取りで、既存 marker 診断 server は PID 32908・`127.0.0.1:1435`、診断 app は PID 52960・既知 marker 専用 exe path で存在。1420 の listener は観測されなかった。これは時点限定で、所有 session が今も生存している証明や今後の非競合を保証しない。これらの process・session・scratch を停止・変更しない。
- 原本 Vite 設定は `127.0.0.1:1420` と `strictPort`、Tauri `devUrl` も `http://localhost:1420`。上記の1435とは別 port のため、本案は原本の port 1420 を変更しない。実行直前に IPv4/IPv6 を含む port の使用・process・他担当を再確認し、占有または所有不明なら起動しない。確認後の競合は残るため fallback せず、異常時は対象 handle を保持して相談する。Vite の CLI 設定と strictPort は[公式 CLI](https://vite.dev/guide/cli)を参照。
- 元版と marker 診断版はいずれも identifier `com.githubissuewidget.app`。実際に固定された Tauri 2.12.7 の `src/manager/webview.rs` と `src/path/desktop.rs` は Windows の WebView data directory を `app_local_data_dir` から定め、`dirs-sys-0.5.0` の実装は Windows Known Folder を参照する。よって子の LOCALAPPDATA を新規値にするだけで旧 profile が分離されるとは断定できない。実際の既存 profile の場所・使用状況は本調査では測定していない。

## 分離した対象版・実行案

1. 親が担当・実行時間・デスクトップ・port・証拠・既存診断 session の所有者と非競合を確認。新規候補 `D:\Projects\GithubIssueWidget-dev-20261119-01` は調査時未存在。実行前に親・絶対/正規化先・同一 volume・ACL・owner・再解析点・ハードリンク・空き容量と作業対象への非重複を再確認し、未存在なら新規の専用領域に17ファイルを通常バイトコピーする。元とコピーの各 SHA・長さ・実行前後 status を manifest に保存し、元 worktree・release/marker の scratch・その ignored 生成物を一切上書きしない。チェックは排他ロックではなく、競合・対象版変更は停止する。
2. 未承認の設定差を隠さないため、製品17ファイルは変更せず、専用 scratch に別の overlay JSON を新規作成・hash 固定する案: `{"app":{"appDirectoriesOverride":"D:\\Projects\\GithubIssueWidget-dev-20261119-01\\app-data"}}`。これは Tauri の app_* directory と Windows WebView default data directory を app 専用の場所に切替える設定差であり、元の既定 data directory を用いた起動は未検証のまま。固定 runtime 2.12.7 の設定能力を確認し、merge 後に原本設定との差が指定の1項目だけであることを検証する。[Tauri の設定](https://v2.tauri.app/reference/config/)と[2.12.0 の変更](https://v2.tauri.app/release/tauri/v2.12.0/)が機能の根拠。専用 app-data は他の用途と共有しない。
3. Node 26.13.1、npm、Rust 1.90.0、VS18、WebView2 Runtime の絶対パス・版・exe hash と source lock を作業直前に固定。各 exe/config の reparse と所有、既存 .npmrc/.cargo 設定、registry/プロキシと通信先を資格情報を表示せず確認。専用 node_modules、dist、CARGO_TARGET_DIR、CARGO_HOME、RUSTUP_HOME、npm cache、TEMP/TMP、APPDATA/LOCALAPPDATA と証跡を新規使用し、旧 cache・credential はコピーしない。rustup / npm の公開取得は実行時の依頼と対象レジストリを明記し、ログを承認済み保護ゲートに通す。toolchain / npm / VS 等の通常 OS cache・システムログやネットワークなどは完全分離できない。
4. 子専用環境に `WEBVIEW2_USER_DATA_FOLDER` を専用 scratch の `webview-data` へ明示する追加案。WebView2 は[公式 API](https://learn.microsoft.com/en-us/microsoft-edge/webview2/reference/win32/webview2-idl)で同環境変数の override を説明するが、registry の override が優先し得る。該当 registry policy の有無・runtime・reparse と専用領域の実際の使用を確認できなければ起動せず、または起動後の未知状態を停止条件として親へ報告する。値・既存 profile の内容は取得・記録しない。環境変数は OS 権限境界ではなく、子プロセスが想定外の場所に書く可能性は残る。
5. 元の dev コマンドに overlay を渡す案は、専用 copy の `widget` を cwd とし、Node 経由で `npm run tauri -- dev --config <専用 overlay の絶対パス> -- --locked`。Tauri の[dev CLI](https://v2.tauri.app/reference/cli/)は `--config` のマージと `--` 後の runner 引数を説明する。実際の 2.12.5 CLI とその子孫への引数・設定適用は実行前に固定版を静的照合し、独立レビューを受ける。Node / npm / Tauri / Cargo / Vite と対象 app はそれぞれ別 process であり、いずれかの成功を他の process の exit に置換しない。

## 同定・正常終了・証跡の案

- release 時に独立承認された方式を土台に、新規専用 driver を別版として設計・固定・静的レビューする。長時間稼働の Node/npm 直起動 `Process` は同一 handle で PID・開始・終了 UTC、stdout/stderr の非同期読取と EOF / fault / cancel を記録。出力は専用 ACL 保護の一時ログに限り、機密走査と別担当の必要最小限確認・対象 hash に紐付く receipt・親判断前に確定名へ公開しない。長時間出力時はサイズ閾値を監視し、限界は厳密な強制停止ではないことを明記する。Node/npx 等の間接子孫に exit が伝播する保証はなく、ログの文言だけで exit を推測しない。
- 対象 debug exe の絶対パス・SHA と生成時刻、実行開始後に現れた PID・作成時刻・親子関係を照合し、一意の対象と判定できる場合にのみその実 process handle を保持する。1420 の listener は所有 PID と専用 server の親子関係・固定コマンドとの対応を照合し、監視前に既存の診断 1435 に触れない。PID・executable path・取得時刻・照合結果だけを保存し、環境変数、プロセスの全 command line、WebView URL やウィンドウ内容は保存しない。未知の子孫・再利用 PID・取得不可は成功扱いせず親に質問。
- **ウィンドウの存在確認案**として、同じ対象 PID の main window が存在するかと、その window を Win32 `GetWindowThreadProcessId`・`IsWindowVisible` で照合した真偽値のみを、明確に親・独立レビューで許可された場合に取得する。タイトル・画像・AX・座標・画面内容・他アプリの情報は収集しない。`IsWindowVisible` は可視属性の確認に限り、実際に遮蔽されずユーザーが見えたことを証明しない。画像不一致2回の同じ画像取得、`getState` / browser inventory、拒否された旧 window metadata review の公開内容の再作成・転記・回避は行わない。予定の照会や記録が以前に拒否された操作と実質同一と判明した場合は実行せず親へ報告する。
- 対象が一意に特定され、ユーザーや他担当の画面操作と競合しないことを確認し、親が作業対象の終了を指示した後だけ、保持した対象 `Process` に `CloseMainWindow()` を最大1回呼び正常 close を要求する案。既存 marker app・server と他者 PID には触れず、フォーカス移動・クリック・ドラッグ・リサイズ・強制終了を行わない。直前に同一 handle の PID / 作成時刻 / exe / window 所有を再照合し、対象が一致しない・他アプリが前面/操作中など不明なら閉じない。[Microsoft の仕様](https://learn.microsoft.com/en-us/dotnet/api/system.diagnostics.process.closemainwindow)上、戻り値 true は close メッセージ送信を示すだけなので、それだけでは成功にしない。
- app 側は同じ process handle の `HasExited`、実 ExitTime・ExitCode を終了後に取得し、正常終了の定義を **所有した app が close 要求後に exit 0** とする。Node/npm 側も同一 handle が正常終了し exit 0、stdout/stderr 両 EOF と log receipt が得られる場合に別々に記録する。Vite listener がなくなることだけでは Vite の exit code を証明しない。保持済みの同一 Vite process handle から明示的に取得できない場合はその exit を未確認とする。wrapper が残る・app が既に異常終了・EOF が来ない場合も成功にしない。
- timeout・標準入力断・子孫や対象 window の所有不明・ログ fault/cancel・secret の混入または読取不能・予期しない profile/port 書込み時は該当 stage を止め、元 handle と保護ログを保持して親へ質問する。操作失敗を重ねない。同じ問題が初回を含め2回で該当操作を中断して別担当が客観調査、3回目で全停止。画面が必要な app の生成時に focus や最前面表示が生じ得るため、開始前に管理がユーザーの作業との競合・可否を確認する。

## 完了の範囲と未確認

成功しても設定 overlay による data-directory と新規依存・debug exe の dev 起動/終了に限定し、既定設定・release exe・旧 diagnostic app の挙動、window が遮蔽されていないこと、close ボタンの実操作、サイズ・移動・リサイズ・topmost、Opener の実 IPC 拒否を証明しない。結果の各ログ・manifest・PID/時刻/EOF/exit・overlay/driver/対象17ファイルの SHA を新規 result に固定し、別担当が独立照合する。各ツールの起動・WebView2 実 profile・pid 親子・window の可視属性・閉じた時の Tauri CLI の挙動は現時点で未実行・未確認。

今回の作業は参照・17ファイル・ポート/PID・依存 source の読み取りと、Tauri/Microsoft 公式資料の確認だけ。実 build・起動・入力・process 停止・外部変更・拒否されたレビュー記録の公開は未実施。削除対象なし。今回のコマンド失敗なし。
