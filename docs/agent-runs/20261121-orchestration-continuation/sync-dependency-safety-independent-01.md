# 同期テスト依存の通信・資格情報経路

2026-11-21、担当 `/root/issue_survey`。依頼 `sync-dependency-safety-request-01.md` SHA256 `4F255AF8EE5E69C2EC8672707121E81A76FA24E6BCF74218364311F397ECF1B1`、担当訂正 `sync-dependency-safety-request-02.md` `332BBE23136763224568359D322A3068C53A713B75FBEA2CE76E285D5CC54837`。読取りのみ。テスト・プロセス起動・通信・Credential Manager・GUI、製品・依存の変更は行っていない。本調査は実行承認ではない。

## 判定

固定テストは Tauri IPC・ウィンドウ・Opener をモックするが、Node/Vitest/Vite/jsdom/esbuild の全依存・起動経路は未追跡であり、**外部・loopback 通信や実 Credential Manager の非アクセスは証明できない**。秘密の環境変数を引き継がないだけでは同一 Windows アカウントの資格情報を隔離できない。実効性を実証した隔離環境・プロセス監督・独立レビューを備えるまで B2 を解消済みとしない。

## 確認した経路

- 隔離版 `main.test.tsx` は jsdom を指定し、`@tauri-apps/api/core` の `invoke`、`@tauri-apps/api/window`、`@tauri-apps/plugin-opener` を `vi.mock`。`main.tsx` はこれらを import し、接続・一覧・作成・移動等を IPC へ渡す。テストでモックが適用されれば製品 Rust と Credential Manager に至る通常の IPC 呼出しは置換される。ただしモックの実行時適用・他依存経路は未検証。対象2ファイルに `fetch(`、`XMLHttpRequest`、`WebSocket` の文字列は検出されなかった（限定文字列検索であり迂回呼出しや依存コードを否定しない）。合成 GitHub URL の構築や表示があるが、文字列存在自体は通信の証拠ではない。
- `widget/vite.config.ts` は React plugin を読み、`envPrefix` に `VITE_` と `TAURI_ENV_` を指定。インストール済み Vite `dist/node/chunks/config.js` の `getEnvFilesForMode`・`loadEnv` は対象 root の `.env`、`.env.local`、モード別ファイルを読み、環境に一部の値を反映しうる。隔離先を新規に作り `.env*`・元環境や秘密をコピーしないことが必要。実際のモードと `envDir` は起動時の解決・CLI オプション検証前で未確定。
- jsdom `lib/api.js` は `loadSubresources: resources !== undefined` とし、コメントで XHR はそれにかかわらず動作すると記す。したがって自動サブリソースを無効にしても全通信抑止とはいえない。Vite・Vitest の内部サービスや loopback を含む全通信経路は本調査で追跡していない。
- esbuild `lib/main.js` の `ensureServiceIsRunning` は `child_process.spawn` でサービス子を生成し、stdio を pipe・inherit で接続、作業場所に cwd を使う。`ESBUILD_BINARY_PATH` 環境値は実行体選択を変えうる。固定環境から除外し、Node だけでなく子の実体と終了を対象にする。Vitest の threads 指定はこれを消す根拠にならない。

## 具体的な前進案と未確認

