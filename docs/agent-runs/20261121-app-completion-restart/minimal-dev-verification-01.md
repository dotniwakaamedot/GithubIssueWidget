# 開発起動の最小確認（2026-11-22）

担当 `/root/minimal_dev_verification`。作業場所 `D:/Projects/GithubIssueWidget`。**開発起動は実施していない。** 現在の利用中アプリを妨げずに開発版の WebView・画面と全子プロセスを安全に隔離・終了できることが確認できず、指示された実行条件を満たさないためである。これはアプリの起動失敗を示さない。

## 確認した事実

- `git rev-parse HEAD` は `4961b25bec53a93439acdf5230de77f959908af0`、終了 0。HEAD は未コミット内容を識別しない。
- 現物 SHA256 は `widget/src-tauri/src/main.rs` が `AEFF1E65AD93EFA04773CEBBB9736A6BDC5B9938DB9FAF937DD88B3D9CBFE86F`、`widget/src/main.tsx` が `B22504C9DE48D275726C912820DA93E02F10C1838CCC68555C156D35030165D7`、`widget/src-tauri/tauri.conf.json` が `0132FCC92345F5D60DE84FD25197FCE853044AD63DAE28757AB4592CC72B991E`、`widget/package.json` が `102235B5686088C87B3221DEC7AE3CBEE20F0C9B1EAE070FD730C8B631DFCEDF`、`widget/vite.config.ts` が `1DAFF9E73019BD4E74489811D13553B3D227DB8457910D7F295E60AD0EB4CC7A`。
- 読取り時点の PID 65144 は session 2 の `widget/release-20261121-selection/github-issue-widget.exe` を示した。プロセスを操作していない。観測後の状態、ウィンドウや全子孫は未確認。
- `netstat -ano -p tcp` の対象文字列を限定した観測では `:1420` に一致する行は 0、PowerShell 呼出し全体は終了 0。`netstat` 自体の終了コードは個別記録していない。これは観測時の限定結果であり、後続のポート予約や全アドレス族での排他の証明ではない。
- UI 起動時の `connection_info` は APPDATA 内のロックと復旧記録、設定を読む。静的に、新規の空 APPDATA で復旧記録・設定が存在しなければ `recover_at` は削除クロージャを呼ばず戻り、`read_token("")` は keyring の entry を作らず戻る。空 APPDATA の排他・維持、実際の GUI の入力経路、実行中の設定変更、WebView2 のデータ先と親子孫の終了は未実証であり、APPDATA の切替だけで実行全体を安全と断定できない。
- `npm.cmd run tauri -- dev` は `beforeDevCommand` から Vite と WebView2 を起動する構成で、Vite は 1420 固定・strictPort。既存 run の `dev-start-check-result-01.md` が記した起動経路と対象版は確認した。実コマンド、GUI、プロセスツリー、正常終了は今回観測していない。

## 未確認と次の条件

ユーザーの既存アプリが作業中のままなので、別ウィンドウのフォーカスや実サービスへの影響、見えない子孫の停止を推測しない。開発起動を追加検証する際は、本人の利用中アプリに干渉しない状態で、空の専用 APPDATA・LOCALAPPDATA と WebView2 データを対象に、実際のコマンド・ポート・子孫・ログと停止方法を確認する。Credential Manager の既存サービスに触れない保証や安全な分離を確認できない場合は実行を保留する。開発 GUI の実起動・正常終了は未検証であり、release ビルド成功とは区別する。

本担当は製品・既存設定・資格情報・GitHub を変更せず、開発アプリを起動していない。読み取りの各呼出しは終了 0（上記 `netstat` 自体の終了コードを除く）、認識した失敗なし。変更は本結果ファイルのみ。削除対象なし。
