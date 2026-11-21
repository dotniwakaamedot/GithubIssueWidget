# 最新 MVP の最小再ビルド依頼

- 担当 `/root/gui_failure_investigation`。共有 run `D:/Projects/GithubIssueWidget/docs/agent-runs/20261121-orchestration-continuation/`。親による依頼。現行製品・旧成果・他担当の変更を保護する。
- `creation-reject-review-01.md` は作成応答喪失後の再取得修正の固定版を承認。対象 `widget/src/main.tsx` SHA `B22504C9DE48D275726C912820DA93E02F10C1838CCC68555C156D35030165D7`、`main.test.tsx` `10AA28AD8D343F969C8BDB8C2D2232BF6007CB926619CB49850020DD53A40FBD`、`src-tauri/src/main.rs` `AEFF1E65AD93EFA04773CEBBB9736A6BDC5B9938DB9FAF937DD88B3D9CBFE86F`、`movement.rs` `88BF21BB45C077DFDA592D906C753D168C5641E35EB12324D4ACF1652374695B` を着手前・ビルド前後に照合。不一致や競合時は中止する。
- 前回の `gui-release-build-01/` は担当者が書込み停止・所有状況と終了を確認してから再利用する。前回 EXE `widget/release-20261121/GitHubIssueWidget.exe` SHA `91EC5D237B71EFC4DDCD12244E0022019FD61A0D1610931CA0B9FE2D4A2D55B2`、旧ログ・manifest・診断版と未コミット変更は上書き・削除しない。専用 build コピーへ現行2ファイルと現行の他入力を照合・反映し、変更前の必要な入力と旧 EXE を安全に保持する。
- 既に承認・成功した offline・専用 target の release ビルド方式を最小限変更し、新しいログ/status を使う。具体的な固定 runner・入力 SHA・バックアップ・出力先を提示し別担当の確認と親の開始通知を受け、1回のみ build。成功したテストは再実行しない。
- ビルド成功時は workspace 内の**新規** `widget/release-20261121-02/GitHubIssueWidget.exe` に通常ファイルとして用意。親ディレクトリ・対象・再解析点・既存同名の不存在を確認し、存在すれば上書きせず中止。生成元とコピーの SHA・サイズ・ユーザーの読取/実行アクセスを確認し、旧 EXE を保護する。ログには秘密を出さず、安全な分類のみ記録する。
- 前後の対象 SHA・コマンド・直接終了・入力と成果物・失敗・未知の子孫・実 GUI/GitHub/credential の未確認を固有結果に記録。ビルド成功を実起動・アプリ完成と混同しない。実 GUI の起動や外部操作はこの依頼に含めない。書込み停止後パスを通知。
