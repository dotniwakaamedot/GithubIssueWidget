# Windows IPC 合成試験の隔離経路

## 結果

対象 `main.rs` を専用領域へコピーし、既存 `windows_ipc_guard` の Credential Manager 呼出し前に、実 OS 経路を無条件に禁止するゲートと service・account の journal 読取りを加えた。別の Python 監督モデルは OS 資格情報 API を持たず、識別子の排他作成・flush・fsync・読戻し、模擬子の監督と失敗時の停止を扱う。**実 OS 試験は未実施であり、この成果から実 IPC・安全な OS 書込み・アプリ完成を主張できない。**

担当 `/root/windows_ipc_adapter`。作業場所 `D:/Projects/GithubIssueWidget`。依頼 `windows-ipc-adapter-request-01.md` の対象本番 `widget/src-tauri/src/main.rs` は開始前・終了後とも SHA256 `AEFF1E65AD93EFA04773CEBBB9736A6BDC5B9938DB9FAF937DD88B3D9CBFE86F`。生成直後のコピーも同一だった。

## 実装と試験

- コピー `windows-ipc-adapter-01/main.rs` SHA256 `F10F86CCC567801D197B92222CE478F0D98F8CC0C4575A9DA0E9CFA8D4DED61E`。元の `windows_ipc_guard` に限定して `REAL_OS_ENABLED=false` の assertion を Credential Manager の `Entry::new` より前に配置。journal の service・account は prefix と正規 UUID を検査する。元の mock IPC 子処理はコピー内に保持した。Rust コピーは今回コンパイル・実行していない。
- `adapter.py` SHA256 `0A350F40863D3E80FA621195EAB8143E404026C594BF802CB382971EA6F421B0` は新規 journal を排他的に作成し、flush・`os.fsync`・読戻し成功後のみ同一プロセス内 receipt を返す。モックの排他 gate、識別子一致、直接子起動・待機・timeout 後 kill と待機を確認し、不明・不一致では自動 cleanup しない。子は Python の合成処理のみで、実 keyring・Tauri IPC を呼ばない。Windows 子は `CREATE_NO_WINDOW`。receipt・排他は模擬値であり OS の所有権の証拠ではない。
- `test_adapter.py` SHA256 `A3682FD85B4A59DFE6FEF24F00712D63C0A758B46B37D2F8277CF00E9FB8DCA5`。専用領域から `python -m unittest -v test_adapter` を実行し終了コード 0、**10 件成功**。`test-03.log` SHA256 `07AB7DEAC9C61F6B53EDA705A9B222BCD9F4C2B7F21129DFDC9E7BA033465CA6` は `Ran 10 tests` と `OK` を記録する。対象は記録 create/flush/readback 失敗、衝突、保存応答不明、cleanup 不一致、子起動失敗・異常終了・timeout・終了不明、識別子不一致・排他 gate、模擬保存・更新・削除と Rust の静的 gate 順序。実行子は Python のみで、実資格情報やネットワークには触れない。
- 実装変更前の試験と receipt 強化後の試験はそれぞれ終了 0、`test.log` と `test-02.log` として保持。最終変更後に `test-03.log` を取得した。実行終了コードは PowerShell が `$LASTEXITCODE` を取得し親処理も 0 で終了した。模擬データのみのため実 PAT は取得・表示・保存していない。

## 未完了・制約

1. Python モデルは Rust `windows_ipc_guard` を起動しない。Rust の journal 消費と Python 監督は未接続であり、今回の10試験は実アプリの機能試験ではない。Rust のコピーもビルド検証していない。
2. ファイル内容の fsync はディレクトリエントリや電源障害時の耐久性・ACL・所有権を証明しない。複数プロセスや外部者を排除する実 OS の排他、親停止時の子孫管理、真の異常終了・IPC pipe 枯渇、秘密ログ保護は未実装・未検証。UUID・不存在確認・模擬 exclusive を実 OS の排他としない。
3. timeout 後の kill は直接子についてのみで、OS 障害下や孫プロセスの無条件停止を保証しない。状態不明時は再送・自動削除しない。journal の識別子と既知の合成値・プロセスの終了を独立に照合できるまで OS 対象を触らない。対象値不一致は他者データの可能性があるため停止する。
4. 実 OS 試験の前に、独立レビュー、実行対象と journal の安全な接続、排他・所有権・ログ ACL・残留確認・子孫の停止条件、復旧手順の具体化と故障試験が必要。条件を証明できない環境では OS 書込みを行わない。

変更は新規 `windows-ipc-adapter-01/` と本結果のみ。既存製品・設定・資格情報・Git・GitHub は変更せず、削除対象なし。作業中に認識した失敗はない。結果確定前に UTF-8 の実データを読戻し、対象を一時ファイルから公開する。
