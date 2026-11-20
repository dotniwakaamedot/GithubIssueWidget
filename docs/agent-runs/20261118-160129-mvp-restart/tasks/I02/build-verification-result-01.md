# I02 B03 専用コピー検証結果 01

- 担当 `/root/build_verification`。依頼 `tasks/I02/build-verification-request-01.md` SHA256 `2EA23BAEAE1CA017EBDCA8296D3D720FCDCE00E79A20B58C4152915BD5F00711`、追加判断 `questions/I02-build-answer-02.md`・`I02-build-answer-04.md`・`I02-build-answer-06.md` および親のセッション停止指示に従う。
- **判定: 検証未完了。** 専用コピーで依存準備後、release のログにはコンパイル完了と exe 出力があるが、外側コマンドが終了せず実際の npm/Cargo 終了コードを取得できなかった。親の指示で所有セッションを中断したため build 成功とは扱わない。dev は未実施。Issue #2 の他の条件と Windows 11 実機の確認も未実施。
- 対象 worktree `D:\Projects\GithubIssueWidget-issue-2-20261118-160129`、branch `task/issue-2-foundation-20261118-160129`、HEAD `a4ac97286785a2a5ba0d070a909b58003fc49ff1`、README 変更と widget 未追跡16件。製品ファイル17件の元とコピーの SHA256 は、作業前 `evidence-build-01/copy-manifest.json` と停止後 `evidence-build-01/post-stop-hashes.json` に各々一覧化し、全件 `result-02.md` の期待値と一致。HEAD 単独で対象を識別しない。
- 作業中の scratch `D:\Projects\GithubIssueWidget-build-20261118-160129`、共有証拠 `D:\Projects\GithubIssueWidget\docs\agent-runs\20261118-160129-mvp-restart\tasks\I02\evidence-build-01`。作業対象の元製品・元の ignored 生成物、GitHub は変更していない。削除対象なし。専用領域の各依存・target・dist・cache・temp と toolchain は保持中。

## 作業と証拠

1. 共有 run の固定参照8件の実在・SHA 一致、Issue #2 の OPEN・着手ラベル・コメント、open PR 0、対象 branch/HEAD/status を照合。調査時点の条件であり外部環境の他 run の不存在を証明しない。
2. 対象ファイル17件の hash、通常ファイル・ハードリンク1件、主要親ディレクトリの reparse なしを確認。`create-manifest.cjs` により `result-02.md` の17行ブロックを一意抽出、exit 0。`copy-source-02.ps1` が専用 scratch に通常バイトコピーし17件を照合、exit 0。元とコピーは停止後も hash 一致。
3. 専用 RUSTUP_HOME と CARGO_HOME で `rustup toolchain install 1.90.0 --profile minimal --no-self-update`、exit 0。専用 npm cache・新規空 npm 設定・コピー内 node_modules で `npm ci --registry=https://registry.npmjs.org/ --no-audit --no-fund`、exit 0。npm stderr には esbuild@0.28.2 の install script が allowScripts 未承認という警告。実際にスクリプトが実行されたとは判断していない。
4. `run-release.ps1` は cwd `D:\Projects\GithubIssueWidget-build-20261118-160129\source\widget`、Visual Studio 18 `VsDevCmd.bat -arch=x64` を当該プロセスに取り込み、`npm.cmd run tauri -- build --no-bundle -- --locked` を専用 Cargo target、Cargo/Rustup home、npm cache、TEMP/TMP、APPDATA/LOCALAPPDATA、profile で実行。秘密らしい環境変数を子プロセス環境から除外し、既存資格情報はコピーしていない。公開取得、OS cache・システムログなど同一 OS の通常副作用はあり得る。
5. `release.stderr.log` は release profile の終了と `cargo-target/release/github-issue-widget.exe` の生成を示す。exe SHA256 `27D94B7B32EC394F89080C5812D04671B7DA8D420E5B85F1D67615FEAE5D2F4C`。外側 `Start-Process -Wait` が10分以上戻らず `release-result.json` 不在。待機理由・npm/Cargo 実終了コードは未特定。親の明示指示により、コマンド一致を確認した所有 PowerShell PID 25560 のみ 2026-11-18T18:08:04Z に `Stop-Process`、操作 exit 0。元セッション 85095 は `exit_code -1` として終了。これは中断した外側シェルの結果で、ビルドの終了コードではない。停止後 PID 不在と元・コピー17件一致を確認。
6. 日本語を含む旧 `copy-source.ps1` は抽出失敗1回。BOM なし UTF-8 と Windows code page 932 でリテラル読取差が見つかったが、実行時値がなく原因は推測。新しい ASCII と機械可読 manifest の方式へ切替えた。lockfile 読取2回失敗は別担当の客観調査後、新スクリプトで成功。追加のプロセス調査で PowerShell 構文エラー1回と出力文字化け1回。詳細は `questions/I02-build-question-01.md`〜`12.md` を参照。

