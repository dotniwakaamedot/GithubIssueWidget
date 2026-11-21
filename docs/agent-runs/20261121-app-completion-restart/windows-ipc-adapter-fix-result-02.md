# Windows IPC 隔離コピーの安全境界修正 02

## 結果

独立レビューの指摘どおり、旧版は親テストのみの禁止で、子テストの直接実行を防げていなかった。隔離コピーに共通の無条件停止関数を設け、親・子の両入口と、この `main.rs` 内の Credential Manager の全 Entry 生成経路に適用した。最終版をオフラインでコンパイルし、親・子・共通 Entry ゲートを各々直接指定したテストは、指定された禁止メッセージで停止した。**実 OS Credential Manager・実 IPC の試験ではなく、実 OS 解禁の根拠ではない。**

担当 `/root/windows_ipc_adapter`。依頼 `windows-ipc-adapter-fix-request-02.md`、作業場所 `D:/Projects/GithubIssueWidget`。開始時 SHA256 は `main.rs` `F10F86CCC567801D197B92222CE478F0D98F8CC0C4575A9DA0E9CFA8D4DED61E`、`test_adapter.py` `A3682FD85B4A59DFE6FEF24F00712D63C0A758B46B37D2F8277CF00E9FB8DCA5`、`adapter.py` `0A350F40863D3E80FA621195EAB8143E404026C594BF802CB382971EA6F421B0` と依頼値に一致。

## 対応

- コピー `windows-ipc-adapter-01/main.rs` に `block_os_credentials()` を追加。この関数は引数や環境変数にかかわらず固定文言で panic する。`windows_ipc_guard` と `windows_ipc_guard_child` の関数本文先頭で同じ関数を呼ぶ。両テストには隔離コピー限定の `should_panic` を付けた。
- 同コピー内の `keyring::Entry::new` 呼出し５箇所を `blocked_keyring_entry` に置換し、その helper も共通停止関数を呼ぶ。これにより別テスト・`entry`・cleanup から constructor が使われても、このコピーの該当経路は実 Entry を作成しない。追加した `adapter_central_gate_direct` は helper の直接呼出しを検査する。依存ライブラリ全体やこのコピー外の全 OS API を網羅した保証ではない。
- `test_adapter.py` は親・子の各先頭ゲート、共通関数、直接 `keyring::Entry::new(` がコピー内に残らないことを静的照合する。`adapter.py` は変更せず、実 OS backend を持たない既存モデルである。既存の模擬 journal・障害試験は維持。
- 更新後 `main.rs` SHA256 `C160E15184C3651F3F17432DF9A5B14A40FEAB899EF8B6EADA7AACA48F7DC862`、`test_adapter.py` `8E9105F2D3B48B069F75DDFAA8636638297DEF2B5ED71DA97D6DAD97B7318F7F`、変更しない `adapter.py` `0A350F40863D3E80FA621195EAB8143E404026C594BF802CB382971EA6F421B0`。本番 `widget/src-tauri/src/main.rs` は終了後も `AEFF1E65AD93EFA04773CEBBB9736A6BDC5B9938DB9FAF937DD88B3D9CBFE86F`。

## 実行と検証

- 新規 `windows-ipc-adapter-01/crate/` に `widget` の rust-toolchain、dist、src-tauri の Cargo 入力・src・tests・icons・capabilities・設定をコピーし、その `src/main.rs` を上記隔離コピーで置換。最終版のコピー SHA は上記 `C160...` と一致。専用 `crate/target` と `CARGO_NET_OFFLINE=true`、`--locked --offline` を使用。既存 target・本番ファイルは変更しない。初回 `cargo test --locked --offline --no-run` は終了 0。後の共通ゲート直接テスト追加時にコピーの旧ハッシュを確認して同期し、最終版で Cargo が再コンパイルした。
- `cargo test --locked --offline tests::windows_ipc_guard -- --exact --ignored --test-threads=1`、同じ形式で `tests::windows_ipc_guard_child`、`tests::adapter_central_gate_direct`（最後は `--ignored` なし）を各別プロセスで実行。各終了 0、各ログは該当名・`should panic ... ok`・`1 passed; 0 failed; 82 filtered out` を記録する。固定の panic 文言を `should_panic(expected=...)` が検査するため、成功は安全ゲートによる停止を示し、本来の IPC 試験成功ではない。各実行の統合テスト側は 0 件。
- ログ SHA256: `parent-direct-03.log` `86BA5C639FA0361512E042DA9E9B4289E21FD01EC60B767B8CE4A6890EF4892F`、`child-direct-03.log` `12CFE6CC71EB7C3A69D3BDD32F63C7DA7C00CFE19AA9CDC6ED17BA6722541CC2`、`central-direct-03.log` `F6405319A42088C6B08F42B89B00740ADAAA56EEAAD7557CBD49563F5AF8B6AD`。各 exit 記録は `0`、SHA256 はいずれも `13BF7B3039C63BF5A50491FA3CFD8EB4E699D1BA1436315AEF9CBE5711530354`。最終 Rust test binary `github_issue_widget-fc25f1eb6ae85386.exe` SHA256 `B3C6054EAE6FEB4339616951665B2BE82381ABD450B5C7748CBC857499954D2D`。
- 同ディレクトリで `python -m unittest -v test_adapter` は終了 0、10 件成功。最終 `test-05.log` SHA256 `729E35916D81DBFC31F764FF3658BF168D3030714593E6EC02CC68BE38D77DD4`。標準出力・標準エラーは各ログへ保存し、ログのテスト結果と別に終了コードを保存。Cargo は rustc・リンカー・テストバイナリを子として起動し、全コマンドは終了済み。Python の模擬監督が起動する子も Python の合成経路であり、実 keyring・実アプリ・GitHub に接続しない。

## 制約と引継ぎ

- 全ゲートは隔離コピーの試験用変更であり、本番アプリへ適用していない。`should_panic` を外した実 IPC の通過、実 Credential Manager の登録・更新・削除、実 WebView、PAT の非露出、OS 隔離・排他・ACL・耐久記録・親子孫の終了は未検証。
- Python journal 監督と Rust guard は未接続。無条件ゲートを削除・迂回して OS 操作を行うことは本依頼の範囲外。別担当による新版レビュー後も、所有権・排他・異常終了時の記録と復旧を別途立証するまでは実書込みを行わない。
- 成果は隔離コピーの `main.rs`・`test_adapter.py`、新規 `crate/`・ログ・exit 記録と本結果のみ。元の結果・旧ログ・`adapter.py`、既存製品・設定・資格情報・GitHub を変更せず、削除対象なし。作業中に認識した失敗はない。UTF-8 実データを読戻し、一時ファイルから確定する。
