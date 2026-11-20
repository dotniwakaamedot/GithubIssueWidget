# I02 旧 geometry 終了と marker 診断版の観測結果 01

- run / 担当 `/root/drag_design`。依頼 `tasks/I02/marker-execution-request-01.md` SHA256 `F153D05C2ECD6416DEAE23214F151B830687651B19D1AE28CBD8215F3F98E098`。依頼の構造化参照16件を基点・相対パスごとに存在確認後 SHA256 照合し全件一致。
- 結論: **診断版の AX から CSS 上の最小余白15px と空イベントを1回観測したが、画面画像・非遮蔽・OS 座標との対応が未確認のため入力は実施していない。** これは製品版のドラッグ成功、過去2回の原因特定、Issue 完了の証拠ではない。
- 対象 `D:\Projects\GithubIssueWidget-marker-diagnostic-20261118-160129`。source の対応は `copy-manifest.json` SHA256 `CA678FB46C1CF89C34C44C5EF7A22E900E391AA13D72E961430AE32CE2FA916D` の18件と main.tsx 変更版 SHA256 `DE50084BEEA52147D7B3482AC84ADEFE70AA2C5E8F211BDF1930A8AED7C1D1DD`。`diagnostic.ts` SHA256 `840D7F69349AEED52C0A5D6FA6319317CCE432390A74463F8E824069F26E412D`、vite config `194D17A739D0F626DDAF94B5D2B6F9335DBF50BB571AB4F7739DA727ED4F772A`、tauri config `EDE0EA3E1EE5C6FE067A5E127EEAAE02A19110E8CD8E057C8B7FCB2B826A55D5`。元 worktree branch `task/issue-2-foundation-20261118-160129` / HEAD `a4ac97286785a2a5ba0d070a909b58003fc49ff1` の未コミット17ファイルは従前の `marker-adjustment-result-01.md` を参照、HEAD だけでは対象を特定しない。

## 旧 process・新環境

- 旧 geometry app session `4498` と server session `68650` は停止前 poll で live。`questions/I02-marker-execution-answer-01.md` の親判断に従い、直前に PID・親・開始時刻・path・app hash / server command line・port 1435 を照合し、担当所有 app PID `9796` を 2026-11-18T19:22:12Z、server PID `54736` を 19:22:25Z に個別 `Stop-Process`、各コマンド exit 0。両操作は **強制終了**。session は両方 terminal `-1`、読み取りで2 PID・既知 app child PID 59548 不在、1435 listener 0。通常終了とは扱わず、全ての別名子孫が不存在と証明するものではない。旧 scratch と既存証拠は保持。
- marker scratch の18件を copy-manifest と前記新版 main SHA に照合（exit 0）。専用 Rustup/Cargo home、npm cache/config、TEMP/TMP、APPDATA/LOCALAPPDATA、target と profile 用領域を作成。Rust 1.90.0 の公開取得・install exit 0、公開 registry 指定 `npm ci --no-audit --no-fund` exit 0、`npm run build` exit 0、VS 18 `VsDevCmd.bat -arch=x64` 後 `cargo build --locked` exit 0。npm の esbuild install script 未承認警告あり。取得と OS cache・system log 等の通常副作用があり得る。利用した依存の全安全性を保証しない。
- cwd は marker scratch の `widget`（npm / dev server）、`widget/src-tauri`（cargo）および scratch（rustup）。dev server session `19143` は port 1435 を表示、app session `70494` は実行中として返り、直近空 poll では両 handle が返っており terminal / 正常終了は未確認。app exe `target/debug/github-issue-widget.exe` SHA256 `54DBCECFD920F6F920942A190C81B28BD7742C6052ABB58AB9C89E1F71C37AA8`、PowerShell `Get-Process` は 2026-11-18T19:25:52Z 頃に同 path の PID `52960`・MainWindowHandle `6559504` を1件確認。既存ユーザーアプリや B03 の process は操作していない。所有 process と scratch は保持し、停止は別途管理判断を要する。

## 公開された AX 値と不足する条件

- `cua.listApps()` は対象 exe path と window ID `6559504` を返し、同 ID / PID / exe の確認後、`cua.getApp({windowId:6559504})` の AX tree は main WebView `localhost:1435`、`diagnostic-ready:[]`、`geometry-ready` を返した。禁止された `cua.getState()`、browser inventory、旧 screenshot、入力・クリック・ドラッグ・リサイズは行っていない。CUA 作業は completion summary 済み。
- AX 出力の**手動転記** `tasks/I02/evidence-marker-execution-01/observation-01.json` SHA256 `587782B7A273C718B059DF8336DF84FC39C636B39CAD0C17E69EFF52F8E74277`。非機密の対象識別・イベント空値・当該 geometry のみ。元のツール出力は会話上にあり、独立の生ログではない。JSON は同一ディレクトリ `.tmp` に作成、UTF-8 で構文・window ID・button 3件・resize 8件・最小余白15を検査、非上書き rename 後 SHA 一致を確認した。
- 当該1サンプル: viewport 900×520、DPR 1、scroll 0、marker `[360,20,424,44]`、上側 resize 下端5、`markerInsideHeader=true`、`markerAtCenter=true`、`stableDuringSample=true`、最小 gap 15 CSS px。全矩形は上記 evidence の手動転記に記載。これは対象の DOM 計算結果の一時点にすぎず、OS DPI awareness / client-to-screen / クリック API の座標対応、対象の実画像・遮蔽、user の操作との非競合、同時刻の全要素の安定を証明しない。
- 依頼が要求する **画像と AX の対象一致・非遮蔽、DPI と実座標** は未確認。前の対象外画像の原因も未確定。以前の同じ screenshot を再試行せず、クリック・押下解放を行わずここで停止。入力対象を推測しない。

## 次の作業・失敗

1. 独立担当が対象版・手動転記を元の tool 出力と照合し、対象画像・DPI・座標・非遮蔽の安全な取得方法と user 操作との競合を判断する。取得不可なら入力は未実施のまま保つ。所有 app / dev server の扱いは親が決める。
2. 今回の担当操作で新しい失敗なし。既往の直接確定逸脱2件、browser inventory failure2回、画像不一致1回、元ドラッグ失敗2回を保持。同種2回で該当操作中断・別担当調査、3回で全停止。
3. 共有 run 内の JSON evidence は `.tmp` → 構造確認 → 非上書き rename で確定。本結果も同様に公開する。元 worktree・旧/新 scratch の既存資産・Git・GitHub の変更や削除はしていない。生成した新 target・cache・toolchain・dist・証拠は保護し保持する。
