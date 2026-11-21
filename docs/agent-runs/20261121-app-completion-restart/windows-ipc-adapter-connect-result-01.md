# Windows IPC 隔離接続の調査結果

## 結論

依頼の安全境界を維持したまま、既存 Rust guard と監督側 journal を模擬 credential backend に接続する小規模な変更は確認できなかった。**接続・Rust コンパイル・追加試験は未実施**であり、実 OS 書込みは行っていない。自己完結した別の模型だけで「既存 guard と模擬 IPC の接続成功」と表現することもしていない。

## 対象と確認

- 担当 `/root/windows_ipc_adapter_connect`、作業場所 `D:/Projects/GithubIssueWidget`。依頼 `windows-ipc-adapter-connect-request-02.md` と適用指示を読取り、対象レビュー・結果・3入力の SHA256 は依頼値に一致し、レビュー02の blocking なしを確認した。
- 新規 `windows-ipc-adapter-connect-01/` に元 `main.rs`、`adapter.py`、`test_adapter.py` をコピーし、SHA256 はそれぞれ `C160E15184C3651F3F17432DF9A5B14A40FEAB899EF8B6EADA7AACA48F7DC862`、`0A350F40863D3E80FA621195EAB8143E404026C594BF802CB382971EA6F421B0`、`8E9105F2D3B48B069F75DDFAA8636638297DEF2B5ED71DA97D6DAD97B7318F7F` で元と一致。コピーは変更していない。
- 親 `windows_ipc_guard` と子 `windows_ipc_guard_child` は関数冒頭で無条件停止し、両者に `should_panic` がある。journal 読取りは親の停止後であり、共通 `blocked_keyring_entry` も無条件停止する。`entry` は `Result<keyring::Entry, String>` を返し、`read_token` は新しい Entry から取得する。したがって停止を残して単に journal を渡しても既存 IPC へ到達しない。
- オフライン依存 `keyring 3.6.3` の `src/mock.rs` は、組込み mock のパスワードを Entry 自体にだけ保持し、新しい Entry は空であると説明する。既存コードの Entry 再生成および親子別プロセス間で値を共有する代替には、そのままではならない。この確認は mock 全体の可能性を尽くした証明ではない。

## 必要な次作業と限界

専用コピーの実 OS 禁止入口はそのまま保ち、別の明示的な mock-only 入口から、service/account に制限したプロセス間共有の合成保存先と IPC 内の Entry 生成経路を接続する必要がある。これは読取り、更新、削除と cleanup の経路をまたぐため、追加設計と所有範囲の明確化・レビューが必要。少なくとも journal の検証失敗時に操作しないこと、対象・値不一致時の cleanup 停止、子の終了不明を独立検証し、合成保存先への書込みと実 Credential Manager の区別を維持する。mock builder やプロセス間保存の選択は未決であり、実 OS の排他・所有権・実 WebView・実 PAT を証明しない。

## 操作・未確認

`Get-FileHash`、限定 `Get-Content`、限定 `rg`、専用ディレクトリ作成と3ファイルコピーを実行し、必要な SHA とコピー結果を確認した。追加の Rust guard 接続、ビルド、テスト、アプリ起動、Credential Manager・Git・GitHub 操作は行っていない。未追跡の新規専用領域と本結果だけを作成し、削除対象はない。作業中に認識した失敗はない。既存の版・証拠は変更していない。
