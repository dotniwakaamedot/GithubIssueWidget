# ランチャー検証失敗の回答

担当 `/root/interactive_launcher`。質問 `launcher-failure-question-01.md` に対し、同担当のツール呼出し記録と当時の出力を基に記す。実 credential や本番 supervisor は実行していない。

## 合成 preflight の正常ケース

- 元コマンドは workspace ルートから `python 'docs/agent-runs/20261121-app-completion-restart/launcher-synthetic-01/test_launcher.py'`。処理は終了1、表示は `success: FAIL exit=1 reached=False`。当時の Python が子 PowerShell の stdout/stderr を捕捉したが、失敗時に内容を保存・表示しない実装だったため、元の子エラー出力は残っておらず直接原因は未確認。元の最初のテスト対象は `launcher-synthetic-01/success/candidate.ps1` に残っている。
- 初回ハーネスの PowerShell 呼出しには `-ExecutionPolicy Bypass` がなく、実際の cmd は同引数を付与する。`Get-ExecutionPolicy -List` は全スコープ Undefined だった。規定 Restricted による拒否は候補であり、失われた stderr がないため確定できない。
- `Get-Item` と `Get-FileHash` による偽 fixture と Python 実体の限定確認は終了0、fixture の各 SHA は当時の candidate の期待値と一致。PowerShell Parser は対象候補の構文エラー0、Python バージョン検査は終了0。
- ハーネスだけを本番 cmd の呼出しと同じ `-ExecutionPolicy Bypass` を付けるよう変更し、未使用 `launcher-synthetic-01/second-run/` に対して同コマンドで実行したところ、７合成ケース成功・終了0。変更後の `test_launcher.py` SHA256 は `39CA97CB65549EB419E076FFC3E28E5CC912072D14D0C79801DF4025C0B67944`。この成功だけでは初回の直接原因を証明しない。元のツール出力は会話に記録され、個別ログファイルは保存していない。

## Read-Host の独立した確認コマンド

- 元コマンドは PowerShell から `python -c` を呼び、Python subprocess の PowerShell `-Command` に `$x` を `\u0024x` の文字列として渡した確認操作。表示は `exit= 1`、外側の `exec_command` は終了0。子 stderr を表示・保存していないため元エラー表示の全文は未確認。`\u0024x` は Python の raw string 中にあり `$x` へ復元されないため、不正な引用・式の構成が原因候補であるが、子 stderr 不在につき詳細な直接原因を確定しない。
- 別操作として ASCII PowerShell here-string から Python に `$x=Read-Host; if ($x -ceq 'YES') {exit 0} else {exit 2}` を渡し、stdin に `YES` を与えたところ、子 exit0、stderr 空、外側終了0。これは入力読取りの限定確認であり最初の失敗表示を復元したものではない。

両事象は別の操作・発生条件で各１回。共有失敗記録の編集権限は親・記録担当にあり本担当は変更しない。同種２回の発生は認識していない。元コードと他者成果を戻さず、秘密を取得していない。
