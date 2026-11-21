# 移動テスト起動の回復案

`movement-rust-execution-failure-01.md` の起動失敗に対し、既存の成功記録 `pagination-regression-execution-result-01.md` は、同じ環境で `powershell.exe -NoProfile -ExecutionPolicy Bypass -File <実行器>` を使用したと記載する。これはその PowerShell プロセスに適用する指定であり、恒久設定の変更は提案しない。ただし今回の runner に対する正確な起動コマンドを含む確認・認可は未取得。

当該 runner は未起動・Cargo とテスト未実行なので、既存ログ不存在、各対象 SHA と保存済み元ファイルを再照合し、同方式の起動が適切と管理・独立確認が判断した場合に限り `powershell.exe -NoProfile -ExecutionPolicy Bypass -File <run-movement.ps1 の絶対パス>` を1回実行する案。機械の設定・権限は変更せず、以降は依頼された3件・停止条件に従う。実行前に別の問題が判明した場合は中止。現時点では提案のみ。
