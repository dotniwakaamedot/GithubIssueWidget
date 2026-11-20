# MVP 検証状況（2026-11-20）









## 現在リポジトリの実 GitHub 読み取り（同日）

- 認証済み GitHub CLI の資格情報を子プロセスの環境に一時的に渡し、Rust の repository GET と Issue ページ GET を現在の公開リポジトリ dotniwakaamedot/GithubIssueWidget に対して実行した。テストは成功し、取得件数は10。Issue やラベルへの書込みは行っていない。
- main.rs SHA256 021488C4C1615EBEFA3185EBDF024A43547657E357715957128F403541A67A97 の ignored テストを、テスト EXE SHA256 F225011E1F911F1D0056D2519CCFA2B576FCFA3261675880F467C28EA2E2A811 から単独起動。実行手順 readonly-execute-09.ps1 SHA256 3EAA3AA727A9B14EA3223DD216DBBD9C24ABAF3FB02585E054269D885AAC4094 は review-12.md で指定範囲の承認を受けた。
- コマンドの出力は test_exit=0、issue_count=10。秘密情報と Issue 内容を含み得る生出力はファイルに保存していない。親プロセスの一時環境変数は finally で復元され、プロセスは終了した。
- 別経路の GitHub CLI でページを取得し、Issue 内容を表示せず件数だけを Python のメモリー内で集計したところ、Issue 10件、Pull Request 0件、Open 10件、Closed 0件で、先の Rust の Issue 10件と一致した。両取得は同時ではなく、今回の実データは Closed や Pull Request 除外を検証できない。
- この結果は指定リポジトリの読み取りとレスポンス解析に限る。実 UI、Credential Manager への実 PAT の登録、書込み権限、作成・移動、非公開リポジトリは検証していない。EXE とソース版の対応はビルド時の記録に依存する。追加したテストは cfg(test) 内であり、リリース版 EXE はこの追加前のビルドである。

## 起動時の Windows 最前面属性（同日）

- Windows 11 Home build 26300 で EXE SHA256 4A58C1363D5ED1A9B2F1FFF7AE45C04DCDA95650E7FB060B2C9B31B40D241C68 を Start-Process -WindowStyle Hidden で起動し、そのプロセス ID とウィンドウタイトルで対象を特定した。
- Windows の EnumWindows、GetWindowThreadProcessId、GetWindowText、GetWindowLongPtrW を用いて対象ウィンドウの拡張スタイルを読み、WS_EX_TOPMOST ビットが設定されていることを確認した。出力は window_found=true topmost=True。新規テストプロセスのみ停止し test_process_exited=True を確認した。
- 非表示起動の属性確認であり、通常表示時に他ウィンドウより前面に表示されること、固定ボタン操作後の OS 属性、DPI ごとの動作は未確認。結果は当該コマンドの出力に基づく。

## GitHub リンクの画面確認（同日）

- Windows 11 上のリリース EXE SHA256 4A58C1363D5ED1A9B2F1FFF7AE45C04DCDA95650E7FB060B2C9B31B40D241C68 で設定画面の GitHub リンクをクリックした。アプリ側にエラー表示はなく、その後 Brave の表示アドレスと文書 URL が https://github.com/ であることを確認した。
- クリック前から Brave ウィンドウが存在し、タブ一覧の事前記録はない。リンク操作で新規タブが開かれたことや、既存ページとの因果はこの確認だけでは証明できない。ブラウザー連携ホストの取得は失敗し、ネイティブウィンドウの表示で確認した。拒否 URL の実ブラウザー検証は未実施。
- アプリの終了を確認した。ブラウザーの既存タブは整理・削除していない。

## 移動処理修正後の IPC 再確認（同日）

- main.rs SHA256 C938EEB22F1C2E97EB03E597D21ED1C87B903868C3673474B05043E36338641E、movement.rs SHA256 ED49F4BA52C6887BCE427E4A6664D335092A1C3311E1B1ED61B4CE322791ABA5 の組合せで tests::windows_ipc_guard を --ignored --exact で単独実行し、1 passed。
- 試験は隔離した設定・合成資格情報と模擬 IPC を使い、古い一覧・作成・移動、保存・削除の拒否と正常削除を照合する。実 PAT・実 GitHub・実 UI や他の Windows セッションは対象外。ログは一時フォルダーの giw-latest-ipc-cb9373f0a94d417fbe0ae546f00f3188.log。 review-08.md で対象版・試験条件とログを独立照合した。ログ自体はソース SHA256 を証明せず、対象版との対応は依頼記録に依存する。

## ラベル削除応答の確認（同日）

- 個別の旧ステータスラベル削除後、応答に移動先ラベルがあり旧ラベルがないことを確認する。応答の矛盾や解析失敗は部分反映の可能性を通知し、以降の削除・状態変更を行わない。削除後のラベル配列は GitHub 公式の Remove a label from an issue に基づく。
- movement.rs SHA256 ED49F4BA52C6887BCE427E4A6664D335092A1C3311E1B1ED61B4CE322791ABA5。cargo test --locked --offline は本体65 passed・9 ignored、URL 範囲1 passed。ログは一時フォルダーの giw-removal-confirm-e31206ac05fa422b8de392a193d98b7f.log。対象版の review-07.md は依頼範囲を承認した。
- 前回のビルドは停止条件により中断。再開後の npm.cmd run tauri -- build --no-bundle は終了コード0、EXE SHA256 4A58C1363D5ED1A9B2F1FFF7AE45C04DCDA95650E7FB060B2C9B31B40D241C68。ログは一時フォルダーの giw-removal-build-a3f7761c7f3b41a3904ce37b35ba4c8f.log。Windows 11 上で同 EXE の接続設定画面表示と終了を確認。実 GitHub に接続した実 UI の操作は未検証。
- 現在の公開リポジトリのラベル一覧では status:in-progress は存在し、status:todo と status:done は確認できなかった。追加や検証用 Issue の作成は行っていない。外部の同時変更を原子的に防ぐ保証はない。

