# 移動競合テストの実行準備

担当 `/root/existing_work_review`。`movement-rust-execution-request-01.md` と固定版の `movement-rust-review-02.md` を読取り。テスト・ビルド・コピー・loopback・GUI・GitHub・credential は未実行。別担当が使用中の `pagination-test-execution-01/` を読取り・変更していない。これは準備結果であり試験成功ではない。

## 対象版と条件

- レビュー対象隔離 `movement.rs` SHA256 `88BF21BB45C077DFDA592D906C753D168C5641E35EB12324D4ACF1652374695B`、製品 `movement.rs` `ED49F4BA52C6887BCE427E4A6664D335092A1C3311E1B1ED61B4CE322791ABA5`、製品 `main.rs` `61595B68317E15FDD8F2BED88B920A468D5516740E3D3124710384709C52FC24` は依頼値に一致。
- 固定候補の `Cargo.toml` `F7981EB1A590A1B6BB19A18C7166AE3CFF3871EF583A24C7B35C1A3390520F7A`、`Cargo.lock` `2486C9E87F8511A2CBB3C09177E278139409C68A308FD48642327EDDF1D88E7F`、`build.rs` `CE55D94EF836CC138523951D99018387FD9EBA7A8AE5875A622AADA3F5B06CE1`、`tauri.conf.json` `5C1F3A0E2E487DD4345A2DD335FE4AACA483FEEEA26C11AAB99B374278EC1364`、`creation.rs` `61FB68E208B231664DB236E50A37BBB4870763AD61C0071383BACA5730982A87`。実際の Cargo ビルドには capability・アイコン・frontend 等の追加入力が必要になり得るため、全入力と依存は未確定。
- `cargo.exe` と `rustc.exe` は `C:\Users\semis\.cargo\bin` に見つかったが、これらは実体や toolchain・SDK・利用可能性の証明ではない。実行前に実体・バージョン・固定版・オフライン依存・権限を照合し、秘密を環境に継承しない。
- 新規 `movement-rust-test-execution-01/` とこの結果の確定前は不存在を確認。他担当の共有作業の終了・書込み停止を親から確認するまでは、その場所を利用しない。所有する新規領域も現時点では作成していない。

## 提案する限定実行

終了通知後、明示された所有と対象版・依存に関する根拠が得られるなら、専用領域へ Cargo プロジェクト・必要な build 入力・依存を独立複写し、`src/movement.rs` のみ承認された隔離版を配置する。専用 target、TEMP、APPDATA/USERPROFILE、ログを使い、既存 target・元のプロジェクトに書込まない。コピー元・先のパス、種類、SHA、ハードリンク・ジャンクション、容量と共有キャッシュへの書込みを事前検査する。利用可能な固定依存がなければ実行を保留する。

候補コマンドは専用 Cargo プロジェクトで `cargo test --locked --offline --bin github-issue-widget movement::tests::post_write_competing_ -- --test-threads=1`。フィルターは3ケースの共通接頭辞であり `--exact` を付けない。出力で3件の完全名と各 `ok`、終了コード0、ログ完全性を独立に確認し、3件以外・0件・欠落・終了不明を成功としない。コンパイル時の build script と通常の子プロセスも対象に含める。限定フィルターだけで Cargo が他の処理や子生成をしないとは扱わない。

各模擬サーバーは loopback 固定、受付5秒・読書き3秒・HTTP要求3秒。失敗時も直接実行体と把握できる子孫の状態・ログを区別し、全体の観測期限と期限超過後に同じハンドルを待つ方法を固定する。未知のプロセスを推測して終了せず、残存・終了不明は専用領域を保持し後続起動を止める。秘密を引き継がず、合成入力だけをログに保存する。

実際の複写内容・launcher・環境変数・実行ファイル・対象の再ハッシュ・終了方法とログ保護を確定ファイルに記録し、別担当のレビューと管理の通知後だけ実行する。前回の pagination 実行器レビューは参考だが、この新たな対象・方式を承認したものではない。

`Get-FileHash`、`Get-Command`、Cargo manifest とソース一覧の限定読取りは終了0。対象SHAや依存の開始直前の不変性、Rust コンパイル、3件の実行、直接終了、未知の子孫は未確認。新しい操作失敗は認識していない。
