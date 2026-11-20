# I02 専用診断 marker の余白調整結果 01

- run / task / 担当: `20261118-160129-mvp-restart` / `I02-marker-adjustment` / `/root/drag_design`。依頼 `tasks/I02/marker-adjustment-request-01.md` 実体 SHA256 `2127961B3C0AAF363BAF43261D16944B3F3099592BF5D4E93C207FE1F3AC6007`。依頼の参照14件は基点と相対パスを分け全件存在確認後に実体 SHA256 を照合、全件一致。
- 結論: **新しい診断コピーで marker の `top` のみ 16 → 20 CSS px に変更。実行・画面・入力は未検証で、安全性を証明するものではない。** 元のドラッグは2回の位置不変で原因未確定・未達。診断版は独立レビューと親判断まで起動しない。
- 対象元 worktree `D:\Projects\GithubIssueWidget-issue-2-20261118-160129`、branch `task/issue-2-foundation-20261118-160129`、HEAD `a4ac97286785a2a5ba0d070a909b58003fc49ff1`。元の内容は未コミット/未追跡を含む17件、共有 `source-manifest.json` を通じて前版結果 `tasks/I02/geometry-instrumentation-result-01.md` と対応。今回変更先は元 worktree ではない。

## コピー・差分と版

- source `D:\Projects\GithubIssueWidget-geometry-diagnostic-20261118-160129` の対象18ファイル（元17件に診断 `widget/src/diagnostic.ts` 1件）を通常バイトコピーし、新 scratch `D:\Projects\GithubIssueWidget-marker-diagnostic-20261118-160129` に作成。source の18ファイルの path、size、コピー時 SHA と同値のコピー SHA は新 scratch `copy-manifest.json` SHA256 `CA678FB46C1CF89C34C44C5EF7A22E900E391AA13D72E961430AE32CE2FA916D` に18件記録。source の元となる17件と前版変更は依頼指定の前版結果・manifest で追跡できる。内容の実行中の一時変更の不存在をこれだけで証明しない。
- コピー先親 `D:\Projects` と source 対象祖先に reparse point がなく、source 18件の hardlink list が各1件、新 scratch は未存在、ACL owner と空き約129GBを確認。固定の異なる絶対パスで元 worktree・旧診断・B03 scratch・共有 run と分離。コピー対象以外の node_modules、cache、build 生成物、既存証拠はコピーしていない。
- コピー後の旧版/新版を再照合し **17件不変、1件変更**。`widget/src/main.tsx` 旧 SHA256 `820F164EF8947DA6B87F6ACE8B74D787E09ED9AF7A6610E2C07678DFA2C4AE5C` → 新 SHA256 `DE50084BEEA52147D7B3482AC84ADEFE70AA2C5E8F211BDF1930A8AED7C1D1DD`。marker の `top:16` から `top:20` への1値だけ変更。header の通常 `startDragging` 呼出しは除去されたまま。既存の resize 処理は残る。
- `widget/src/diagnostic.ts` は旧版と同じ SHA256 `840D7F69349AEED52C0A5D6FA6319317CCE432390A74463F8E824069F26E412D`。非機密イベントと geometry の収集方式・上限は旧版そのまま。ポート1435、各 lock と設定も同一であり、旧版と同時起動可能とはみなさない。新旧元18件の各 SHA・サイズは copy-manifest に記載し、変更後の1件は上記の新版 SHA を使用する。

## 静的根拠と残るゲート

既報の一時点サンプルでは上側 resize の下端が5、旧 marker 上端が16で差11 CSS px。その他をそのままと仮定した場合、新 top 20 の上側差は15、marker 下端44と header 下端56の差は12となる。しかしこれは**固定した過去サンプルからの算術上の推定**であり、実行時の viewport、全 button・8 resize 帯、DPI、重なり、target / hit test、画面座標、responsive 変化、AX と対象画像の一致、他者操作の非競合はいずれも未証明。12px の条件を満たすことや単一クリックの安全性を主張しない。前回の画像不一致と browser inventory エラーも解消済みとはしない。

独立レビュアーが元/旧/新の各実体・18件の hash と変更差分、診断領域・非機密データ・port・安全条件を確認し、親が次版の作業と対象を判断する。さらに実施時は別依頼で、対象 PID/実行ファイル SHA・window ID、同一時点の全矩形・CSS/DPIと画面・入力座標、12px以上の余白、非遮蔽と安全な記録回収を確認できない限り入力しない。診断版の1回の押下・解放も、元版の移動成功や過去の因果を証明しない。

## 実施記録・失敗

- cwd `D:\Projects\GithubIssueWidget`、PowerShell `Get-Content -Encoding UTF8` で依頼・固定計画・公開方式を読取（exit 0）。依頼の構造化参照 **14件**を `Test-Path` 後に `Get-FileHash -Algorithm SHA256` で全件照合（exit 0）。
- 旧 geometry の担当所有 server session `68650`・app session `4498` を開始時に空 poll し、ともに session ID が返った。空 poll は観測時点の live を示し以後の状態を保証しない。新旧 process の起動・停止・入力は行わず、旧の状態を変更していない。
- `Get-Item`・`Get-Acl`・`Get-PSDrive`・`fsutil hardlink list`・`Get-FileHash` で元/旧の対象、親・リンク・所有・容量を確認（各 exit 0）。`[IO.File]::Copy` と copy manifest 作成前後の18件照合（exit 0）。manifest は同一ディレクトリの新規 `.tmp` を `CreateNew` で作成し flush/close、UTF-8 と JSON 構造・18件を検査後、既存先を置換しない rename で確定し SHA を確認。質問・結果の直接確定3回目を起こさないよう同じ手順を用いる。
- 今回の担当作業に新しい失敗なし。直接公開違反2回、画像不一致1回、browser inventory failure2回などの履歴を保持。ユーザー指示に従い同種問題2回で該当操作を中断・別担当調査、3回で全作業中止。**build、依存導入、起動、UI 表示、画像取得、クリック、ドラッグ、リサイズ、通信試験はいずれも今回未実施。** 既存プロセス・scratch・共有証拠・Git・GitHub の変更、削除なし。結果は確定後に作者以外が実体 SHA を確認する。
