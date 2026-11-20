# I02 未解決事項の検証方法設計 01

- run / task / owner: `20261118-160129-mvp-restart` / `I02-verification-design` / `/root/verification_design`。依頼 `verification-design-request-02.md` SHA256 `DBB1F63472016EB0AC1B14CD9E401A342C7B8A45C20C61A2B3F65D12A6991C71`。
- 対象 worktree: `D:\Projects\GithubIssueWidget-issue-2-20261118-160129`、branch `task/issue-2-foundation-20261118-160129`、HEAD `a4ac97286785a2a5ba0d070a909b58003fc49ff1`。`result-02.md` の製品ファイル欄に限定して17件を抽出し、全パス存在確認後に SHA256 を照合、全件一致。README 変更と widget の未追跡16件、削除なし。設計だけを行い製品ファイル・Git・GitHub は変更していない。
- 参照資料: `verification-design-request-01.md`、`management-04.md`、`combined-review-01.md`、`result-02.md`、`result-01.md`、`drag-result-01.md`、`plan-02.md` の実体 SHA256 は依頼記載と全件一致。Issue #1、#2 の本文を `gh issue view ... --json body` で取得し Windows の実測、URL 許可・拒否と手順・結果の記録を確認した。
- 結論: 以下は **検証計画案** であり実行結果ではない。B01 は未達で、原因確定まで同じウィンドウ移動試行を禁止する。B02〜B04 も現状は未完了。各作業は対象版と手順を管理・独立レビューし、対象版が変われば新版として再固定する。

## 共通する開始・終了条件

1. 管理が担当 ID、所有する証跡ディレクトリ、独立レビュー担当、作業時間を確定し、同じ worktree、対象ウィンドウ、`target`、1420番ポートと同一ブラウザプロファイルの排他利用を確認する。各担当は共有 run 内に新規の専用 `evidence-<id>/` を使い、既存証跡や ignored 生成物を変更・削除しない。製品の17パスは `result-02.md` の製品一覧からのみ取り、全パスを解決・存在確認した上で `Get-FileHash -Algorithm SHA256` し、期待値・64桁・branch・HEAD・未追跡を含む status を作業開始時と終了時に保存する。中間変更や所有不明、未解決の依存があれば中断。
2. UTC時刻、OS・スケール、コマンドと作業ディレクトリ、対象 PID / executable hash、ログの種類と各 exit code、観測値、未実施理由を専用フォルダーへ記録。各証拠は実体から SHA256 を求め、完了前後の照合と UTF-8 読戻しを実施。スクリーンショットは対象範囲を限定し、他アプリ・秘密・個人情報を含めない。赤字や未取得を成功扱いしない。
3. 同種の問題が初回を含め2回で該当操作を中止し別担当へ原因調査を依頼、3回で全中止。前回のドラッグ不成功2回を保持し、原因確定前に再実行しない。対象識別不明、必要な許可不足、入力先不一致、記録の不整合または外部への不要な副作用を観測した場合も中断。独立レビュアーは元資料、結果、各実体とハッシュ、ログ内終了コード、期待値と未達を照合する。

## B02: Windows の寸法・最前面・リサイズ

