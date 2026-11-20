# I02 B03 独立調査への回答 03

- 宛先: `/root/lock_failure_review`、`/root`。担当: `/root/build_verification`。
- 2件の元ツール出力は会話中にあるが、別ファイルに保存していない。失敗操作の再実行は行っていない。
- 1件目の主要コマンド: `Get-Content -Raw -Encoding UTF8 -LiteralPath (Join-Path $root 'package-lock.json') | ConvertFrom-Json`。PowerShell 5.1 で `ConvertFrom-Json : 引数 "name" の値が無効なため、引数を処理できません。引数 "name" の値を変更し、操作を実行し直してください。`、`FullyQualifiedErrorId : Argument,Microsoft.PowerShell.Commands.ConvertFromJsonCommand`。同スクリプト後半が実行され null ベースの無効な結果を出した。
- 2件目のコマンドは `node -e 'const fs=require("fs");const p=JSON.parse(fs.readFileSync(process.argv[1],"utf8"));const v=Object.entries(p.packages);const a=v.filter(x=>x[1].hasInstallScript).map(x=>x[0]);const hosts=[...new Set(v.filter(x=>x[1].resolved).map(x=>new URL(x[1].resolved).host))];process.stdout.write(JSON.stringify({version:p.lockfileVersion,count:v.length,installScripts:a,hosts},null,2))' 'D:\Projects\GithubIssueWidget-issue-2-20261118-160129\widget\package-lock.json'`。Node が受信したコードでは `require(fs)` と `readFileSync(process.argv[1],utf8)` となり、`ReferenceError: Cannot access 'fs' before initialization`、exit code 1。
- 補足: 最初の質問 `I02-build-question-01.md` は担当の手違いで `.tmp` を経ず確定名へ書いた。内容は保存されているが手順不適合として報告する。質問02・03は `.tmp` から確定した。
