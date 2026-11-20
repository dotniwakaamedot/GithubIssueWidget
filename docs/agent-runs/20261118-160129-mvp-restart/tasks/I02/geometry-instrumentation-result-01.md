# I02 診断版の DOM 境界観測 修正結果 01

- run / task / 担当: `20261118-160129-mvp-restart` / `I02-geometry-instrumentation` / `/root/drag_design`。依頼 `tasks/I02/geometry-instrumentation-request-01.md` SHA256 `FE760D81FD0C614E2BAA547C139D39596AB9A4DFCD22AF490FEDECDE43C8CE4E`。依頼の13参照は基点を分離し存在確認後に実体 SHA と照合、全件一致。
- 結論: **同一 CSS viewport 座標による矩形観測の診断コードを新しい専用コピーに用意した。実行・型検査・画面表示・入力は未実施。** 独立レビューと親判断まで新診断版を起動しない。取得や accessibility 公開の成功は未確認。画像と AX の不一致の原因や製品版のドラッグは未解決。
- 元 worktree `D:\Projects\GithubIssueWidget-issue-2-20261118-160129`、branch `task/issue-2-foundation-20261118-160129`、HEAD `a4ac97286785a2a5ba0d070a909b58003fc49ff1`。共有 `tasks/I02/evidence-drag-click-01/source-manifest.json` の元17件を対象に元 worktree と旧 scratch の全34ファイルを存在・SHA 照合し、元17件は期待 SHA と一致。元版との差は既報どおり元14件不変・3件変更、別途診断1件新規。
- 旧 scratch `D:\Projects\GithubIssueWidget-drag-diagnostic-20261118-160129` の18対象ファイルを通常バイトコピーし、新 scratch `D:\Projects\GithubIssueWidget-geometry-diagnostic-20261118-160129` に保存。新旧各18件の path / size / SHA は新 scratch `copy-manifest.json` SHA256 `F75B6663167FADC32277BE610DAA67C1C3B96B7FC74BA1AEB6890C9EF3C56DC1` に記録。親 `D:\Projects`・各対象祖先の reparse がないこと、旧18ファイルの hardlink list が各1件、専用先未存在・親 ACL owner と空き約132GBを確認してから作成。既存 node_modules、生成物、B03 scratch はコピーせず、元・旧 scratch と異なる専用領域。コピーの安全条件は時点限定で排他を保証しない。
- 新旧18ファイルのうち17件はコピー後も SHA 一致。変更は `widget/src/diagnostic.ts` 1件のみ（旧 `56A62632E8D63AEE5DD0AB636C954E30FBB76562C90C88CD9C0D53AB497B6D37`・1775 bytes → 新 `840D7F69349AEED52C0A5D6FA6319317CCE432390A74463F8E824069F26E412D`・4624 bytes）。元からの他3変更は `main.tsx` `820F164EF8947DA6B87F6ACE8B74D787E09ED9AF7A6610E2C07678DFA2C4AE5C`、`vite.config.ts` `194D17A739D0F626DDAF94B5D2B6F9335DBF50BB571AB4F7739DA727ED4F772A`、`tauri.conf.json` `EDE0EA3E1EE5C6FE067A5E127EEAAE02A19110E8CD8E057C8B7FCB2B826A55D5`。既存 dev URL / Vite port 1435 は変更していないため、旧版と同時起動できない。実施前に所有者・ポート・プロセスの調整と別依頼が必要。

## スクリーンショット観測レビュー B01 への対応

`diagnostic.ts` に第2の `output`（固定 `aria-label="diagnostic-geometry-record"`、`pointer-events:none`）を追加し、1秒ごとの同期的 DOM 読取りで JSON `geometry-ready:...` を表示する設計。値は header 1、固定 marker 1、header button 3、resize 帯8の `getBoundingClientRect()`（left/top/right/bottom の CSS viewport px）、`innerWidth/innerHeight`・devicePixelRatio・scrollX/scrollY、UTC milliseconds・単調開始/終了時刻、marker が header 内か、marker 中心の `elementFromPoint` が marker 自身か、最小余白、計測中の header/marker/viewport の安定フラグ。対象不足（buttonが3件・resizeが8件でない、header/markerなし）は `geometry-unavailable` を出す。DOM text・URL・キー・Issue 内容は取得しない。既存 `diagnostic-event-record` と event 項目・最大20件は変更せず、出力自体は入力を受けない。

