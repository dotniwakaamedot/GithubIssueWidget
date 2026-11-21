# 同期統合後の隔離型検査・ビルド結果

担当 `/root/project_survey`。承認済み `sync-post-integration-check-plan-01.md` SHA `99E859B8E4D27A706ACA58E1D29A02D93FB5E77935169BE4EF2BE58623C14C76` とレビューを確認し、対象版一致の隔離 fixture で各1回実行した。

## 結果

- 型検査 `node node_modules/typescript/bin/tsc --noEmit`: 直接プロセス終了0。専用 stdout・stderr はともに0 bytes。
- フロントエンド `node node_modules/vite/bin/vite.js build`: 直接プロセス終了0。Vite 7.3.8 は35 modules を変換し、`built in 423ms` と出力。stdout 1,074 bytes、stderr 0 bytes。`project/dist/index.html`（366 bytes）、`assets/index-Ad6Wo6ov.css`（5,671 bytes）、`assets/index-aBk6mjuG.js`（251,200 bytes）を生成。
- 作業ディレクトリは `sync-test-fixture-01/project`、Node は `C:/Program Files/nodejs/node.exe`。各 PowerShell で `$LASTEXITCODE` を直後に取得し、両処理とも即時完了でセッション ID は発行されなかった。直接終了と子孫終了は別であり、子孫終了は未確認。
- ログは `sync-test-fixture-01/logs/tsc-stdout.log`、`tsc-stderr.log`、`vite-stdout.log`、`vite-stderr.log`。Vite stdout SHA `15BE867137F0053649ADC5BBFFB75BD996120DE6DC810BEA1D937416761387F5`、他3つは空ファイル SHA `E3B0C44298FC1C149AFBF4C8996FB92427AE41E4649B934CA495991B7852B855`。生成 dist の SHA は HTML `EF5A83340D977BE31BCCE31862CAD1711091BFDEBCA5178506EB5516A62A579F`、CSS `782FA6F9394E0E7FEEE04721BFE3524D5262CA13B93503C89289432660CFD7FF`、JS `226AB322EC980D9209067D5C2032363DE187B72483F0E83AEFA3F7A112A827A2`。

## 固定入力と隔離

- 製品と fixture の main SHA `6FCF2A34145450C5029B7FF80DDB3EC1DA5E27173AAB84E57F16D6D54C0E6A3E`、test SHA `66E118A5A7062854C23BE405D2E9578C974FA283BEA332F2A7C51DCA1BFFD13C` は前後に一致。HTML は新規コピー前後で製品・fixture とも `14DE8CE58EF498F1BBE934D42DBADD9120B0052724F03FFFE532A31DC403D97E`。style `EB49BF65AE4FCE6701FBEF277055EDCAD42011248E5EA49905B6AFA4FD288B37`、tsconfig `2B5ED6E2C49DFDC635A27F83CBF5E2B835959CFA513F51215617B143D7F5D9DC`、package `102235B5686088C87B3221DEC7AE3CBEE20F0C9B1EAE070FD730C8B631DFCEDF`、lock `A5D65370135F83B08F02D5F1EE329706F08551743444264BCB10DFC7600B0C73`、vite config `1DAFF9E73019BD4E74489811D13553B3D227DB8457910D7F295E60AD0EB4CC7A` は事前確認。
- Node 実体 SHA `F5109DC868EC38703AD8F305E0ACEECE717CB60D4889873F434017AF4E96CCCB`、tsc 入口 `8D5FA5BD883FEC0979FC2004F1FE1D99AEF40570155D550EADC0B03B55513BF0`、Vite 入口 `FA03478846D229651A3C6AA64833BA2C6CBF580A798B92BD8F47C7480BAFB5D8`。実行前の manifest 4,918ファイルは欠落・差異・未知ファイル・再解析点0。空き容量は 104,089,919,488 bytes。
- 実行後 manifest 対象の欠落・差異0、再解析点0。既知の前回ログ・Vitest token、今回 HTML・4ログ・dist3ファイルと、専用 TEMP 配下の Node compile cache 33ファイルを確認。token 内容は読まず保持。削除なし。
- 環境は `SystemRoot`、`WINDIR`、`ComSpec`、`PATHEXT` のみ継承し、PATH は Windows と Node に固定。TEMP、TMP、APPDATA、LOCALAPPDATA、USERPROFILE は fixture 内。親のトークン・proxy・NODE_OPTIONS 等を引き継がない設定。

実製品の統合後直接試験ではなく同一 SHA 入力による隔離検証。Tauri 実行ファイル、Windows UI、GitHub・Credential Manager、スリープ、依存全体の外部アクセス不存在、子孫終了は未検証。今回失敗なし。製品・共有依存・旧 worktree を変更していない。結果と対象版の独立レビューが必要。
