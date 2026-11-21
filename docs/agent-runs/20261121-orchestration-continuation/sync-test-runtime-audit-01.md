# 同期テストの依存と実行方式の読取り調査

担当 `/root/process_safety`。`sync-test-next-actions-01.md` の 1・2 を対象に読取りで調査。テスト、ビルド、製品変更、依存コピー、外部通信はしていない。以下は独立レビュー前の具体案であり、実行許可ではない。

## 対象と依存

- 修正版 `sync-isolated-01/main.tsx` SHA256 `6FCF2A34145450C5029B7FF80DDB3EC1DA5E27173AAB84E57F16D6D54C0E6A3E`、`main.test.tsx` `66E118A5A7062854C23BE405D2E9578C974FA283BEA332F2A7C51DCA1BFFD13C`。テストの固定入力は React、Testing Library、jsdom、CSS と Tauri API のモック。前者は `./style.css`、後者は `./main` を相対参照する。
- `widget/src/style.css` SHA256 `EB49BF65AE4FCE6701FBEF277055EDCAD42011248E5EA49905B6AFA4FD288B37`、`widget/tsconfig.json` `2B5ED6E2C49DFDC635A27F83CBF5E2B835959CFA513F51215617B143D7F5D9DC`。`package.json`、`package-lock.json`、`vite.config.ts` の対象 SHA は `sync-test-design-result-01.md` にある。実行準備時にこれら全てを再計測し、出典と複写の一致を確認する。
- 既存 `widget/node_modules` の PowerShell 読取り列挙は 5,467 項目、4,975 ファイル、合計 118,621,128 bytes、reparse point 0。列挙時点の結果であり実行準備時の不変・アクセス権・ハードリンクの不存在を証明しない。通常ファイルを新規独立領域にコピーする方式なら、出典のハードリンクをコピー先へのリンクにしてはならない。コピー前後の正規化パス・各ファイル内容ハッシュ・ファイル名・件数とリンクなしを照合し、欠落・変更時は中止する。`.vite`、`.vite-temp`、`.vitest-cache` は既存依存に存在し、共有側に書かない。隔離先で独立作成する。容量には一時領域・余裕を含める。
- Node の実体は `C:\Program Files\nodejs\node.exe`、ファイル版 `26.13.1.0`、SHA256 `F5109DC868EC38703AD8F305E0ACEECE717CB60D4889873F434017AF4E96CCCB`。Vitest は 5.1.1、エントリ `node_modules/vitest/vitest.mjs` SHA256 `39DB22F579ACF5639BBB17A261408DEBBDE03F4692C0C439E77E7F13AEBA74D6`。esbuild の Windows 実体 `node_modules/@esbuild/win32-x64/esbuild.exe` SHA256 `C7BEE37877D0AA6A04652783FA0A2CF1A9CE5579D68BB3083BDA99D4BFF18EF`。上記バージョン・ハッシュは今回の読み取り時点であり、起動前に再照合する。

## 固定案

workspace 内の新規所有済み専用ディレクトリを予約し、その中の `project/src` に2つの隔離版と上記 CSS、`project` に widget の package/lock、tsconfig、vite.config を通常ファイルとして配置する。依存は link を作らず通常コピーし、コピー先に既存の `.vite` 系キャッシュを引き継がず独立に生成する。コピー方法と無関係な旧領域を走査・変更しないことは作成前に別途確認。専用 `TEMP`、`TMP`、`APPDATA`、`LOCALAPPDATA`、`USERPROFILE`、ログを個別に予約し、プロセス環境は必要な Windows システム変数だけを明示して作る。秘密を含む引継ぎ環境変数、`NODE_OPTIONS`、プロキシ、既存資格情報は渡さない。OS のアカウント単位 Credential Manager が環境変数だけで隔離されるとは主張しない。

実行候補は隔離 `project` をカレントディレクトリとして次の単一コマンド。**未実行であり、オプションと出力・環境を独立確認してから固定する。**

```powershell
& 'C:\Program Files\nodejs\node.exe' '.\node_modules\vitest\vitest.mjs' run 'src/main.test.tsx' --pool=threads --maxWorkers=1 --no-file-parallelism --testTimeout=10000 --hookTimeout=10000 --no-cache --no-fsModuleCache
```

インストール済み Vitest CLI 定義は pool、maxWorkers、fileParallelism、testTimeout、hookTimeout、cache、fsModuleCache を含む。`threads` は fork worker を回避する候補だが、Vite/esbuild の起動経路は別で、インストール済み esbuild 実装はサービス用 `child_process.spawn` を持つ。CLI の否定オプション解釈、正確な worker・サービス構成、書込み先・終了はこの読取りだけでは未確認。候補のオプション受理を根拠なく確実とせず、前段の小さな検証と方式レビューで確認する。`npm ci`、watch、共通 cache 消去はしない。

テストコードは `invoke`、Tauri ウィンドウ API、opener を `vi.mock` し、表示用の GitHub URL は合成文字列である。テスト本体に `fetch`・実資格情報呼び出しは確認されない。ただし依存全体のネットワークアクセスや Credential Manager 非使用をこの検索だけで保証できない。ネットワークと資格情報を扱わない隔離を実施・確認できなければ、その未確認範囲を記録し実行可否を独立レビューする。

## 監督・終了・隔離

- この試験は旧 GUI の fixture、旧予約、実 GitHub・実資格情報を使用しない別領域の合成入力であり、旧プロセスの不在を要求する根拠はない。ただし旧対象の正体は未確認なので絶対的な非干渉保証はしない。新規出力先は旧 fixture と共有しないことを正規化パスと所有権で確認する。
- 実行者が唯一所有する監督器から Node を開始し、開始予約、実体ハッシュ、専用入力・出力、生成後の保持ハンドル・PID と開始時刻、終了コードを記録。全体120秒を上限候補にし、stdout/stderr は専用ログへ分離し秘密を渡さない。Vitest の個別タイムアウトと全体期限は別である。切り詰めや空結果を成功としない。
- Windows Job による所属と kill-on-close、あるいは別方式による登録・停止・子孫の追跡を、実装前に別担当が確認する。直接の Node の終了だけでは esbuild を含む全子孫の終了証拠にならない。未所属の生成窓、監督者中断、期限超過、出力取得失敗、終了不明は結果を未完了として領域・ログを保護し、無関係な PID や旧プロセスは操作しない。未追跡の子孫や OS 障害の無条件終了を保証しない。実行者が専用テストを自発的に監督し、全体終了が確認できる具体的実装とレビューは未了。
- 通常 `it(` は29宣言、`it.each` は6・2・2例で合計39ケースと静的に数えたが、実行時の収集結果は未検証。成功判定は収集したファイル・ケース数、テスト結果、直接実行体の終了コード、ログの完全性と追跡対象の終了を別々に確認する。0件や途中切断を成功としない。単体テストは実 GUI、実 GitHub、実資格情報やスリープ時の検証ではない。

この調査では失敗を認識していない。隔離複写、実行オプションの実動作、監督器、全子孫の停止、依存内の全副作用は未検証。
