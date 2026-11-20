# I02 進捗ファイルの文字化け調査 02

担当 `/root/build_verification`。`I02-dev-driver-preparation-question-01.md` は日本語が破損しており、正本として内容を利用しない。既存ファイルは保持する。新規問題の文字化けは今回1回であり、該当する PowerShell から Python 標準入力へ日本語を渡す書き込み経路を停止した。

## 確認した事実

- 同ファイルは 722 bytes、UTF-8 strict で読めるが、非 ASCII 文字が0個、`?` が249個。UTF-8 として読めることだけでは内容の一致を確認できなかった。
- 作成時は PowerShell 5.1 の here-string を `python -` へ pipe し、Python 内で UTF-8 encode した。調査時の PowerShell `$OutputEncoding` は `US-ASCII`、`[Console]::OutputEncoding` は `Unicode (UTF-8)`。native process への pipe の途中で ASCII 外が置換された可能性が高い。Python の UTF-8 encode より前に失われたと考えられるが、書き込み当時の環境値は直接保存しておらず厳密な因果は未実証。
- その直前に送った tool call の Python ソースには日本語の元文が残っており、そこから作者が意図を確認できる。破損ファイルのバイト列だけから置換前の文字を復元することはできない。
- 同じ方式で新規作成した `D:\Projects\GithubIssueWidget-dev-20261119-01\copy-manifest.json` と `dev-overlay.json` は ASCII のみ。JSON として解析でき、`?` は0個。SHA256 はそれぞれ `BD683EE7F754CC57DCFFE296960BB0DC3245BEBA0ED34A31548F09D95E47BEC7`、`5DD74560FEAC1C32C39BD1A5EBE8618FFC6EE7A716977A820CB6C856D416AE75`。コピーした17ファイルは bytes をそのまま読み書きし、各元・コピーの SHA と manifest が一致。日本語文字列の stdin 経由変換は行っていない。

## 元の進捗と未解決点

依頼 `tasks/I02/dev-driver-preparation-request-01.md` SHA256 `F2BDF15B70587859C13FB7235ABB22B5A87A63F39A91A2644089B6D7F0E4C98D` の8参照は実体 SHA が一致した。新規候補 root は作成前は不存在、親 `D:\Projects` に reparse 属性なし、owner は `HOME_PC01\semis`。元17件の hash と manifest が一致し、各ファイルの hard link 数は1。元 worktree の branch は `task/issue-2-foundation-20261118-160129`、HEAD は `a4ac97286785a2a5ba0d070a909b58003fc49ff1`、README の変更と widget 未追跡を保持した。専用 copy と overlay を作成済み。dev 起動、依存取得、build、GUI 操作は未実施。

実効 WebView2 profile の分離は overlay と環境変数の宣言や registry 不在だけで証明できず、既存 profile に触れず実効 directory を取得する確実な観測方法は未確定。確認できなければ成功とせず handle を保持し親へ報告する。driver は未完成、独立レビュー前の実行は禁止。

## 安全策

該当 stdin 経由の日本語書き込みを停止する。今後は Unicode を保つ別経路で同一ディレクトリの一時ファイルを新規作成し、strict UTF-8 復号だけでなく元文の重要な日本語・全内容を独立して照合し、文字置換がないことを確認してから上書きしない rename を行う。新しい経路の失敗や内容不一致は成功扱いにせず即停止する。質問01の上書きや削除は行わない。
