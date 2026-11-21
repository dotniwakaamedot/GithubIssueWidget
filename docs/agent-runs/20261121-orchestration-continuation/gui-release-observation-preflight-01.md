# release GUI 観測の固定版と実行前確認

担当 `/root/gui_failure_investigation`。依頼 `gui-release-observation-request-01.md` SHA256 `DE10D7ED8B4EC76173EFA3E3BDBCE6687F44802F1645AF982A27C91A2D307A07`。これは実行前準備であり GUI は未起動。実行器と具体方式の独立レビュー、管理の開始通知まで起動しない。

## 版と画面の前提

- release EXE を `gui-release-build-01` の保護領域から新規 `gui-release-observation-01/github-issue-widget.exe` にコピー、両 SHA256 は `23C383EB2E46CB205E052A9D4A24EE73B968EB2C776F7150826350685419B306` で一致。専用観測フォルダーは親と別担当が参照できる共有 run 配下、既存 exe は変更していない。ビルド中の元入力変更は `src/main.test.tsx` のみ。固定 `index.html` は `src/main.tsx` を起点、`main.tsx` はそのテストを import せず、`tsconfig.json` は `noEmit=true`。テスト差分が Vite の本番エントリに取り込まれる経路は当該静的範囲に見当たらないが、依存全体やビルド過程の完全な因果を実測したわけではなく、型検査の対象には含まれる。実行時対象はあくまで上記 EXE ハッシュ。
- CUA の Windows インベントリで複数のデスクトップ窓が見えた。親スクリプトを起動する PowerShell は session 2。UI 操作環境と子が同一セッションになること、WebView2 実行と表示は未確認。GUI が見えない場合を成功としない。CUA を使用する際は対象ウィンドウを識別して操作する。
- 観測フォルダー内の新規 `appdata`・`localappdata`・`temp`・`profile`・`webview2` は空、再解析点0。5ディレクトリのみ、実行者・SYSTEM・Administrators にアクセスを限定した。`WEBVIEW2_USER_DATA_FOLDER` も専用に指定する。OS credential store は共有であり環境変数では隔離されない。同一ユーザーが同じ場所へ並行書込みするリスクは残る。実操作直前に対象実体・空・ACL・他作業を再確認し、設定・復旧記録がない条件を維持する。

## 実行器と観測

`gui-release-observation-01/observe.py` SHA256 `F82AD7CADEC4F94B1E7832A32A701C0CE59CEE9E713AAD1833834FC839F48CF7`。候補 `python docs/agent-runs/20261121-orchestration-continuation/gui-release-observation-01/observe.py` を1回だけ実行する。コードは EXE SHA・5領域の空・状態ファイル不存在を確認し、排他的な `launch_pending` を確定後、トークンを含まない明示環境で EXE を引数なし直接起動。stdin/stdout/stderr は DEVNULL。UI 保存・入力・credential・GitHub 操作はしない。既存の設定・fixture・不明プロセスは操作しない。

直接子の保持ハンドルで実行パスと作成時刻を読み、PID・照合結果を出す。識別できなければ他窓に触れない。30秒後にまだ生存していれば `direct_alive_at_30s` と表示して待機を継続し、そのことだけで表示成功としない。CUA の窓情報を取得し、PID に対応する実際の HWND を保持した直接子のプロセス情報と照合、対象が一意なら設定画面の存在・判読性を確認し、対象のみ通常の閉じる操作を行う。閉じた後、同じ実行セッションを期限付きで照会し、直接子の終了コードを確認する。識別不能、UI 不可、終了未確認時は再実行・不明プロセスの停止をせず対象と記録を保持する。実行器の停止やプロセス不明は `launch_pending` を残して成功にしない。

DEVNULL は機密を保存しない代わりに panic の詳細を得ない。直接プロセス終了は WebView2 等の未知の子孫の終了を証明しない。WebView2 と OS credential は既存ユーザー環境の一部を共有し、非干渉・秘密の完全保護を保証しない。終了コード101の再発、UI 検出失敗や不明は事実を記録し、安易に反復しない。２回同じ失敗なら別担当が調査する。開発モードは別検証。

## 確認

`Get-Content`・限定 `rg`・`Get-FileHash`・ディレクトリ再解析点/空確認は終了0。`rg` の `main.test` 参照検索は一致なしで終了1、検索成功の空結果として扱い実行エラーではない。実行コード・GUI・資格情報・GitHub・合成子は起動していない。新規 EXE・実行器・空領域のみ作成し、5領域の ACL を設定した。EXE と実行器のアクセスを共有する一方、入力領域への広い ACL を除いた。削除対象なし。完成後に版が変われば再審査する。
