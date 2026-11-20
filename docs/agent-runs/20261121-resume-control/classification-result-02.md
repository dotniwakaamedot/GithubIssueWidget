# Closed 優先の結果 02

- ユーザーは Closed の Issue を Done に表示する案を選択した。`classification-decision-01.md` に質疑と判断を記録した。
- `README.md` SHA256 `54D113C712BA0B4B8C062172086EDFC2A27195C1F4BE9E6DF6064F1A73647D1C` に分類順と不一致時の確認方法を明記した。`widget/src-tauri/src/main.rs` SHA256 `A3BAA6E82701F6D80DADBB688C22F0C54A466B358647F3D285EF9F4FF4100B56` に Closed と3つの status ラベルを組み合わせたテストを追加した。製品関数の差分はない。
- 対象テストは終了コード0・1 passed。独立確認 `classification-review-02.md` は対象版に限り承認した。Open で Done ラベルが付いた Issue は Done に表示され、同じ列へのドロップでは Closed に修復できない。要件上の違反とは未確定で、README は GitHub 上での確認を案内する。
- リリース EXE SHA256 `82670A9107F85FC645D1D7813A355288BD8A1CB816AAE2D4C9AC2152796ACD6B` はテスト追加前の `main.rs` SHA256 `021488C4C1615EBEFA3185EBDF024A43547657E357715957128F403541A67A97` を使用し、ビルド終了コード0。差分は `#[cfg(test)]` 内のテストのみだが、完全同一ファイルのビルドではない。Windows で設定画面の表示を確認し、終了時のボタン操作は F-017 のため失敗、Alt+F4 後にウィンドウの消失を確認した。実 UI 上の分類と実 GitHub での操作は未検証。