1. 実施可能性をまず環境側で調査する。専用 Windows VM 等で独立アカウント・空のユーザープロファイルと資格情報ストア、新規 root・TEMP/cache・ログを用意し、共有フォルダ・旧 fixture・既存プロファイル・資格情報をマウントしない。隔離先での Credential Manager 境界は専用合成資格情報による読取拒否で別途実証する。実資格情報の列挙・内容取得はしない。VM の存在や利用権限は未確認。
2. 外部ネットワーク接続を VM 設定等で遮断し、さらに loopback は実際のサービス連携に必要かを確認したうえで、隔離 VM 内に他サービスを起動しないか、強制制限を行う。合成プローブで外部・loopback 両方の失敗と許可する最小経路を検証する。単に NIC を外すだけでは loopback は遮断されず、ホストの firewall や同一アカウントの環境変数変更だけで保証しない。遮断に必要な権限、Node/Vite が必要とする loopback、規則の全子への適用は未確認。
3. 新規プロジェクトに固定対象と固定依存を通常コピーし、`.env*`、既存 cache、リンクや元のユーザーデータを含めない。出典とコピー先をハッシュ・種別・パスで照合、空の出力・一時領域を予約し、容量とアクセス権を確認。Node 実行体、設定、依存と test 対象の固定版を独立レビューする。旧不明プロセスは同ホストの OS・アカウント・ネットワーク・依存キャッシュ等を共有し得るが、専用 VM と専用アカウントが実際に分離されれば共有面を減らせる。完全な非干渉は主張しない。
4. 環境は必要な OS 変数のみの allowlist とし、`NODE_OPTIONS`、`ESBUILD_BINARY_PATH`、認証・プロキシ・秘密を継承しない。実際に最小環境で起動可能かは未検証。stdout/stderr を限定アクセスの専用領域へ別々に排出し、監督器は上限・空き容量不足を検出して安全に停止・排出する具体方式を固定する。合成値以外をログしない。ログのサイズ・終端・終了コード・欠落を確認し、失敗や不明では成功とせず記録と領域を保持する。
5. プロセス生成時から所属・子孫・終了を確認する Job 等の監督方式、正常・期限超過・登録失敗・監督者停止の合成試験が別途必要。隔離・監督のどちらかが未確認なら実行しない。これらは実装提案であり、調査時には一切実行していない。

## 対象版・証拠

- 修正候補 `sync-isolated-01/main.tsx` SHA256 `6FCF2A34145450C5029B7FF80DDB3EC1DA5E27173AAB84E57F16D6D54C0E6A3E`、`main.test.tsx` `66E118A5A7062854C23BE405D2E9578C974FA283BEA332F2A7C51DCA1BFFD13C`。`widget/vite.config.ts` `1DAFF9E73019BD4E74489811D13553B3D227DB8457910D7F295E60AD0EB4CC7A`、`package.json` `102235B5686088C87B3221DEC7AE3CBEE20F0C9B1EAE070FD730C8B631DFCEDF`、`package-lock.json` `A5D65370135F83B08F02D5F1EE329706F08551743444264BCB10DFC7600B0C73`。
- `widget/src/style.css` `EB49BF65AE4FCE6701FBEF277055EDCAD42011248E5EA49905B6AFA4FD288B37`、`tsconfig.json` `2B5ED6E2C49DFDC635A27F83CBF5E2B835959CFA513F51215617B143D7F5D9DC`。固定依存のうち調査した `esbuild/lib/main.js` `61EC0BA144FF93F4BF0E96AD6FAB6F430EDE0AA13E27ACE8063ED63BA416B3CD`、`vite/dist/node/chunks/config.js` `8E3075D1575EC341B1455BEACC73E2531F616E61658875EB15D74A45DAF5874A`、`jsdom/lib/api.js` `6CBF5F5EA5A6048CB01E7125913D7D01AA669025F5596171C11EECB0D54709F7`、`vitest/vitest.mjs` `39DB22F579ACF5639BBB17A261408DEBBDE03F4692C0C439E77E7F13AEBA74D6`。残る依存・Node 実行体の今回のハッシュは未測定。
- `sync-test-runtime-audit-01.md` SHA256 `7B4002EFE998F32894BDFD249AF926635BB96CD9ABB3BFEF8489F1D5EDA39588`、`sync-test-runtime-review-01.md` `8644BC65211500473E32FDD2C87AFFC554209197B749E641330BA2AC269CA64D`。既存の監督・終了・収集件数の未確認事項を維持する。
- 読取りは `Test-Path`、`Get-Content`、限定 `rg`、`Get-FileHash`。Vitest entry の環境文字列検索は一致なしで終了1、これはその限定検索に限る。他コマンドは終了0。失敗は認識していない。
