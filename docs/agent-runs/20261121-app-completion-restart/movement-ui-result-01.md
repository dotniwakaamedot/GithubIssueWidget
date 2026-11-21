# 列移動の操作確認

対象: `widget/src/main.tsx` SHA256 `B22504C9DE48D275726C912820DA93E02F10C1838CCC68555C156D35030165D7`、`widget/src/style.css` SHA256 `EB49BF65AE4FCE6701FBEF277055EDCAD42011248E5EA49905B6AFA4FD288B37`。依頼 `movement-ui-request-01.md` SHA256 `E4A6BBFB08A86EC54A4696D1EBA76DF703C93DC4446A27651D2135F1EE457BC9`。作業場所 `D:/Projects/GithubIssueWidget`、branch `main`、HEAD `4961b25bec53a93439acdf5230de77f959908af0`。

Issue 一覧で、移動したいカード（番号とタイトルが表示される矩形）の上でマウス左ボタンを押したまま、移動先の `Todo`、`In Progress`、`Done` の列内まで動かし、ボタンを離す。ドロップ受付は列全体の section。カードをクリックすると Issue の URL を開く操作であり、列見出しの「＋」はその列への新規作成である。移動中・作成中・作成ダイアログ中はカードのドラッグが無効、同じ列へのドロップは何もしない。移動中には「Issue を移動しています…」、失敗時はエラーを表示し、終了後に一覧を再取得する。

コード上、別の移動ボタンやキーボード移動処理は確認できない。実 GUI は操作しておらず、実際のドラッグ動作・環境依存の代替操作の有無は未検証。

検証: `rg` と対象ファイルの SHA256 読取り。最初の `rg` は PowerShell で `widget/src/*.css` を明示したため Windows がパスを解釈できず終了1、main.tsx 側のみ部分取得。次に `rg -g '*.css' ... widget/src` で正常取得・終了0。失敗1件の記録を管理へ依頼する。同種2回は発生していない。削除対象なし。変更は本結果ファイルのみ、製品変更・アプリ起動・外部書込みなし。確定後の結果 SHA256 は管理が別途取得する。
