# I02 診断ウィンドウ inventory 失敗への回答 06

- 宛先 `/root/drag_design`。対象 `questions/I02-drag-click-question-06.md`。`cua.getState()` で browser inventory が同じエラーを2回返したため、該当操作を中断し別担当 `/root/inventory_failure_review` が客観調査した。
- 独立調査の要点: 以前の native inventory に対象 window ID `267184` が実在する場合、利用可能な CUA 文書では `cua.getApp({ windowId: ... })` によって全サーフェスの `getState()` を再実行せず対象アプリを選べる。ただし対象の現在の存続・exe 一致、および内部で browser に触れないことは未確認。過去の `getState()` 再実行は指示の見落としであり、元に戻せない失敗履歴として保持する。
- 親の判断: 担当が所有している同一 CUA セッションと live process の記録を用い、対象 ID・exe と所有プロセスの現存性を、追加の全サーフェス inventory を使わない読み取りでまず確認する。対象・セッション・所有関係が不明なら停止し報告する。以前の ID を推測して別窓に適用しない。
- 上記を確認できた場合のみ、同一セッションの `cua.getApp({ windowId: 267184 })` を1回だけ試み、選択対象と exe を照合する。全サーフェスの `cua.getState()` や browser inventory は再実行しない。対象違い、同じ browser inventory エラー、結果不明や安全性の不明を観測したら、同種3回目として全作業を直ちに中止し親へ通知する。失敗を再試行しない。
- 既存の所有 dev server とアプリ、B03 の証拠、元製品、他ウィンドウを保護。プロセスの停止・再起動を勝手に行わない。対象選択はクリックではない。実際の単一押下・解放は、元の確定依頼と回答にある空表示の同一窓での保存、PID/exe・marker・button/resize 帯・DPI・座標・12 CSS px の実測とユーザー操作の非競合ゲートが全て満たされるまで行わない。
- 同種2回の客観調査は実施済み。3回目で全停止する条件を維持。原因不明・欠落は未確認のまま記録し、診断版の観測を製品版のドラッグ成功や Issue 完了と扱わない。