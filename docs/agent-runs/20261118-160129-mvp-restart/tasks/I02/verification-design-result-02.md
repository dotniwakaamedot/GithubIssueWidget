# I02 検証方法設計の修正版 02

- run / task / owner: `20261118-160129-mvp-restart` / `I02-verification-design` / `/root/verification_design`。依頼 `verification-design-request-03.md` SHA256 `7CB095F5217F840136A668AE9F8726457259DEEC40D3A26F2FAB06719C38D6AE`。
- 対象: `D:\Projects\GithubIssueWidget-issue-2-20261118-160129`、branch `task/issue-2-foundation-20261118-160129`、HEAD `a4ac97286785a2a5ba0d070a909b58003fc49ff1`。README 変更および widget 以下未追跡16件、削除なし。`result-02.md` の製品一覧のみから17ファイルを取得し、全パス存在確認後に実体 SHA256 が一致。
- 前版 `verification-design-result-01.md` SHA256 `E821627AF74FF335D29433BF47F8682D36C2BF970B18036365495769F0153B4D`。独立レビュー `verification-design-review-01.md` SHA256 `829758054D5DF5B0D8D920CF56F46D8EE58725D07200B95C269CABE486A6D6EE` の **設計指摘 B01** への修正案を以下に示す。これは Issue 機能の B01（ウィンドウ移動）とは別。前版・レビューは変更しない。
- 前版の B02・B04、移動禁止、同種2回で該当操作中止・独立調査、3回で全中止、対象版ハッシュ・未実施・並行性・安全条件は全て維持する。B03 の隔離に関する記述は本版で置き換え、曖昧な「対象 widget または別出力先」は採用しない。これは設計案であり、独立レビュー・担当割当・隔離条件が整うまでは実行しない。

## 現状と隔離方針

対象 `widget/src-tauri/tauri.conf.json` は `beforeBuildCommand: npm run build`、`frontendDist: ../dist`、`beforeDevCommand: npm run dev`、dev URL `http://localhost:1420`。対象 `package.json` の build は `tsc --noEmit && vite build`、`vite.config.ts` は outDir を指定しない。`widget/.gitignore` は `node_modules/`、`dist/`、`src-tauri/target/`、`src-tauri/gen/` を generated/ignored として挙げる。したがって Cargo target の切替だけでは Vite が既存 `dist` を書き換えないと保証できない。現在の既存 worktree・ignored 生成物・共有 cache を保護し、**対象 worktree 内では build/dev を実行しない**。

管理は書込みを完全に隔離できる使い捨て Windows 検証環境（例: 既存 worktree やユーザープロファイルにアクセスできない別の Windows VM）と、専用の一時作業領域・担当・成果保管先を確認する。VM の利用可能性やライセンス・デバイス操作は未確認。隔離方法の検証、既存資産の非公開・競合排除、アクセス制御、作成・保持の認可のいずれかが不明なら **build/dev は未実施** とする。共有フォルダ・ホスト worktree や共用 npm/Cargo cache を書込み可能に接続しない。実行中の VM との成果搬出は専用証拠領域に新規作成し、既存物を上書きしない。削除や VM 廃棄は別途認可・確認が必要。

## 書込み先と検査

担当は以下を起動前に一覧化し、実際の絶対パス、所有者、ボリューム、存在・再解析ポイント/シンボリックリンク、権限、空き容量、保持期間、外部への書込み経路を記録する。予定外の書込み先が見つかれば継続せず管理へ報告。起動プロセスだけでなく子プロセスのファイル・レジストリ・ネットワーク書込みも対象とし、隔離 VM 内の OS 管理領域はゲスト専用であることを確認する。書込み監視・拒否方式が実行可能かは未検証であり、先に管理と独立レビューが審査する。管理者権限が必要な方法を安易に前提にせず、テスト用制限アカウントで実施する。

