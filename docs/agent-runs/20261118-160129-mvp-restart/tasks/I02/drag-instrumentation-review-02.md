# I02 診断版コードの独立レビュー 02

- run_id / task_id / review_revision / 回数: 20261118-160129-mvp-restart / I02-drag-instrumentation-review / 02 / 2
- owner_role / owner_agent_id: verification_reviewer / /root/plan_review。作者 /root/drag_design とは別担当。
- `drag-instrumentation-revision-result-01.md` は SHA256 43AE521C9934642B3CA71553BACAA7C7E0129C4DD8421D59B4713A84000DCEE1、前回レビュー `drag-instrumentation-review-01.md` は 93D043E8D9DC5F13AA9C27005B7FC05838DA01C05348C5FC7A31D5D5AC6CFAAF。依頼の構造化参照12件は基点・相対パス・SHA を分離し、全件存在確認後に実体ハッシュを照合して一致。
- 元 worktree `D:\Projects\GithubIssueWidget-issue-2-20261118-160129`、branch `task/issue-2-foundation-20261118-160129`、HEAD a4ac97286785a2a5ba0d070a909b58003fc49ff1。元17ファイルは result-02 の実体 SHA と一致し、README 変更・widget 未追跡16件、削除なし。診断 scratch の元17件は14件不変・3件変更、別に1件新規で計4変更。訂正記録と一致。
- 判定: **対象ファイル SHA256 と記載の実行前ゲートに限定して承認**。blocking 指摘なし。実際の build・起動・入力、診断機能の実動作、製品完成の承認ではない。

| 対象 | 今回の実体 SHA256 と静的確認 |
| --- | --- |
| `widget/src/main.tsx` | 820F164EF8947DA6B87F6ACE8B74D787E09ED9AF7A6610E2C07678DFA2C4AE5C。header の drag 呼出しは除去されたまま。固定の非対話 marker を追加。resize 処理と nav の button は残存 |
| `widget/src/diagnostic.ts` | 56A62632E8D63AEE5DD0AB636C954E30FBB76562C90C88CD9C0D53AB497B6D37。最大20件の指定されたイベント項目のみをメモリと `output.textContent` に表示。ログ本文・URL・キー・Issue の取得コードなし |
| `widget/vite.config.ts` | 194D17A739D0F626DDAF94B5D2B6F9335DBF50BB571AB4F7739DA727ED4F772A。前診断版と一致、port 1435 |
| `widget/src-tauri/tauri.conf.json` | EDE0EA3E1EE5C6FE067A5E127EEAAE02A19110E8CD8E057C8B7FCB2B826A55D5。前診断版と一致、devUrl 1435 |

## 前版指摘と条件

| 対象 | 設計と判定 | 実行前に残る条件 |
| --- | --- | --- |
| B01 メモリ記録の取出し | 空の `diagnostic-ready:[]` を対象窓の accessibility 又は読み取れる screenshot で**入力前**に保存し、同一 PID・exe SHA・window ID と結び付ける方法を追加。入力後も同窓から記録を取得・保存できない場合は反復せず停止 | **設計上是正**。output の tree 公開・画像での判読・正確な内容の取得は未実測。空表示を対象窓で識別・保存できない場合は入力しない |
| B02 安全な非対話領域 | marker の矩形中心を唯一の候補とし、header・全 button・resize 帯の実境界から少なくとも12 CSS px、DPI・座標系・対象窓・画像と client の対応を確認。対象不明・余白不足・重なりで停止 | **設計上是正**。実際の境界・同一窓の識別・表示・DPI 変換は未実測。z-index 11 の marker は resize の z-index 10 より上なので、静的値に頼らず実測ゲートが不可欠 |
| N01 件数 | 管理の `drag-count-correction-01.md` と修正結果は、元17件中14件不変・3件変更と別の新規1件に訂正 | 解消。旧質問は書き換えない |
| drag と残存権限 | 前版から header の `startDragging` は除去。`startResizeDragging`、button と drag capability は残る | 設計に従い button/resize 帯を避ける必要あり。capability 削除や OS の挙動が検証されたわけではない |
| 版・停止・並行 | 診断用差分、対象 PID・exe・window ID・時刻を固定し、元製品と分離。B03 scratch、元 worktree、既存証拠を保護。port・profile・desktop・他者作業の競合は不明なら停止 | 診断版・単一押下の成功は過去のドラッグや製品版移動の実証ではない。依存導入、build、実機 UI・型・抽出方法、環境競合は未確認。起動・入力には別途親判断と直前の各ゲート確認が必要 |

前回と同じ意味の blocking 指摘は再発していない。未確認の動作を合格とせず、記録取得・安全領域のゲートが満たされない場合は同じ入力を試さず停止する。同種2回で該当操作停止・別担当調査、3回で全中止を維持する。

## 実施内容

- cwd `D:\Projects\GithubIssueWidget`。構造化依頼を UTF-8 で読み、JSON の担当と参照12件を抽出。基点・相対パス・SHA を分離し、全件存在確認後に `Get-FileHash -Algorithm SHA256` で照合。作者結果・依頼・訂正・診断版コードを読取。各コマンド群終了コード 0。
- `result-02.md` の製品欄に限定した元17件と scratch17件を全件存在確認後に hash 照合。`git -C <元worktree> branch --show-current`、`rev-parse HEAD`、`status --short --untracked-files=all` と scratch の `rg --files --hidden` を確認。終了コード 0。
- 親ディレクトリ存在・確定先と tmp の不存在を確認し、UTF-8 往復と代表文字を検査後に確定。入力・ドラッグ・UI 操作・アプリ起動・build・通信試験・製品/scratch/既存証跡・Git/GitHub の変更は行っていない。レビュー担当の失敗なし。