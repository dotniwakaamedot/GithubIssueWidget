# 検索エラー照会への回答

担当 `/root/move_review`。順に以下の2回。実行場所はいずれも `C:\Users\semis\orca\workspaces\GithubIssueWidget\issue-6-move-resume`。失敗は再実行していない。

1. コマンド: `rg -n -C 7 'get_api_response_locked|HTTP 404|404|state_reason|drag|競合' widget/src-tauri/src/main.rs widget/src-tauri/src/creation.rs widget/README* widget/src/style.css`
   - エラー: `rg: widget/README*: ファイル名、ディレクトリ名、またはボリューム ラベルの構文が間違っています。 (os error 123)`
   - 終了コード: ツール結果で output のみを出力したため未確認。
   - `widget/README*` は事前の存在確認・列挙をしていない。PowerShell のネイティブ実行における wildcard の展開を誤って期待した。
2. コマンド: `rg -n -A 42 'async fn check_api_response' widget/src-tauri/src/main.rs; rg -n '競合|同時|ドラッグ|ステータス' widget/README.md README.md`
   - エラー: `rg: widget/README.md: 指定されたファイルが見つかりません。 (os error 2)`
   - 終了コード: ツール結果で output のみを出力したため未確認。
   - `widget/README.md` は事前に存在確認せず推測して指定した。

その後、リポジトリ直下の `README.md` は `Test-Path -LiteralPath 'README.md'` で存在を確認後、`Get-Content -LiteralPath 'README.md' -Encoding UTF8` で読めた。
