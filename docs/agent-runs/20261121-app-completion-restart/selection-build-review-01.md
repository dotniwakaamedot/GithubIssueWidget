# 文字選択抑制版ビルドの独立レビュー 01 — 対象版を承認

- 担当 `/root/selection_build_review` はビルド作者 `/root/selection_build` と別。対象 `selection-build-result-01.md` の依頼値・実測 SHA256 は `471185044A0D807DE1AF2D8799CE0C9095D27334ABCB7A1E2E69314CB43E371D`、EXE は `0D8AB47088B7A19233EF0D7D08CF033B681B20CA0E729FEAF73EAB74DCC98F55` で一致。承認は対象版と確認した証拠に限り、blocking なし。
- 対象・入力: 修正版 CSS と設定、および隔離コピーの SHA256 はそれぞれ `FFE871A15BDA8FAD0BB5E28E1717C93D2878ABB1F5885F730EC624DEC983FCFE`、`0132FCC92345F5D60DE84FD25197FCE853044AD63DAE28757AB4592CC72B991E` で一致。Git 差分はカード・見出しの `user-select: none` と `dragDropEnabled: false` を示し、`git diff --check` は終了 0。
- 入力一覧の SHA256 は `6ADA506F08DC90354B34095B5ABA3F2E3EC47098A99FEF140D55740331A5D122` で記録と一致。Python で一覧の全 18,540 行の元とコピーの実体・SHA を再計算し、不一致 0、欠落 0、終了 0。これは一覧内の現時点の一致であり、一覧外入力の不存在やビルド中の全期間の不変性を証明しない。
- ログと終了記録、ビルド用スクリプトの SHA は作者の結果と一致。ログにフロントエンドの型検査・ビルド、隔離されたアプリの release 完了と EXE パスがあり、保存された終了値は 0。終了値は作者が保存したものであり、独立して元プロセスを監督した証拠ではない。ビルドの再実行はしていない。
- 未追跡の結果と専用領域の実体を確認した。作者は削除対象なしと記載し、本レビューも削除対象を指定しない。既存 release の全ファイルを変更前と比較しておらず、非変更の完全な独立証明ではない。

## 未確認・実施内容

Windows の実 UI における複数文字のドラッグ像、カード移動、フォーム入力・コピー、＋ボタン、リンク・キーボード操作の回帰は未確認。実行ファイルを起動しておらず、製品完成や問題解決の承認ではない。ログの macOS 向け bundle identifier 警告は Windows の実動作を示さない。

作業場所 `D:/Projects/GithubIssueWidget`。対象の `Get-FileHash`、`git diff --check`、限定 `git status`・`git diff`、ログ・スクリプトの読取り、Python の一覧照合はいずれも終了 0。Git 個人 ignore 設定の読取拒否警告はあったが、対象の状態・差分は返った。アプリ起動、外部書込み、製品・他者成果の変更は行っていない。今回認識した作業失敗はない。
