# I02 診断版の記録取得・安全領域の修正結果 01

- run / task / 担当: `20261118-160129-mvp-restart` / `I02-drag-instrumentation-revision` / `/root/drag_design`。依頼 `tasks/I02/drag-instrumentation-revision-request-01.md` 実体 SHA256 `78213D01E56433DE9F31E47E3B9A4410E20B6CD1F158DC986064CC6C2533C217`。参照と対象は依頼の構造化フィールドと前レビューを読み、診断版4ファイルの初期 SHA を依頼値と照合してから編集した。過去の質問03の15件という記述は誤り、正しくは **元17件中14件不変・3件変更、別途1件新規**。
- 結論: **B01/B02 の対処方法と診断版を準備した。起動・型検査・クリックは未実施で、取得や領域識別が実行環境で可能かは未確認。** 新版は独立レビュー・親判断まで起動・入力しない。元版のドラッグ再試行は禁止、製品の動作や Issue 完了を主張しない。
- 元 `D:\Projects\GithubIssueWidget-issue-2-20261118-160129` の branch `task/issue-2-foundation-20261118-160129`、HEAD `a4ac97286785a2a5ba0d070a909b58003fc49ff1`。`source-manifest.json` SHA256 `E5C0FA60CB40DF2E59F15FABEF884D545FDABC04076DBCD2C8FDD220367990DC` の17件について、元と scratch の全34パスを存在確認後に SHA を照合。元17件は期待どおり、scratch は14件不変・3件変更、診断コード1件新規。各変更は下表。削除対象なし。既存 run の証跡、B03 scratch、元製品・Git・GitHub は変更していない。

| scratch 相対パス | 元 SHA256 / 前診断版 SHA256 | 今版 SHA256・内容 |
| --- | --- | --- |
| `widget/src/main.tsx` | `07318B40089C01EEB897EFDB5A933DD15982C51E51CDAEABC097AC219C867AFE` / `F318C76FE54F081F55656B84C0C9680C1416560E272C003C480C6B7C03CA4FA2` | `820F164EF8947DA6B87F6ACE8B74D787E09ED9AF7A6610E2C07678DFA2C4AE5C`: header の通常 drag 呼出し除去を維持。固定の非対話診断マーカーを header 内に追加 |
| `widget/src/diagnostic.ts` | 元なし / `76163B634209ABE4976F2A954E463EDFAEF7AB97B110E0B1D96858135682ADDF` | `56A62632E8D63AEE5DD0AB636C954E30FBB76562C90C88CD9C0D53AB497B6D37`: 非機密イベントをメモリと画面上の output に記録 |
| `widget/vite.config.ts` | `3CF43B5EFBCB9255F4867F2F89CDDEFB2FD901F9788227B12745D5588A5EBE81` / `194D17A739D0F626DDAF94B5D2B6F9335DBF50BB571AB4F7739DA727ED4F772A` | 前版から不変: `194D17A739D0F626DDAF94B5D2B6F9335DBF50BB571AB4F7739DA727ED4F772A`、port 1435 |
| `widget/src-tauri/tauri.conf.json` | `3F736678BC54BFD18A13669721D150F39ADD16978B5CBD5E10B26EB407457DC6` / `EDE0EA3E1EE5C6FE067A5E127EEAAE02A19110E8CD8E057C8B7FCB2B826A55D5` | 前版から不変: `EDE0EA3E1EE5C6FE067A5E127EEAAE02A19110E8CD8E057C8B7FCB2B826A55D5`、devUrl 1435 |

## B01: 取得と識別の方法案

診断 module は開始時に `aria-label="diagnostic-event-record"` の output を `document.body` に作り、`diagnostic-ready:[]` を表示する。header 内の `mousedown`・`mouseup` の capture/bubble だけを最大20件保持し、type、phase、button、buttons、`isTrusted`、header/非button子/button の区分、header内の左右3分割、`performance.now()` を JSON として output の `textContent` に表示する。URL・本文・キー・Issue・DOM テキストは読まない。`pointer-events:none` とし、レコード領域自体を入力対象にしない。配列は `window.__dragDiagnostic` にも保持するが、外部から JS を実行する手段に依存しない。表示によるイベント処理時間の変化と版差は残る。

実施担当はまず静的に確認された4 SHA と残り14 SHA・依存 lock・専用 profile/port が一致する版を build し、専用 PID と実行ファイル絶対パス・SHA を作業記録に固定する。対象ウィンドウ ID と OS PID を取得し、PID→exe の一致、main のタイトル・範囲・時計、単一の候補であることを確認。未確認や複数候補なら入力しない。起動時刻・UTC と単調時刻の関連、OS・DPI・ディスプレイを記録する。デスクトップの他者操作・干渉が不明なら停止。

