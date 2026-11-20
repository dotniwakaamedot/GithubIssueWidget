# I02 子プロセス終了コード・出力の無害試験設計 01

- 担当 `/root/build_verification`。依頼 `tasks/I02/process-test-request-01.md` SHA256 `EAFAFD748888ADEE32FDFD0C5B6A238E2039EB446898052E16929733ADE6332C`。依頼の固定参照4件は全て実在・SHA 一致。作業環境 `D:\Projects\GithubIssueWidget`、元 worktree `D:\Projects\GithubIssueWidget-issue-2-20261118-160129` の branch `task/issue-2-foundation-20261118-160129`、HEAD `a4ac97286785a2a5ba0d070a909b58003fc49ff1`。元製品・旧 build scratch・既存証跡は変更していない。
- 結論: 以下は **実行前の設計案**。子プログラムと driver の固定版を準備したが、子プロセス試験は一度も実行していない。独立レビューと親判断まで開始しない。元ビルドの終了コード・待機原因・Issue 完了は証明しない。

## 対象コードと実行環境

| 対象 | 絶対パスと SHA256 | 検査 |
| --- | --- | --- |
| 子 | `D:\Projects\GithubIssueWidget-process-test-20261119-01\child.cjs` / `55FFB888FAA4F5E2B93D0F806DF6892834258B15756535895BC68937C621276C` | Node `--check` exit 0。構文検査のみ、子動作は未実施 |
| driver | 同 scratch の `driver-03.ps1` / `D00C204A2DE136EC411366D540E80AD72BFC84338BE78DEB4A9B59F84876AD31` | PowerShell parser `ParseFile` エラー0、対象版を tmp から確定後再 hash 一致。実行は未実施 |
| Node | `C:\Program Files\nodejs\node.exe` / `F5109DC868EC38703AD8F305E0ACEECE717CB60D4889873F434017AF4E96CCCB` | v26.13.1。開始前に版と hash を再確認 |

PowerShell 5.1.26100.9818 / .NET Framework 4.0.30319.42000 で `ProcessStartInfo.RedirectStandardOutput`、`StandardOutputEncoding`、`StreamReader.ReadToEndAsync`、`Process.WaitForExit(int)` の存在を reflection で確認、exit 0。利用可能な API の存在のみで実行成功は未証明。旧版 driver.ps1 と driver-02.ps1 も scratch に保護し、実行対象にはしない。

## 方式と制約

1. 固定版 driver が `ProcessStartInfo.UseShellExecute=false` の `System.Diagnostics.Process` を保持し、Node 実行ファイルの絶対パスと固定の子ファイル・列挙された case 引数だけを渡す。cmd の文字列連結や以前失敗した `node -e` は用いない。子の環境変数はクリアして Windows の SystemRoot と専用 scratch の TEMP/TMP だけを渡す。資格情報や環境全体をログに記録しない。
2. 起動後に stdout/stderr をそれぞれ `ReadToEndAsync` で**同時に**読み、`WaitForExit(int)` の結果、PID と UTC を保持する。実子が終了した場合のみ `ExitCode` と実 `ExitTime` を読み、両ストリーム task の完了を最大5秒確認し、EOF が不明なら成功としない。ASCII 出力の UTF-8 バイト数と末尾マーカーを個別に判定する。Process の終了と EOF は別に記録する。
3. 出力は所有 scratch 内の case 別 `*.stdout.log`、`*.stderr.log`、`*.result.json`、timeout 時のみ `*.timeout.json`。事前に確定名・同じ名前の `.tmp` が未存在か検査し、CreateNew・flush/close・バイト一致・非置換 rename・読戻しを行う。各試験は直列で、既存ファイルや他者資産を上書き・削除しない。実行前後にスクリプト・Node exe と結果の SHA を取得する。
4. この Node 子コードは `node:fs` の標準出力・標準エラー書込みと timeout の timer のみを用い、外部通信・ポート・GUI・子孫起動・ファイル書込みを行わない設計。driver と output は専用領域のみを書込み、OS の通常ログ等は残り得る。コードの静的確認は環境や実行時の全副作用を保証しない。
5. 各 case は別起動し、処理資源は flood で stdout/stderr 各 524,296 ASCII bytes、双方で約 1 MiB とメモリ上の読取文字列・一時複製を見込む。非 timeout の終了待ちは20秒、終了後の EOF 待ちは5秒。これらは自動強制終了の期限ではない。Node バイナリ・driver・child などが変更、他者との衝突、出力先不明、ハンドル断絶や開始成否不明なら停止して管理へ質問し、同一 case を再起動しない。