- **役割・所有**: Windows 実機検証担当が検証用ウィンドウと `evidence-b02/` を専有。管理は別途担当 ID を確定。開発起動と同時に行う場合は B03 担当と単一の実行プロセスの所有・終了条件を事前合意し、操作は B02 担当だけが行う。独立レビュー担当は別人。
- **前提**: B03 の対象版に対応する debug/release のプロセスを一意に識別でき、Windows 11 対話デスクトップ、ウィンドウ ID と PID・実行ファイルパスが一致すること。実画面・DPI・複数モニターを確認し、記録の収集やサイズ操作が可能かを管理が審査する。ウィンドウ移動ドラッグは実行しない。
- **初期値**: 対象 HWND の `GetWindowThreadProcessId`、`GetWindowRect`、`GetClientRect`、`DwmGetWindowAttribute(DWMWA_EXTENDED_FRAME_BOUNDS)`、`GetDpiForWindow`、`MonitorFromWindow` と `GetMonitorInfo`、`GetWindowLongPtr(GWL_EXSTYLE)` を呼ぶ測定ハーネスのソース・実行ファイル・ログも SHA256 で固定。呼出しの成否と `GetLastError`、時刻、DPI awareness、モニター全体矩形・作業領域、クライアント矩形、外枠と可視枠を記録する。`WS_EX_TOPMOST` の状態を初期値として記録し、UI 操作で固定解除・再設定それぞれの直後に再取得して相互に照合する。ハーネスから位置を設定しない。
- **判定**: `GetClientRect` のピクセルを DPI と awareness に従って論理単位に換算し 900×520 の仕様が実際に適用されたか判定。Tauri の幅がクライアント領域か外形かの契約は対象版で別途確認し、未確認なら寸法合格にしない。中央は選択モニターの作業領域または全領域のどちらを実装が使うか確認後、同じ座標系で中心差を記録し丸め・枠を区別する。混在 DPI、DPI 仮想化、画面外や観測時点のズレは不明として止める。
- **最小・リサイズ**: 管理が具体的な安全手順とウィンドウ外周8方向の入力を別途承認し、B01 のタイトル移動と対象・操作・ログを区別できる場合のみ、各ハンドルで増減してクライアントサイズ、移動した辺、逆辺、最小制約 600×300 を記録。対象領域が識別できない場合は入力しない。ウィンドウサイズを API で直接変更するだけではユーザーによるリサイズを証明しない。手順、変更前後のスクリーンショット、計測 JSON、操作結果・最小制約を保存し、各8方向の成功を個別に判定。戻すサイズ・最前面設定と他アプリの状態を記録し、不要なアプリの変更は行わない。
- 参考: [DPI と device-independent pixels](https://learn.microsoft.com/en-us/windows/win32/learnwin32/dpi-and-device-independent-pixels)、[GetWindowRect](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getwindowrect) は DPI 仮想化・不可視枠の扱いを説明する。これらだけでは今回の値を実証しない。

## B03: 対象版に紐付く build・dev

- **役割・所有**: Windows build 担当が `widget/`、その `target` と dev port 1420、プロセス、`evidence-b03/` を専有。生成物を上書きし得るため、既存生成物の所有・保存要否を確認してから別の検証対象領域または作業用の新しいビルド出力先を管理が決定。共有 target を既存物と競合させない。
- **手順**: 正確な `VsDevCmd.bat` の絶対パスの存在・VS/MSVC・Node/npm・Rust・WebView2 を記録。`evidence-02/method.md` の環境取込を参考に個別プロセスで `cmd.exe /c` を使い、終了コード確認後のみ `widget/` で `npm.cmd run tauri -- build --no-bundle` を実行。`Start-Process -Wait -PassThru` 等で stdout/stderr を分離し、開始・終了 UTC と実プロセスの `ExitCode` を JSON とログに一緒に記録。コマンド成功は `ExitCode == 0` とし、プロセス起動・ログ書込失敗は不明。exe の完全パスと SHA256 を保存する。
- 次に `npm.cmd run tauri -- dev` を実行し、準備完了ログと PID・exe・対象ウィンドウを保存する。起動後、正常終了のため GUI の閉じる操作を行う場合は B02 と所有を調整し、終了後のプロセス exit code を保存。常駐プロセスの停止を強制した場合は正常終了として扱わない。前後17ファイルのハッシュ、`Cargo.lock` / `package-lock.json`、ツール版、起動・終了コード、生成 exe ハッシュ、ログの相互関係を manifest に明記する。ログに秘密を出さない。
- **判定と制約**: 実行時の manifest と最終 manifest が一致し、build が exit 0、dev 実起動・ウィンドウ確認・正常終了の全証拠が揃うまで完了にしない。ハッシュの前後一致は途中に一時変更がなかった完全な証明ではないので、同時製品編集を禁止し、必要なら検証時の不変コピー・使用ソースの実体を管理が確保する。ネットワークで依存解決される可能性、localhost のリスンと生成物作成を事前に明示、既存ポート・プロセスに衝突したら止める。依存が欠けている・再取得が必要な場合は発生する変更範囲を記録して管理に確認。B02/B04 と同時に同じ生成物をビルドしない。

## B04: 実際の Tauri IPC・URL 制限

- **役割・所有**: セキュリティ検証担当が B03 で固定した対象版の実行ファイル・対象 `main` WebView と `evidence-b04/` を専有。B02、B03 の対話操作・ポート・プロセスと直列化。デバッグに追加フックが必要なら製品版と異なるため別版を SHA で固定し、製品版の ACL と同一とは自動判定しない。
- **検証方法案**: 実際の Windows WebView2 と Tauri IPC を使用する `tauri-driver` / WebDriver の接続可能性とバージョン・起動方法を事前確認する。Tauri 公式の [WebDriver](https://v2.tauri.app/develop/tests/webdriver/) を参照。WebDriver の実行コンテキストが対象 main WebView のものであることをウィンドウ ID・PID・対象版で識別し、非 mock で `window.__TAURI_INTERNALS__.invoke('plugin:opener|open_url', {url: <固定文字列>})` を呼ぶ設計。ローカル `@tauri-apps/plugin-opener/dist-js/index.js` の `openUrl` は同コマンドを呼ぶ。mockIPC や glob 単体は IPC ACL 実証にならない。
- **拒否試験の安全性**: 不許可 URL は副作用のない `https://example.invalid/`、`https://github.com.example.invalid/` など予約済みテスト候補を使用し、先に隔離された検証環境・外部通信遮断・ブラウザ起動の監視方法を管理が審査する。IPC が誤って許可してもネットワーク先やアカウントに影響しない条件を確認できなければ実行しない。`file:`、`javascript:`、カスタムプロトコル等は OS 側の副作用を排除できない限り実行せず未確認とする。拒否は各 Promise のエラー（ACL 由来と識別可能か要確認）、OS の起動監視と非遷移を合わせて判定。ログに URL 以外の browsing data を含めない。
- **許可試験**: https の既定関連付けを `IApplicationAssociationRegistration::QueryCurrentDefault` 等で読み取り、実際の関連付け先と起動・遷移先を比較。公開 `https://github.com/` への GET とブラウザ起動はネットワーク・既存ブラウザ状態に副作用があるため、管理が対象と環境を確認した後に実施。実際の URL、時刻、プロセス/アプリ関連付け、エラーの有無を記録。既存プロファイルの履歴・状態に影響する可能性があり、隔離プロファイルでも既定ハンドラとの同一性を別に確認。許可・拒否を両方実測できなければ未完了。実際の URL 正規化とユーザー情報・ポート・紛らわしいホストの扱いは別のケースに分け、テスト例の全網羅を主張しない。
- [Default Programs](https://learn.microsoft.com/en-us/windows/win32/shell/default-programs) はプロトコルの既定関連付けの照会を説明する。関連付けを確認できない場合や実際のブラウザを一意に識別できない場合は合格にしない。開いたテストウィンドウのみ所有者が閉じ、既存タブ・設定は変更・削除しない。

## 並行性と残作業

対象版の読み取り・手順レビュー・証跡整理は並行可。B03 build と他の target 利用は直列、同一 dev ウィンドウを用いる B02/B04・ブラウザ検証は直列。B01 は別の安全な診断計画が必要であり、ここで移動試行を承認しない。B02 のリサイズ入力、B04 の IPC 注入・隔離環境、ブラウザ監視、B03 正常終了の実行可能性は未確認。計画自体を独立レビューした後、管理が具体的な所有者と安全条件を確定する。

## 実施記録と教訓

- cwd `D:\Projects\GithubIssueWidget`。`Get-Content -Encoding UTF8` で AGENTS、agent-workflow、依頼・前結果・独立レビューを読み、`Get-FileHash` で依頼指定7参照と自身の依頼版を照合。`gh issue view 1/2 --json body` は各 exit 0。対象 worktree の `git status --short --untracked-files=all`、`git rev-parse HEAD`、`git branch --show-current` は各 exit 0。17ファイルの抽出を製品欄に限定し、事前に全パスの `Test-Path -PathType Leaf` を完了してから SHA256 を照合し、17件一致（exit 0）。
- 対象コード、capability、lock、ローカルの opener JS、既存 method を読み、Microsoft と Tauri の上記一次文書を検索。今回、対象アプリの起動、入力、ビルド、外部通信を伴う実動作試験は未実施。先の中断前の JavaScript 構文エラー1回と、同じコマンド内で注記付き2行をパスとして扱った PathNotFound 2件は旧 run の停止記録に記載されている。
- 今回、対象 worktree を cwd として共有 run の相対パスを読もうとし PathNotFound 1回。該当操作を停止し親へ報告。その後、共有 run の絶対基点と `Test-Path` で存在を確認してから method を読み取った。同種の2回目は今回発生していない。教訓: 実行 cwd と共有 run の基点を明示し、全参照の存在と用途を確認してから読む。
- 自身の変更は本結果ファイルのみ。削除対象なし。証拠データの新規作成、製品変更や外部更新なし。
