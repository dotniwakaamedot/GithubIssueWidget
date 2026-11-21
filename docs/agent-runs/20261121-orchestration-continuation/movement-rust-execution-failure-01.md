# 移動テスト実行器の起動失敗

担当 `/root/existing_work_review`。固定版・所有・新ログ不存在を確認、元 movement を `movement-rust-test-execution-01/original-movement.rs` に保存して SHA `ED49F4BA52C6887BCE427E4A6664D335092A1C3311E1B1ED61B4CE322791ABA5` を照合した後、fixture のみ承認済み movement SHA `88BF21BB45C077DFDA592D906C753D168C5641E35EB12324D4ACF1652374695B` に差替えた。

`& 'docs/agent-runs/20261121-orchestration-continuation/movement-rust-test-execution-01/run-movement.ps1'` は PowerShell の実行ポリシーにより拒否され、`PSSecurityException`、終了コード1。スクリプトは開始されず、Cargo・テストは未実行。新しいログの生成は未確認。実行ポリシーの正確な設定・理由は未確認。同じ呼出しを再試行していない。

直前のレビュー対象はスクリプト本体であり、その起動コマンド・execution policy の扱いはこの呼出しで失敗した。過去の類似 F-関連記録との照合、失敗記録および具体的な起動方法を管理に依頼する。fixture と backup、承認済み入力を保持し、テストは成功扱いにしない。追加操作は独立した判断を待つ。
