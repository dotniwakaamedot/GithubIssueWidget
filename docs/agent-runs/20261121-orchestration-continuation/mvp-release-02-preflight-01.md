# 最新 MVP release ビルド固定版

担当 `/root/gui_failure_investigation`。`mvp-release-02-work-request-01.md` の対象20入力を専用 `gui-release-build-01/mvp-input-02/` に保存し、元・保存版・ビルド版の SHA を照合。manifest `mvp-input-manifest-02.json` SHA256 `2EB56D635C9CCC132D46BA80B641B8A81444DFE79CA9D75E37D58349302B586C`。前回 manifest とファイル集合は同一、変更は承認済み `src/main.tsx` `B22504C9DE48D275726C912820DA93E02F10C1838CCC68555C156D35030165D7` と `src/main.test.tsx` `10AA28AD8D343F969C8BDB8C2D2232BF6007CB926619CB49850020DD53A40FBD` の2つのみ。`main.rs` `AEFF1E65AD93EFA04773CEBBB9736A6BDC5B9938DB9FAF937DD88B3D9CBFE86F`、`movement.rs` `88BF21BB45C077DFDA592D906C753D168C5641E35EB12324D4ACF1652374695B` は要求版と一致。現製品・ビルド版は20件すべて一致。

旧 build 直接セッション `92722` は終了0、書込み停止済み。上書きされ得る前回の EXE・変更前2入力を `mvp-backup-02` に保存し元と SHA 一致。前回利用者向け EXE `widget/release-20261121/GitHubIssueWidget.exe` SHA `91EC5D237B71EFC4DDCD12244E0022019FD61A0D1610931CA0B9FE2D4A2D55B2` は触らない。診断・以前の通常版のバックアップ、ログ、manifest と他者の製品変更も保持する。

候補 `python docs/agent-runs/20261121-orchestration-continuation/gui-release-build-01/run_mvp_build_02.py` SHA256 `804316F47E161BD79B2C21D5A95B96006129BB292AFE3FA78C7B621F68A62267`。前回承認 runner のログ/status 名だけを `mvp-build-02-output.log`・`mvp-build-02-status.txt` に変更。既存 `build.cmd`、offline と専用 Cargo/target・TEMP/APPDATA、秘密を渡さない明示環境・保護ログ・直接終了の記録は前回方式。cache/target は前回利用済みで、依存や Windows の通信/credential 非到達・未追跡子孫の終了は保証しない。

直前に現製品・ビルド版の20 SHA、レビュー対象4 SHA、旧 EXE・バックアップとログ保護、新しいログ/status と `widget/release-20261121-02/` の不存在、親・対象の再解析点を再確認。独立レビューと親の通知後1回ビルドし、成功後だけ新規配布先へ生成 EXE を通常ファイルとしてコピー、SHA・サイズ・ユーザー読取/実行 ACL を確認する。現製品・旧成果を上書きせず、同じ成功版を無理由に再実行しない。実 GUI・credential・GitHub 操作は行わない。
