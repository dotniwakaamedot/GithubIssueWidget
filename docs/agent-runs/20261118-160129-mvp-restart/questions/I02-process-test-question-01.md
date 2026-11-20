# I02 無害な Process 試験の準備状況 01

- 担当 `/root/build_verification`、宛先 `/root`。依頼 `tasks/I02/process-test-request-01.md` の4参照は存在・SHA 一致。依頼 SHA256 `EAFAFD748888ADEE32FDFD0C5B6A238E2039EB446898052E16929733ADE6332C`、担当 ID 一致。
- `D:\Projects\GithubIssueWidget-process-test-20261119-01` を未存在・親の通常ディレクトリを確認して作成。無害子プログラム `child.cjs` SHA256 `55FFB888FAA4F5E2B93D0F806DF6892834258B15756535895BC68937C621276C` を tmp から確定。正常0、非0=7、stdout/stderr を各 524,296 bytes、120秒で自然終了する timeout ケースのコードを用意したが、いずれも未実行。
- 読取で PowerShell 5.1.26100.9818 / .NET Framework 4.0.30319.42000、`ProcessStartInfo.RedirectStandardOutput`・`StandardOutputEncoding`、`StreamReader.ReadToEndAsync`、`Process.WaitForExit(int)` の API 存在を確認。API の実行成功ではない。
- これから保持する Process と同時非同期読取・timeout 保留・ログ確定を記述した driver の固定版と計画を作成し、実行前の独立レビュー・親判断を待つ。製品・旧 scratch・他者資産の変更、子プロセス起動・試験はしていない。
- この担当の今回の依頼下でのコマンド失敗はなし。前回の質問01の直接公開について独立調査への補足は `questions/I02-build-question-13.md` に記録した。
