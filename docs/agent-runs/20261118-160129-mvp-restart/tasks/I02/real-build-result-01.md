# I02 専用コピーによる release build 結果 01

- 担当 `/root/build_verification`。依頼 `tasks/I02/real-build-execution-request-01.md` SHA256 `B3C0C3A65BE8A3980F79A10D8676C1971B24F26869612EE9FA7E92580249B898`、対象 `real-build-driver-design-02.md` `0D6F42F22537F7DE5C943BC0E53AD98795227EE5719A20FDD7476EB505B17E21`、独立承認 `real-build-driver-review-02.md` `708128F1D84F95059A505BEAB38C8E0A70E575AD845F3D6A993C6E4242B7EDAD`。開始直前と各 stage 前に固定参照9件、driver・ツールの実体 hash を照合した。
- **対象17ファイルの専用コピーで、release ビルドの直接起動プロセス exit 0 と両 stream の正常 EOF、exe 生成を確認した。** release stderr の承認後の限定読取でも release profile の終了と当該 exe パスを確認。npm→shell→Tauri→Cargo の個々の exit code は直接取得しておらず、伝播を別途証明したものではない。ログ内容は本結果に転記しない。
- 対象元 worktree `D:\Projects\GithubIssueWidget-issue-2-20261118-160129`、branch `task/issue-2-foundation-20261118-160129`、HEAD `a4ac97286785a2a5ba0d070a909b58003fc49ff1`。README 変更と widget 未追跡16件を含む17ファイルを識別し、HEAD のみには依存しない。専用 scratch `D:\Projects\GithubIssueWidget-build-20261119-02`、driver `build-driver-03.ps1` SHA256 `BA0269227A5239F5E7B9CA0EF2802D3A5D5C7678821E67C141CC4A1958927FA9`。
- 元・コピー17パスの各 SHA とサイズは scratch の `source-manifest.json` `BB5253F1D02C83E183C86A6981CA3768A01892E3D1B40EF3AC4AA6A014DE0033` と `copy-manifest.json` `BCE67077C4CB323586F52E527318B6584B3D48BBC956E879B75774ED6F5482E0` を参照。各 stage の前後 JSON は全17件あり、全6ファイルの SHA256 は `D747AA1C2D1BF9A2ACAC9FD0CE55712062B58B714EEFCE4B6F3E796D56CBEDF8`。実行後の元・コピー実体再照合も17件一致。前後一致だけでは途中一時変更や他環境の完全な排他を証明しない。

## stage の結果

PowerShell 5.1 / .NET Framework 上で `powershell.exe -NoProfile -File <scratch>\build-driver-03.ps1 <stage>` を別 session で実行。driver は専用 RUSTUP_HOME/CARGO_HOME、CARGO_TARGET_DIR、npm cache、空の npm 設定、TEMP/TMP、HOME/USERPROFILE、APPDATA/LOCALAPPDATA を子へ設定し、環境全体や資格情報を記録しない。固定ソース17件、ツール7件、専用 rustc/cargo、出力の不存在、各 stage の独立 receipt と前 stage 成功を確認した。開始直前の D: 空き約125.5 GB、空きメモリ約39.2 GB、CPU 観測7%。OS cache/ログ、ネットワーク・依存取得など同一 OS の副作用はあり得る。

| stage | 直接子と主な引数 | PID | 開始 → 終了（2026-11-18 UTC） | exit / stdout EOF / stderr EOF |
| --- | --- | ---: | --- | --- |
| rustup | `rustup.exe toolchain install 1.90.0 --profile minimal --no-self-update` | 30792 | 20:47:10.0706867 → 20:47:19.6320290 | 0 / RanToCompletion / RanToCompletion |
| npm | Node + `npm-cli.js ci --registry=https://registry.npmjs.org/ --no-audit --no-fund` | 5164 | 20:52:56.5122663 → 20:52:58.5912724 | 0 / RanToCompletion / RanToCompletion |
| release | Node + `npm-cli.js run tauri -- build --no-bundle -- --locked` | 12016 | 20:57:27.7251057 → 20:58:49.8939530 | 0 / RanToCompletion / RanToCompletion |