| 対象 | 検証用の所有先と確認事項 |
| --- | --- |
| ソースコピー | VM 内の新規専用 `work/source/`。対象17ファイルのうち app 必要な `widget/` 16件と説明の `README.md` を実バイトでコピーし、再解析ポイント・ハードリンクを使わない。機密・ignored・他担当ファイルはコピーしない。ファイル単位は読取専用にし、生成ディレクトリだけは別所有領域として許可する。元 worktree を writable mount しない。 |
| Vite frontend | コピー配下 `work/source/widget/dist/`、Vite/依存の `node_modules/.vite` 等。コピー内にのみ生成し、元 worktree の `widget/dist` や `node_modules` は触らない。実際の config や依存ツールが書く追加先も監視して判別する。 |
| Cargo / Tauri | `CARGO_TARGET_DIR` は VM の新規 `work/cargo-target/`、Tauri 生成はコピー内の `widget/src-tauri/gen/`、想定される bundle・debug/release・build-script 出力や WebView2 のプロファイル、CLI キャッシュも列挙する。`gen` の生成を許可しても17対象の設定やソースへの書込みを許さず、対象ファイルの前後ハッシュで検出する。 |
| 依存取得 | VM の新規 `work/npm-cache/`、`work/cargo-home/`、`work/rustup-home/` とコピー内 `node_modules/`。固定済みツールチェーンと、出所・版・ハッシュが確認された offline パッケージを検証担当が別途準備。`npm ci --offline`、Cargo の offline 設定と lockfile 固定を使う案。npm lifecycle scripts や native binary が必要な場合は個別に内容・作用を審査し、未準備・offline 失敗は停止。外部のグローバル cache、既存資格情報、ネットワークを利用しない。依存の選択や設定差は結果に記録。 |
| ログ・一時物 | VM の新規 `work/evidence/` と `work/temp/`、プロセス単位 `TEMP` / `TMP` を専用領域へ。stdout/stderr、プロセス終了コード、開始終了 UTC、ファイル監視ログ、manifest は evidence 下に保存。npm・Rust・MSVC・WebView2・Windows が追加利用する temp、registry、user profile、CrashDumps も事前に特定し VM 内に閉じる。想定外の host path 書込みは拒否し中止。 |
| OS・外部 | VM 内に必要な SDK、MSVC、WebView2 とツールチェーンを事前配備し、版・導入元・既知の制約を記録。対象ユーザーの本来の `APPDATA`、Credential Manager、ブラウザ、既存ツール設定に接続しない。VM は外部ネットワークを遮断し、dev port 1420 は VM loopback のみ、衝突があれば中止。 |

表は対象の設定と一般的なツール挙動からの事前整理であり、実際の全書込み先を確認した証拠ではない。`npm ci`、MSVC の子プロセス、Tauri、WebView2 の挙動や環境差は実行前に具体的に確認する。隔離の検証が不足する場合は試さない。

## コピー・ビルド・対象版の対応

