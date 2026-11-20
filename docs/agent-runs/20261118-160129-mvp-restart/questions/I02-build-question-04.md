# I02 B03 lockfile 検査結果・続行判断 04

- 担当 `/root/build_verification`、宛先 `/root`、`/root/restart_planner`。根拠: `questions/I02-build-answer-02.md`。
- scratch `D:\Projects\GithubIssueWidget-build-20261118-160129` は開始前に未存在、親 `D:\Projects` は通常ディレクトリで再解析ポイントなし、ACL owner `HOME_PC01\semis`、空き約 137.9 GB。新規作成に成功。共有 run 内 `tasks/I02/evidence-build-01/` も未存在を確認後新規作成。既存資産は上書き・削除していない。
- 入力 `D:\Projects\GithubIssueWidget-issue-2-20261118-160129\widget\package-lock.json` SHA256 `E300C165CA2F81735D2073B19FB1ADC886EAB484F5D837D968C726AFB2EF3B38` は前版と一致。Node `C:\Program Files\nodejs\node.exe` v26.13.1。
- 検査スクリプト `D:\Projects\GithubIssueWidget-build-20261118-160129\inspect-lock.cjs` SHA256 `322909D7D925762E2B23629B1F665C740A8FA4BA857BFFC66EE6C7E8EDDCE494`。例外処理・構造検査と JSON.parse を使用。作業ディレクトリ `D:\Projects\GithubIssueWidget`、コマンド `& 'C:\Program Files\nodejs\node.exe' $s $l | Out-File -LiteralPath $o -Encoding utf8`、実プロセス `$LASTEXITCODE` は 0。
- 結果 `tasks/I02/evidence-build-01/lock-summary.json` SHA256 `28C48016BAB93E25A3D166311A8487641C3CC07D97D25296ED3BB5A63EEF21B6`。lockfile version 3、134 package、registry host `registry.npmjs.org`、install script 対象 `node_modules/esbuild`・`node_modules/fsevents`。各スクリプトの内容・依存の安全性とオフライン完全性は未確認。
- 全参照8ファイルは実在・SHAが依頼に一致。元の対象17ファイルは存在と hash 一致、各ファイルの hardlink list は1件。対象 branch/HEAD、Issue、PR の状態は既存記録と整合。今回の検査は lock 内容の構造要約であり、取得・インストール・build/dev は未実施。
- 質問: 依存導入・ビルドに進むため、esbuild/fsevents の lifecycle scripts と必要な Cargo build scripts の審査・オフライン依存準備を担当範囲内で続行してよいか。公開 registry への取得の可否は未判断。回答まで依存導入・build/dev は行わない。
- 同種の lockfile 解析失敗2回があり独立調査後の今回の検査は成功。過去の失敗は維持し、同種3回目では全停止。
