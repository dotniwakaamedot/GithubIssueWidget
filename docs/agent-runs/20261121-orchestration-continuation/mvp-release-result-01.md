# 現行 MVP release ビルド結果

担当 `/root/gui_failure_investigation`。`mvp-release-work-request-01.md`、独立承認 `mvp-release-review-01.md` と親の開始通知に従い、現行の通常製品版を専用環境で1回ビルドした。セッション `92722` は終了し runner 終了0、直接 cmd の status `exit_code=0`。`Finished `release` profile` を固定分類で確認し、`error:` の一致なし。直接の終了は未追跡子孫の終了を証明せず、ログの以後の変更を無条件には否定しない。

## 配布対象

- `D:\Projects\GithubIssueWidget\widget\release-20261121\GitHubIssueWidget.exe`、12,883,456 bytes、SHA256 `91EC5D237B71EFC4DDCD12244E0022019FD61A0D1610931CA0B9FE2D4A2D55B2`。生成元との SHA 一致、通常ファイル・再解析点でないこと、Windows Users の ReadAndExecute 許可を確認。実アカウントの実行成功・GUI 表示は未確認。
- 入力20ファイルの manifest SHA256 `673FDABA5A91336866FB4A8F073F773E071FF67146F2F04516CD9DD337B89024`。開始直前に現製品・保存版・ビルド版が全件一致し、終了後も現製品との差分0。主要対象 `main.tsx` `6FCF2A34145450C5029B7FF80DDB3EC1DA5E27173AAB84E57F16D6D54C0E6A3E`、`main.rs` `AEFF1E65AD93EFA04773CEBBB9736A6BDC5B9938DB9FAF937DD88B3D9CBFE86F`、`movement.rs` `88BF21BB45C077DFDA592D906C753D168C5641E35EB12324D4ACF1652374695B`。旧診断 stage を含まない通常版。
- 保護ログ `gui-release-build-01/mvp-build-output.log` SHA256 `2513579A881F05EB433B4C1A9E476467FCCE711FAAD0779C99DCB6B70227DB4A`、1654 bytes。旧通常 EXE と診断 EXE・旧入力の backup、旧ログ・status は保持。今回のログは実行者制限済み専用ルートに保護し内容を公開しない。

## 起動・接続と未確認

通常 Windows 11 の対話デスクトップで EXE を起動。WebView2 Runtime が必要。アプリの設定画面で GitHub の owner/repository と対象リポジトリ限定の fine-grained PAT（Metadata: Read、Issues: Read and write）を本人が直接入力する。トークンは会話・ログに貼らない。詳細は `README.md`。書込み権限や通信・レート制限は実環境で確認が必要。

この成果はビルド成功とアクセス権の静的確認であり、**実 GUI・通常終了・credential の保存復元・GitHub での操作を検証していない**。以前の旧版と別の診断版では CUA が対象窓を識別できず、今回の EXE が正常表示されることは未証明。診断情報 `gui-diagnostic-observation-result-01.md` は別バイナリの結果として扱う。MVP を完成済みと判断しない。

作業場所 `D:\Projects\GithubIssueWidget`、branch `main`、HEAD `5e74e1642fd18fe630dce0e684afd853721276ae`。コマンドは `python docs/agent-runs/20261121-orchestration-continuation/gui-release-build-01/run_mvp_build.py`、同 runner 内で `VsDevCmd.bat -arch=x64` 後に `npm.cmd run tauri -- build --no-bundle`。専用 CARGO_HOME/target、offline、専用 temp/設定と最小環境を使用し、製品の元ファイル・旧 fixture・実 credential・GitHub は変更していない。新たなビルド失敗はなし。新しい配布 EXE と本結果を含む専用成果は保持し、削除対象なし。独立レビューは提出後に行う。