## ラベル追加応答の確認（同日）

- Issue 移動時、ステータスラベルの追加応答を解析し、移動先ラベルが確認できない場合は旧ラベル削除・state 更新の前に停止して部分反映の可能性を通知する。GitHub 公式の Add labels to an issue の成功応答例はラベルオブジェクトの配列。応答が空配列または不正 JSON となる条件はローカル模擬試験であり、実 GitHub で発生したと確認したものではない。
- movement.rs SHA256 571C67DD98F148CF7B97F4056F1C8A3CAFB6ADE454DBE29C34B075588EED777D。cargo test --locked --offline は 64 passed・9 ignored、URL 範囲 1 passed。ログは一時フォルダーの giw-move-confirm-7f3dc760556c4600bc8378af6bbbb53c.log。review-06.md は対象版の依頼範囲を承認した。
- npm.cmd run tauri -- build --no-bundle は終了コード 0。EXE SHA256 09FCD8D92B98653BF4AD28CBE6F35A1380F83A5E80339405E8F3D41BA3792618、ログは一時フォルダーの giw-move-build-d84cbcea176a482f8ad96f151553cd09.log。画面操作ヘルパーが使用中と返したため、この EXE の画面起動は未確認。
- 実 PAT・実 GitHub・実 UI の更新は未検証。ラベル追加と削除・状態更新は原子的ではなく、外部の同時変更や部分反映は引き続き起こり得る。

## 現行版の合成資格情報と IPC（同日・再確認）

- main.rs SHA256 C938EEB22F1C2E97EB03E597D21ED1C87B903868C3673474B05043E36338641E を対象に、通常の Windows ユーザー環境で isolated_windows_credential_lifecycle、windows_inflight_guard、windows_ipc_guard をそれぞれ --ignored --exact で単独実行し、各 1 passed。
- 各試験は新規の合成資格情報と分離した設定領域を使う。作成・更新・削除、取得中の変更による古い結果の拒否、模擬 IPC の世代確認を検証する。試験終了時の独自クリーンアップの範囲は各テストコードを参照する。
- 一時フォルダーのログ: giw-current-credentials-ead3bdc206034369b8376b55a5805665.log、giw-current-inflight-41434254896e4db08d2bbde9ddfe3cd6.log、giw-current-ipc-474403dcdbd849598c66db7ad301480e.log。
- 実 PAT・実 GitHub・実 UI や異なる Windows セッションは対象外。試験ログと対象版の対応は管理者記録による。review-05.md でログとテスト本体を独立に照合したが、レビュー担当は試験を再実行していない。

## Issue 移動の非ステータスラベル保護（同日）

- 独立レビューで、従来の全ラベル PATCH が取得後に追加された他者のラベルを消し得ることを指摘された。変更前の模擬試験にも同じ消失の再現がある。
- 移動先ラベルの個別追加、取得時に存在した旧ステータスラベルの個別削除、必要時のみ state の PATCH、最後の再取得と照合に変更した。非ステータスラベルの配列全体を送らない。途中で拒否・通信結果不明が起きた場合、部分反映の可能性を明示する。
- 取得後に第三者ラベルを追加する模擬サーバーで６方向の保持を検証した。複数ステータスの削除、途中拒否、最終状態の競合、最終再取得失敗も検証した。cargo test --locked --offline は本体 62 passed・9 ignored、URL 範囲 1 passed。ログは一時フォルダーの widget-rust-135938fbd7014b4883d7ba90764e77f0.log。対象 movement.rs SHA256 320E75DFD5FA2ACBE0A0210669321E95721C76557E6CF4BD13A97EEEEB27DA37。
- 非 blocking 指摘を反映し、PATCH 応答消失の模擬試験を追加し、既存試験の名前を修正。README に競合と部分反映を明記した。現行 movement.rs SHA256 4CFF3C3FB28B6C70D93166D09FA1962FA3CD3B786357B51561F02EC6BB2EA19A、README.md SHA256 9B9E08F1B70FF5007E6A770E4B966A8BB06E7041AD1D3A9384CCBEEA4E6AC06F。Rust は 63 passed・9 ignored、URL 範囲 1 passed。ログは一時フォルダーの widget-rust-b87fb45e0f1a4a93a33bc13994c25f04.log。独立レビュー review-03.md と review-04.md は各指定版を承認した。
- 各 API 操作は原子的ではなく、中間状態やステータス・state に対する同時更新との競合が残る。最終照合はその後の変更や同じ見かけの上書きを検出できない。実 GitHub と実 UI での移動検証は未実施。

- 現行版の npm.cmd run tauri -- build --no-bundle は終了コード 0。実行ファイル SHA256 78E24581343922502ACD036734755A7A4D2DE08ABFE75D6FBA8A881C70FDC47C。ログは一時フォルダーの widget-build-fa5733b4c0f54bee83dbeeda8ea76b18.log。Windows 11 上で接続設定画面が表示され、終了ボタンでウィンドウが消えることを確認した。接続・作成・移動の実 UI 検証ではない。

## 直前リリース版の Windows 表示確認（同日）

- Windows 11 Home build 26300、EXE SHA256 24B86C6FB434BDEAF53B663414E46A796B7674D1407063FF046BC6C4A8C564E3 で、設定画面への起動、固定ボタンの「固定中」から「固定」への表示切替と復帰、ウィンドウ終了を確認した。OS の最前面属性や他ウィンドウとの前後関係は未確認。

## Issue 作成フォームの送信先保護（同日・現行版）

