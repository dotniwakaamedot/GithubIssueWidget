# 文字選択抑制版の隔離ビルド結果

## 結果

担当 `/root/selection_build`。`selection-build-request-01.md` に基づき、分離した領域で Windows release ビルドを実施し、終了コード 0 で EXE を生成した。実アプリは起動していない。実 UI の選択抑制、ドラッグ、認証や回帰は未検証であり、独立レビューが必要。

- EXE: `D:/Projects/GithubIssueWidget/docs/agent-runs/20261121-app-completion-restart/selection-build-sandbox-01/cargo-target/release/github-issue-widget.exe`
- EXE SHA256: `0D8AB47088B7A19233EF0D7D08CF033B681B20CA0E729FEAF73EAB74DCC98F55`
- 作業場所 `D:/Projects/GithubIssueWidget`、HEAD `4961b25bec53a93439acdf5230de77f959908af0`。HEAD は未コミットの変更を識別しない。
- `widget/src/style.css` は作業前後 SHA256 `FFE871A15BDA8FAD0BB5E28E1717C93D2878ABB1F5885F730EC624DEC983FCFE`、`widget/src/main.tsx` は `B22504C9DE48D275726C912820DA93E02F10C1838CCC68555C156D35030165D7`、`widget/src-tauri/tauri.conf.json` は `0132FCC92345F5D60DE84FD25197FCE853044AD63DAE28757AB4592CC72B991E`。専用コピーも `input-hashes.tsv` によるコピー時の照合で一致した。

## 入力と実行

開始前 D: 空き容量は 85,201,973,248 bytes。新規 `selection-build-sandbox-01/` に `widget` の package.json、package-lock.json、rust-toolchain.toml、tsconfig.json、vite.config.ts、index.html、src、node_modules、src-tauri の Cargo.toml、Cargo.lock、build.rs、tauri.conf.json、src、icons、capabilities をコピーした。Cargo registry は前回隔離領域の `drag-build-sandbox-01/cargo-home/registry` からコピーし、前領域は読み取りのみ。既存 target、dist、release は使用・上書きしない。

`prepare.py` が全入力 18,540 ファイルについて元とコピーの SHA256 を比較し、終了コード 0、全件一致を記録。入力一覧 `input-hashes.tsv` SHA256 `6ADA506F08DC90354B34095B5ABA3F2E3EC47098A99FEF140D55740331A5D122`。`prepare.py` SHA256 `A8E1C54BB0B421E87818218F9D9BE9794986D0B1BC757987EDE0B26340A48C07`。

専用領域内の `widget` で `npm.cmd run tauri -- build --no-bundle -- --locked --offline` を `build.py` から実行。専用 CARGO_HOME、CARGO_TARGET_DIR、npm cache と `CARGO_NET_OFFLINE=true`、`npm_config_offline=true`、`CI=true` を設定。npm、Tauri CLI、Cargo、rustc、リンカー等の子プロセスを `subprocess.run` で待機し、監督セッションと `build-exit.txt` は終了コード 0。外部 GitHub、実資格情報、アプリ起動は扱っていない。依存取得の成功を主張せずオフラインでビルドした。

- `build.py` SHA256 `4B60D7F080CEDA41B9F7EBA066549A8396171CD32AF90DC3B814E5546EA717B7`
- `build.log` SHA256 `12061571A958EF5060854FFF2B738D90118ACD7908B8CDBE1B90C49FA323E09B`。ログは frontend build と release 完了および EXE パスを記録。Rust release は 3分35秒。
- `build-exit.txt` は `0`、SHA256 `5FECEB66FFC86F38D952786C6D696C79C2DBC239DD4E91B46729D73A27FB57E9`。

## 失敗と未確認

プロセス状態確認時の `Get-CimInstance Win32_Process` はアクセス拒否（1件）。元ビルドや入力照合の失敗ではなく、セッションの生存と Python CPU 時間で代替確認し、各監督セッションの終了を待った。プロセス所有範囲や OS 障害全般の終了保証は主張しない。失敗の共有記録への反映は管理担当に依頼する。同種失敗2回は認識していない。

新規専用領域と本結果以外は変更していない。削除対象なし。入力一覧はコピー時点の一致を示すが、その後の全期間の不変性や OS・ツール全体の再現性を保証しない。表示上の問題が解消したか、入力欄やリンクの回帰は新 EXE を実 Windows で操作して確認する必要がある。
