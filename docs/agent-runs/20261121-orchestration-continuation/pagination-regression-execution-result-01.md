# 大量ページ回帰の隔離実行結果

2026-11-21、担当 `/root/issue_survey`。実行依頼と `pagination-regression-review-01.md`、`pagination-runner-review-02.md`、管理の開始判断に基づく。作業場所 `D:/Projects/GithubIssueWidget/docs/agent-runs/20261121-orchestration-continuation/pagination-test-execution-01/project`。専用領域以外の製品・Git・共有 target・依存を変更せず、合成 loopback 以外の通信、実資格情報・実 GitHub・GUI は試験対象にしていない。実際の依存全体のネットワーク非使用・実資格情報非アクセスを証明するものではない。

## 結果

- `pagination_regression_tests::mixed_pages_preserve_exact_issue_set_and_classification`: 修正実行器の直接終了0、期限超過なし。stdout は `running 1 test`、指定ケース `ok`、`1 passed; 0 failed; 0 ignored; 78 filtered out`。1,103件の固有 Issue を含む合成12ページの集合・分類・PR/重複除外の試験。
- `pagination_regression_tests::later_failures_never_return_the_first_page_as_success`: 修正実行器の直接終了0、期限超過なし。同様に指定1件が `ok`、1 passed、0 failed、78 filtered out。最初の100件の後の HTTP 500、接続切断、不正 JSON に対する合成試験。
- コマンドは実行領域で `powershell.exe -NoProfile -ExecutionPolicy Bypass -File <専用run-test-02.ps1> -Case <上記末尾のケース名>` を各1回。実行器は専用 `run-test-02.cmd` から `cargo.exe test --locked --offline --bin github-issue-widget -- pagination_regression_tests::<ケース名> --exact --test-threads=1` を呼ぶ。修正版の2回はいずれも stdout/stderr の排出タスクが完了して直接プロセス終了を取得。子孫全体の OS レベルの終了は追跡しておらず、完了を断定しない。

## 初回失敗と対処

最初の実行器 `run-test.ps1` では第1ケースの直接 cmd 終了1、期限超過なし、指定ログが存在しなかった。VS 初期化か Cargo か、テストに到達したかは不明。引数にパスの二重引用があり、またリダイレクトが Cargo 部分のみで VS 初期化の出力を捕捉できない点を確認したが、どちらが原因か未確定。同じ実行器を再試行せず、修正版は固定 batch と stdout/stderr の別ファイルへの並行排出を採用、`pagination-runner-review-02.md` が固定版を承認した。修正版の開始後は両ケースが上記の結果となった。初回失敗は共有失敗記録の担当に引き渡す。

## 入力・環境の証拠

- 製品 `widget/src-tauri/src/main.rs` の実行前後 SHA256 `61595B68317E15FDD8F2BED88B920A468D5516740E3D3124710384709C52FC24`。専用 `project/src/main.rs` `AEFF1E65AD93EFA04773CEBBB9736A6BDC5B9938DB9FAF937DD88B3D9CBFE86F` と `pagination_tests.rs` `BF690E818EB182FE16395E0AE3946B043D41DADD2B75235EFEACDC4873F4DE3D` はレビュー版と一致。
- `run-test-02.ps1` `733FF4BC3D12ED823BCA206BEB39E8DD4302DF3C2D8785C0CF4E9BFDCBA21F30`、`run-test-02.cmd` `75329EA73C642862A9AB8D049564659F0B2063D0F4848D2C3598C7108CB16161`。実行前にレビュー対象ハッシュと両ログの不存在を確認。
- 新規の project、Cargo home/registry、target、TEMP、USERPROFILE、APPDATA とログを用意。registry は原本とコピーの各13,546ファイルの SHA 比較で不一致0、リンク検出0。Cargo.toml、Cargo.lock、build.rs、tauri.conf.json、creation.rs、movement.rs の原本とコピーは6組一致。icons・capabilities・dist は独立コピーしたが全ファイルのハッシュ照合は未実施。レジストリ元は実行環境の既存キャッシュであり、コピー後の同時変更を全期間排除する証拠はない。CARGO_HOME・target は専用コピー、直接の子環境は明示 allowlist とし秘密・PAT は渡していない。Windows 資格情報ストアが環境変数だけで隔離されるとは主張しない。
- 空き領域は実行前に約104 GB。Visual Studio 2022 の `VsDevCmd.bat` と Rust 1.90 の toolchain を使用。Windows の実画面や実データには触れていない。

## ログ SHA256

- `logs/mixed_pages_preserve_exact_issue_set_and_classification.02.stdout.log`: `8FBF288C5599C9315C49F3B389F76EE2E736D646AEA6B01724B66DE97515F122`
- 同 `.stderr.log`: `7B4D415B5F44BBB6B0D2EC63CAD5EEA199C560E8DDC8ED540FE382200CCEF8D3`
- `logs/later_failures_never_return_the_first_page_as_success.02.stdout.log`: `A96549050E3FD895F83DBE3D6BC7E63FFA98D86F95D5391C16BC827A58D25A07`
- 同 `.stderr.log`: `BFD8E4E8F7543E8909959E23F211E1209201833035D7E0682A0E7090F1EF73A8`

両 stdout のケース名、件数と終了行を読み戻した。stderr はハッシュと保存を確認したが全文の独立レビューは未実施。直接試験の成功と Issue #4 全体、実環境の成功を区別し、固定版で別担当の結果レビューを受ける。
