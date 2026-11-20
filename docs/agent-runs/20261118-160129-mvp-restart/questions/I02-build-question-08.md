# I02 B03 依存準備とビルド進捗 08

- 担当 `/root/build_verification`、宛先 `/root`。専用 scratch に Rust 1.90.0 toolchain を別の `RUSTUP_HOME`・`CARGO_HOME` に取得し exit 0。`npm ci --registry=https://registry.npmjs.org/ --no-audit --no-fund` は専用 cache と空の npm config・コピー側の node_modules に準備し exit 0。各ログと終了コードは `tasks/I02/evidence-build-01/` に保存。
- `run-release.ps1` で元・コピー17件の事前 hash を照合し、Visual Studio 18 の VsDevCmd 環境をプロセス内に読み込み、専用 Cargo target/cache・TEMP・APPDATA・profile で `npm run tauri -- build --no-bundle -- --locked` を実行中。操作対象はコピー側、出力は専用 evidence。セッション 85095 はこの担当のツール内で稼働中。終了コードは未確定、成功とは扱わない。
- B01 のウィンドウ移動は行っていない。既存 worktree の製品・生成物や GitHub は変更していない。通常のネットワーク取得と Windows システムログ等の副作用はあり得る。環境内に残ったデータは保護し、削除しない。
