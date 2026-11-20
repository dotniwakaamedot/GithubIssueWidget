# 統合候補の合成資格情報テスト

- 対象 `issue-6-move-resume`、HEAD `ab6122ef26bd62d5ec8dffbda1d35cd1a09606e0`。`widget/src-tauri/src/main.rs` は読み取り専用 GitHub テストを追加した版で SHA256 `C140670F7DCD568DEF33401EFE5B409973E44347D12F41D8CBC20AA611CCE4BC`。
- Windows 上で `cargo test --locked isolated_windows_credential_lifecycle -- --ignored --nocapture` を実行し、1 passed、終了コード0。UUID 付きのテスト専用 Credential Manager エントリに合成値を登録・取得・更新し、削除後に存在しないことを確認した。
- 実トークン、アプリの通常のサービス名、既存資格情報は使用していない。アプリ画面からの登録・再起動後の取得・保存途中の異常終了や実トークンの操作を証明するものではない。
