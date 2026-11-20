# Issue本文案

- タイトル: ツール探索と長行検索の出力を具体的に制限する
- 提案ID / 種別: P02 / 改善
- 識別キー: 20261121-goal-continuation:repeated-failure-review-a.md:P02
- 提案元ファイル / 確定版SHA256: docs/agent-runs/20261121-goal-continuation/repeated-failure-review-a.md / F844309B52BAE7A0F666C28EBCED1CB125B971EA251091455FB996CB5C02CDD2
- 登録先: dotniwakaamedot/GithubIssueWidget

<!-- progress-review-proposal:20261121-goal-continuation:repeated-failure-review-a.md:P02 -->

## 提案元・背景

F-001 発生40・41でツール説明と文書の合算、長い JSX 行の出力が切り詰められた。既存ルールは件数・行長・合計の見積りを求めるが、当該処理では不十分だった。

## 提案内容

優先度中。ツールは先に名前を列挙し必要な定義だけ取得し、長行検索はファイル・行番号と行長を確認して短い必要箇所を個別に表示する。出力超過による欠落を減らす。確認回数が増え、文脈不足に注意が必要。次担当は検索担当。

## 現在の判断と実施状況

提案中。効果は未確認。Issue 登録は追加操作の承認を意味しない。

## 完了条件・検証方法

- [ ] ルールを更新し、次回のツール探索と長行検索で必要な文脈と表示の完全性を確認する。
