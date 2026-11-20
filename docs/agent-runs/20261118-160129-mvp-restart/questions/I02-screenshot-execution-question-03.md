# I02 個別窓画像の不一致 03

- 担当 `/root/screenshot_design`、宛先 `/root`・`/root/restart_planner`。対象 `tasks/I02/screenshot-execution-request-01.md` SHA256 `088E9F037AA2CDB01F5F623018D2B8C3E526AF0870A62E24FD688888542FA018`。
- 結論: **画像不一致が同種2回に達したため画像観測を停止し、別担当の客観調査を依頼する。** クリック・ドラッグ・入力・process停止は行っていない。browser inventory / `getState` は呼んでいない。
- 開始条件: 依頼内12参照は基点ごとに全件の存在と SHA256 を照合し一致。所有者の `questions/I02-screenshot-execution-question-02.md` は 2026-11-18T19:31:20Z 頃に app session `70494` と server session `19143` が live と記録。自分は handle を poll していない。直前の `Get-Process -Id 52960` と exe SHA256 は exit 0、exe path `D:\Projects\GithubIssueWidget-marker-diagnostic-20261118-160129\target\debug\github-issue-widget.exe`、window ID `6559504`、SHA256 `54DBCECFD920F6F920942A190C81B28BD7742C6052ABB58AB9C89E1F71C37AA8` と一致。ただし現在性は観測時点限り。
- 実施: 新しい CUA session の入口で `cua.getApp({windowId:6559504})` を1回選択。返却 AX には `GitHub Issue Widget`、`diagnostic-safe-area`、`diagnostic-ready:[]` および geometry の marker・button・resize 8件、minimum gap 15 CSS px が表示された。AX は対象の実画像・OS 座標・非遮蔽を証明しない。
- 続いて文書化された画像専用 API `app.getScreenshot()` を1回実行。前回の `getAXStateAndScreenshot()` とは API が異なり、対象は改訂された marker 診断版だが、返された画像には診断窓ではなく背景が表示された。画像 ID に依存する位置操作・画像保存・追加観測は行っていない。原因は未確定。画像は個人情報保護のため保存しない。CUA の部分作業は `completion_summary` で終了した。
- 元の質問08における対象外画像1回と同じ結果として今回を **2回目** に数え、getScreenshot および同じ画像問題に関する操作を中断する。同種3回目は全作業停止。前回 browser inventory error 2回・直接確定逸脱2回も維持する。第三の同種発生を誘発しない独立調査が必要。
- 未確認: 画像の取得元・対象範囲の食い違い、OS の DPI/client/frame とスクリーンショットの対応、非遮蔽、ユーザー操作との競合および安全余白。ロック状態・原因・ユーザー不在は推測しない。診断アプリの所有 session は保持し、画面再取得・入力・停止・再起動しない。
- 記録は同じディレクトリの `.tmp` を UTF-8 読戻し後、非上書き rename して公開。既存製品・証拠・Git/GitHub の変更や削除はしていない。
