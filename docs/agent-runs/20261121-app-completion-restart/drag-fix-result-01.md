# HTML ドラッグ設定の修正結果

担当 `/root/drag_fix`。作業場所 `D:/Projects/GithubIssueWidget`。依頼 `drag-fix-request-01.md` に基づき `widget/src-tauri/tauri.conf.json` の `app.windows[0].dragDropEnabled` を `false` にした。変更前 SHA256 は指定どおり `5C1F3A0E2E487DD4345A2DD335FE4AACA483FEEEA26C11AAB99B374278EC1364`、変更後 SHA256 は `0132FCC92345F5D60DE84FD25197FCE853044AD63DAE28757AB4592CC72B991E`。

## 根拠と検証

- ローカルの `widget/node_modules/@tauri-apps/cli/config.schema.json` に対象プロパティがあり、型は boolean、既定値は true。説明は Windows の WebView2 で HTML5 ドラッグを使う場合に無効化が必要と記載する。この変更はユーザーが報告したドラッグ不能への候補修正であり、原因確定ではない。
- JSON を `ConvertFrom-Json` で読み、ウィンドウ1件と値 false を確認した。`git diff -- widget/src-tauri/tauri.conf.json` は同プロパティ追加のみを示す。いずれも終了コード0。
- `rg -l -i -g '*.ts' -g '*.tsx' -g '*.rs' -g '*.json' 'onDragDropEvent|tauri://drag|DragDropEvent|FileDrop|dragDropEnabled' widget/src widget/src-tauri/src widget/src-tauri/tauri.conf.json` は設定ファイルのみを返し、終了コード0。確認範囲では Tauri のネイティブファイルドロップへの参照は見つからなかった。依存ライブラリ等の検索範囲外は保証しない。

実 UI の操作、ビルド、実 GitHub、資格情報は扱っていない。新しい実行ファイルでのドラッグおよびウィンドウ・リンク等の回帰は独立ビルド後に確認が必要。操作中に認識した失敗はない。変更は指定設定と本結果のみで、削除対象はない。
