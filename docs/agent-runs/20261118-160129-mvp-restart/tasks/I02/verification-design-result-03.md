# I02 同一 Windows 環境での専用コピー検証方式 03

- run / task / 担当: `20261118-160129-mvp-restart` / `I02-verification-design` / `/root/verification_design`。依頼 `verification-design-request-04.md` SHA256 `A3A4DC5CFE8F178E5EB9480A35EBF90F73708BB71E75C007D278B5B37784F3C6`。
- 対象 `D:\Projects\GithubIssueWidget-issue-2-20261118-160129`、branch `task/issue-2-foundation-20261118-160129`、HEAD `a4ac97286785a2a5ba0d070a909b58003fc49ff1`。README変更・widget内未追跡16件。`result-02.md` の製品17ファイルについて全パス存在確認後に SHA256 一致。削除対象なし。
- 前版 `verification-design-result-02.md` SHA256 `971D1BE06A8D735AB75C57B5398F2E667D0F288D991B34F473D15080140A2926` の専用 VM 案は独立承認済みだが、環境調査で VM 実在・利用権・実機性は未確認。本案は **同一 Windows 上での実体コピーを用いた別案** であり、前版の承認を流用しない。方法の独立レビュー・管理判断まで実行指示ではない。
- Issue #1/#2 は Windows の開発起動、release build、ウィンドウ移動・リサイズ・最前面、外部リンクと制限の実測を要求。B01 の移動は原因不明・2回不成功であり、同一の移動再試行は行わず未達。ユーザー指示により同種失敗2回で該当操作を停止し別担当調査、3回で全中止。

## 対象版とコピーの準備

1. 管理が検証者、独立レビュー担当、開始・終了、専用出力領域、保護対象と競合担当の停止を確定。OS・ディスク空き・開発ツールと対話セッションを調べ、同一アプリやポート 1420 の使用中プロセス・ユーザー操作に競合があれば保留する。現在 Windows 11 Home とツール存在は `isolation-inventory-result-01.md` の時点の観測であり、物理実機や動作保証ではない。
2. `result-02.md` の「現在の製品ファイル SHA256」欄のみを解析し、17ファイルの相対パスと64桁値を分離。元 worktree の絶対基点の下に解決した全件の存在、通常ファイル、再解析ポイント・ハードリンクの有無を確認し、全件の SHA256、サイズ、git branch/HEAD と未追跡を含む status を manifest に保存。原本や他担当のファイルを編集しない。現状の ignored `widget/node_modules`、`widget/dist`、`widget/src-tauri/target`、`widget/src-tauri/gen` は実在・所有・保持状況を読み取り記録して保護する。ユーザーの内容を無用に出力しない。
3. 未存在と確認した同一 PC の書込み可能な専用 scratch 親の下に新規ディレクトリを作る。絶対パス・正規化先・ボリューム・ACL・所有者・上位を含む reparse/junction/symlink を確認し、worktree、既存生成物、共有 run または他者ディレクトリと重複しない。各ファイルは **通常のバイトコピー** とし、ハードリンク・junction・共有 writable mount は使用しない。README と widget の16ファイルだけを元と同じ相対配置でコピーし、コピー内の全17件の存在・サイズ・SHA256 を元 manifest と照合。copy 前後で元17件も再照合。差異・所有不明・コピー失敗・追加の対象入力が見つかれば実行しない。
4. 対象17件をコピー以外から読む可能性（環境設定、絶対パス、依存コマンド、Vite の env、親ディレクトリの npm/Cargo 設定、symlink、グローバル設定、build script）を作業前に列挙・確認。コピー側の設定やソースに変更が必要なら新しい対象版・差分・ハッシュを固定し、元版への適合性を独立レビュー。再現できない差異は未確認とする。依存内容と tools は別に記録し、元版と同一と決めつけない。

## 書込み先、分離と通常の副作用

