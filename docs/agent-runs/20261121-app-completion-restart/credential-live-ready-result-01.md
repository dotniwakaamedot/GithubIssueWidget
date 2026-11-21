# 合成資格情報の実 OS 試験候補

担当 `/root/credential_live_execution_prep`。対象は新規 `credential-live-ready-01/`。**実行可能版だが未承認であり、実 Credential Manager のテストは実施していない。** 製品、既存資格情報、実 PAT、GitHub、実アプリを変更していない。

## 実装と安全境界

- 先行 `credential-supervisor-01/src/main.rs` と Cargo manifest/lock を専用領域へコピーし、`LIVE_APPROVED` のみ true にした。Windows の `keyring` 3.6.3・`Entry::new` と `get_password`/`set_password`/`delete_credential` を使用するが、実アプリの設定・サービスや PAT は使用しない。アプリそのもの、実 WebView や実 PAT の実証ではない。
- 子は `github-task-widget-synthetic-check-<UUID>` と account `synthetic` を生成し、Credential API 呼出し前に専用領域の新規 journal に両識別子・PID・実行ファイルを記録、flush、sync_all、全文読戻しする。各段階も記録する。NoEntry 以外なら書込みせず、合成値 A/B の保存・読戻し後、削除前に B と完全一致することを確認し、削除後 NoEntry を確認する。エラーは停止し再送・自動 cleanup しない。
- 新規 `supervisor.py` は `--execute` の明示を要求し、専用領域内のバイナリの SHA を照合、起動前に start 記録を新規作成・同期・読戻しし、単一の直接子を非表示で起動する。標準入出力は DEVNULL とし秘密や無制限ログを保存しない。子 PID を新規記録し、30秒で timeout なら kill と5秒の wait を試み、確認不能なら `termination_unknown` と記録する。監督者の停止で子が必ず停止する保証はない。
- `RECOVERY.md` は journal、PID・実体・終了状態の照合、同名の他値や不明時の停止と、対象限定の診断・復旧を別途レビューする手順を記載する。現在、認証情報の読取りや手動 cleanup 用の承認済み utility はない。自動 cleanup の対象はテスト内で直前確認した合成 B のみ。
- Windows API の NoEntry→write と値読取→delete は原子的でない。UUID と限定名は衝突可能性を低くするが同名第三者による競合を排除せず、万一の上書き・削除リスクはゼロではない。親は実行環境・許容リスクと異常後の確認手段を独立レビュー後に判断する。

## 検証と版

作業場所 `D:/Projects/GithubIssueWidget/docs/agent-runs/20261121-app-completion-restart/credential-live-ready-01/`。Cargo 1.90.0、先行領域の `cargo-home` を読取り利用、専用 target、`--locked --offline`。模擬 `cargo +1.90.0 test --locked --offline --features live` は終了0、**9 passed**。feature 付き `cargo +1.90.0 build --locked --offline --features live` は終了0。PowerShell は stderr の通常のコンパイル進捗を `NativeCommandError` と表示形式で記録したが元 Cargo 終了値は0。`python -m unittest -v test_supervisor` は終了0、**6 tests OK**。後者は実 launcher の異常時や OS 資格情報を実行していない。ログ・exit ファイルを専用領域に保存した。

| 対象 | SHA256 |
| --- | --- |
| `src/main.rs` | `06EDCEB1D72A3EAE5A09FB7462B39A37E9BFD6B457AF9D5923438F268C0A847B` |
| `Cargo.toml` | `38A06A4CCBAE1312682033DE8741DBFBB65C9BFAE40129F85B6C80A546583DA1` |
| `Cargo.lock` | `79B1644981C6B122726DDD3AC55E54616C815F9E56AF52ADE760C3B402D8EAA5` |
| `supervisor.py` | `8063631DEDC6CFA445CA22072A583B46F861EAD66A8C49EEA125A5E1797CFB76` |
| `test_supervisor.py` | `125F2A4BD05DF581FAAA0878B89DEE8E6978A0BF8524C405C152F9B42D2F10DF` |
| `RECOVERY.md` | `F85717FE9CCDA29A41E38B3D931163B74D4F588260B4E8FF1573EC4B9A3C3EED` |
| `target/debug/giw-credential-synthetic-check.exe` | `EFA493B07535142A3D9B98469795B4DA88E6979278BD2EBC1087E1E3AA8E649E` |
| `rust-test.log` | `2A7294C0E029E21582E1A4E3FEEDC97DE684DCFA416EED57F71F2CB6CD877F02` |
| `rust-build.log` | `190CBE2BDA4744FF06919E8F1F3F53B7DE58336965CF65BDFB73A52F1F868BA0` |
| `supervisor-test.log` | `DB60ED952DACB3E34A8EFC2EFA9A1FD671575EF846CA28C803D3C5A002C47F9C` |

各 exit ファイルは ASCII `0` で SHA256 `5FECEB66FFC86F38D952786C6D696C79C2DBC239DD4E91B46729D73A27FB57E9`。この候補の実行方法は対象領域から `python supervisor.py --execute` だが、**独立レビュー・親の実行判断前には起動しない**。専用 EXE を直接起動する場合も実 credential 呼出しに到達し得るため実施禁止。

## 失敗と未確認

作業中 `rg` 検索の出力が 200 tokens 上限で1回切り詰められた。終了0は完全な表示を意味せず、欠落は未確認。後から keyring の正確な `windows.rs` の冒頭を限定取得し、`username.service` の対象名を確認した。元出力を復元したとは扱わない。同種2回目は認識していない。失敗記録は親へ引き継ぐ。

新規専用領域と本結果のみ変更。親停止時・突然の OS 障害、同名競合の排除、実 OS の動作、既存資格情報と実アプリの登録・更新・削除・復旧、実 WebView は未検証。削除対象なし。結果は UTF-8 で確定前に読戻し後、別担当によるコード・対象版・試験・安全条件のレビューが必要。
