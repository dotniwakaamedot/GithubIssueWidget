# 同期修正版の限定フロントエンド試験

担当 `/root/project_survey`。`sync-practical-test-plan-01.md` の SHA `C8DA60D729EFA188FF2D34291DDDB94D1BA93CB533948B075DF0C76CBE3C0BEF` と承認 `sync-practical-test-review-01.md`、担当訂正と管理開始通知を確認。一度だけ実行した。

## 結果

- 隔離コピーの `src/main.test.tsx` は Vitest 5.1.1 で **39件成功、失敗0、1ファイル成功**。直接実行された PowerShell プロセスの終了コード0。標準出力 `sync-test-fixture-01/logs/stdout.log` は1,070 bytesで完了表示を含み、標準エラー `logs/stderr.log` は0 bytes。出力に切断は観測されない。ログは合成入力のみを意図したもので、実ユーザーデータを渡していない。
- 実行場所 `sync-test-fixture-01/project`、実行時ログ記載開始 `10:37:54`、報告された実行時間 1.74 秒。即時終了のためセッション ID は発行されなかった。直接終了は確認済みだが Vitest worker や esbuild など子孫の終了は未確認。
- コマンド: `C:\Program Files\nodejs\node.exe .\node_modules\vitest\vitest.mjs run src/main.test.tsx --pool=threads --maxWorkers=1 --no-file-parallelism --testTimeout=10000 --hookTimeout=10000 --no-cache --no-fsModuleCache`。PowerShell で stdout/stderr を専用ログに分け、`$LASTEXITCODE` を直ちに取得した。

## 入力と隔離

- 実行前後に manifest SHA `71628155B7E23E5C90C74E2FFA3D688A8C76EF0C180D0426690831D2B7C66F96`、隔離版と fixture の main SHA `6FCF2A34145450C5029B7FF80DDB3EC1DA5E27173AAB84E57F16D6D54C0E6A3E`、test SHA `66E118A5A7062854C23BE405D2E9578C974FA283BEA332F2A7C51DCA1BFFD13C` を確認。実行直前の manifest 4,918 ファイルは追加・欠落・差異0、再解析点0、専用ディレクトリは空、`.env*` なし。Node、Vitest、esbuild の SHA は監査の固定値と一致。
- 試験後の manifest 4,918 ファイルは欠落・差異0、再解析点0。追加3ファイルは専用 stdout/stderr と `localappdata/vitest/.vitest-secret-token`。後者は生成された値を読み取らず保持し、内容を出力していない。
- 実行プロセスは `SystemRoot`、`WINDIR`、`ComSpec`、`PATHEXT` のみ継承し、PATH をシステム・Node に固定。TEMP、TMP、APPDATA、LOCALAPPDATA、USERPROFILE を fixture の専用ディレクトリに設定。元のプロキシ、トークン、NODE_OPTIONS、ESBUILD_BINARY_PATH は継承しない設定。既存設定や依存を変更していない。

合成 UI テストは Windows 実画面、実 GitHub、実認証、スリープ、製品全体の検証を代替しない。OS 資格情報ストアや依存全体の外部アクセス不存在は証明しておらず、子孫終了も未確認。対象版を別担当がレビューし、統合前の所有権・競合照合と統合後の検証が必要。今回の試験中に失敗は認識していない。生成物を保持し削除していない。
