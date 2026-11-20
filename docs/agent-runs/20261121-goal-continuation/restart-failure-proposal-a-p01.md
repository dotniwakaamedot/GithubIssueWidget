# Issue本文案

- タイトル: 出力量が不明な複数資料の表示を分ける
- 提案ID / 種別: P01 / 改善
- 識別キー: 20261121-goal-continuation:restart-failure-review-a.md:P01
- 提案元ファイル / 確定版SHA256: docs/agent-runs/20261121-goal-continuation/restart-failure-review-a.md / 3536DA611BFFE328A42B9C675861A714712EB8117D9144ADEEB92188E99A80B3
- 登録先: dotniwakaamedot/GithubIssueWidget

<!-- progress-review-proposal:20261121-goal-continuation:restart-failure-review-a.md:P01 -->

## 提案元・背景

F-001 発生42は複数資料の一括表示が上限を超えた。欠落部分は未確認。既存ルールは合計量の見積りを要求するが、今回の操作では徹底できなかった。

## 提案内容

優先度高。長さが不明な資料は１文書ずつ短い範囲で取得する。一括する場合は各結果の上限と包装の増加を確認する。切り詰めを減らすが呼出しは増え、分割による文脈不足に注意する。次担当は資料取得担当。

## 現在の判断と実施状況

提案中。ルール反映と効果は未確認。Issue 登録は追加操作の承認を意味しない。

## 完了条件・検証方法

- [ ] ルールを更新し、次回の複数資料取得で対象・範囲・完全性を確認する。
