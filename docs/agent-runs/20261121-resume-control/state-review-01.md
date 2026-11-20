# Open / Closed 表示の独立確認 01

- 判定: **要修正（blocking 1件）**。作者 `/root`、確認者 `/root/state_review` は別担当。実 GitHub、資格情報、製品 UI の操作とテストの独立再実行は行っていない。対象版の合否でありアプリ全体の完成を判定しない。
- 4対象・2ログの SHA256 は依頼値と一致し、レビュー終了直前にも4対象の一致を再確認した。対象の `git diff --name-status` は4件すべて M、対象削除ファイルなし。未追跡の対象追加・削除は依頼に指定されていない。

## B01: Done 作成後の close 失敗から実状態表示までが試験されていない

- 根拠: Issue #5 は Done 作成後に close が失敗したとき、作成済み Issue、エラーと実際の状態を表示・再取得することを完了条件とする。`widget/src/main.test.tsx` の既存の `shows an already created issue when closing a Done issue fails` は `list_issues` が常に空配列を返し、作成済み番号とメッセージを検査するが、部分失敗後の一覧再取得と `Open` 表示を検査していない。新規試験は Done 列に置かれた Open と Closed の初期表示、および移動失敗後の再取得を検査するが、作成の部分失敗経路ではない。
- 影響: この変更の主要な動機である Issue #5 の一連の動作をテスト証拠からは確認できない。`submitIssue` の `setBoardEpoch` とボード再読込みは静的には存在するため、現時点で製品が必ず誤動作するという指摘ではなく、対象要件の回帰検証が不足しているという指摘である。
- 修正案: 作成前の一覧は空、Done 作成は部分結果を返し、後続一覧には同じ Issue 番号・status done・state open を返す合成 UI 試験を追加する。再取得・Done 列の Open 表示・作成済み番号・重複作成抑止を検証し、再取得失敗時に古い実状態を成功扱いしないことも確認する。再検証後に対象 SHA を固定し再レビューする。

## 条件と確認結果

- Rust の `BoardIssue` は `state` を渡し、一覧の Issue に `open` / `closed` 以外があればエラーとし、既存の `classify_issue` は Closed を Done とする。Rust テストは Open / Closed の値、Closed 優先、PR 除外と不正 state を検査する。API 応答への同フィールドの実環境確認は未実施。
- React はカードに `Open` / `Closed` を表示する。ボード一覧の取得成功時に `setIssues`、失敗時に配列を消してエラーを表示し、移動の成功・失敗後に再取得を起こす。新規の画面テストは Done 列の両状態を区別し、移動結果不明後の Open 表示を検査する。
- ユーザーが Closed を優先すると回答したことは `classification-decision-01.md` の確定記録を確認し、実装・README と一致する。Open + Done ラベルは Done に表示され Open と明示されるが、同一列ドロップは早期 return のため Closed に修復しない。Issue #6 は列間移動を明記しており同一列操作の要件違反とは断定しない。README は不一致時に GitHub 状態の確認を案内する。
- Rust ログは66 passed・10 ignored、URL 1 passed、Vitest ログは32 passedを示す。終了コード0は管理者の依頼記録による。ログ自体は元の終了コードの独立証拠でない。実 UI、リリースビルド、実 GitHub 書込み・資格情報および途中失敗時の実環境状態は未確認。

## 対象版

`main.rs` `620E4707A41026061B90631F3A4CAB1BC9C1FADABBBBA07E3D7D197702B4D9AB`、`main.tsx` `01A7E8A6961BE2DC4F00D96F9369098321EF5137F6959650931E36ABFB357658`、`main.test.tsx` `87E8412175F58E28E8EB32B27BFCE3FF5E6EEF39E1F78EDD53C82A633A8E1648`、`README.md` `54D113C712BA0B4B8C062172086EDFC2A27195C1F4BE9E6DF6064F1A73647D1C`。Rust ログ `9540BBB2611821DDED03D5D782E36D2A06763CBB8A10E1B1A95EBF65F30228B1`、Vitest ログ `21C8DF2B146ED2388A8085075877185736542D5F7363AE6424D05C3B5DBAD5AF`。

本確認中の操作失敗はない。
