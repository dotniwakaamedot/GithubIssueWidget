# Issue本文案

- タイトル: 外部一覧の配列形状と安全な失敗診断を確認する
- 提案ID / 種別: P02 / 改善
- 識別キー: 20261121-goal-continuation:restart-failure-review-a.md:P02
- 提案元ファイル / 確定版SHA256: docs/agent-runs/20261121-goal-continuation/restart-failure-review-a.md / 3536DA611BFFE328A42B9C675861A714712EB8117D9144ADEEB92188E99A80B3
- 登録先: dotniwakaamedot/GithubIssueWidget

<!-- progress-review-proposal:20261121-goal-continuation:restart-failure-review-a.md:P02 -->

## 提案元・背景

F-075 は一覧取得失敗の診断情報が失われた後、配列を１件と誤集計した。F-064 に配列の処理を誤った既存記録があるが、内部原因の同一性は未確認。

## 提案内容

優先度高。合成例で配列の階層・件数とページ結合を確認し、実応答では終了結果と全ページの取得完了を別々に確認する。失敗時は資格情報・応答本文を出さず安全に選別した例外種別や HTTP 状態を記録する。誤集計の低減が期待できるが、診断情報の選別と試験には費用が必要。次担当は GitHub 検証担当。

## 現在の判断と実施状況

提案中。効果は未確認。Issue 登録は追加操作の承認を意味しない。

## 完了条件・検証方法

- [ ] ルールを更新し、合成の２件以上の配列、ページ失敗と空結果を確認する。
- [ ] 秘密を露出せず取得完了・失敗の根拠を確認する。
