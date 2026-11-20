# 統合候補の Windows リリースビルド

- 作業場所 `C:/Users/semis/orca/workspaces/GithubIssueWidget/issue-6-move-resume`、HEAD `ab6122ef26bd62d5ec8dffbda1d35cd1a09606e0`。未コミット差分はレビュー済みの ignored 実 GitHub テストを `widget/src-tauri/src/movement.rs` に追加したもののみ（SHA256 `D7C987DB85F69482EF005FB95D9DA4C85DA5CBAACC2C1207B6A5F26EE5496E9F`）。
- Visual Studio x64 環境で `npm run tauri -- build --no-bundle` を実行し終了コード0。フロントエンドの TypeScript と Vite のビルドも成功。
- 実行ファイル `widget/src-tauri/target/release/github-issue-widget.exe` SHA256 `F704EFA24350D7295F5C3FABBF92818A7421A598865551A21C9DA53B877EFB65`。アプリの起動・画面操作・インストーラーの生成は本検証に含まない。
- Tauri は bundle identifier の `.app` 終端が macOS のアプリ拡張子と競合する旨を警告した。今回の対象は Windows。