- 作成フォームを開いている間は設定画面への切替を無効にし、フォーム内に作成先の owner / repository を表示する。旧版では入力途中に接続先を切り替えると、旧入力が新しい接続先のフォームへ持ち越され得た。実際の誤送信は未観測。
- 画面モックで、入力中の設定切替抑止、作成先表示、キャンセル後に別の接続先へ切り替えた際の旧タイトル・本文の消去を確認し、画面テスト 31 passed。テスト追加後の `tsc --noEmit` は終了コード 0。実 UI の作成フォーム表示と実 GitHub への送信は未検証。
- `main.tsx` SHA256 `1AB362408673E420BED96D0D77127E5FB9DA70B3C6C6B4305386589F9B58DE75`、画面テスト SHA256 `98504EC4DB2B733A6C726C094EF242617C8BA6B7915D2CEC776A980A2849C65A`、`style.css` SHA256 `EB49BF65AE4FCE6701FBEF277055EDCAD42011248E5EA49905B6AFA4FD288B37`。リリースビルドは終了コード 0、実行ファイル SHA256 `24B86C6FB434BDEAF53B663414E46A796B7674D1407063FF046BC6C4A8C564E3`。Windows 11 で設定画面の表示と Alt+F4 後の終了を確認した。
- テストファイルはリリースビルド開始後に検証項目を追加し、後から画面テストと型検査を実行した。製品用の画面コードと CSS はビルド開始前に確定している。ログは一時フォルダーの `github-widget-vitest-e696aa2bdf364f17bec68802dc3c9d4a.log`、`github-widget-tsc-4e49532af17a417a8a103501e90147f0.log`、`github-widget-build-e77081eff50d4a29bda8b18f504971e4.log` にある。

## 移動テストの待機上限（同日）

- `movement.rs` の２つのテスト用サーバーについて、接続受入れと要求の読取りに期限を設け、途中切断を検出し、応答書込みにも期限を設定した。変更は `#[cfg(test)]` 内の試験コードに限る。
- 要求が来ない場合、接続後に送信されない場合、要求途中で接続が終了した場合の３テストと、既存の移動テストを合わせて 8 passed。`cargo test --locked --offline` 全体は本体 60 passed・9 ignored、URL 照合 1 passed。`movement.rs` SHA256 `B240B50FDCDB34AFB3156B2EDC2AAFD11B03F808B033939716AB1B0FAFCA8791`。ログは一時フォルダーの `github-widget-movement-test-24842c55c8eb43d5a0ec988a5283a1e8.log`、`github-widget-rust-test-e72968a23d8247ee931e81760498c757.log` にある。
- 実際のタイムアウトを伴う書込み異常や全 OS 状況は未確認。テストコードのみの変更後、リリースビルド・実 UI・実 GitHub は再検証していない。以下の製品版記録は本変更前の対象版を指す。

## ボード操作の資格情報世代確認（同日・現行版）

- 一覧取得・Issue 作成・移動でも、画面が取得した接続設定の世代を Rust 側で照合するようにした。同じリポジトリで別ウィンドウが資格情報を変更していた場合も、古い画面から新しい資格情報を使って操作せず、再読込みを求める。既存の取得完了時のスナップショット確認と設定領域のロックも維持する。
- 試験専用 service と分離設定を使う模擬 IPC で、同じリポジトリ・同じ credential ID の合成値だけを変更した後、古い世代の一覧・作成・移動がそれぞれ拒否されることを確認した。回帰時にも外部送信されない無効な合成ヘッダーを使用。既存の古い保存・削除の拒否と正常削除も成功し、試験用資格情報は同じユーザー・セッションの別プロセスでも見つからなかった。実 PAT、実 GitHub と実 UI の２ウィンドウ操作は未検証。
- `main.rs` SHA256 `C938EEB22F1C2E97EB03E597D21ED1C87B903868C3673474B05043E36338641E`、`creation.rs` `61FB68E208B231664DB236E50A37BBB4870763AD61C0071383BACA5730982A87`、`movement.rs` `604C9EAE0E7DD3038F9AB27959A82F8C686A01C7A6242BD2CD2F36AEE729FEB9`、`main.tsx` `42B76CA5E4AE98A4DFAF9E094B383AB870D7D4AFAA57CE894B181649FD31CA7C`。Rust 本体 57 passed・9 ignored、URL 照合 1 passed、画面テスト 30 passed、隔離 IPC 1 passed。
- リリースビルドは終了コード 0、実行ファイル SHA256 `899D556D0C760781820F9E1844A73FCEF386BCAC5FF4E10716140677DA41E788`。Windows 11 で設定画面の表示と Alt+F4 後の終了を確認した。実 UI の認証・Issue 操作、移動・サイズ変更はこの版では未確認。
- 一時フォルダーのログ: `github-widget-ipc-guard-ffaa5a00d2f74efb85d9c10e96e818bf.log`、`github-widget-rust-test-401fb66991894a908c9ce6d13573894f.log`、`github-widget-vitest-d445719225084d318f531b37114278d5.log`、`github-widget-build-1c6b9a0d843f48bfba603978d985fa21.log`。

## 作成後のステータスラベル競合（同日・現行版）

- Issue 作成後の応答に選択したラベルと別のステータスラベルが共存する場合、成功ではなく、作成済み Issue を示す部分成功として扱うようにした。通常の非ステータスラベルは共存できる。進行中の作成直後と Done のクローズ後の競合、通常ラベルの併存をローカル HTTP の模擬応答で確認した。実 GitHub や実 UI での競合は未検証。
- 作成テスト用 mock サーバーの受入れ・読み書きに期限を設け、要求が来ない場合に終了するテストを追加した。`cargo test --locked --offline` は本体 57 passed・9 ignored、URL 照合 1 passed。`creation.rs` SHA256 `4587F26B643E785FC6484A75C5EE3AA436904D28F1A19E7D0088656DF7D69AD8`、`main.rs` SHA256 `75CF447A2FD9560F7F4CAC7C96E063FB1FCC343229F414137EF0510D0B4BDD3B`。ログは一時フォルダーの `github-widget-rust-test-c474a50219e54c178b9f2bea893af456.log`。
- リリースビルドは終了コード 0、実行ファイル SHA256 `09AF6A18FFDF396F07D6310618352E0078FD0BD0A6FC47B702688B2EA9BC6BF5`。Windows 11 で設定画面の表示と Alt+F4 後の終了を確認した。ログは一時フォルダーの `github-widget-build-da39678e44dc42ebb3516308228cf26b.log`。実 UI の作成操作やウィンドウ移動・サイズ変更はこの版では未確認。

