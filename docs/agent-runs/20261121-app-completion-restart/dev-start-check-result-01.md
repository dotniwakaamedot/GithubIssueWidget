# Windows 開発起動の事前確認

## 結論

**現行版の開発起動・正常終了は実施していない。** 検証中、ユーザーが実機確認に使用した修正版アプリのプロセスが同一セッションに存在し、別の開発ウィンドウを表示して画面や実サービスに干渉しないこと、全子孫を識別・終了できることを立証できなかった。依頼の安全条件に従い起動を保留した。これはアプリの起動失敗を意味しない。

## 対象と要件

- 担当 `/root/dev_start_check`、依頼 `dev-start-check-request-01.md` SHA256 `36885ACF85DC650315C5C1B09F983281DFD96DC7BC46790CA1C5C0135E290C2C`。作業場所 `D:/Projects/GithubIssueWidget`、branch `main`、HEAD `4961b25bec53a93439acdf5230de77f959908af0`。HEAD は未コミットの変更を識別しない。
- `widget/package.json` SHA256 `102235B5686088C87B3221DEC7AE3CBEE20F0C9B1EAE070FD730C8B631DFCEDF`、`widget/src-tauri/tauri.conf.json` `0132FCC92345F5D60DE84FD25197FCE853044AD63DAE28757AB4592CC72B991E`、`widget/src-tauri/src/main.rs` `AEFF1E65AD93EFA04773CEBBB9736A6BDC5B9938DB9FAF937DD88B3D9CBFE86F`、`widget/src/main.tsx` `B22504C9DE48D275726C912820DA93E02F10C1838CCC68555C156D35030165D7`、`widget/src/style.css` `FFE871A15BDA8FAD0BB5E28E1717C93D2878ABB1F5885F730EC624DEC983FCFE`、`widget/vite.config.ts` `1DAFF9E73019BD4E74489811D13553B3D227DB8457910D7F295E60AD0EB4CC7A`。設定と CSS は未コミット版。
- 承認計画 `plan-approved-03.md` SHA256 `355CD3D07C9CEAD72576CACC02D31B27484F7C165AEAB967E2FBCC548DD175C1` と `plan-review-03.md` の承認を確認。公開 GitHub REST から #2 と #7 を番号指定で取得し HTTP 200、両 Issue は Windows 上の開発起動とリリースビルドを要求する。リリース版の起動成功を開発起動の証拠に代用しない。

## 実行経路と安全条件

- README の通常経路は `widget` で `npm.cmd run tauri -- dev`。Tauri CLI が `beforeDevCommand` の `npm run dev` を通じて Vite を起動し、`http://localhost:1420` を読み、Rust の開発アプリと WebView2 を使う。`vite.config.ts` は 1420 の strictPort。Cargo、rustc、リンカー、Node、WebView2 等の子孫を伴い得る。ここではコマンドを実行しておらず、実際のプロセスツリーは未観測。
- `main.tsx:62-65` は起動時に `connection_info` を呼ぶ。Rust 側は設定ロック取得、`recover_pending`、設定読取、必要な場合の keyring 読取を行う。設定パスは `APPDATA/github-task-widget/settings.json`、通常の Credential Manager service は固定の `github-task-widget`。service の環境変数切替は `cfg(all(test, windows))` の試験用分岐であり、通常 dev binary に一般化できない。
- 新規専用 APPDATA を与えて未設定で起動する案は、既存設定に触れる可能性を減らすが Credential Manager 自体は別ユーザー領域に変わらない。空であることの保証、設定回復の副作用、専用権限、WebView2 のユーザーデータ先、画面フォーカスと子孫の終了は別に立証が必要。既存ユーザーデータ・実 PAT を取得・表示・変更していない。
- 2026-11-21 15:33 JST 頃 `Get-Process -Id 39224` は session 2 の `github-issue-widget` を返し、パスは `selection-build-sandbox-01/cargo-target/release/github-issue-widget.exe`。存在確認時点の情報で、その後の稼働は保証しない。多数の node と WebView2 も表示されたが、親子関係や所有者は識別していない。既存プロセスを操作していない。
- `netstat -ano -p tcp` を Python で実行し終了 0、文字列 `:1420 ` を含む行は 0。これは観測時点・該当文字列のみで、将来のポート確保や DNS/IPv6 を含む完全な排他を保証しない。作業時の競合状態の再確認が必要。
- 旧別版 EXE の終了 101 は stderr 本文が失われ原因不明という `../20261121-orchestration-continuation/startup-independent-investigation-20261121.md` を確認。現行の修正済 EXE と異なるため、現版の開発起動失敗と同一視しない。再実行前に秘密を含み得る出力の保護・捕捉と失敗分類、既存プロセスに影響しない監督を具体化する。

