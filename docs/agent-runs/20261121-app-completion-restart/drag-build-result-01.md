# ドラッグ修正版の隔離 release ビルド結果

## 結果

依頼 `drag-build-request-01.md` に従い、専用領域で Tauri release ビルドを実行し、終了コード 0 で EXE が生成された。アプリは起動していない。実 UI でドラッグ不能が解消したことは未検証であり、独立レビューが必要。

- 担当: `/root/drag_build`。基準 HEAD `4961b25bec53a93439acdf5230de77f959908af0`。HEAD は未コミット内容を表さない。
- 実行ファイル: `D:\Projects\GithubIssueWidget\docs\agent-runs\20261121-app-completion-restart\drag-build-sandbox-01\cargo-target\release\github-issue-widget.exe`
- EXE SHA256: `DC0E8489631CB6558014D8B541CC523A8E9E0DA7409E15CA1270B0F38A87AD02`
- 対象設定 `widget/src-tauri/tauri.conf.json` はビルド前後とも SHA256 `0132FCC92345F5D60DE84FD25197FCE853044AD63DAE28757AB4592CC72B991E`。

## 入力と手順

新規 `drag-build-sandbox-01/` を作成し、`widget` の `package.json`、`package-lock.json`、`rust-toolchain.toml`、`tsconfig.json`、`vite.config.ts`、`index.html`、`src`、`node_modules`、および `src-tauri` の `Cargo.toml`、`Cargo.lock`、`build.rs`、`tauri.conf.json`、`src`、`icons`、`capabilities` をコピーした。既存の `dist`、`target` や release 出力はコピー・上書きしない。既存 Cargo registry も専用 `cargo-home/registry` にコピーした。

元とコピーの全入力 18,540 ファイルを個別に SHA256 照合し、一致を確認した。対象一覧は `drag-build-sandbox-01/input-hashes.tsv`、SHA256 `0457909B253A759915AD5629005F921E331A1C0175C60BDAF607B57B5DD12A4F`。一覧は npm 依存とコピーした Cargo registry を含む。入力照合の Python プロセスは終了コード 0。

作業ディレクトリは専用領域の `widget`、コマンドは `npm.cmd run tauri -- build --no-bundle -- --locked --offline`。`CARGO_HOME`、`CARGO_TARGET_DIR`、npm cache を専用領域へ設定し、`CARGO_NET_OFFLINE=true`、`npm_config_offline=true`、`CI=true` を指定した。実行前に Tauri CLI の build 引数を確認。npm、Tauri CLI、Cargo、rustc、リンカー等の子プロセスを Python `subprocess.run` で待機し、監督コマンドも終了 0。外部 GitHub や資格情報の操作、アプリ起動、パッケージ取得は行っていない。

完全ログ `drag-build-sandbox-01/build.log` SHA256 `065D1071BD150547349CBFF4309E38F31CA712B415ADE4E7721557DE5EC94E40` は frontend build と release ビルド結果を記録し、Rust の release 部分は 3分41秒。実プロセス終了コードを `drag-build-sandbox-01/build-exit.txt` に別途保存し値は `0`、SHA256 `13BF7B3039C63BF5A50491FA3CFD8EB4E699D1BA1436315AEF9CBE5711530354`。

## 制約

出力 EXE は専用領域にのみ存在し、既存 release は変更していない。実 Windows の操作、ドラッグ、リンク、サイズ変更、実 GitHub と認証の回帰は未確認。入力のハッシュ検証はビルド開始前のコピー一致を示し、すべてのビルドツールや OS の再現性を保証しない。新規専用領域と本結果のみを作成し、削除は行っていない。作業中に認識した失敗はない。対象 EXE・ログ・入力の対応は別担当の独立レビューが必要。
