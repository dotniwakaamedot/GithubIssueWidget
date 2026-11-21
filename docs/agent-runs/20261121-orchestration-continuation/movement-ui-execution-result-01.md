# 移動後の一覧失敗 UI 限定試験結果

担当 `/root/project_survey`。承認 `movement-ui-regression-review-01.md` と `movement-ui-regression-execution-request-01.md` に従い、新規独立 fixture で指定の1ケースを一度だけ実行した。

## 実行結果

- Vitest 5.1.1 は **1 passed、39 skipped**、対象ファイル1件成功。直接 PowerShell の終了コード0。stdout 1,214 bytes、stderr 0 bytes。ログ `movement-ui-test-execution-01/logs/stdout.log` SHA `72587E94D0E8A98A652982EB1330DF8D724163C974FAAD50A34939A93F1A8CF0`。ログは完了行と実行件数を含む。ケース選択は `-t 'clears stale cards and shows both errors when refresh after an uncertain move fails'`。39件は今回再実行していない。
- コマンドは Node 経由 `node_modules/vitest/vitest.mjs run src/main.test.tsx`、上記 `-t`、`--pool=threads --maxWorkers=1 --no-file-parallelism --testTimeout=10000 --hookTimeout=10000 --no-cache --no-fsModuleCache`。作業場所 `movement-ui-test-execution-01/project`。stdout/stderr を専用ログに分離し `$LASTEXITCODE` を直ちに取得。即時終了でセッション ID は発行されなかった。ログの時刻 11:21:52、duration 775ms。
- 直接終了は確認済み。Vitest worker や esbuild 子孫の終了、実 Windows ウィンドウ・GitHub・資格情報、実競合は未確認。UI モック試験の結果を製品全体の成功とは扱わない。

## 入力と隔離

- 製品 main SHA `6FCF2A34145450C5029B7FF80DDB3EC1DA5E27173AAB84E57F16D6D54C0E6A3E`、製品 test `66E118A5A7062854C23BE405D2E9578C974FA283BEA332F2A7C51DCA1BFFD13C`、隔離・実行 test `3123D7B77EA1E283082FDFF198BA882ACA1E0AAF216E9D7D567F20B55D615B63` を開始前後に照合。実行 main は製品版と一致。
- 新規領域へ style・package・lock・tsconfig・vite config を固定 SHA で複写。`widget/node_modules` の再解析点を検査しキャッシュを除いた4,911ファイル・486ディレクトリを通常コピー。コピー前、出典コピー後、コピー先で相対パスと全 SHA が一致。試験後も4,911ファイルの追加・欠落・差異・再解析点0。manifest SHA `D538DFECCA80A7D31818A7BF7875EF3C0F7369FEAE5B5A60ED337E214380A02A`。
- Node SHA `F5109DC868EC38703AD8F305E0ACEECE717CB60D4889873F434017AF4E96CCCB`、Vitest `39DB22F579ACF5639BBB17A261408DEBBDE03F4692C0C439E77E7F13AEBA74D6`、esbuild `C7BEE37877D0AA6A046E52783FA0A2CF1A9CE5579D68BB3083BDA99D4BFF18EF` を照合。`SystemRoot`、`WINDIR`、`ComSpec`、`PATHEXT` のみ継承、PATH はシステムと Node に固定。TEMP、TMP、APPDATA、LOCALAPPDATA、USERPROFILE は専用領域。親の秘密・proxy・NODE_OPTIONS 等は引き継がない設定。OS の Credential Manager や依存全体の外部アクセス不存在を証明するものではない。

製品、旧 fixture、既存依存や旧 worktree を変更せず、生成物を保持。削除対象なし。実行中に失敗なし。結果と版の別担当レビュー、製品テストへの適用判断は後続。
