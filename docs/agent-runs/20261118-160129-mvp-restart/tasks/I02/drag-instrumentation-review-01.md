# I02 診断版コードの独立レビュー 01

- run_id / task_id / review_revision: 20261118-160129-mvp-restart / I02-drag-instrumentation-review / 01
- owner_role / owner_agent_id: verification_reviewer / /root/plan_review。作者 /root/drag_design とは別担当。
- 構造化依頼の参照13件は基点・相対パス・期待 SHA を分離し、全件存在確認後に SHA256 を照合して一致。診断版の4ファイル SHA は下表に示す。
- 元 worktree `D:\Projects\GithubIssueWidget-issue-2-20261118-160129` の branch `task/issue-2-foundation-20261118-160129`、HEAD a4ac97286785a2a5ba0d070a909b58003fc49ff1。元17件は結果02の期待 SHA と一致、元 status は README 変更・widget 未追跡16件、削除なし。
- 診断 scratch `D:\Projects\GithubIssueWidget-drag-diagnostic-20261118-160129`: 元17件のうち **14件が一致、3件が変更**、`diagnostic.ts` が1件新規で計4変更。source-manifest の17件は元実体の SHA と一致。質問03の「上記以外の15件」は算術上の記録誤り（N01）。
- 判定: **要修正**。コード上の header の通常 drag 呼出し除去は確認できたが、単一入力に進むための記録取得と対象領域の安全条件に blocking がある。診断版の起動・入力や製品完成を承認しない。

| 診断変更 | 実体 SHA256 | 静的確認 |
| --- | --- | --- |
| `widget/src/main.tsx` | F318C76FE54F081F55656B84C0C9680C1416560E272C003C480C6B7C03CA4FA2 | header の `startDragging` 呼出し削除、診断 module を import。リサイズ用の `startResizeDragging` は残存 |
| `widget/src/diagnostic.ts` | 76163B634209ABE4976F2A954E463EDFAEF7AB97B110E0B1D96858135682ADDF | mousedown/mouseup の capture/bubble のみ、type・phase・button・buttons・trusted・3区分 target/位置・`performance.now()` を最大20件メモリに保持。テキスト・URL・キー入力を読み取るコードなし |
| `widget/vite.config.ts` | 194D17A739D0F626DDAF94B5D2B6F9335DBF50BB571AB4F7739DA727ED4F772A | dev server の port 1435 |
| `widget/src-tauri/tauri.conf.json` | EDE0EA3E1EE5C6FE067A5E127EEAAE02A19110E8CD8E057C8B7FCB2B826A55D5 | devUrl の port 1435。元と同じ main capability を参照 |

## 要求ごとの確認

| 対象 | 根拠・判定・未確認 |
| --- | --- |
| 通常 drag と権限 | `main.tsx`・`diagnostic.ts`・`index.html`・`style.css` から `startDragging`、`data-tauri-drag-region`、`app-region` の記述は見当たらず、header の直接呼出しは除去。capability は元版と同じで drag 許可を引き続き含むため、権限自体を無効化したわけではない。実行時に他のライブラリが操作を行わないことは静的確認だけでは完全に証明できない |
| 入力安全 | header 内の nav は pin/minimize/close のボタンを保持。周辺の幅5〜9pxの固定 resize 要素は `startResizeDragging` を呼ぶ。header 内と指定するだけではボタンやリサイズ帯への誤入力を防げない（B02） |
| 記録内容と取得 | 実際のコードは指定の非機密項目のみを保持するが、配列は WebView 内メモリだけであり、作者自身が実行時の取得方法を未確認と記載。証跡回収方法なしの単発入力は未確認や反復を招く（B01） |
| ポート・資産 | devUrl と Vite は1435で一致。専用コピーであり他の元17件との差分は上表のみ。実行時の空き port・PID・WebView・desktop・profile・cache・子プロセスの分離は未確認で、親判断まで起動・入力しない |
| 期待する証拠 | 診断版は対象版と異なり、たとえ単一クリックが記録されても過去のドラッグ到達・IPC 認可・OS 移動・Issue の完成は証明しない |

## 指摘

| ID | 種別 | 根拠・影響 | 是正条件 |
| --- | --- | --- | --- |
| B01 | blocking | `diagnostic.ts` は `window.__dragDiagnostic` のメモリ配列にのみ格納。質問03は取得方法未確認と記載。入力後に取得できず、結果不明や単一入力の反復を余儀なくされ得る。 | 入力前に対象 main WebView の識別、配列を読み取る具体的方法と非機密証跡への保存、時刻・PID・exe・版・失敗時停止を設計し、作者以外の確認と親判断を受ける。抽出方法や利用可能性が不明なら入力しない。技術的な動作確認が必要なら今回の同じ入力を使わない安全な方法を別途審査する。 |
| B02 | blocking | header 内にもアプリ制御ボタンがあり、タイトルの外周にはリサイズ帯が重なる。依頼の「header 内」のみでは誤操作対象を一意に排除できない。 | 対象 HWND/PID・スケール・header とボタン/resize 要素の境界と安全余白を操作前に確認し、**button と resize 帯を除いた非対話領域**を一意に選択する手順を固定する。対象不明・領域不十分なら入力しない。単一押下・解放以外は禁止。 |
| N01 | non-blocking | 質問03の「上記以外の15件」は、元17件から変更3件を除くと14件となり不正確。新規1件を含めても同文言は元17件の不変数と整合しない。 | 質問03は書換えず、管理の確定記録または次版結果で14件不変・3件変更・1件新規と明確に訂正する。 |

## 実施内容・制約

- cwd `D:\Projects\GithubIssueWidget`。依頼の JSON 参照13件について基点・相対パス・SHA を分離し、全件存在確認後に実体ハッシュと照合。質問・管理回答・四変更ファイルと source-manifest を UTF-8 で読み取った。各コマンド群終了コード 0。
- `result-02.md` の製品欄のみから元とコピーの計34パスを抽出、全件存在確認後に SHA を照合。`git -C <元worktree> branch --show-current`、`rev-parse HEAD`、`status --short --untracked-files=all` を実行。診断 scratch の `rg --files --hidden` と対象 HTML/TSX/TS/CSS の `Select-String` により候補箇所を静的検索。終了コード 0。
- 親ディレクトリ存在・確定先と tmp の不存在を確認し、UTF-8 往復と代表文字を検査して確定。実機操作・入力・診断起動・build・通信試験・元製品や scratch・既存証拠・Git・GitHub の変更は行っていない。今回のレビュー担当の失敗なし。