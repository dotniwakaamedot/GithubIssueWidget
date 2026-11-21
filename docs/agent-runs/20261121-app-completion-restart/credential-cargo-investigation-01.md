# Cargo オフライン依存解決失敗の独立調査

担当 `/root/cargo_dependency_investigation`。対象 `credential-live-test-01/mock-test.log` と `mock-test-02.log` はどちらも `no matching package named keyring found`、対応する exit ファイルは双方 101。元のテストを再実行せず、ネットワーク取得・実 OS 資格情報への操作を行っていない。

## 確認した事実

- 依頼元 `credential-live-test-01/Cargo.toml` は `rust-version = "1.90"`、`keyring = { version = "3", features = ["windows-native"], optional = true }`、`uuid` を指定。Cargo.lock と同ディレクトリ／専用 CARGO_HOME の config は存在しない。
- 対象ディレクトリからの `cargo --version` と `rustc --version` は各 1.84.1。成功済み `selection-build-sandbox-01/widget` の同コマンドは各 1.90.0 で、同ディレクトリの `rust-toolchain.toml` は 1.90.0 を指定する。対象の 1.84.1 は manifest の必要 Rust 1.90 を満たさず、仮に依存解決できてもそのままでは使用できない。
- `verify.py` は `CARGO_HOME` を設定せず、`verify-02.py` は新しい `credential-live-test-01/cargo-home` を設定する。両方 `CARGO_NET_OFFLINE=true` と `cargo test --offline` を使用し、それぞれ最初の test で終了。後続の live compile は行われていない。
- ２回目の専用 registry は `index.crates.io-1949cf8c6b5b557f` に keyring index、`keyring-3.6.3.crate` と展開ソースを含む。index の `keyring` キャッシュは 197,798 bytes、SHA256 `3258E26760ADB71E6972234D8ED8B79F02D20D2434ABCC4A8CE2F4362AEB6BF8` で、成功済み selection-build のものと一致。エントリ `3.6.3` は yanked=false、rust_version=1.75、schema=2。元とコピーの index が存在することは Cargo 1.84 がそれを実際に選択・読取した証明ではない。
- workspace から対象ディレクトリまでの確認した `.cargo/config.toml` と、既定 CARGO_HOME の config は存在せず、確認した CARGO/RUST 環境変数名はなかった。調査時点の環境であり、元実行プロセスの全環境を証明しない。

## 解決候補と未確定

別担当はコピー済み cache と新規 target を保護し、対象実体・ログ・終了状態を確認してから、成功例と同じ `cargo +1.90.0` および Rust 1.90.0 を明示し、専用 `CARGO_HOME` を完全絶対パスで指定して offline のみで試験する。プロジェクトの `rust-toolchain.toml` を隔離コピーへ配置する方法もあるが、どちらもインストール済み toolchain の存在を先に確認する。新規 runner／ログを使い、既存の2件を上書きしない。`--locked` は対象 lockfile がないままでは適用できず、生成時と固定後の操作を区別する。テスト対象の実 credential 書込み禁止を先に確認し、無関係な本番ファイル・実 credential・ネットワークには触れない。

Cargo 1.84 とキャッシュの組合せが `keyring` 不在を生んだ直接原因か、index の識別・形式・設定・その他の要因かは、元実行の Cargo trace や完全な環境がなく未確定。1.90 利用は確認すべき修正候補であり、この調査だけでは依存解決の成功も当該失敗の解消も主張しない。コピーされた他の依存全件、異常時プロセス終了、live 機能、実 Credential Manager は未検証。

調査操作は限定したファイル読取り、`Get-ChildItem`、`Test-Path`、`Get-FileHash`、Cargo/rustc のバージョン表示で、認識した新たな失敗なし。対象 run の既存ファイル・資格情報・GitHub を変更していない。削除対象なし。