1. `result-02.md` の「現在の製品ファイル SHA256」欄だけを用い、元の各相対パスを worktree の絶対基点で解決し、全17件の存在・64桁 SHA・branch/HEAD/status を記録。リンク・所有不明・対象外ファイルの混入があれば停止。管理がレビュー済み元版 manifest を固定する。
2. 17ファイルを読み取り専用で新しい転送物に収め、転送物の SHA256、個々の相対パスとサイズ・SHA256 を記録し VM へ一方向搬入する。コピー後、コピーの全17ファイルと元版 manifest を比較。コピー元とコピーを同一物として参照しない。ハードリンク・junction・共有ディレクトリは使わない。転送中に元版が変われば中止し、新版を固定する。
3. 別途準備した offline 依存キャッシュ・ツールチェーンの provenance、版、lockfile、npm/Cargo 設定（秘密を除く）を manifest に記録。依存の解決結果 `npm ls --all` や Cargo lock の照合、install scripts の有無・結果、`node_modules` 等の識別を記録する。ローカル既存環境の依存との完全な同一性は未確認であり、差分や再現不能な項目を明示。利用できなければ実行しない。
4. 共有済みの各書込み先と排他を確認後、VM のコピー `work/source/widget/` で `npm.cmd run tauri -- build --no-bundle`、および別 run で `npm.cmd run tauri -- dev` を、対象版・環境・実コマンド・cwd・開始/終了 UTC・stdout/stderr・実プロセス終了コードを保存するハーネスから起動。`VsDevCmd.bat` の VM 内パスを存在確認してプロセス単位で環境を投入し、失敗したら後続を開始しない。dev のウィンドウ・PID と正常終了は前版どおり確認し、強制終了を成功としない。
5. ビルド前後のコピー17ファイルと元 worktree 17ファイルの SHA256、作業中の監視ログ、lockfile・ツール版・実行環境・生成 exe SHA256 を一つの run manifest に記録。証拠は read-only 一方向搬出し、コピー結果のハッシュを搬入元と照合する。対象変更・想定外書込み・秘密露出・失敗や版不一致があれば停止。前後一致は実行中に一時書換えがなかった完全な証明ではないため、ACL と監視証跡を併用する。
6. コピーから生成したバイナリと元の対象17ファイルは上記 manifest・依存・toolchain・コマンドで対応付ける。これは**元 worktree の既存 exe と同一バイナリである証明ではない**。元 exe の hash との異同を任意に明示し、差の原因やビルド再現性を未確認とする。VM 上の成功は Windows ビルド証拠となるが、Issue が求める Windows 11 実機の画面操作を代替しない。実機操作時は同じ確認済みソース版・コピー exe の SHA と環境差を記録し、異なる版や未実施は未達のままとする。

## 前版との対応・並行性

- B02 は前版の OS 測定・topmost・DPI・枠・中央・8方向リサイズを維持。実機対象ウィンドウの所有と操作安全性を管理が確定し、原因不明の移動ドラッグはしない。VM 内での計測だけを実機成功としない。
- B04 は前版の実 Tauri IPC（mock/glob 単体で代替不可）、許可 URL の既定ブラウザ遷移・不許可 URL の拒否・副作用と隔離の条件を維持。VM のネットワーク遮断下の拒否試験が可能でも、許可 URL が既定ブラウザに開く実試験は別途ネットワーク・環境を確認し記録する。
- B03 の隔離 build、B02 のデスクトップ・ウィンドウ、B04 の IPC・ブラウザはそれぞれ担当・保存領域を分離。別環境なら読み取りや証拠整理は並行可能だが、同じ VM・dev port・プロセス・画面・ブラウザ・生成物を共用する実操作は直列。ログと証拠の hash、未実施理由、観測値と終了コード、対象版は前版の条件を維持。
- 独立レビューは本版、元17ファイル、転送・コピー manifest、書込み隔離と依存設定、全ログ・exit code・exe hash を照合し、環境差や証拠不備は未検証とする。要求の一部のみをもって Issue #2 全体を完了しない。

## 実施・未実施と教訓

- cwd `D:\Projects\GithubIssueWidget`。`Get-Content -Encoding UTF8` と `Get-FileHash -Algorithm SHA256` で依頼・前版・レビューほか依頼指定参照の存在と SHA を照合。対象 worktree の設定・package・Vite・build.rs・gitignore を存在確認後に読取り、`git status --short --untracked-files=all`、`git rev-parse HEAD`、`git branch --show-current` は exit 0。製品17ファイルは `result-02.md` の製品一覧のみを解析し、全パスの存在を確認した後に SHA 照合、全件一致（exit 0）。
- 本担当は方法設計のみ。VM、隔離、依存準備、コピー、build、dev、アプリ・OS・UI 操作、通信試験を実施しておらず、これらの実現可能性は未確認。今回新たな失敗はない。前回の PathNotFound 1回は既報。削除対象なし。自身の変更は本結果のみで、製品・既存生成物・Git・GitHub の変更なし。
