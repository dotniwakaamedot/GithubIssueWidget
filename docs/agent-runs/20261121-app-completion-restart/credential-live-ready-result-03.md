# 合成資格情報の監督・復旧経路候補 03

担当 `/root/credential_live_execution_prep`。本結果は `credential-live-ready-result-02.md` を対象版について置き換える。**実行可能版は未承認・未起動であり、実 Credential Manager、実 PAT、既存資格情報、製品、GitHub を操作していない。** 対象は新規 `credential-live-ready-02/`、作業場所 `D:/Projects/GithubIssueWidget`。

## 実装と安全境界

- 実行前に supervisor は専用ディレクトリ内の固定 SHA の EXE を照合し、`live-once.json` を create-new・flush・fsync・読戻しする。マーカーは削除されず、成功・失敗・不明のいずれの後も同じディレクトリの再起動を防ぐ。その後に start 記録を同様に確定し直接子を起動する。起動失敗でもマーカーを解除しない。
- 子は一意な `github-task-widget-synthetic-check-<UUID4>` と account `synthetic`、PID・EXE を新規 journal に記録・sync・読戻しし `awaiting_grant` を記録して待機。親が child PID・EXE・SHA・journal を create-new・sync・読戻し後、stdin に `GO:<run_id>` と EOF を送信。子は全体一致を検証し `grant_received` 記録後に keyring を呼ぶ。grant 不足・EOF・15秒 timeout は Credential API より前に停止する。
- grant の完全な送信後に親が停止した場合、子が進行する可能性がある。送信中例外は `grant_delivery_unknown_*` と記録し kill/wait を試みるが、終了不明なら再送・削除しない。実 Credential API の NoEntry→write と read→delete は原子的ではなく、UUID は事故的衝突の可能性を下げても同名の第三者との競合を排除しない。OS 障害時の無条件停止を保証しない。
- `recovery.py` は明示的 inspect/cleanup のみ。one-shot marker・start・child・outcome・journal の run ID、PID、EXE SHA、journal 正規パス、service UUID4・account を照合し、直接子の wait 済み結果があるときだけ対象 `synthetic.<service>` の NoEntry／固定合成 A・B／他値／エラーを、値を出力せずに分類する。grant delivery 不明で停止済みのケースは診断のみ許し削除を禁止。cleanup は診断結果・journal SHA・成功した write の次 stage・合成値一致を要求し、削除直前に再分類、削除後 NoEntry を確認する。不在・他値・変更・曖昧 stage・終了不明は削除しない。各診断・復旧は create-new 記録で自動再試行しない。read/delete 間には残余競合がある。workspace 記録は同じ場所を書き換えられる者に対するセキュリティ境界ではない。
- 操作・異常時の停止と復旧条件を `RECOVERY.md` に記録。実 OS・WebView・PAT の要件を合成結果で代用しない。

## 対象版と検証

`cargo +1.90.0 test --locked --offline --features mock` と `--features live` はそれぞれ fake Store/Journal の9件成功・終了0。各 feature の build は終了0。live EXE は起動していない。`python -m unittest -v test_ready` は最終版で **20件成功・終了0**。実 mock 子による EOF・15秒 timeout・子登録失敗・親の終了・正常 grant と、fake backend の NoEntry/他値/不明 stage/終了不明/値変更/復旧成功/削除不明・再送禁止、one-shot marker の試験を含む。mock 子統合では one-shot をテスト用に patch し、marker の排他は別の単体試験で確認した。実 Windows Credential API・実環境の全障害は未検証。

| 対象 | SHA256 |
| --- | --- |
| `src/main.rs` | `214DEF1642AAA6BE30F4E5A5C95CEF3E62083084EAC617527E9A10DC6C509F1B` |
| `supervisor.py` | `D0D9EFDDC4812B9D9F22C2A3E714F8C491825F3DCC386F4DADA385C2D047421D` |
| `recovery.py` | `04A3911C622E09B1416962C388972443041916D6E06A14C0FB7F01646B86A302` |
| `test_ready.py` | `B84E406BD94F3F9180946AFF07F99830ED53B086B353D0A9E8CDF11E5872462F` |
| `mock_parent_exit.py` | `7B3CB967146F9EC6CD44C48B03904B43FCB40F7473847BE7B0A5BF3177FA1B65` |
| `RECOVERY.md` | `F2F2D33AA2B3BC4CDC057384E38F2FBCF1BDAE10199D70946A5868C9A5B47E45` |
| live `target/debug/giw-credential-synthetic-check.exe` | `211CAD118123794B3C15C27E14F88BEC60B4AD0906ED9145AEF71939A2706910` |
| mock `mock-target/debug/giw-credential-synthetic-check.exe` | `D442465C1252D8E5E57E16F188CB1070FB724E9D928C471360BA364F29D8A97E` |
| `ready-tests-oneshot-02.log` | `D6D3CEFF93C83D1C289D29621B1AF90FC6158DD749B0E9DD69987DEAE7A581E5` |
| `ready-tests-oneshot-02.exit` | `5FECEB66FFC86F38D952786C6D696C79C2DBC239DD4E91B46729D73A27FB57E9` |

`Cargo.toml`・`Cargo.lock`・Rust build/log の SHA は結果02のとおりで、その後変更していない。PowerShell の Cargo stderr は `NativeCommandError` と表示されたが元 Cargo の保存終了値は0。模擬実行で生成された journal・記録は専用領域に保護して残す。

作業中、one-shot 追加の `apply_patch` が文脈不一致で1回拒否され、現行ファイルを読み直して適用した。同種2回目は認識していない。新規 live 候補と OS 実行の判断は独立レビューが必要であり、親の判断前に live EXE・`supervisor.py --execute`・`recovery.py` の実 OS 経路を起動しない。削除対象なし。
