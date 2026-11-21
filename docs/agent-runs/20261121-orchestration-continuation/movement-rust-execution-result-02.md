# 移動競合テスト: 0件収集

担当 `/root/existing_work_review`。`movement-execution-policy-investigation-01.md` と管理の再開指示を受け、固定入力・保存元・実行器と新ログ不存在を再照合した。前回の PowerShell 直接呼出しはポリシー拒否でスクリプト・テスト未起動。

今回 `powershell.exe -NoProfile -ExecutionPolicy Bypass -File D:\Projects\GithubIssueWidget\docs\agent-runs\20261121-orchestration-continuation\movement-rust-test-execution-01\run-movement.ps1` を1回実行。ランナーは `direct_exit=0 deadline_exceeded=False` と報告したが、stdout は **`running 0 tests`、0 passed、79 filtered out**。3件を収集できていないためテストは未実行・未達であり、終了0を成功と扱わない。stderr は `Finished test profile` と test binary 起動を示す。ログはそれぞれ114・280 bytes。

直接の原因は未確定。Cargo が変更済み fixture の movement を再コンパイルせず既存バイナリを再利用した可能性、フィルター・モジュール名の不一致などを区別する必要がある。確認のため同じテストを再実行していない。コピー元が元日時を保持した可能性も仮説にとどまり、独立担当によるファイル時刻・依存追跡・対象バイナリと実行経路の読取り分析を依頼する。既存 target を消去せず、ログと fixture を保護する。

## 版・証拠

- 実行前後の fixture `project/src/movement.rs` SHA `88BF21BB45C077DFDA592D906C753D168C5641E35EB12324D4ACF1652374695B`、`project/src/main.rs` `AEFF1E65AD93EFA04773CEBBB9736A6BDC5B9938DB9FAF937DD88B3D9CBFE86F`。製品 movement は `ED49F4BA52C6887BCE427E4A6664D335092A1C3311E1B1ED61B4CE322791ABA5`、製品 main は `AEFF1E65AD93EFA04773CEBBB9736A6BDC5B9938DB9FAF937DD88B3D9CBFE86F`。
- stdout `movement-rust-test-execution-01/movement.stdout.log` SHA `B3218D6252BB99365764959EA2634DCD30FB8AF15496ED7DC7FD6761DCDE5331`、stderr `movement.stderr.log` `E68F302B3C649520CEACCA3010D34C72760C87C6385967617C19C6747AFAD4F4`。ランナーの直接 cmd は終了0、期限超過なし。未知の子孫全体の終了は未確認。
- 実 GitHub、credential、GUI、旧プロセスには触れていない。製品と既存 pagination のログ・ソースは変更せず、新ログ・専用 target は保持。後続への書込み・再試行は管理判断と独立分析を待つ。失敗記録と過去の0件収集との照合を管理へ依頼する。
