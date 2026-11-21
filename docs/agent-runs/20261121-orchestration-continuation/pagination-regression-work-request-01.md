# 大量ページ回帰試験の隔離実装依頼

- 担当 `/root/issue_survey`。共有 run `D:/Projects/GithubIssueWidget/docs/agent-runs/20261121-orchestration-continuation/`。新規 `pagination-isolated-01/` と結果 `pagination-regression-result-01.md` のみを所有。既に存在する場合は上書きせず中止。製品・Git・他担当の変更を戻さない。
- 承認済み `pagination-regression-plan-02.md` SHA256 `336EF903F20C0719D529751C675A88522AB6721F817C08DD124381D0738CD463` と `pagination-regression-plan-review-02.md` を読む。製品 `widget/src-tauri/src/main.rs` SHA256 `61595B68317E15FDD8F2BED88B920A468D5516740E3D3124710384709C52FC24` を開始前に照合し、未コミットの基準が異なれば変更しない。
- 対象を独立コピーし、PR・重複除外後に固有 Issue 1,001件以上となる複数ページ、番号集合・分類・重複・PR除外と後続ページ失敗の合成回帰を追加。期待値は実装の写経にせず、ページ枠・next・有限待機と終了方法を明示。計画の切断/期限超過の扱いも守る。
- 本依頼は隔離コード作成と静的な検査まで。合成サーバー・Cargo 等の試験実行は、固有の具体方式と対象版の独立レビュー・開始通知後に行う。ソース本体や既存 target、GUI、GitHub、資格情報には触れない。
- 変更・未追跡ファイルの SHA、元ファイルの前後 SHA、差分、実施結果・未検証・有限待機方式を結果に記録。確定後に書込みを停止しパスを通知し、別担当のレビューを受ける。
