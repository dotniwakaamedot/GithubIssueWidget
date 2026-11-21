# 最新 MVP release ビルド結果

担当 `/root/gui_failure_investigation`。`mvp-release-02-work-request-01.md`、`mvp-release-02-review-01.md`、親の開始指示に従い、最新通常製品版を専用環境で1回ビルド。セッション `26019` は終了、runner 終了0、直接 cmd の status `exit_code=0`。ログに release 完了の固定文字列があり、`error:` は一致なし。直接終了は未知の子孫すべての終了や以後のログ変更を保証しない。

## 配布対象

- `D:\Projects\GithubIssueWidget\widget\release-20261121-02\GitHubIssueWidget.exe`、12,883,456 bytes、SHA256 `D19A72750006DF5BB6CBCA4FB15F87F69BFF48A5785F5569C5B05E40C9B93F26`。生成元と一致、通常ファイル・非再解析点、Windows Users の ReadAndExecute 許可を確認。別アカウントの実効アクセス・実際の GUI 起動は未確認。
- 入力20ファイル manifest SHA256 `2EB56D635C9CCC132D46BA80B641B8A81444DFE79CA9D75E37D58349302B586C`。開始直前に現製品・保存版・ビルド版の全件が一致し、終了後も現製品との差分0。`src/main.tsx` `B22504C9DE48D275726C912820DA93E02F10C1838CCC68555C156D35030165D7` と `src/main.test.tsx` `10AA28AD8D343F969C8BDB8C2D2232BF6007CB926619CB49850020DD53A40FBD` は `creation-reject-review-01.md` の承認版と一致。`main.rs` `AEFF1E65AD93EFA04773CEBBB9736A6BDC5B9938DB9FAF937DD88B3D9CBFE86F`、`movement.rs` `88BF21BB45C077DFDA592D906C753D168C5641E35EB12324D4ACF1652374695B` も要求版と一致。診断専用 stage は含まない。
- 保護された `gui-release-build-01/mvp-build-02-output.log` は1654 bytes、SHA256 `4B7950FE49DF79597490C0618E3FEB2F1A71E307D8F2BBB4C4BBFB30DB3D9364`。内容は公開しない。前回のユーザー向け EXE `widget/release-20261121/GitHubIssueWidget.exe` は元 SHA `91EC5D237B71EFC4DDCD12244E0022019FD61A0D1610931CA0B9FE2D4A2D55B2` のまま保持し、旧ログ・診断版・バックアップも上書きしていない。

## 検証と限界

作業場所 `D:\Projects\GithubIssueWidget`、branch `main`、HEAD `5e74e1642fd18fe630dce0e684afd853721276ae`。実行コマンド `python docs/agent-runs/20261121-orchestration-continuation/gui-release-build-01/run_mvp_build_02.py` は承認済み方式で `VsDevCmd.bat -arch=x64` の後 `npm.cmd run tauri -- build --no-bundle` を実行。専用 target/Cargo/temp/設定と offline を使用、固定入力・ログ/配布先不存在を直前確認した。既合格テストの再実行、GUI・実 credential・実 GitHub の操作は行っていない。新たな操作失敗なし。成果と旧資産を保護し、削除対象なし。

EXE は通常 Windows 11 の対話デスクトップと WebView2 Runtime を想定。GitHub 接続は対象リポジトリ限定の fine-grained PAT（Metadata: Read、Issues: Read and write）を利用者が設定画面で直接入力し、トークンを会話に送らない。詳細は `README.md`。ビルド・ACL の確認は GUI 表示、通常終了、認証の保存復元、実 GitHub・障害条件やアプリ完成の証拠ではない。以前の別版の GUI 観測では対象窓が識別できず、その原因も未解決。独立レビューは提出後に行う。