## 主要ファイル SHA256

証拠は `tasks/I02/evidence-build-01/` 相対:

```text
230F07C865D4A0F37C0D2ED7408F3D5A0086832E16BC0A76EAA444B14165D9A4  copy-manifest.json
28C48016BAB93E25A3D166311A8487641C3CC07D97D25296ED3BB5A63EEF21B6  lock-summary.json
70C7A4B32AE96F5004F55C4B8F1855E06839812559FE5F2D3064663B5F60DEDA  npm-result.json
BD592DEF5321DED86B06E1996BA45D8D9DD1929FE9A1931B8D87D467594B9076  npm.stderr.log
A31F2B5ED80674EC49BD3216F5956352826052B64D9B1B460FC76D7B2E15154E  npm.stdout.log
6D587133C9E9C6A3BD16F76910C64669E36EB07D77041A4159C3F9381E54953E  post-stop-hashes.json
D747AA1C2D1BF9A2ACAC9FD0CE55712062B58B714EEFCE4B6F3E796D56CBEDF8  release-hashes-before.json
42F8BAFC1634223F690162B9CB904750F8505AD11CD19C7BD796B24D8C902119  release.stderr.log
AC6DCB8667F41B0AD387F6FD3A6C7EA66FB295E1F7C0E9D030DF90F5DF1BB29F  release.stdout.log
01F040D8306256A0969B31DC9B30662340D72A1BEC4813808338DAAA67E4FC07  rustup-result.json
B221EB4122453884168CD67BEB24241647E8B8653986BA3162D3163A1E53256C  rustup.stderr.log
BBBB3EB31124866016C34BAFEF844DE0A7F20F5B1545B2B29102F90F5C565E08  rustup.stdout.log
BB5253F1D02C83E183C86A6981CA3768A01892E3D1B40EF3AC4AA6A014DE0033  source-manifest.json
```

scratch 直下の担当作成スクリプト・設定:

```text
BDC411FE994FD775F5B2674324F6BE191A1BF8835BCEFE499DEE62C28F724F3F  copy-source-02.ps1
F6D43B587B03E20C3C478E62D250D5A3099FD71C33066831FE78F61F81691862  copy-source.ps1
A81A803525E895E8743058F6A2F8754F1D5DC96FF6682B95C3D33970DE129544  create-manifest.cjs
322909D7D925762E2B23629B1F665C740A8FA4BA857BFFC66EE6C7E8EDDCE494  inspect-lock.cjs
E3B0C44298FC1C149AFBF4C8996FB92427AE41E4649B934CA495991B7852B855  npm-global.npmrc
E3B0C44298FC1C149AFBF4C8996FB92427AE41E4649B934CA495991B7852B855  npm-user.npmrc
8A117C1BF1B5C167D9F0612742D54DA06C7C6DE873F91374BD50D06E24BD39F5  prepare-npm.ps1
A2398C0A14EFD687D5F65E4A1520295C970426C3F875FA4CFD6E57EC51ABA3E4  prepare-rustup.ps1
A9B243DF8FAB493ACCB85674E0D6F45B94945AB603F6747F0DE984EAC6E6FA08  run-release.ps1
```

依存・toolchain・ビルド生成物の全個別ファイル hash は未列挙。生成物全体の完全性は未確認であり、個別 hash を必要とする整理・レビューは追加作業が必要。既存成果・未保存物として保護する。

## 次の作業・教訓

- 終了コード不明と待機原因を独立に確認し、元セッションを再実行せず、必要なら別の保存先・新規方式で対象版と終了コードを結び付ける計画を管理・レビューする。dev 起動と正常終了の検証は別途必要。
- 機械可読 manifest は基点・パス・SHA を分け、存在確認を先行させる。PowerShell 5.1 と UTF-8 スクリプトの互換性、シェル引用符、空の検索結果は実行前に確認し、失敗後は同種数を管理する。
- `Start-Process -Wait` が戻らない場合、出力に完了文があっても exit code の証拠とはしない。既存の稼働ハンドルを確認し、再起動や停止前に所有・副作用・証拠を判断する。