| 領域 | 実行前の指定・確認 | 許容可能な副作用・制限 |
| --- | --- | --- |
| Vite `dist` / `.vite` | コピー内 `widget/dist`、コピー内 `widget/node_modules/.vite` の想定を `vite.config.ts`・package script で確認 | コピー内の新規生成物のみ。元 `dist` や元 `node_modules` を参照・変更しない。想定外の絶対出力先は中断。 |
| Cargo target / Tauri gen | 子プロセスに新規専用 `CARGO_TARGET_DIR`、コピー内 `widget/src-tauri/gen` を使わせる。`tauri.conf.json` の `frontendDist` と `beforeBuildCommand` をコピー上で確認 | 専用 target、コピー内 gen に生成。設定/ソース、元 worktree の target/gen は保護。Tauriや依存スクリプトの追加出力先は実行前に調査し、不明なら保留。 |
| `node_modules` と npm cache | コピー内 node_modules と専用 `npm_config_cache` を新規使用。npm の project/user/global config、registry・proxy・環境値は資格情報を表示せず出典と有効範囲を調査 | `npm ci` がコピー・cache を作成。`package-lock.json` と script、依存を確認し、lifecycle script を実行するなら対象と副作用を先に審査する。登録済み認証や他人の npm cache は読み書きしない。offline 不可時の通常公開 registry 取得は管理が対象先とログ内容を判断、依存 hash/lock を記録。 |
| Cargo / rustup | 新規専用 `CARGO_HOME`・`RUSTUP_HOME`。必要な registry cache/toolchain は出所、版、ライセンス・サイズを確認し、秘密の含まれない資産のみ通常コピー又は必要分の新規取得 | 共有 Cargo registry、`.cargo/credentials`、ユーザー token と rustup 設定をコピーしない。固定版 Rust 1.90.0、依存 lock で `--locked` 相当を使用。専用領域に依存や一時物が生じる。オフライン完全性は未確認。 |
| TEMP / TMP / logs | プロセス単位で専用 `TEMP`/`TMP` と専用 evidence dir。build・dev stdout/stderr、子プロセス終了コード、manifest はここへ | 専用領域の新規ファイル。既存ログと証跡を上書きしない。標準出力に環境変数や資格情報を丸ごと保存しない。 |
| APPDATA / LOCALAPPDATA / profile | 実行プロセス専用の新規ディレクトリを指定し、子プロセスの継承を確認。環境変数が一部 API に無視される可能性も調査 | app や WebView2 が別の通常ユーザープロファイルを使う可能性があるため、実際の profile path が特定できない場合は該当 UI/ブラウザ操作を保留。Credential Manager へアクセスする処理は現対象基盤には未実装だが、現状の確認だけで安全性を保証しない。 |
| OS / MSVC / registry | `VsDevCmd.bat`・SDK・WebView2 Runtime は既存インストールから読み取り使用。Windows・ツールによる限定的なシステムログや OS cache 発生の可能性を記録 | レジストリ・Credential Manager・システム設定・既存アプリ設定の意図的変更は禁止。通常実行の副作用は完全隔離できない。未確認の重要な書込みや秘密へのアクセスがあれば当該処理を止める。 |
| Browser / network / UI | B04 は既定ブラウザ関連付けを読み取り、対象ブラウザと profile が一意に特定可能なこと、実行時の窓・ユーザー操作の競合を確認 | 公開 GitHub URL の GET、ブラウザ起動・履歴や WebView2 cache が生じ得る。既定 browser のプロファイルと別隔離 profile の違いを記録し、必要な観測ができないなら未達。既存タブ・履歴・設定は削除しない。テスト対象以外の外部書込みはしない。 |

子プロセスに対する環境変数は新規プロセスにのみ設定し、親のユーザー設定・資格情報を永続変更しない。元 worktree に OS レベルの書込み拒否があるとは仮定しない。実行する npm/Cargo build script・設定の読み取り確認、正規パスとリンク検査、専用の出力変数、元ファイル・生成物の前後確認で通常の誤書込みを防止する。これらは悪意あるツールや未観測書込みを完全に封じる保証ではない。権限・監視の実現可能性は管理が決める。既存生成物の前後は取得可能なファイル一覧・ハッシュ・メタデータを保護の範囲で確認し、他者の同時利用があれば停止する。Windows やウイルス対策の通常ログ等を全て排除することは前提にしない。

## B03 build / dev と証拠

