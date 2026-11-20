# Issue本文案

- タイトル: 既知の Issue 番号を直接指定して取得する
- 提案ID / 種別: P03 / 改善
- 識別キー: 20261121-goal-continuation:repeated-failure-review-a.md:P03
- 提案元ファイル / 確定版SHA256: docs/agent-runs/20261121-goal-continuation/repeated-failure-review-a.md / F844309B52BAE7A0F666C28EBCED1CB125B971EA251091455FB996CB5C02CDD2
- 登録先: dotniwakaamedot/GithubIssueWidget

<!-- progress-review-proposal:20261121-goal-continuation:repeated-failure-review-a.md:P03 -->

## 提案元・背景

`state-review-02.md` によれば Issue 番号をタイトル検索語にして空となり、その後番号の直接指定で対象を確認した。空の結果は不存在を意味しない。

## 提案内容

優先度中。既知のリポジトリと Issue 番号は番号指定で取得し、空や失敗は検索対象・権限・取得方法を確認する。検索意味の取り違えを減らす。ツール引数の確認費用があり、取得失敗を不存在と誤認しない。次担当は Issue 取得担当。

## 現在の判断と実施状況

提案中。効果は未確認。Issue 登録は追加操作の承認を意味しない。

## 完了条件・検証方法

- [ ] ルールを更新し、次の既知番号による取得で対象番号と結果を照合して記録する。