## 再開条件

1. ユーザー側アプリと他担当の起動処理について状態・所有者・ウィンドウを確認し、ユーザーの作業を妨げない専用 Windows ユーザー／セッション等を用意する。既存のアプリや子孫を推測で停止しない。
2. 専用のソースと依存、APPDATA／LOCALAPPDATA、実在する起動ラッパー、Credential Manager service、WebView2 データと 1420 番ポートの隔離・権限を検証する。通常コードの service は固定であり、単に APPDATA を変更しただけで credential isolation を主張しない。実認証情報や既存 service を読まず、公開先やログに機密を出さない。
3. 実際の `npm.cmd`・Node・Tauri・Cargo・Rust 実行体と対象入力 SHA、終了コード、ログ保護、プロセス生成前の登録・識別・監督・停止／親停止時の扱いと子孫終了証拠を計画し、実際の制約と合成試験を独立確認する。不明なら送信・起動・他者プロセス終了をしない。隠れた別窓の生存だけで GUI の操作や正常終了を主張しない。
4. 必要に応じて親から本人へ明確に案内して可視 GUI を確認し、操作前後の対象・終了・未確認を記録する。現在の制約では完了条件は満たしていない。

## 実行・失敗・変更

- 作業場所は上記。`git branch --show-current`、`git rev-parse HEAD`、`Get-FileHash`、対象ソースの限定読取と `rg`、`netstat`、公開 Issue 番号指定 GET はそれぞれ終了 0。JSON は ASCII エスケープ表示で秘密を含めず取得。`Get-Process -Id 39224` は終了 0。
- `Get-Process -Name` の複数名指定は、存在しない名前を `-ErrorAction SilentlyContinue` で抑制して一覧を返したが終了 1。対象一覧の不在や全プロセスの不存在の証明には使わない。`Get-NetTCPConnection -LocalPort 1420 -ErrorAction SilentlyContinue` も終了 1・無出力で、アクセス拒否／該当なし等の理由を区別できなかった。後者の代替 `netstat` は上記条件で終了 0。双方の診断・累計記録は管理担当に引き継ぐ。
- `rg --max-columns-preview` を用いた2つのソース・教訓の検索は一部長行を省略した。省略行全体を確認したとは扱わず、必要な起動経路は限定行読取りで確認した。取得済み以外の全文・検索対象外は未確認。
- 最初の UTF-8 代表文字検査は実際の文面にないアンカーを指定したため `valid=False` となった。文字破損の証拠ではない。実際に存在する「結論」「未検証」などのアンカーと置換文字なしを厳密 UTF-8 読取りで再確認し終了 0。誤指定1件は管理に引き継ぐ。
- 新規成果は本結果ファイルのみ。確定後の SHA256 は管理側で実体を取得する（自己参照回避）。一時ファイルから公開し UTF-8 を読戻す。製品、Git、既存ログ、設定、資格情報、アプリ、GitHub に変更なし。削除対象なし。実アプリ・開発サーバー・ビルド・外部書込みは起動していない。全作業における秘密非露出とアプリの実 UI・開発起動・正常終了は未検証。
