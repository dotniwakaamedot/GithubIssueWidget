# I02 専用診断 marker 調整の独立レビュー 01

- run_id / task_id / review_revision: 20261118-160129-mvp-restart / I02-marker-adjustment-review / 01
- owner_role / owner_agent_id: verification_reviewer / /root/plan_review。作者 /root/drag_design とは別担当。
- 対象 `marker-adjustment-result-01.md` SHA256 BBCC775114D265FD76EEDEF09335B7EAC2180EFC557FEE4B6EF601F8D7C18F58。構造化依頼の参照15件は基点・相対パス・SHA を分離し、全件存在確認後に実体を照合して一致。
- 旧 scratch `D:\Projects\GithubIssueWidget-geometry-diagnostic-20261118-160129`、新 scratch `D:\Projects\GithubIssueWidget-marker-diagnostic-20261118-160129`。新 `copy-manifest.json` SHA256 CA678FB46C1CF89C34C44C5EF7A22E900E391AA13D72E961430AE32CE2FA916D の18件について旧新36パスを全件存在確認後 hash 照合。旧18件は manifest と一致、新版は17件不変・`widget/src/main.tsx` 1件のみ変更。
- 変更 main.tsx の旧 SHA256 820F164EF8947DA6B87F6ACE8B74D787E09ED9AF7A6610E2C07678DFA2C4AE5C、新 SHA256 DE50084BEEA52147D7B3482AC84ADEFE70AA2C5E8F211BDF1930A8AED7C1D1DD。`diagnostic.ts` 840D7F69349AEED52C0A5D6FA6319317CCE432390A74463F8E824069F26E412D、`vite.config.ts` 194D17A739D0F626DDAF94B5D2B6F9335DBF50BB571AB4F7739DA727ED4F772A、`tauri.conf.json` EDE0EA3E1EE5C6FE067A5E127EEAAE02A19110E8CD8E057C8B7FCB2B826A55D5 は旧新で一致。
- 元 worktree `D:\Projects\GithubIssueWidget-issue-2-20261118-160129`、branch `task/issue-2-foundation-20261118-160129`、HEAD a4ac97286785a2a5ba0d070a909b58003fc49ff1。元17件は `source-manifest.json` の実体 SHA と一致。status は README 変更と widget 未追跡16件、削除なし。
- 判定: **対象版の診断コード差分と指定の実施前ゲートに限定して承認**。blocking 指摘なし。実際の build・起動・画面取得・入力や製品完成の承認ではない。

## 要求ごとの確認

| 要件 | 独立確認 | 判定・未確認 |
| --- | --- | --- |
| 変更範囲 | 旧 main.tsx の `top:16,width:64,height:24` を `top:20,width:64,height:24` に1回だけ置換すると新版全文と一致。残り17件に変更なし | 適合。元製品は変更していない |
| 旧観測と12 CSS px | `observation-01.json` の手動転記は marker top 16、上側 resize bottom 5、最小余白11。旧観測の他条件が同一なら新 top 20 は上側15、header bottom 56 と marker bottom 44 の差12 | 算術は一致。実行時の他 button/resize、viewport、responsive、DPI、遮蔽、実画像・AX の対応は未確認であり入力の安全性は未証明 |
| drag・resize・収集範囲 | 通常 header の drag 呼出し除去、旧版の `diagnostic.ts`、event / geometry の固定非機密項目を維持。button・resize と drag capability は残存 | 適合。実際の hit target・実画面・空出力と全矩形の確認前に入力しない |
| 所有と競合 | 別 scratch、旧/新の資産を保持。port 1435 は旧版と共通であり、旧 process、画面・profile・cache の所有と非競合は開始前に確認する計画 | 適合。旧 process の live/terminal やユーザー操作は本レビューでは直接確認していない。不明なら起動・入力しない |
| 失敗・証拠 | 直接確定の逸脱2件と画像不一致・browser inventory・drag の履歴を保持。2回で当該操作中断と客観調査、3回で全停止 | 適合。旧逸脱ファイルを書換えず、公開は一時ファイルの新規作成・読戻し・非上書き rename を使う |

## 指摘・残条件

blocking および新規 non-blocking 指摘なし。未実行の設計・静的差分に限る。対象 HWND/PID/exe と画面の同一性、DPI・座標、全矩形・正寸法・12 CSS px、OS の非遮蔽、記録の読取り可能性、画面・対象・他者との非競合は実施前に独立に満たす必要がある。得られなければクリックしない。診断版の成功は元版ドラッグや Issue #2 の完了を証明しない。

## 実施内容・未実施

- cwd `D:\Projects\GithubIssueWidget`。依頼の構造化参照15件を基点・相対パス・SHA に分離し全件存在確認後に `Get-FileHash -Algorithm SHA256` を実施。対象結果・旧/新 main と元観測 JSON を UTF-8 で読取。終了コード 0。
- `copy-manifest.json` の旧新18件は全36パス存在確認後に hash 照合。旧全文から対象文字列を1箇所のみ置換して新版全文と一致することを比較。元17件は `source-manifest.json` と照合。`git -C <元worktree> branch --show-current`、`rev-parse HEAD`、`status --short --untracked-files=all` を確認。終了コード 0。
- 実際の build・起動・画像取得・UI 操作・クリック・ドラッグ・通信試験・既存プロセスや対象ファイル・Git/GitHub の変更は行っていない。レビュー担当の失敗なし。
- 出力親の存在と確定先・tmp の不存在を確認し、tmp を新規作成専用で開き write/flush/close、UTF-8 往復と代表文字検査後に非上書き rename で確定。対象版変更後は再レビューする。