release 前の `VsDevCmd.bat -arch=x64` bootstrap は PID 31708、20:57:26.9579847 → 20:57:27.6567118 UTC、exit 0、stdout/stderr EOF とも RanToCompletion。設定値・ログ内容は共有していない。npm の承認済み stderr を限定確認し、install scripts に関する警告を検出。該当スクリプトの実行・未実行はこの警告だけでは断定しない。

各 stage は同一 session の Process handle を保持したまま保護領域の stdout/stderr tmp と機械走査結果の hash を別担当へ渡し、hash と担当・stage を結んだ receipt および親の判断後に `publish` を各1回送信。stage ごとに session exit 0。走査 `clear-patterns-only` と独立確認は秘密不存在を完全保証しない。権限付き他ユーザーや同一ユーザーの別プロセスは ACL の完全な隔離対象ではない。

## 証拠 SHA256

各 `*.result.json` と `*.before.json` / `*.after.json` は scratch 直下、ログは scratch の `<stage>.protected-logs/` 内。承認 receipt は共有 run `tasks/I02/real-build-<stage>-log-approval-01.json`。

| stage | 結果 JSON | stdout / stderr | 承認 receipt |
| --- | --- | --- | --- |
| rustup | `58E4EFDCB6E212E0BD2F640D2CB015EA7DC896CB055C38C25B290E9FC94FAEBE` | `BBBB3EB31124866016C34BAFEF844DE0A7F20F5B1545B2B29102F90F5C565E08` / `B221EB4122453884168CD67BEB24241647E8B8653986BA3162D3163A1E53256C` | `0A008E59EFC56D5FD2B69A8D295ABB9F479FCB2EFA5B622E491880D2FA7F238E` |
| npm | `6F138DBD599471CA977587C9D2E19ED7CFF8DE9051513620702BAF0E782814C5` | `CCD84407B9A07765F902DDC23F3517F6952D23D6B41B36D4B3F9D2D1339288FF` / `960379AB2B6E16C97F4063BCD816D9DF4CC6C32CBCC2D926FAC49ACA2DC92829` | `759AA996CE4FC9DC88573077F8C20C57432662F8C4DB6C28235A671C7559CE8D` |
| release | `2F339B7395902D13A3FCF42C23D3FE88E0A65D279714E0A9CF2AE3C684F0307E` | `8C615277D28A2E9C3A2B7CB3272E7C37B660F7B8678659C4EE19A00D7B88AD87` / `35D143952405B01086E538741F7142D833882CBA626219E2160CC19D96647F71` | `DF528B1EBE829F9133C0687A5CB4D68C37310FA5754D5D337605A45063AF09CA` |

生成 exe `cargo-target/release/github-issue-widget.exe` SHA256 `D5FD13D80B6FE7F72ACD0BA6CEA3E0E19F261E417F8F5331E81F94D09A379D44`。専用 toolchain rustc SHA256 `1A9F8C489A90009CE5DB828D0A7436A1354789C42590D73FF5CC8B4E53695612`、cargo `5615085F39570E13B4ABD6F23E3302521A0EC570B2DD5182E39A98A87E5E9584`。生成された依存・cache・target 全個別ファイルの hash は未列挙であり、完全性と整理要否は未確認。すべて保持し削除対象なし。

## 未実施・残作業・教訓

- dev 起動と正常終了、実機の移動・リサイズ・最前面・初期/最小サイズ、Opener の許可外 URL に対する実 IPC 拒否は本依頼で未実施。Issue #2 は未完了。元 build の過去セッションの exit は依然不明であり、今回の成功で置き換えない。B03 全体の判定と対象版・ログの独立レビューを受ける。
- 今回3 stage の実行・読戻しでコマンド失敗なし。最初の rustup before/after 読取では PowerShell のパイプと配列化により件数を1と表示したが、JSON 実体を直接取得し各17件・全 hash 一致を再確認した。判定の前に対象件数と構造を確認する必要がある。
- 依存・保護ログ・生成物は新規 scratch に保持し、元の製品・旧 scratch・GitHub・GUI・他者プロセスは変更していない。結果は独立レビュー対象として確定後変更しない。
