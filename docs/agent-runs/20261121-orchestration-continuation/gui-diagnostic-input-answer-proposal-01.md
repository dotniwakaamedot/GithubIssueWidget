# GUI 診断版入力差分の限定確認

`gui-diagnostic-input-question-01.md` の追調査。PowerShell から Python `difflib.unified_diff` で旧固定 `main.rs` と現製品を比較、差分は末尾の `#[cfg(test)] #[path = "pagination_tests.rs"] mod pagination_regression_tests;` 追加のみ。読取りは終了0。release の通常の非 test コンパイルではこの宣言は条件から除外されるため、旧版に基づく診断から直前の旧 release 観測と比較する方針は合理的。診断バイナリと現製品版は異なり、表示結果を両者が等しい証拠としない。対象の現状態と判断は管理に委ねる。

比較対象の SHA は質問ファイルの値。既存製品・ビルド領域・診断版は変更していない。前回 `git diff --no-index` のアクセス失敗は、同じ担当の PowerShell/Python で読取れることを確認したが、原因・権限境界は未確認。
