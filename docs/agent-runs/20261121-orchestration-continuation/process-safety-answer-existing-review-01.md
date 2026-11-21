# 操作失敗の回答

- 2026-11-21、`Get-CimInstance Win32_Process` のプロセス照会はアクセス拒否で終了コード1。後に `Get-Process` で名前・PID を確認したが親子・パス・終了状態は未確認。詳細は `../20261121-issue-completion/existing-work-survey-20261121-01.md`。既存 ID への記録は管理担当に依頼したが、この担当は記録の有無を確認していない。
- 同日、`Measure-Object -Line -Maximum -Property Length` はパラメーターセットが曖昧となり終了コード1。後続は配列の Count と `Measure-Object -Property Length -Maximum` で行数・最大長を取得。詳細は `output-failure-independent-01.md`。既存 ID への分類・記録を管理担当に依頼したが、記録の有無は未確認。
