# 合成資格情報試験の監督経路（実 OS 禁止）

担当 `/root/credential_supervisor`。作業場所 `D:/Projects/GithubIssueWidget`。親からの依頼に従い、新規 `credential-supervisor-01/` のみへ試験用コード・コピー・ログを作成した。**実 Credential Manager、実 PAT、既存サービス、アプリ本体、GitHub の書込みは行っていない。実 OS の保存・更新・削除は未検証であり、この結果は実行許可でも完成判定でもない。**

## 実装と安全境界

- `src/main.rs` は先行 `credential-live-test-01/src/main.rs` の隔離コピーで、合成 service `github-task-widget-synthetic-check-<UUID>` と account `synthetic` を使用。既存アプリの service `github-task-widget` や PAT は使用しない。書込み前に service・account に加え、自身の PID と実行ファイルパスを新規 journal に記録し、flush・sync・実データ読戻しを行う。各操作前の stage も同期する。PID 単独では PID 再利用やプロセス終了の証拠にならない。
- 保存前は NoEntry を確認し、合成値 A を保存・読取、B に更新・読取する。削除直前に値が B と完全一致した場合だけ削除し、NoEntry を確認する。失敗・応答不明では再送・自動 cleanup をしない。実行体はコード上で子プロセスを生成しない。UUID と read-check は原子的な予約ではなく、他プロセスによる同名操作の競合・上書き・削除を完全に防げない。
- Rust の `LIVE_APPROVED` は固定 `false`。`--features live` でコンパイルしても `main` はコード 2 で終了する。別の `supervisor.py` も CLI で無条件に実行禁止。Python の監督関数は試験用の注入 fake でのみ動く設計で、実 Credential Manager の実行経路には接続していない。
- 模擬監督はバイナリの専用領域・SHA 照合、開始 record の排他作成・同期・読戻し、直接子の待機、期限時 kill と待機、不明時の `termination_unknown`、結果 record とログ保持を実装。処理終了不明を成功にせず、対象 credential を自動削除しない。標準出力はファイルにリダイレクトし、完了後に 64 KiB 超を記録するが、実行中の上限は強制しない。実環境でのログ上限は未実装。

## 対象版と検証

実行場所は新規専用領域。先行 cargo-home を新規ディレクトリにコピーし、`CARGO_HOME` と `CARGO_TARGET_DIR` を同領域へ指定、`CARGO_NET_OFFLINE=true` と `cargo +1.90.0 --locked --offline` を使用した。合成値だけを使うテストで、既存の資格情報領域には触れない。

| 実体 | SHA256 |
| --- | --- |
| `src/main.rs` | `53196188DB8CD8D26288797A06DFAF512A554476AEDF87F678814D97C7EF8ADB` |
| `Cargo.toml` | `38A06A4CCBAE1312682033DE8741DBFBB65C9BFAE40129F85B6C80A546583DA1` |
| `Cargo.lock` | `79B1644981C6B122726DDD3AC55E54616C815F9E56AF52ADE760C3B402D8EAA5` |
| `supervisor.py` | `435DF23AEBA4810B29ED832C59A03D960D316F6B3A780CB60ABCF13E271A0EAA` |
| `test_supervisor.py` | `16B01B2D33D1545EA9AA9C34DAD36455D7F13F115F874CC8066D572704AC745C` |
| `py-test-02.log` | `FFFE5D1F64222BEEC2F81D0A6CE042CE7E041EF5E0AE3E26A5872A4625A77C7A` |
| `rust-test-02.log` | `7005A90A69734714EC91B3C77F85F8465A7D3B6026C8B5A8617544C349B6C690` |
| `rust-test-02.exit` | `13BF7B3039C63BF5A50491FA3CFD8EB4E699D1BA1436315AEF9CBE5711530354` |
| `rust-live-build-02.log` | `63D3D4DBECF7675E97220E4E56E0A015496C5674C18F5AA220DD316D7DD28036` |
| `rust-live-build-02.exit` | `13BF7B3039C63BF5A50491FA3CFD8EB4E699D1BA1436315AEF9CBE5711530354` |
| `rust-disabled-02.log` | `40D41E615B0DE8CF195842AF3FB5D04802E91DA6B014D9C25452A7BCC479B305` |
| `rust-disabled-02.exit` | `DF4E26A04A444901B95AFEF44E4A96CFAE34690FFF2AD2C66389C70079CDFF2B` |
| `supervisor-disabled-01.log` | `53C79E2A57601381830CADC92E0195BB1CA89DB9B39A546853AEF1F31C8916B5` |
| `supervisor-disabled-01.exit` | `DF4E26A04A444901B95AFEF44E4A96CFAE34690FFF2AD2C66389C70079CDFF2B` |
| `target/debug/giw-credential-synthetic-check.exe` | `12D523A457F25E25D249F01221DAEC579F62B61B35469A08AACCA384BC0E97C2` |

- `python -m unittest -v test_supervisor`: 最終版７件成功、終了 0（`py-test-02.log`）。正常、開始失敗、期限超過後停止、終了不明、同じ record の再実行禁止、ハッシュ不一致、実ランチャー禁止を fake process で確認した。実プロセスの停止・PID・Windows OS 障害は未検証。
- `cargo +1.90.0 test --locked --offline`: 最終 Rust ９件成功、終了 0。NoEntry、journal 失敗、未知の書込み・削除、対象不一致、入力経路を模擬 store で確認した。
- `cargo +1.90.0 build --locked --offline --features live`: 終了 0。生成 EXE と `python supervisor.py` はそれぞれ禁止メッセージで終了 2。いずれも資格情報 API 呼出しには至らない。

## 実行前に必要な作業と停止条件

実 OS 試験を行うには、まず実際に専用 target の他者による同名操作を排除できるかを評価し、できない場合は試験を実施しない。`CredWriteW` は既存同名を上書きするため、乱数・NoEntry だけを原子的所有権とは扱わない。実行用のゲート解除と supervisor の実子起動接続、実行時ログ上限・journal/ログ ACL、プロセス開始時刻など PID 再利用を含む識別と終了・残留確認の方法は未実装であり、その新しい版・バイナリを独立レビューする必要がある。

実施する場合も、対象 service/account と journal の読戻し、実行体ハッシュ・PID・開始時刻、専用対象の事前 NoEntry、実プロセス終了を確認し、曖昧な応答・停止状態・journal 障害では停止して再送・削除を行わない。中断後は対象 journal と実行体の終了を識別してから読み取り、合成 A/B の完全一致と対象の所有関係を確認できない限り cleanup しない。別途、残留時の具体的な実行方法と復旧証拠が必要。OS クラッシュや第三者の同名操作に対する無条件の停止・復旧保証はない。

成功しても keyring と OS API の合成ターゲットに限られ、実アプリでの PAT 初回登録・変更・削除、実 WebView IPC、ネットワーク異常を証明しない。

## 失敗と未完了

最初の `python -m unittest -v test_supervisor` は Windows パスの JSON エスケープを考慮せず、文字列の生バイト比較で１件失敗・終了 1。元出力はツール上で確認したが独立ログに保存していない。JSON をデコードしてパスを比較する修正後に、上記の模擬７件はすべて成功した。同種２回目は認識していない。共有失敗記録への反映と独立レビューは親に引き継ぐ。削除対象なし。専用領域の cache・build・ログは残しており、他者の成果は変更していない。
