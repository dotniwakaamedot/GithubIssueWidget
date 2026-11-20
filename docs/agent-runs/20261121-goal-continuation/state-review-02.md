# Open / Closed 表示の独立再レビュー 02

判定: **対象版の B01 修正を承認**。作者 `/root` と確認者 `/root/state_review` は別担当。Issue #5 全体またはアプリ全体の完成を承認するものではない。blocking 指摘なし。

## 完了条件と根拠

- [Issue #5](https://github.com/dotniwakaamedot/GithubIssueWidget/issues/5) を GitHub から取得し、Done 作成後の close 失敗では作成済み Issue の表示、重複作成防止、エラーと実状態の表示・再取得を要求していることを確認した。
- 前回 `../20261121-resume-control/state-review-01.md` の B01 に対し、`main.test.tsx` の `shows an already created issue when closing a Done issue fails` は最初に空一覧を返し、部分成功後に同一番号123・Done・Open を返す。作成済み番号とエラー、再取得、Done 列での Open、ヘッダー作成ボタン無効と create 呼出し1回を検査する。
- `clears stale cards if refreshing after a partial creation fails` は既存カードを表示後、部分成功して一覧取得が失敗する経路を用意し、エラー表示、古いカードの消失、作成済み番号、作成ボタン無効、create 呼出し1回を検査する。
- `main.tsx` の `submitIssue` は結果保存と pending 更新後に `boardEpoch` を変え、一覧取得の effect は成功時に表示を更新し失敗時に配列を消してエラーを表示する。コードと合成テストは前回指摘の経路に対応している。
- `frontend-regression-02.log` は33 passed を示す。終了コード0は親の実行記録であり、ログ単体からは独立に証明できない。独立再実行はしていない。実 GitHub、Credential Manager および Windows 実 UI の動作は未確認。Issue #5 の他の完了条件も本レビュー対象外。

## 対象版

開始時とレビュー終了時に SHA256 を再計算し依頼値と全て一致: `widget/src/main.test.tsx` `6D1F45AFDBFC3BD66071C81FD7BA0025A186BD6A1CC0CE89022D06FB19B7D1E1`、`widget/src/main.tsx` `01A7E8A6961BE2DC4F00D96F9369098321EF5137F6959650931E36ABFB357658`、`widget/src-tauri/src/main.rs` `620E4707A41026061B90631F3A4CAB1BC9C1FADABBBBA07E3D7D197702B4D9AB`、`README.md` `54D113C712BA0B4B8C062172086EDFC2A27195C1F4BE9E6DF6064F1A73647D1C`、`frontend-regression-02.log` `23A2A2CEF504BF9E9540D5009D0337CB047D4B54A1435FC46D8C8D43674D994C`。対象4ファイルは Git 上で変更済み・削除なし、指定対象に未追跡追加なし。ログは未追跡の run 内の成果物。

## 確認中の失敗と引き継ぎ

GitHub の Issue #5 を最初に検索した際、誤って Issue 番号をタイトル検索語にしたため結果が空だった。番号で直接取得し対象を確認した。原因は検索条件の選択誤りで、既存の F-016 と同種。親に失敗記録・累計照合と独立分析の手配を依頼する。今回この確認者の同種失敗は1回。ほかの操作失敗なし。
