# PowerShell パーサーエラーの独立調査

## 結論

2 件はいずれも PowerShell のパース段階で発生したエラーと記録されている。後の事例は `foreach` 文を直接パイプにつないだ際の `EmptyPipeElement` である。前の事例は具体的なコマンドとエラー ID が記録されていないため、同じ根本原因かは判定できない。

## 対象と確認した事実

- 調査依頼: `tasks/powershell-parser-diagnosis-request-20261119.md`、SHA256 `C4F62EC0BB18B322DE68545FC93D3B928AAE1E7CBC84E11D4AA8CE097DBB03AA`。
- 前の事例: `tasks/plan/resume-review-20261119.md`、SHA256 `F91262019429414DFF9AF862AC2AA4316BDB213A6B111FAA950C75DE5DE1B2BD`。Issue 情報の補足集約でパイプラインの `ParserError` が発生し、集約は実行されなかったと記録されている。具体的な式とエラー ID は記録されていない。
- 後の事例: `questions/gh-quoting-parser-warning-20261119.md`、SHA256 `C2ED8F5F3175CA00DEC57F89A89FD731A3D20555B4F40959CA971DB573176C5F`。Windows PowerShell 5.1 で `foreach` 文を直接 `Format-Table` にパイプしたところ `EmptyPipeElement` となった。パース時点の失敗のため、このコマンドによるパス読み取りや GitHub 操作は行われていないと記録されている。その後、出力を変数に集めて別に処理する方法は成功したと記録されている。
- 後の事例は、同じ資料で検討された `gh` の引数引用に関する失敗とは別の現象である。

## 原因の範囲と再発防止

PowerShell の `foreach` は言語構文であり、パイプラインで入力を処理する `ForEach-Object` コマンドレットとは区別する必要がある。参照: [about_Foreach](https://learn.microsoft.com/powershell/module/microsoft.powershell.core/about/about_foreach?view=powershell-7.4)、[ForEach-Object（Windows PowerShell 5.1）](https://learn.microsoft.com/en-gb/powershell/module/Microsoft.PowerShell.Core/foreach-object?view=powershell-5.1)。

- `foreach` の結果は `$rows = @(foreach ($item in $collection) { ... })` のように先に変数へ集め、次の文で表示や加工を行う。パイプラインを使う場合は、処理対象と環境に適した `ForEach-Object` の使用を検討する。
- パースエラー時はコマンド全体が実行されていないことを前提に、集約や外部操作を成功扱いにしない。使用する PowerShell のバージョンと実際の終了結果も確認する。
- ファイルを読む前に確定した絶対パスを `Test-Path -LiteralPath` で確認し、推測したパスを読まない。

## 検証と未確認事項

- 失敗した構文は再実行していない。独立に検証した安全な代替例では、Windows PowerShell `5.1.26100.9818` で `foreach` の出力を配列に格納し、2 件の値を取得できた。
- 前の事例の元コマンドと詳細なエラーが不明なため、両者の完全な原因一致は未確認。後の事例に関する原因と結果は記録の照合に基づき、失敗構文の再現による確認ではない。

## 教訓

1. パースエラーの同一性を判断できるよう、秘密情報を含めずに対象コマンド、PowerShell のバージョン、エラー ID を記録する。
2. `foreach` 文の出力とパイプライン処理を別の文に分け、構文上の曖昧さと誤った成功判定を避ける。