## 無効な合成トークンによる実 GitHub 認証失敗（同日）

- 読み取り専用の公開リポジトリ `octocat/Spoon-Knife` に対し、実在する PAT ではない固定の合成値を使って、アプリと同じ `verify_connection` を呼び出した。GitHub の HTTP 401 がエラーとして返り、合成値が返却エラーに含まれないことを確認した。対象は Rust の通信処理であり、実 UI、実 PAT、書き込み権限不足、別の認証失敗条件は対象外。
- 現行 `main.rs` SHA256 `75CF447A2FD9560F7F4CAC7C96E063FB1FCC343229F414137EF0510D0B4BDD3B` で当該のネットワークテスト 1 passed。`cargo test --locked --offline` は本体 53 passed・9 ignored、URL 照合 1 passed。ログは一時フォルダーの `github-widget-public-auth-9c06f42000bc41cb82ef95577c4b7f07.log`、`github-widget-rust-test-f12b0cc18cf04c3eaa1a5df967c37c23.log` にある。
- リリースビルドは終了コード 0、実行ファイル SHA256 `E60E3B6457253BF130C50E96053B45A64E6E40D4781D85DD671D7861A71CE2D0`。Windows 11 で設定画面の表示と Alt+F4 後の終了を確認した。ログは一時フォルダーの `github-widget-build-2a6753541bca47ec9ce45f386574ebed.log` にある。変更はテストコードのみで、実 PAT を扱う成功フローは引き続き未確認。

## RustSec 依存関係監査（同日 11:42 JST）

