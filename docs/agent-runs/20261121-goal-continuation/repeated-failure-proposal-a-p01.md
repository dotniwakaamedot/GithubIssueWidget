# Issue本文案

- タイトル: 制約付き試験の実行経路を事前に確認する
- 提案ID / 種別: P01 / 改善
- 識別キー: 20261121-goal-continuation:repeated-failure-review-a.md:P01
- 提案元ファイル / 確定版SHA256: docs/agent-runs/20261121-goal-continuation/repeated-failure-review-a.md / F844309B52BAE7A0F666C28EBCED1CB125B971EA251091455FB996CB5C02CDD2
- 登録先: dotniwakaamedot/GithubIssueWidget

<!-- progress-review-proposal:20261121-goal-continuation:repeated-failure-review-a.md:P01 -->

## 提案元・背景

F-073 の単一プロセス Job 合成試験が２回終了コード1となり、２回目には Python 起動子プロセスの生成失敗が表示された。Job 制限と起動経路の関係は仮説であり、原因は未確定。

## 提案内容

優先度高。制約適用前に実行ファイル、ランチャーと想定子プロセス、期待結果、診断経路を確認する。制約への適合性は合成環境と実際の対象で区別して検証し、安全性を自明視しない。手戻り低減が期待されるが、確認の費用がある。次担当は進行管理・ハーネス担当。

## 現在の判断と実施状況

提案中。ルール追記と試験は未確認。Issue 登録は追加の外部操作の承認を意味しない。

## 完了条件・検証方法

- [ ] 対象のルールを更新し、次の制約付き合成試験で事前確認と対象実行ファイルごとの結果を記録する。
