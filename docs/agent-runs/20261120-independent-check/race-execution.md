# 隔離競合テスト実行記録

- 対象 `widget/src-tauri/src/main.rs` SHA256: `7052631058511BBBBC70477CE4A8D807444CF8E9DCC64828283C06CF78CA5AE2`。実行前後の対象 SHA は一致。
- 実行前レビュー: `race-code-review-05.md`。Cargo.toml / Cargo.lock は同レビューに記載した SHA と一致。
- 作業場所: `D:/Projects/GithubIssueWidget/widget/src-tauri`。Windows の x64 Visual Studio 環境で `cargo test --locked -- --exact tests::windows_inflight_guard --ignored --test-threads=1 --nocapture` を単独実行し、終了コード0。
- ログ: `%TEMP%/githubissuewidget-real-lock-test-20261120.log`、SHA256 `3B5388B4BF36F8F82640A615D73DF2E4FD735A141FE9E0AE3BEC5E80BADB528D`。テスト実行ファイル SHA256 `F4E38714CE92D34B8FDC98DC7C107927F26BED89D6383C67A9FB428AE4FF8F54`。
- 結果: 1 passed。正常取得と、HTTP 応答ヘッダー受信・共有ロック解放後に別プロセス側から設定を変更したケースを実行。テストは古い Issue を返さないことを確認し、両合成資格情報の削除と削除後の不存在を確認した。ログに `stage=normal-and-stale-cases-confirmed` と `stage=credential-cleanup-confirmed` がある。
- 親テストは子の終了と server / changer スレッドの終了を確認してから cleanup を許可する。子 PID `26608` はテスト後の確認でも存在しなかった。PID は再利用されるため、将来の削除・終了操作の対象根拠には使わない。
- 合成 ID と一時 fixture はテストログに記録。fixture は証跡として保持し、実トークンとユーザーの既存 APPDATA は使用していない。同一ユーザー権限内の fixture marker は攻撃者に対する隔離境界ではない。
- 実際の `list_issues` の Tauri 入口全体、`save_connection`、WebView、実 GitHub、最終照合後の別プロセス変更はこのテストの対象外。