- 一時領域に `cargo-audit` 0.22.2 を構築し、現行 `Cargo.lock` SHA256 `2486C9E87F8511A2CBB3C09177E278139409C68A308FD48642327EDDF1D88E7F` を RustSec Advisory Database の commit `074ea9a7ed619f511b5b7374c7a97e98d1e98be4` で監査した。終了コード 0、脆弱性0件、情報警告6件だった。監査対象のロックファイルには 516 依存が含まれる。
- 情報警告は未保守5件（`chrono 0.4.45`、`proc-macro-error 1.0.4`、`zbus_names 4.3.5`、`zvariant 5.15.1`、`zvariant_derive 5.15.1`）と、[glib 0.18.5 の健全性に関する警告](https://rustsec.org/advisories/RUSTSEC-2024-0429.html) 1件。`chrono` は [未保守の通知](https://rustsec.org/advisories/RUSTSEC-2026-0543.html) がある。
- `cargo tree --locked --offline --target x86_64-pc-windows-msvc --edges all --invert <対象>` では、上記6パッケージはいずれも Windows 対象の依存グラフに現れなかった。正の対照として `serde` は検出された。この確認は指定した対象・ロックファイルのグラフに限り、他のプラットフォームや未登録の問題が安全であることを示さない。
- 監査ツール・データベース・JSON は一時フォルダーの `github-widget-rustsec-166770daa61f4c2e84b906f8fd99bb28/` に保持。ツールを通常の Cargo ホームへインストールせず、プロジェクトの依存更新も行っていない。

## npm 依存関係の監査（同日 11:34–11:35 JST）

- `widget/` で `npm audit --omit=dev --json` を実行し、終了コード 0、報告された本番依存の脆弱性は critical / high / moderate / low ともに0件だった。`package-lock.json` SHA256 `A5D65370135F83B08F02D5F1EE329706F08551743444264BCB10DFC7600B0C73`。生の監査出力は保存せず、件数のみ確認した。
- 続けて開発依存を含む `npm audit --json` も実行し、終了コード 0、報告された脆弱性は critical / high / moderate / low ともに0件だった。
- この結果は実行時点の npm 監査対象に限る。Rust の依存、未登録の脆弱性やアプリ独自コードは対象外。Rust 依存は別途上記の RustSec 監査を実施した。

## 接続設定の世代確認（同日・現行版）

- 別ウィンドウで接続先または資格情報が変更された後、古い画面から現在の資格情報を保存・削除し得る旧版の欠陥を修正した。画面が取得するランダムな一時識別子と、Rust のメモリー内に保持する設定・資格情報の照合値を使用し、操作前に状態が変わっていれば中止する。トークンとその照合値、Credential Manager の ID は画面へ返さない。設定と資格情報が変わらない再読込みでは同じ識別子を使用する。
- `main.rs` SHA256 `01748044CFEB5181B931C7FE26CFBAE885ADB246C366AD488D8C81B0C2F27D4E`、`main.tsx` SHA256 `E2BAD5D4B8B298AC143516963D30B67DF1930182E35BC5FE4AAC32C0BC5166A6`、画面テスト SHA256 `2ADE2F3581F896CD7144634479DC32F3C288CA9A8395CA781FF619A940FC6D75`。Rust 通常テスト 53 passed・8 ignored、別の照合テスト 1 passed、画面テスト 30 passed。
- 試験専用 service・合成資格情報・分離設定を使う模擬 IPC で、設定変更後の古い保存・削除を拒否し、資格情報が保持されることと、その後の正常削除を確認した。別プロセスからも同じユーザー・セッション内で試験用資格情報の不存在を確認。加えて製品と同じ service 内の UUID 付き合成資格情報による取得中変更テストも 1 passed で、対象2件の削除後不存在を同一セッションの別プロセスから確認した。実 PAT、実 GitHub、複数の実 UI、別セッションは未検証。
- リリースビルドは終了コード 0、実行ファイル SHA256 `6DAE759CACAA3E9916CEE6F0C11066B98D35D49E01A9ADE95B86B61433AC58AC`。Windows 11 で設定画面、未保存トークンの案内、設定読込み後の保存ボタン、Alt+F4 後の終了を確認した。移動・サイズ変更・実認証はこの版では未確認。
- 一時フォルダーの証拠: `github-widget-vitest-061c8006771e4fe4899ff8e4ae4895e4.log`、`github-widget-rust-test-74a989f45c9945dea59478ce45ab7608.log`、`github-widget-ipc-guard-107aee0044a340d0bb0d477e964cd9c7.log`、`github-widget-race-test-bcb66b0954cf40768f79c9c79cc1db0e.log`、`github-widget-build-7310825de2e14cd7ada673cf21fc9921.log`。旧対象ファイルの退避は `github-widget-snapshot-backup-ee150280cb624882990ced8692d60c68/` に保持。

## 現行版のウィンドウ自動操作（同日）

- リリース実行ファイル SHA256 `9ADDDEBF722F006BE92D76396ECE43B87D0C6AB5B531C0B544D18CA7771C5769` を起動し、取得画像は 902 × 522、原点は `(1477,796)` だった。タイトルバーを自動ドラッグした後も同じ原点だった。入力が移動処理に到達したかは未確認のため、製品の移動不能と断定しない。
- システムメニュー確認の Alt+Space では検索画面が現れ、メニュー操作は確認できなかった。検索画面の表示元とキーの受信経路は未特定。アプリを再有効化し、Alt+F4 によるウィンドウ消失を確認した。現行版の手動ドラッグ、サイズ変更、最前面の OS 属性は引き続き未検証。

## 現行版の合成資格情報検証（同日）

- 現行 `main.rs` SHA256 `C13F983C457CA6AB838D0E4A8F282D7579679023982B2D2A1D721F3434E52678` の `isolated_windows_credential_lifecycle` を通常 Windows ユーザー環境で単独実行し、1 passed。UUID を含む試験専用 service と合成値を使い、Credential Manager の登録・読取り・更新・再読取り・削除と、同じプロセスから削除後に見つからないことを確認した。
- この試験は製品用 service、実 PAT、実 UI、再起動後の読取りを対象としない。削除後の別プロセス・別セッションからの照会も行っていない。ログは一時フォルダーの `github-widget-credential-test-0ff53ecfcb63452c9da1055b364812dd.log` にある。
- 同じ現行 `main.rs` の `tests::windows_inflight_guard` を単独実行し、1 passed。分離した設定ファイル、ローカル HTTP、実ファイルロックと、製品と同じ service 内に新規 UUID で作成した合成資格情報を使用し、通常取得と取得途中の資格情報変更後の古い応答拒否を確認した。合成資格情報の削除後、同じユーザー・セッションの別プロセスによる読取りでも対象2件は見つからなかった。実 UI・実 GitHub・実 PAT・他のセッションは対象外。ログは一時フォルダーの `github-widget-race-test-38bf4c1681ed4036b8721a5da4120c1f.log`、SHA256 `BB2C9418B21D4D783431A47B45EA458F5CA59918930C47A92A3B5C085BC8D517`。当初は不完全なテスト名に `--exact` を指定して0件実行となり、完全な名前を指定し直した。

## 現行版の完了条件照合（同日）

親 Issue #1 の完了条件を、以下の記録と照合した。この節の「現行版」は照合当時の版を指し、上記の世代確認修正後の版ではない。旧版の画面確認やモックの成功を修正後の実機・実 GitHub 検証に読み替えない。対象は未コミットの `widget/`、`main.rs` SHA256 `C13F983C457CA6AB838D0E4A8F282D7579679023982B2D2A1D721F3434E52678`、`main.tsx` SHA256 `18A68A2059C80E6D41FD07DAA7A4B90435BC31149DB90CD28B68308DC83CCC11`、リリース実行ファイル SHA256 `9ADDDEBF722F006BE92D76396ECE43B87D0C6AB5B531C0B544D18CA7771C5769`。

| 完了条件 | 確認済みの範囲 | 未確認・次の作業 |
| --- | --- | --- |
| 開発起動・ビルド | 開発手順を README に記載。現行版の開発起動・通常終了とリリースビルドに成功。 | この条件の残作業なし。他の実機・実 GitHub 検証は下記に残る。 |
| Windows 表示・操作・外部リンク | 現行版の設定画面表示と Alt+F4 による終了を確認。旧版では手動移動・サイズ変更と、別の旧版でリンク操作を確認。 | 現行版で移動、サイズ変更、最前面の OS 状態、既定ブラウザーの遷移先、禁止 URL の実 WebView での拒否を確認する。 |
| 一覧・分類・ページ送り | モックで分類・PR 除外・1,050件の取得を確認。現行 Rust の公開 GitHub 試験で2ページ目の取得と応答の解析に成功。 | 実 UI と実 GitHub データを組み合わせ、各分類と一覧表示を確認する。 |
| 作成・各列移動 | モックで作成の部分失敗、6方向の移動と非ステータスラベル保持を確認。 | 許可されたテストリポジトリで実 UI から作成・移動し、GitHub 上のラベルと Open / Closed を照合する。 |
| 接続設定・資格情報 | 現行版の分離した合成資格情報で Credential Manager の登録・読取り・更新・削除を確認。別途、一部 IPC と画面の遅延応答はモックで検証。 | 実 UI で PAT の登録・再起動後の取得・更新・削除、接続先切替を確認する。 |
| 障害・レート制限・競合 | モックで API 失敗、レート制限、作成途中の失敗、旧接続の応答拒否を確認。現行版の分離テストで、実ロックと合成資格情報を使う取得中変更も確認。 | 実 UI と適切な検証環境で権限不足、通信障害、レート制限、部分失敗、応答競合の表示と整合性を確認する。 |
| トークンの非露出 | 合成値によるエラー応答・設定保存のテストと、ソース6ファイルへの限定的な形式検査を実施。 | 実 UI の応答、平文設定、実行ログ、リポジトリ全体について対象と方法を定め、秘密を表示せず確認する。 |

## 現行版の開発起動・終了（同日）

- Windows 11 で `APPDATA` と `LOCALAPPDATA` を新しい一時領域に分離し、Visual Studio 2022 の x64 開発環境から `npm.cmd run tauri -- dev` を実行した。起動前にポート 1420 とアプリプロセスが存在しないことを確認した。
- 開発版がコンパイルされ、`http://localhost:1420/` の接続設定画面が表示された。Alt+F4 で終了後、開発コマンドの終了コード 0、ポート 1420 のリスナーとアプリプロセスの不在を確認した。開発実行ファイル SHA256 `760F487F43C1D7A0180A50897B530880E0B3ECFB53C62E78798E4CA57E03D25B`。
- 分離領域とログは一時フォルダーの `github-widget-dev-7e8a153e3aff4ec7a714719e2ec0a6ff/` に保持。実 GitHub やトークン操作は行っていない。

## Opener の模擬 IPC 検証（同日）

- 同じ `main.rs` を使い、認証情報なしで GitHub の公開リポジトリ `octocat/Spoon-Knife` にアクセスする既存の `public_github_next_link_smoke` を実行し、1ページ目の次ページリンク解析と2ページ目の取得・Issue 応答のデシリアライズが成功した。これは読み取り専用の直接 API 試験であり、実 WebView・PAT・ボード表示・書き込みは対象外。ログは一時フォルダーの `github-widget-public-test-c45416381da0429093641b78c0649e76.log` にある。
- アプリ本体の単体テスト内で実際の capability と Opener を読み込み、模擬 IPC から `http://127.0.0.1:1/blocked` を渡した結果、Opener の許可範囲による拒否を確認した。これはローカル URL 1件と模擬ランタイムでの確認であり、実 WebView や全 URL に対する検証ではない。
- `cargo test --locked --offline` は本体 51 passed・8 ignored、別のパターン照合 1 passed。`widget/src-tauri/src/main.rs` SHA256 `C13F983C457CA6AB838D0E4A8F282D7579679023982B2D2A1D721F3434E52678`。テストログは一時フォルダーの `github-widget-rust-test-a946b76119bc4ca49188f7a65c7cdfb6.log` にある。
- リリースビルドは終了コード 0、実行ファイル SHA256 `9ADDDEBF722F006BE92D76396ECE43B87D0C6AB5B531C0B544D18CA7771C5769`。Windows 11 で設定画面の表示と Alt+F4 後のウィンドウ消失を確認した。ビルドログは一時フォルダーの `github-widget-build-6218446ba5834ea9bf6583aacc2baa33.log` にある。
- 前回の別テスト実行ファイルの起動失敗について、Windows ログの限定範囲では該当記録が見つからず、直接原因は引き続き未特定。

## 外部リンクの許可範囲の照合（同日）

- `capabilities/default.json` の `opener:allow-open-url` に設定されたパターンを、使用中の Opener と同じ `glob::Pattern` で評価するテストを追加した。GitHub の HTTPS URL 2件を許可し、HTTP、類似ドメイン、ユーザー情報を含む URL、ポート指定、別ドメイン、ファイル URL の6件を拒否することを確認した。これは設定値の照合であり、実 WebView の IPC や OS における拒否を証明するものではない。
- `cargo test --locked --offline` は通常テスト 50 passed・8 ignored、追加の照合テスト 1 passed。テスト `opener_scope.rs` SHA256 `DF59084DE6E957DC0D8B971739F03D55015B96F1ECAC365F546D4FDE3483CA38`、capability SHA256 `FC67C4593DCFA5D79C09E08D7F1FC5CC3474FF343E74D4095719CBF83C67B564`。テストのため `glob` を開発依存に追加した。
- 追加で実 capability と Opener を使う模擬 IPC テストを試みたが、コンパイル後、テストプロセスが `0xc0000139 STATUS_ENTRYPOINT_NOT_FOUND` で開始前に異常終了した。原因は未特定で、IPC による拒否は未検証。動作しない試験コードを除去した後の照合テストは 1 passed、現行テスト SHA256 `8C7C7AF3E8FFC4F867FA8294F4AC9D0135EE2385629F7FE4DFA0191C01DF493B`。この最後の実行は照合テストのみで、全テストの再実行ではない。
- リリースビルドは終了コード 0、実行ファイル SHA256 `F35B1721236B59567D92745D15EC890CBC9F68973C6F54C17483549042D49A10`。Windows 11 で接続設定画面の表示と Alt+F4 後のウィンドウ消失を確認した。テストログは一時フォルダーの `github-widget-rust-test-4bdffd7b3fef4f9e94b39fdcc4354cf8.log`、ビルドログは `github-widget-build-f53c924926a2463da49fd44ac0695577.log` にある。
- 親 Issue の共有チャット参照先は Web の抽出結果に会話本文が含まれず、原資料は未確認。Issue 記載の要件と区別する。

## 接続失敗後の入力保持（同日）

- 接続設定の保存が失敗し、保存済み設定を再取得した場合も、入力済みの接続先を保持するようにした。失敗と保存済み設定の再取得をモックした画面テストを追加し、`npm.cmd exec -- vitest run` は 28 passed。
- `widget/src/main.tsx` SHA256 `18A68A2059C80E6D41FD07DAA7A4B90435BC31149DB90CD28B68308DC83CCC11`、テスト SHA256 `04FE83598067B75DE9B91064D9DE39F82EEC47B9EDB54121ED319BB1D9338527`。リリースビルドは終了コード 0、実行ファイル SHA256 `5E16999B0666CDC9C81F1B3B5ED4CFA18E4D485E70B89BCF63A50CAA5710E9FE`。Windows 11 で接続設定画面の表示と Alt+F4 後のウィンドウ消失を確認した。実 GitHub による失敗時動作は未検証。
- `widget/src` と `widget/src-tauri/src` の TypeScript・CSS・Rust 計6ファイルを、一般的な GitHub トークン形式の2種類のパターンで検査し、一致ファイルは0件だった。他の形式、他のファイル、実行時の設定・ログへの漏出はこの検査で確認していない。
- テストログは一時フォルダーの `github-widget-vitest-9b93d7bcc73449ef8797350dbdec6e5b.log`、ビルドログは `github-widget-build-64330638b9124360b94c559a3a5e591e.log` にある。以下の画面操作記録は過去版を対象とする。

## 設定画面の再読込みと保存中の保護（同日）

- 設定画面を開いた際の遅い `connection_info` 応答について、入力済みの値を保持し、保存やボード復帰後には古い応答を反映しないようにした。設定の読込み失敗後は入力を保持したまま再試行でき、成功時は読込みエラーを消す。設定の保存・削除中は入力や設定再読込み、ボード復帰を無効にする。
- `npm.cmd exec -- vitest run` は 27 passed。遅延応答、再試行、保存中の操作制限はモックで確認した。`widget/src/main.tsx` SHA256 `1C7F4292A293912C0BCE22D3AA0DBC5225979BD1929EF2451F93D3AD90D0BF32`、テスト SHA256 `F74C783DBD4AF8DA1D74F2D8BA84F957A1FF6A033D3D266B47EDEAB047B08ED2`。
- リリースビルドは終了コード 0、実行ファイル SHA256 `769B228EDF99436DBF59D275A20EC1D136DD52374FEDED798AC4C2F7E9788162`。Windows 11 で接続設定画面の表示と Alt+F4 後のウィンドウ消失を確認した。実 GitHub と実際の WebView 遅延による競合検証は未実施。
- テストログは一時フォルダーの `github-widget-vitest-2d737b99a9c445109638dcbbf68ae8d1.log`、ビルドログは `github-widget-build-3a35af3c34274ece8a2beaf674911cc2.log` にある。以下のウィンドウ操作の記録は過去版のものであり、この版での移動・サイズ変更・外部リンクを検証済みと扱わない。

## 起動時の入力保護（同日）

- 同じ実行ファイルで Windows の右下端を自動ドラッグしたが、操作前後の画像寸法は 902 × 522 のままだった。自動入力の到達とリサイズ処理の実行は未確認であり、この版でのサイズ変更は未検証。試験後は Alt+F4 で終了し、ウィンドウの消失を確認した。
- 同じ実行ファイルの設定画面で「GitHub を開く」を押した後、既定ブラウザー Brave のウィンドウ名が `GitHub - Brave` になったことを確認した。アプリ側にエラー表示はなかった。ブラウザーの実 URL と禁止 URL の拒否はこの操作では確認していない。
- 起動時の `connection_info` 応答が入力途中で返っても、編集済みのオーナー・リポジトリ・トークンを上書きせず、設定画面を閉じないようにした。未編集項目とトークンの保存状態は取得結果から反映する。遅延応答を模擬する画面テストを追加し、`npm.cmd exec -- vitest run` は 22 passed。
- `widget/src/main.tsx` SHA256 `01E1305A309FFDC713803D64B014EF709E2E36ABD374CF176D755291EB1624AA`、画面テスト SHA256 `821F7F92DDF29B9BE84DC422D3A36EF9E4936D112E3CDE556E1A6C7411CF4613`。リリースビルドは終了コード 0、実行ファイル SHA256 `379447F38051D24C92E3BBF2E44FEA6B9D26AB8D03E8D5F316D6DE3ADD96DAC3`。Windows 11 で接続設定画面の表示と Alt+F4 後のウィンドウ消失を確認した。実 GitHub や実際に遅延させた WebView での競合再現は未実施。
- リリースビルドログは一時フォルダーの `github-widget-build-0981b6d31e55498da37de8b2970a9c11.log`、画面テストログは `github-widget-vitest-40d82344afe5449294d8623f3bab8cc4.log` にある。

## 設定取得の競合修正後（同日）

- 起動時の古い `connection_info` 応答が新しい接続設定を上書きしないよう修正し、遅延応答を再現する画面テストを追加した。`npm.cmd exec -- vitest run` は 21 passed。`npm.cmd run tauri -- build --no-bundle` は終了コード 0。
- `widget/src/main.tsx` SHA256 `2F219540DCB115BB3F0FF7ED74CF54A06E15A1CE4EC9DC34B1777A4A7C3258FD`、画面テスト SHA256 `510BD09920193B79275A8C27607BD80102DF6265D5C1DA6DFA3485C7450C095F`、リリース実行ファイル SHA256 `AF75BD3E8E196B5C3464D62FA081752309A4201BE8DAC1C454C55F75F29C74F4`。Windows 11 で実行ファイルを起動し、接続設定画面が表示されたことを確認した。実 GitHub による競合再現は未実施。
- 終了ボタンの自動操作で座標情報を取得できず、今回の同種2回目として停止した。Alt+F4 送信後、対象ウィンドウの消失を確認した。リリースビルドログは一時フォルダーの `github-widget-build-266f25b72aed46baac0c0d161716cfe6.log` にある。

## リリース版の再確認（同日）

- Windows 11 Home build 26300 で、記録済み実行ファイル SHA256 `DED93B43736E261CA732118A4475BD228CEBA0ABBCBB47310BB35B3DC455F10E` を起動すると、設定画面の代わりに localhost の `ERR_CONNECTION_REFUSED` が表示された。実行ファイルがその状態になった原因は未確定である。
- `npm.cmd run tauri -- build --no-bundle` を Visual Studio 2022 の x64 開発環境で実行し、終了コード 0。新しい実行ファイルの SHA256 は `649564F83DAEA35F8DA1586DD2978D02C955C5AEDD0C3C220EEBAB6B65EC949A`。通常のユーザー環境で起動し、`http://tauri.localhost/` の接続設定画面、トークン権限の案内、最前面ボタンの表示切替、画面上の終了ボタンによるウィンドウ消失を確認した。
- 最前面ボタンは表示状態の切替のみ確認し、Windows の最前面属性や他ウィンドウとの重なりは未確認。接続設定の入力・保存、GitHub への実通信、移動・サイズ変更、リンクはこの版で未確認。ビルドログは一時フォルダー内の `github-widget-build-a714f1723c844b9282ac4ed93ce8a049.log` にある。

## 追加検証（同日・現行版）

- 現行 `main.rs` SHA256 は `91E544E355597D8A90EE0D26E9A5E12DD7433AB7F2191CE9F787B318AE8F2835`。テスト用 IPC の service・ID 検証と、資格情報書込み失敗時の固定カテゴリ診断を追加した。通常 Rust テストは 50 passed、8 ignored。release build は成功し、生成物 SHA256 は `DED93B43736E261CA732118A4475BD228CEBA0ABBCBB47310BB35B3DC455F10E`。この現行版の GUI 起動・操作は未確認。
- 別途、通常 Windows ユーザー環境の独立 fixture と合成資格情報で mock IPC の接続情報・削除テストが 1 件成功した。試験終了後、同ユーザー・セッションからの読み取りでは対象の合成資格情報は見つからなかった。詳細と検証範囲は [実行記録](agent-runs/20261120-independent-check/ipc-resume-execution.md) を参照する。実 WebView、実 GitHub、製品用資格情報はこの試験の対象外。
- 以下の「対象版」「最新リリース版」はこの追加検証より前の版を指す。過去の実機確認を現行版での確認と扱わない。

## 対象版

- ソースは `widget/` の未コミット変更。基礎は `ab6122ef26bd62d5ec8dffbda1d35cd1a09606e0` の `widget/` で、その後に接続スナップショット確認、低い画面向け CSS、画面テストを追加した。
- `widget/src-tauri/src/main.rs` SHA256: `5DE67CA065A45C3637B4DD0755D6ED9846E57C903ED93099214E9C03A16C0022`。`widget/src/main.test.tsx` SHA256: `8D80DC270044669A98BFBEEABDF4B670DEF8CC64EB184FC917814CD55D3EEB64`。`widget/src/style.css` SHA256: `EBD74154CE93D6A8E731FF48D48CF0EBCE899568A6EB0D5FC3EB7CABB8D302B5`。
- リリース実行ファイル `widget/src-tauri/target/release/github-issue-widget.exe` SHA256: `EE2F9A0839FD6B1A08D0062992B8BBAEABA8A0DD17F715453BB192B06D15025B`。`npm.cmd run tauri -- build --no-bundle` は終了コード0。

## 自動テストと実機確認

- `cargo test --locked`: 49 passed、6 ignored。API エラー本文に含まれる合成トークンと Issue 名を画面向けエラーに返さないこと、通常の権限不足と二次レート制限を区別することを確認。別途、ignored の隔離競合テスト1件を単独実行して成功。実ファイルロック・合成 Credential Manager・別プロセスでの変更を通し、正常取得と古い結果の拒否を確認した。対象範囲は [実行記録](agent-runs/20261120-independent-check/permission-execution.md) と [独立レビュー](agent-runs/20261120-independent-check/permission-review.md) を参照。
- `npm.cmd exec -- vitest run`: 20 passed。タイトルバー・リサイズ端から Tauri API への呼び出し、旧接続の表示済み Issue をエラー時に消す動作をモックで確認するテストを含む。
- 別途、合成資格情報による Windows Credential Manager の登録・取得・更新・削除テストと、認証なしの公開 GitHub 2ページ取得テストが各1件成功した。これらは本対象版の一つ前の Rust ビルドで実施。
- 一つ前のリリース版 SHA `B7F9C5EF68D34B5D4CDBD211E671064DC5F49847723C823125561D70E03CB423` は隔離した一時設定で起動し、接続設定画面と PAT 権限の案内をアクセシビリティ情報で確認した。終了ボタンの自動操作は座標情報を取得できず未確認。その後、専用プロファイルの作成時刻と同一 OS ハンドル上の実行パス・高精度作成時刻を照合し、当該検証プロセスのみ終了した。最新リリース版の実画面操作は未確認。
- 最新リリース版 SHA `EE2F9A0839FD6B1A08D0062992B8BBAEABA8A0DD17F715453BB192B06D15025B` は、独立した一時 APPDATA / LOCALAPPDATA で非表示起動し、8秒間の生存と隔離設定のロックファイル作成を確認した。起動時に取得した同一 OS ハンドルを使って検証プロセスを終了し、終了を確認した。画面表示や通常の終了操作はこの確認に含まない。
- ユーザーは以前の release SHA `C2284679034F82506DC3855C5C2B8173975284A19AAC54C76BDF3C9F43F25A90` で移動とサイズ変更を手動確認した。低い画面向け CSS を含む SHA `C5F95443883A1A2782429342D0F62F8EA63EDE55C0366F16F00DD1409B88B79B` では最小付近の設定項目表示を確認した。詳細と診断版の限界は [ドラッグ診断記録](drag-diagnostic-20261120.md) を参照。
- 開発モードはコンパイル成功、開発アプリのプロセスと接続設定のアクセシビリティ情報を確認した。非表示ランナーの画面終了操作は失敗し、所有プロセスを終了したためランナーは `-1` で終了した。開発版の通常終了・実画面操作は未確認。

## 残る検証

- 実 UI での PAT 登録・更新・削除、再起動後の読取り、接続先切替と旧応答の抑止。
- 実行中のアプリからの Issue 作成・状態変更、通信障害・書込み権限不足・レート制限・競合時の表示。既存 run の実 GitHub テスト結果はあるが、本対象版の実 UI を証明しない。
- 最前面の OS 属性、許可 URL の既定ブラウザ起動・拒否 URL、各 DPI・文字拡大率と Windows の論理最小サイズ。
- 実機での秘密情報が応答・設定・ログ・リポジトリに漏れないことの全体確認。ソースと単体テストの確認だけで完了扱いにしない。
- `widget/` は未追跡、README と失敗記録などは未コミット。既存の `docs/agent-runs/` も未追跡であり、所有・保持を確認せず削除しない。
