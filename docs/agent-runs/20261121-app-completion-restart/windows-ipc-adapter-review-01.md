# Windows IPC 隔離経路の独立レビュー 01 — 要修正

- 確認担当 `/root/windows_ipc_adapter_review` は作者 `/root/windows_ipc_adapter` と別。対象結果、Rust コピー、`adapter.py`、`test_adapter.py` の期待・実測 SHA256 は依頼値と一致した。対象本番 `widget/src-tauri/src/main.rs` も `AEFF1E65AD93EFA04773CEBBB9736A6BDC5B9938DB9FAF937DD88B3D9CBFE86F` と一致。承認対象版は依頼記載の4実体に限る。
- 判定 **要修正**。以下の安全境界は実 OS 実行を認めるための審査以前に、コピーに対する「実 OS 経路を無条件に禁止」という結果の表現と一致しない。

## blocking

1. **Rust 子テストは親の禁止ゲートを通らず直接実行できる。** `windows-ipc-adapter-01/main.rs` の `REAL_OS_ENABLED=false` と assert は `windows_ipc_guard` 内のみ。一方、別の `#[test] #[ignore] fn windows_ipc_guard_child` はそのゲートを通らず、必要な fixture 環境を与えれば `read_token`、`entry(...).set_password(...)`、mock IPC の `delete_token` に到達する構造である。`test_adapter.py` の静的確認は親の `keyring::Entry::new` より前に assert があることしか確かめず、子の独立起動を検査しない。依頼は実 OS 書込みを明示的に無効にした経路を要求し、結果も「実 OS 経路を無条件に禁止」と記載している。現行の Python 模擬試験は子を実行しないため今回の実 OS 書込みを意味しないが、後続担当がコピーの安全性を過信し直接実行すると安全境界が破れる。**修正案:** 子テストを含む、実資格情報に到達し得る実行入口に共通の fail-closed ゲートを設けるか、当該コピーを実行不能な参照資料として扱い、結果・依頼・試験で親のみの禁止と明記する。前者の場合、親・子それぞれの直接起動で実 credential API に到達しないことを安全な模擬経路で検査し、独立再レビューする。実 OS の解禁は別途所有権・監督の検証が必要。

## 完了条件と確認範囲

1. **識別と journal:** `adapter.py` は prefix と正規 UUID を検査し、`open("x")`、flush、`os.fsync`、JSON 読戻し後に同一プロセスの receipt を発行する。`supervise` は receipt と期待値、journal、模擬 `exclusive` を照合する。ディレクトリエントリの耐久性・ACL・現実の排他を証明しないという結果の制約は妥当。
2. **監督・失敗:** Python 子は合成 calls のみを返し、unknown・mismatch・spawn failure・timeout・不明終了で自動削除を行わない。直接子に対する `communicate`、kill、再待機を確認。親停止・孫・OS 障害時の終了保証ではない。`exclusive` は真偽値の模擬で実所有権の根拠ではない。
3. **10試験・ログ:** `test_adapter.py` の10試験名と `test-03.log` の10件 `OK` を照合。ログ SHA256 `07AB7DEAC9C61F6B53EDA705A9B222BCD9F4C2B7F21129DFDC9E7BA033465CA6` は結果と一致。終了コード0は作者記録で、当担当は元プロセスを監督しておらず、試験再実行もしていない。作成・flush・読戻し失敗は例外と、receipt を持たない別呼出しで gate が閉じることを確認しており、実 OS の journal 障害を確認したものではない。
4. **版・差分・変更:** `git diff --no-index` は本番とコピーの `windows_ipc_guard` 内の識別・禁止ゲート追加を示し、差分ありのため終了1。`git status --short` は結果と専用領域を未追跡として表示し、本番 `main.rs` に変更表示なし。Git 個人 ignore 読取りは権限拒否の警告。作者は削除対象なしと記し、今回の依頼も削除対象を指定しない。workspace 全体や ignored ファイルの網羅確認ではない。

## 未確認と次作業

Python と Rust は未接続、Rust コピーは未コンパイル。真の子孫停止、並行者の排除、ACL、秘密ログ、耐久性、実 Windows Credential Manager・実 WebView・PAT は未検証。上記 blocking の修正後に実行経路の接続・障害試験と別版レビューを行い、その後も実 OS 試験の前提は個別に判断する。実 OS、GitHub、アプリを起動・変更していない。作業中の認識した失敗はなし。