**入力前の取得手段ゲート:** 対象ウィンドウ限定のアクセシビリティ観測で output の `diagnostic-event-record` と正確な `diagnostic-ready:[]` が読み取れ、同じ対象 ID に紐づくことを確認し、空状態の対象範囲だけの証拠を保存する。output が公開されない場合でも対象窓の screenshot で空表示を判読し保存できるなら、目視可能な唯一の代替として管理が採否判断する。いずれも未確認・識別不能・データ取得できない場合は入力しない。WebDriver は `Get-Command tauri-driver` と `Get-Command msedgedriver` のいずれも本環境では検出されなかったため、存在や JS 読取能力を前提にしない。

入力後に同じ窓から output を読み、対象版 SHA、PID、window ID、記録時刻、1回分のイベントの配列を `evidence-drag-click-01/` 内の新規ファイルへ UTF-8 で保存し、その SHA256 と終了/読取状況を結果へ記載。取得の失敗・空のまま・target/button の想定外・データ混入・対象喪失なら反復せず停止して報告。処理が複数行に切れる・読みづらい場合も同様。元/診断版とイベントの対応は PID・SHA と時刻で記録し、単一クリックの到達だけを示す。取得が可能という設計であり、実動作は未実証。

## B02: 安全領域の選択方法案

diagnostic marker は `role="img"`・`aria-label="diagnostic-safe-area"` を持つ固定ラベルの span で、診断版で header の `left:40%`・`top:16px`・幅64px・高さ24px・z-index 11 に配置。クリックハンドラは持たず、矩形を視覚的に縁取る。これ自体が DOM・hit-testing を変えるので、ここでの到達は元版ヘッダーや過去ドラッグの証明ではない。

起動後・入力前に Win32 の PID 対応 HWND、`GetClientRect` / `ClientToScreen`、`GetDpiForWindow` と当該ウィンドウの可視枠を読み、DPI 仮想化を含む座標系を確認。同時に当該窓に限定した snapshot / screenshot で marker、header、nav の各 button、外周の resize 帯の境界を実測する。UIA が marker を公開しない場合、marker の視認可能な枠と画像座標・窓の client 境界を対応付けられなければ停止する。縮尺・複数モニタ・スクリーン座標と画像座標の対応が不明な場合も停止し推測しない。

header CSS は高さ56、resize 帯は上端5、四隅9 CSS px。marker の境界が header 内に収まり、marker と **すべての button / resize 帯が重ならず、各境界から最低12 CSS px の余白**を確認し、`left:40%` に基づく marker 矩形の中心（left+32、top+12 CSS px）を唯一の候補とする。DOM と UIA/screenshot の座標差、DPI変換、余白は対応する物理座標で再計算し、クリック API の座標契約と一致を確認。表示・測定・空 output の取得で安全と判断できない場合は入力しない。境界や重なりを静的値だけで成功扱いしない。marker は z-index 11 で既存の resize z-index 10 より上にあるため、実測で十分離れていることが特に重要。

上記ゲートがすべて満たされ、独立レビュー後に親が版と画面・所有を判断して初めて、対象窓の marker 中心で単一押下・解放を1回のみ行う。移動、ドラッグ、リサイズ、buttonの操作はしない。イベント不達や対象不明は繰り返さず保護し報告する。観測と起動時の画面変化、プロセス生成、専用 cache・OS の通常副作用は記録し、所有する診断プロセスだけが終了対象。scratch・ログは削除しない。

## 検証・残課題

- cwd `D:\Projects\GithubIssueWidget`。`Get-Content -Encoding UTF8` で依頼・レビュー・訂正・scratch コードと CSS を読取。依頼記載4対象の前版ハッシュは照合した。`Get-Command tauri-driver`, `Get-Command msedgedriver`, `Get-Command cargo`, `Get-Command node` は各照会完了、前2つ非検出・後2つ検出（exit 0）。存在しないコマンドは起動していない。
- `Get-Content ... source-manifest.json | ConvertFrom-Json` と全34パスの `Test-Path`、`Get-FileHash -Algorithm SHA256` により元17件が期待どおり、scratch 14件不変・3件変更・新規1件を確認（exit 0）。本結果の親・確定先・tmp の不存在を確認し UTF-8 往復検査後に確定する。コード・対象版は提出後に変更しない。
- 起動・build・依存導入・通信・画面表示・入力・OS操作は未実施。マーカーや output が実際に accessibility tree で公開されるか、screenshot で判読できるか、CSS/Win32/操作API座標の一致、単一入力時の記録、型検査、診断プロセス起動・終了は未確認。実現性不足の場合は同じ入力を試さず方法を再設計する。B01/B02 は今回解決済みと断定せず、作者以外の独立レビューを依頼する。
- 今回の担当実行の新たな失敗はなし。先行するプロセス照合の自己一致1回は `I02-drag-click-question-02.md` に記録済み。同種2回で該当操作を中断し別担当の客観調査、3回で全中止。前回の記録件数誤りは管理の訂正01を正とし、旧質問を書換えない。
