# C: テスト実行方式の読取り監査

担当 `/root/process_safety`。2026-11-21。対象の読取りのみで、テスト・ビルド・アプリは実行していない。以下は実行の承認ではなく、固定方式と独立レビューを要する提案である。

## 結論

Rust の個々のテスト本体に子生成がなくても、`cargo test` はコンパイラ・ビルドスクリプト・テスト実行体などを子として起動し得る。フロントエンドも Vitest の既定 pool は forks であり、インストール済み esbuild はサービス子プロセスを起動する実装を持つ。既存の旧合成子の終了は不明であり、いずれも単にテスト本体を絞るだけでプロセス隔離・終了証明が成立したとは言えない。具体的な監督・終了・書込み隔離の方式を独立レビューするまで実行しない。

## Rust

- `tests::next_relation_is_case_insensitive` は `main.rs` の Link ヘッダー解析 `next_page` を合成文字列だけで検証し、当該本体と解析関数にはファイル、通信、認証、明示的な子生成は見当たらない。同様に `tests::quoted_parameters_do_not_forge_a_next_relation`、`tests::next_page_cannot_change_host_or_repository` も候補。いずれも今回の未コミット `main.rs` 対象版に結びつけて検証すべきで、GUI や実 API の証明ではない。
- 候補の厳密なフィルタ例（`widget/src-tauri` 作業ディレクトリ）: `cargo test --locked --offline --bin github-issue-widget -- tests::next_relation_is_case_insensitive --exact --test-threads=1`。フィルタはテスト本体の範囲でありビルド対象や build.rs の実行を防がない。0 件成功を防ぐため、対象 1 件の表示、終了コード、テスト実行体の終端を必ず確認する。他 2 件も名前を置き換え別々に実行する案。
- `lock_excludes_an_independent_process` と `rate_limit_deadline_is_shared_across_processes` は `current_exe` の子をそれぞれ複数回生成し、後者は一時ディレクトリを APPDATA に設定する。両者は明示的に除外し、全テストは保留。`#[ignore]` の Windows Credential Manager・実 GitHub・IPC 補助試験も対象外。
- `build.rs` は `tauri_build::build()` を呼び、Cargo は `target`、ビルドスクリプト出力・一時領域へ書込み得る。既存 `target` は他作業と共有されるため無断で更新しない。専用 `CARGO_TARGET_DIR` と一時領域、依存キャッシュのアクセス先・書込みの有無、ディスク容量、プロセス木と終了証拠を実行前に確認する。`--offline` は取得通信抑止でありファイルの読み取り専用化や子停止保証ではない。既存 test EXE は複数あるが現在の `main.rs` 対象版との一致は証明できず、直接実行の根拠にならない。

## フロントエンド

- `widget/src/main.test.tsx` は jsdom と Tauri `invoke`、ウィンドウ操作、opener をモックする。`widget/vite.config.ts` は通常 Vite 設定でテスト固有 pool を指定しない。インストール済み Vitest の CLI 定義は pool の既定値を `forks` とする。`widget/node_modules/esbuild/lib/main.js` は `child_process.spawn` によるサービス起動を含み、スレッド pool 指定だけで全子生成が消えるとは証明できない。
- Vitest は結果・変換キャッシュを Vite cacheDir や `node_modules/.vitest-cache` に書込み得る。提案は `node` から固定されたローカル Vitest エントリを直接起動し、専用 cache と一時ディレクトリ、明示的 pool・単一ワーカー、固定テストファイルと標準出力／標準エラー分離を指定する方式を別途設計すること。CLI の指定可能性、実際の子孫構成、すべての書込み先、親子孫の期限付き終了は本監査では未検証。`npm ci`・`npm run build`・watch モード・既存 cache 消去は行わない。

## 共通の実行前条件

基準ファイル・依存関係と実行ファイルの SHA256、対象の所有権と並行変更なしを確認する。専用の空の出力・一時領域を予約し既存のものを上書きしない。保存ログは合成試験に限定し、環境変数や秘密を出力しない。監督者・ツールの実行経路と子孫、起動失敗・期限超過・監督者停止時の扱いを固定し、終了結果を確認できない場合は不明として後続を止める。実行者と別担当が方式と対象版を確認後に限り実行する。実環境・WebView2・認証・通信障害の代用としない。

## 監査対象の SHA256

- `widget/src-tauri/src/main.rs`: `61595B68317E15FDD8F2BED88B920A468D5516740E3D3124710384709C52FC24`
- `widget/src/main.tsx`: `01A7E8A6961BE2DC4F00D96F9369098321EF5137F6959650931E36ABFB357658`
- `widget/src/main.test.tsx`: `6D1F45AFDBFC3BD66071C81FD7BA0025A186BD6A1CC0CE89022D06FB19B7D1E1`
- `widget/vite.config.ts`: `1DAFF9E73019BD4E74489811D13553B3D227DB8457910D7F295E60AD0EB4CC7A`
- `widget/package.json`: `102235B5686088C87B3221DEC7AE3CBEE20F0C9B1EAE070FD730C8B631DFCEDF`
- `widget/package-lock.json`: `A5D65370135F83B08F02D5F1EE329706F08551743444264BCB10DFC7600B0C73`
- `widget/src-tauri/Cargo.toml`: `F7981EB1A590A1B6BB19A18C7166AE3CFF3871EF583A24C7B35C1A3390520F7A`
- `widget/src-tauri/Cargo.lock`: `2486C9E87F8511A2CBB3C09177E278139409C68A308FD48642327EDDF1D88E7F`
- `widget/src-tauri/build.rs`: `CE55D94EF836CC138523951D99018387FD9EBA7A8AE5875A622AADA3F5B06CE1`
- `widget/src-tauri/tauri.conf.json`: `5C1F3A0E2E487DD4345A2DD335FE4AACA483FEEEA26C11AAB99B374278EC1364`

## 調査中の失敗と制約

広い `rg` を複数結果と併せて表示した出力が外側で切り詰められた。欠落を不存在とみなさず、今回の結論に必要な部分を対象を絞って読み直した。別に Windows の `rg` に未展開の `widget/vite.config.*` を渡して OS error 123（終了 1）となり、その後実在の `widget/vite.config.ts` を読んだ。各 1 回として管理者に失敗記録を依頼済み。node_modules と Rust 依存コードの全副作用は監査しておらず、終了方式も未実証。