同じ `captureGeometry()` の同期呼出し中に全矩形を取得するため別々のイベント時点を混ぜない。開始・終了時刻と、前後で viewport・marker・header の再取得が一致したかの `stableDuringSample` を提示する。ただし各要素を順に読むので物理的な完全同時性や button/resize の途中変化がないことを証明しない。DOM の layout shift、スクロール、フォントやユーザー操作、別窓による遮蔽は残る。取得時の layout 強制と1秒更新、長い output による再描画・AX 負荷は診断の副作用であり製品版の挙動に流用しない。診断版は元版 SHA と異なり、診断版の成功は過去のドラッグ・製品版・Issue 完了を証明しない。

## 判定と実操作前のゲート案（今回未実施）

1. 親は新版のコード・差分、依存・profile・port と生成 exe SHA を固定し、作者以外のレビューを得る。旧版の handle 22902 / 17031 と PID 21312 はこの担当が開始時に poll・read-only で live と確認したが、最新状態・起動許可・別環境は確認し直す必要がある。旧プロセス・B03 の状態を勝手に変更しない。port 1435 を共有しない。
2. 実施時は新しい app の main WebView、window ID、PID と exe の絶対 path/SHA・実行時刻を一意に照合し、対象個別の `diagnostic-event-record` に `diagnostic-ready:[]`、`diagnostic-geometry-record` に `geometry-ready` の JSON を同一窓で読み取れることを入力前に確認する。候補が複数・出力欠落・記録不可なら入力しない。実際の公開可否、JSON の欠落・長文の切り詰め、時刻対応は未検証。output が読めなければ推測で代用しない。
3. JSON の `stableDuringSample=true`、3 button / 8 resize、空でない矩形、`markerInsideHeader=true`、`markerAtCenter=true`、スクロール0、正の viewport と DPR、`minimumGapCssPx>=12` を実値で確認。最小余白は各矩形ペアで `max(button.left-marker.right, marker.left-button.right, button.top-marker.bottom, marker.top-button.bottom)` を求め、全button/resizeペアの最小をとる。非重複側の一軸で12以上あれば少なくともその軸に12 CSS px の分離があり、負/0は重複・接触。count・境界・hit target が想定外なら停止する。サンプルを保存し hash、取得時刻・PID・ID・対象版と紐付ける。`elementFromPoint` は DOM 上の最上位要素の確認で、他の OS 窓による遮蔽や別 desktop を証明しない。
4. 複数回観測する場合は自動更新された独立サンプルの viewport・全矩形が安定していること、サンプル時刻と実際の画面との前後関係を確認し、移動・resize・スクロール・内容変化があれば破棄して停止する。DPI / CSS-to-client-to-screen・操作APIの座標系は実測し、変換と対象窓・実画面の対応が証明できないときは入力しない。過去の画像が背景であった不一致を AX の存在だけで解消扱いせず、同種2回目で観測を止め別担当調査、3回目で全停止。`cua.getState()` / browser inventory は既に2回同種 error があり再実行禁止。
5. 直前にユーザー操作・他窓との干渉と marker の非対話性、実画面・PID・exe・画像由来座標と DOM 情報の対応・12px 余白・空イベント取得・保存可能性を確認し、親が具体的な単一押下解放を認めた場合に限って1回のみ実行する。対象が不明、結果が空・失敗・読取不能なら反復せず停止。元ドラッグ、移動、resize は行わない。

## 実施記録と未確認

- cwd `D:\Projects\GithubIssueWidget`。依頼構造化参照13件は各基点・相対パスの存在確認後に `Get-FileHash -Algorithm SHA256` で全件照合（exit 0）。`Get-Content -Encoding UTF8` で依頼、screenshot 観測設計・レビュー、旧診断コード・HTMLを読取（exit 0）。
- 所有する旧セッション `22902` と `17031` を各1回空 poll し両方 session ID が返った。`Get-Process -Id 21312` は exe path と MainWindowHandle `267184` に一致（exit 0）。poll は経過時点の生存であり以後の生存・対象窓の存在・実デスクトップの非競合を保証しない。
- `fsutil hardlink list` で旧対象18件の link 数各1、PowerShell の `Test-Path` / `Get-Item` で対象祖先の reparse point なしと専用先未存在を確認。新 scratch に `[IO.File]::Copy` で18件通常バイトコピーし、全件旧版 SHA とコピー時に一致（exit 0）。コピー後の `copy-manifest.json` と旧/新18件照合は exit 0。共有結果のみを新規作成し、旧 scratch、旧 process、元 worktree・既存証跡・Git・GitHub は変更していない。削除対象なし。
- 今回の担当作業で新たな失敗なし。診断版の起動・依存導入・build・通信・画面 capture・入力・移動・UI 操作は行っていない。`GetWindowRect` 等の OS 計測、DOM値の実行取得と accessibility 表示、実座標、画像の一致、テスト成否、製品版の移動はすべて未確認。実際に利用可能な計測と判断は別担当の独立レビューと親判断を待つ。結果確定後の SHA は親が実体から算出する。