- 検証者は専用コピーの `widget` を cwd にし、存在確認済み `VsDevCmd.bat -arch=x64` の環境を専用子プロセスに投入する。`npm.cmd ci` で lockfile ベースの依存をコピー内に準備し、`npm.cmd run tauri -- build --no-bundle` と別 run の `npm.cmd run tauri -- dev` を実行する。各コマンド・cwd・開始終了 UTC・stdout/stderr と OS の process exit code、依存・環境差・ログ hash を同一 run manifest へ保存。環境構築・依存インストールの失敗や不明時は以降を実行しない。dev port は loopback を確認し、プロセスと窓が一意に対応し、閉じる操作後の正常 exit 0 を別々に記録する。
- 各実行の直前・直後に元とコピーの17 SHA、lockfiles、生成 exe の絶対パスと SHA、設定・環境差、起動 PID と版を保存し、不一致や外部出力があれば中止。コピーで build した exe が元の既存 exe と同一とは主張しない。得た成功はそのコピー・依存・環境に限定。実機操作時はその exe SHA と対象版の対応、Windows 実機性・環境差を記録し、VM/別版の結果を実機成功に置換しない。

## B02 と B04

- **B02**: 前版設計を維持。Windows 上で HWND/PID と exe SHA を結び、`GetClientRect`、`GetWindowRect`、DWM frame bounds、`GetDpiForWindow`、モニターの work area、`GetWindowLongPtr(GWL_EXSTYLE)` を記録。初期 900×520、最小 600×300、中央・最前面と解除/再設定を DPI と枠の差を考慮して実測。8方向リサイズは別途対象領域と操作安全を確かめて行い、位置移動ドラッグと区別できなければ未実施。実際に Win11 実機か不明なら確認まで未完了。操作前後の対象窓、スクリーンショットと非機密の観測値を保存。
- **B04**: 前版の非 mock Tauri IPC 経路の許可・拒否を維持。対象 exe/ウィンドウを固定し、通常アプリの許可 URL の実際の遷移と Windows の https 既定関連付け・実ブラウザを照合。許可外の URL は `example.invalid` 等の安全な候補で行うが、もし誤許可でも不要な外部書込み・敏感情報流出・危険な protocol handler 起動がない方法と観測方法を管理が確認できた場合のみ実行。実際の IPC の拒否エラーとブラウザ側に新規起動/遷移がないことを記録する。`file:` や `javascript:` 等の危険な候補の安全性が不明なら実行せず未検証。glob 単体・mockIPC は実際の ACL 証明にならない。別 profile を使う場合、既定ブラウザの証明とは別に扱う。

## 所有、排他、終了とレビュー

- 実担当 ID、scratch パス、既存キャッシュやアカウントの利用可否、プロセス・証跡の所有者は管理が作業前に確定する。コピー・依存準備・build・dev は一担当が領域を専有。B02 と B04 は同一窓、WebView2、デスクトップ、ブラウザ、port の共有があるため操作を直列にする。静的な読み取りや独立レビューは競合しない範囲で並行可。
- 予期しない書込み・環境差・秘密の混入・不明な PID・コピーやハッシュの不一致、ユーザーのデスクトップ操作との衝突は該当作業を中止し、成果を保持して管理に報告。各種停止条件と同種失敗回数を記録し、同種2回目で別担当による客観調査、3回目で全作業を中止。通常終了後も専用コピー・生成物・ログは保管し、削除・既存 ACL の変更・設定復元操作は個別確認する。
- 別担当は一次 Issue・元版/コピー17ハッシュ・設定と依存差・実際のコマンドと exit code・生成 exe・OS/IPC/ブラウザ観測・ファイル一覧と未検証を照合。前後のハッシュと設定確認は全ての予期しない Windows 副作用の不在を証明せず、同一 PC と外部 GET の残余リスクを結果に記載する。

## 読取り・変更と未確認

- cwd `D:\Projects\GithubIssueWidget`。依頼記載の8資料と本依頼は共有 run の絶対基点に解決し全件存在を確認後、SHA256 を取得し依頼値と一致。`Get-Content -Encoding UTF8` で環境調査と前版レビューを読取。`result-02.md` の製品一覧17件を全パス存在確認後に SHA 照合し全件一致。`git -C <対象>` の status/branch/HEAD は exit 0。Issue #1/#2 は前版調査で `gh issue view` の本文を取得し照合済み。
- 今回は方法設計のみ。専用ディレクトリ・コピー・環境変数・依存取得・build・app 起動・UI・IPC・通信試験は実施していない。専用 Rustup/Cargo/npm の構築、子プロセスの書込み先、ブラウザの profile、実機操作、監視方法は未確認。成果は本ファイルのみで、既存製品・生成物・Git・GitHub は変更しない。今回失敗なし。前回の基点取違え1回は既報であり、今回の同種再発はない。
