# 同期テスト入力の隔離準備結果

担当 `/root/project_survey`。実行方式のレビュー待ちであり、テスト・ビルド・GUI・外部通信・プロセス終了は行っていない。製品・既存依存・旧 worktree は変更していない。

- 新規領域 `sync-test-fixture-01/` に `project/src` と設定、依存の独立通常コピーを作成。作成前に対象2ファイル、CSS、package、lock、tsconfig、vite config の全 SHA を依頼・監査の固定値と照合し、コピー後と処理末尾にも照合した。対象コピーの SHA は main `6FCF2A34145450C5029B7FF80DDB3EC1DA5E27173AAB84E57F16D6D54C0E6A3E`、test `66E118A5A7062854C23BE405D2E9578C974FA283BEA332F2A7C51DCA1BFFD13C`。
- 出典依存の全階層は再解析点0、全 5,467 項目・4,975 ファイル、118,621,128 bytes。`.vite`、`.vite-temp`、`.vitest-cache` の64ファイルを除外し、4,911ファイル・486ディレクトリを対象とした。ファイル内容ハッシュと相対パス一覧、ディレクトリ一覧をコピー前・出典コピー後・コピー先で比較し一致。`shutil.copyfile` で新規通常ファイルを作成し、リンクを作成していない。対象と親の再解析点を検査。実行直前の不変性と race の完全排除は未証明。
- 領域は workspace 内 `D:/Projects/GithubIssueWidget/docs/agent-runs/20261121-orchestration-continuation/sync-test-fixture-01/`。新規 `temp`、`tmp`、`appdata`、`localappdata`、`userprofile`、`logs`、`output` を予約。実際の環境変数、キャッシュ指定、プロセス監督は設定・実行していない。
- 所有ファイル 4,918 件のパス・SHA は `sync-test-fixture-01/manifest.json` に記録（manifest 自身は含まない）。manifest SHA256 `71628155B7E23E5C90C74E2FFA3D688A8C76EF0C180D0426690831D2B7C66F96`。検査・コピーの Python 処理は終了0。
- 事前の PowerShell `Get-PSDrive D` の Free が空で、1 GiB 比較が誤って「insufficient space」となり終了1。容量不足と断定できず、別の `.NET DriveInfo.AvailableFreeSpace` の読み取りで 104,218,918,912 bytes を確認。最初の失敗はコピー前で領域未作成。失敗記録・教訓への反映は管理に引き渡す。

削除対象なし。入力コピーは実行の承認ではなく、準備完了後の不変性・テスト方式・プロセスと出力の隔離は別途確認する。
