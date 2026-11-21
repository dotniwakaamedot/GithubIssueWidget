# Windows IPC 合成試験の隔離実行経路の作成依頼

- 担当予定 `/root/windows_ipc_adapter`。親 `/root` が担当 ID を確認して起動。共有 run `D:/Projects/GithubIssueWidget/docs/agent-runs/20261121-app-completion-restart/`、作業場所 `D:/Projects/GithubIssueWidget`、基準 HEAD `4961b25bec53a93439acdf5230de77f959908af0`。他者が共有作業中であり、その変更を戻さない。
- 根拠: Issue #3、`windows-ipc-feasibility-result-01.md` SHA256 `0DF47E994AFE406EDB2B3C42E7C64B79CD2B2C68138889A632DBE45701148ACA` と独立レビュー `windows-ipc-feasibility-review-01.md` SHA256 `E9C015A0D50A8EDD3FED91AADCDE627104C6BE21DBAF11C13CDA1D8CF6B677C3`。対象本番 `widget/src-tauri/src/main.rs` SHA256 `AEFF1E65AD93EFA04773CEBBB9736A6BDC5B9938DB9FAF937DD88B3D9CBFE86F` を開始前に照合し違えば停止・質問。
- 目的: 既存 `windows_ipc_guard` と mock Tauri IPC の実装を活用し、新規専用領域 `windows-ipc-adapter-01/` の隔離コピーだけに、実 OS 試験のための最小限の識別・監督経路を作る。本番ファイルは変更しない。合成 service と account の完全な識別子を書込み前に新規記録、flush・耐久同期・読戻しして不一致なら書込みを禁止し、既存 service や実 PAT を対象としない。直接子・終了状態・期限・不明時の停止、対象値一致条件と削除後確認、残留時の識別・復旧手順を明示する。競合防止・排他が成立しない場合の停止条件を実装・記録し、UUID や存在確認のみを原子的所有権の証明としない。
- 今回は **実 Credential Manager・実資格情報・実アプリ・GitHub に触れず、OS 書込みを明示的に無効** とした模擬経路だけを実行。保存前の記録失敗・衝突・異常終了・応答不明・子の終了不明・cleanup 対象不一致・正常保存更新削除の代表ケースを確認し、結果・終了コード・対象版・ログ・SHA を記録する。過剰な追加フレームワークを避け、実 OS・実 WebView・実 PAT の証拠と混同しない。
- 実 OS 試験はこの依頼に含まない。成果について独立レビューし、実環境の所有権・排他・復旧・監督を別途確認した後に親が実施可否を判断する。現実的に安全条件が成立しない場合は無理に動かさず具体的な不足を記録する。
- 所有は `windows-ipc-adapter-01/` と新規 `windows-ipc-adapter-result-01.md` のみ。既存コード、資格情報、Git、Issue、他者のビルドや素材を変更・削除しない。実行前に対象・子プロセス・出力先・終了方法を確認し、秘密をログ出力しない。新規結果と変更ファイル・削除対象・失敗・未確認・検証結果を記載し、UTF-8 読戻し後に一時ファイルから確定、パスのみ通知する。同種失敗２回は停止して親へ報告。