## ケースと検証期待値

| case 引数 | 固定子の動作 | 期待する exit code | stdout / stderr 期待 byte | timeout |
| --- | --- | --- | --- | --- |
| `normal` | `OUT_END\n` / `ERR_END\n` | 0 | 8 / 8 | なし |
| `nonzero` | 同上、意図的な非0 | 7 | 8 / 8 | なし |
| `flood` | 4,096 byte を128回ずつ交互書込み、各末尾マーカー | 0 | 524,296 / 524,296 | なし |
| `timeout` | `WAITING\n` 出力後、120秒経過で末尾マーカーを出し自然終了 | 0（終了後のみ取得） | 16 / 8（終了後のみ取得） | 500 ms 時点で必ず記録 |

各実行案（実施前に担当・資源・hash と出力先を再照合）:

```powershell
& 'C:\Windows\System32\WindowsPowerShell\v1.0\powershell.exe' -NoProfile -File 'D:\Projects\GithubIssueWidget-process-test-20261119-01\driver-03.ps1' normal
& 'C:\Windows\System32\WindowsPowerShell\v1.0\powershell.exe' -NoProfile -File 'D:\Projects\GithubIssueWidget-process-test-20261119-01\driver-03.ps1' nonzero
& 'C:\Windows\System32\WindowsPowerShell\v1.0\powershell.exe' -NoProfile -File 'D:\Projects\GithubIssueWidget-process-test-20261119-01\driver-03.ps1' flood
& 'C:\Windows\System32\WindowsPowerShell\v1.0\powershell.exe' -NoProfile -File 'D:\Projects\GithubIssueWidget-process-test-20261119-01\driver-03.ps1' timeout
```

プロセスの終了コード、実終了時刻、終了観測 UTC、driver のプロセス結果、stdout/stderr EOF、各バイト数・末尾、ログと子・driver・Node の SHA を検証者と独立レビュー担当が照合する。意図された非0はケースの成功条件だが、他の失敗を0や成功に置き換えない。EOF・出力・期待値不一致は未成功として記録。

## timeout の扱い・停止条件

- timeout 時点で `timeout.timeout.json` に PID、開始 UTC、timeout 観測 UTC と exitCode null を保存し、driver は同じ Process handle を保持したまま標準入力で管理判断を待つ。子は120秒後に自然終了し得るが、driver は管理判断なしに待機を解除しない。コンソール入力が閉じた場合も handle を保持して待つ。親が同じ handle と子の終了を確認し `continue` を指示した後のみ exit code / ExitTime・EOF を取得する。子がまだ稼働中の `continue` は状態を変更しない。停止・強制終了を自動では行わない。
- 正常・非0・flood でも20秒を超えた場合は同様に `*.timeout.json` と handle を保持し、成否不明として管理へ質問。独立レビューで不安全と判断された場合や競合・予期せぬ出力が起きた場合は実行しない。
- 同種の問題が初回を含め2回生じたら当該操作を止めて別担当に客観調査を依頼し、3回目は全作業中止。直近の直接確定名への公開逸脱は2件として維持し、同種3件目で全停止。拒否されたレビュー記録の公開を再実行・回避しない。

## 未確認と成果・教訓

Node の子が上記出力・終了挙動になること、.NET の process/stream API・タイムアウト制御の実行可能性、ツールの標準入力維持は未確認。具体コード・対象・専用領域を別担当がレビューし、親が明示的に判断するまで起動しない。test の結果は実行後に新規 `tasks/I02/process-test-result-01.md` へ証跡・未実施・削除対象なしを記録し、別担当が対象 SHA と独立に照合する。

この依頼下でコマンド失敗なし。検査コードは ASCII で保存、既存の質問01直接公開は独立調査へ `questions/I02-build-question-13.md` で報告した。
