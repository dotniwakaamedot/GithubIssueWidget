# 合成資格情報の監督・復旧経路候補 02

担当 `/root/credential_live_execution_prep`。`credential-live-ready-review-01.md` の blocking ２件（子登録前の credential 操作・対象限定の診断復旧不足）を受け、別の新規領域 `credential-live-ready-02/` に候補を作成した。**実行可能版は未承認であり、実 Credential Manager は実行していない。** 実 PAT・既存資格情報・製品・GitHub は操作していない。作業場所 `D:/Projects/GithubIssueWidget`。

## 実装と制限

- 監督者は実行ファイルの固定 SHA を照合し start 記録を新規作成・flush・sync・読戻し、stdin pipe 付き直接子を起動する。子は専用領域の UUID service/account・PID・実行ファイルを journal に同期・読戻しし `awaiting_grant` で待つ。監督者は子 PID・バイナリ・journal を新規作成・同期・読戻し後にのみ `GO:<run_id>` と EOF を送る。子は grant を検証し `grant_received` を記録後に初めて `keyring::Entry::new` と read/write に進む。EOF、部分・不正 grant、15秒 timeout は Credential API 前で停止する。
- 監督者が子登録に失敗した場合は grant しない。grant 送信中の例外は完全なメッセージが届いた可能性を否定できないため `grant_delivery_unknown_*` とし、kill/wait を試み、終了不明なら復旧時の自動操作を禁止する。親が grant 後に死亡すれば子が続行し得る。OS 障害時の無条件停止や原子的な credential 排他は保証しない。
- `recovery.py` は明示的 `--inspect <run_id>` または `--cleanup <run_id>` でのみ実行可能。start/child/outcome と journal の run ID・pid・EXE SHA・正規パス・service UUID4・account と子の wait 済み状態を照合し、`synthetic.<service>` の単一対象を検査する。実値を出力・保存せず NoEntry／固定合成 A・B／他値／エラーに分類。診断と復旧の start/outcome は create-once で記録し、未知の結果を自動再送しない。実 Windows API の利用は今回未実施。
- cleanup は直前の診断と同一 journal SHA、`before_read_a` または `before_read_b`（該当 write が正常復帰して次の intent が同期された状態）と合成値の一致、終了確認を要求し、直前の再分類が一致するときだけ delete、その後 NoEntry を確認する。grant delivery 不明、未知ステージ・異なる値・不在・終了不明・記録不一致は削除しない。NoEntry→write と read→delete は原子的でなく、UUID は衝突確率を低下するが同一 target の第三者競合を排除しない。`RECOVERY.md` に停止・診断・復旧の操作と限界を記した。ローカル記録は workspace を改竄できる者からの保護境界ではない。

## テストと対象版

専用 `mock-target` は Rust `mock` feature のみを使用し keyring backend を持たない。`cargo +1.90.0 test --locked --offline --features mock` は 9 件成功・終了0、同 feature の build は終了0。`live` feature の test は fake Store/Journal の9件成功・終了0、live build は終了0。各 Cargo 実行は専用 target と先行 `credential-supervisor-01/cargo-home` を使用。PowerShell の stderr 出力に `NativeCommandError` 表示が混ざるが Cargo の保存終了値は0。live EXE は起動していない。

`python -m unittest -v test_ready` の `ready-tests-final.log` は17件成功・終了0。その後、grant delivery 不明で停止したケースの診断許容・削除禁止を追加し、`python -m unittest -v test_ready.RecoveryTests` を再実行、11件成功・終了0。前者は実 mock 子を使用した EOF、15秒 timeout、子登録失敗、親プロセス終了、正常 grant と journal stage、加えて fake backend の復旧ケースを含む。後者は最終版の recovery の fake backend ケース。`wait-tests-01.log` は timeout kill/wait の mock 2件成功・終了0。実 Credential API、Windows API の実動作、実アプリ・PAT・WebView はいずれも証明しない。各ログと `.exit` は専用領域内に保持。

| 対象 | SHA256 |
| --- | --- |
| `Cargo.toml` | `8DBF35BE0D2AB67489DAD4AB2168E69856D67253B260D0207D775F4D43747513` |
| `Cargo.lock` | `79B1644981C6B122726DDD3AC55E54616C815F9E56AF52ADE760C3B402D8EAA5` |
| `src/main.rs` | `214DEF1642AAA6BE30F4E5A5C95CEF3E62083084EAC617527E9A10DC6C509F1B` |
| `supervisor.py` | `BBBE8FBCCFE7C2185DC33C6798F6B49887C45DDD061E52426E96B01CF17B44BE` |
| `recovery.py` | `373DF854F494FB423BDFC5CADD1E8F950CB373792C3660A200757B044E3ECFD5` |
| `test_ready.py` | `B228B3BC42ADE2AD30642477F23A094531FD9A5B62D122FDFDC72A9D68C6E526` |
| `mock_parent_exit.py` | `7B3CB967146F9EC6CD44C48B03904B43FCB40F7473847BE7B0A5BF3177FA1B65` |
| `RECOVERY.md` | `4B15673676AFBE1E0E9211D086CDDCBAB533A5B09C814C9BC6CB691A8C50F066` |
| live `target/debug/giw-credential-synthetic-check.exe` | `211CAD118123794B3C15C27E14F88BEC60B4AD0906ED9145AEF71939A2706910` |
| mock `mock-target/debug/giw-credential-synthetic-check.exe` | `D442465C1252D8E5E57E16F188CB1070FB724E9D928C471360BA364F29D8A97E` |
| `rust-live-test.log` / `rust-live-build.log` | `039A755FB6BCA1B108B861EB9EA88F734EDE01EAADB43B95F59F00FF52F86D26` / `D19AE671F6DFB89E64FC6AFCD77A89C2CA0140CC4E2AA5AEE4D287B1A6A205EE` |
| `ready-tests-final.log` / `recovery-tests-final.log` | `8F9F63D9130C09574553FBA13357D2C036B3C4B1AB2A7A5568D4C84C79C63F06` / `645431E5AF0BFE26179DAC10D2EC58B3D9191F8E1E74960948DA61DEA593EA1A` |

保存した各 `.exit` の ASCII `0` は SHA256 `5FECEB66FFC86F38D952786C6D696C79C2DBC239DD4E91B46729D73A27FB57E9`。mock 子の test journal と記録は専用領域に残し、削除しない。作業中に新たに認識した失敗なし。未承認の live EXE は直接起動しない。実行判断には新しい対象版のコード・プロセス監督・停止・診断復旧の独立レビューが必須。
