# I02 before/after 件数誤表示に関する回答 06

- 作成 `/root/build_verification`、宛先 `/root/path_failure_review`。`real-build-result-01.md` と `I02-real-build-execution-question-02.md` に記録された rustup の before/after 件数の読取について、失敗操作は再実行していない。
- 当初の正確な該当コードは `$before=@(Get-Content -Raw -Encoding UTF8 -LiteralPath (Join-Path $root 'rustup.before.json')|ConvertFrom-Json)` と、ファイル名・変数名だけを `rustup.after.json`・`$after` に替えた同じ構文。`$root` は `D:\Projects\GithubIssueWidget-build-20261119-02`。`$before.Count`・`$after.Count` は各1と表示され、期待する製品ファイル件数は各17だった。
- その後、`$a=ConvertFrom-Json -InputObject (Get-Content -Raw -Encoding UTF8 -LiteralPath (Join-Path $root 'rustup.before.json'))` と after を直接代入して `.Count` を確認し、各17件であることと、両方の全17行が元 manifest のパス・hash と一致することを照合した。結果ファイルの内容は変更していない。
- 当該コマンドを実行した shell は PowerShell 5.1 系。近接した環境確認では `5.1.26100.9818` と記録されているが、件数の各コマンド自体はバージョン出力を含まない。元の2回のコマンドと出力は会話上のツール実行記録にあり、専用の生ログファイルには保存していない。読み取った JSON の実体は上記 scratch 内に保持され、各 SHA256 は `D747AA1C2D1BF9A2ACAC9FD0CE55712062B58B714EEFCE4B6F3E796D56CBEDF8`。
- 仕組みとして配列の入れ子が示唆されるが、原因の客観判定は独立調査担当に委ねる。表示の違いを成功判断として扱わず、正しい17件と hash を別途照合してから進行した。
