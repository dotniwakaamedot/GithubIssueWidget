# 移動後競合の部分反映通知の統合

担当 `/root/existing_work_review`。`movement-rust-integration-request-01.md`、静的レビュー `movement-rust-review-02.md` と試験結果レビュー `movement-rust-rebuild-result-review-01.md` に従い、`widget/src-tauri/src/movement.rs` へ最小差分を適用した。他担当の変更、未コミット `main.rs`、新規 pagination 試験は変更していない。

## 変更

- 最終 GET で state・ラベル等が期待と異なる時、従来の不一致・最新状態確認の案内に「更新の一部が反映された可能性があります」を加えた。成功済みと断定せず、ユーザーに GitHub の実際の状態を確認するよう促す。API の送信順・他ラベルの保持・失敗後の再取得や自動再試行しない方針は変更していない。
- 他者のステータス追加、移動先ラベル削除、state 変更を最終応答で模擬する3ケースと共通ヘルパーを追加。実際の同時更新を完全に防ぐ・検出する保証ではない。

## 対象版と確認

- 開始前の製品 movement SHA `ED49F4BA52C6887BCE427E4A6664D335092A1C3311E1B1ED61B4CE322791ABA5`、試験済み隔離版 `88BF21BB45C077DFDA592D906C753D168C5641E35EB12324D4ACF1652374695B`、main `AEFF1E65AD93EFA04773CEBBB9736A6BDC5B9938DB9FAF937DD88B3D9CBFE86F` は依頼値に一致。`git status` は当時 movement の既存差分なし、main は既存変更あり。Git はユーザー ignore ファイル読取り拒否警告を返した。
- 変更後の製品 movement SHA は **`88BF21BB45C077DFDA592D906C753D168C5641E35EB12324D4ACF1652374695B`** で試験済み隔離版とバイト単位で一致。main は `AEFF1E65AD93EFA04773CEBBB9736A6BDC5B9938DB9FAF937DD88B3D9CBFE86F` のまま。`git diff --stat` は39行追加・1行削除、`git diff --check` は終了0。削除対象なし。
- 同一 SHA の隔離入力で追加3件は `movement-rust-rebuild-result-01.md` のとおり3 passed、独立レビュー済み。同じ試験を不要に繰り返していない。今回新たなテスト・GitHub・credential・GUI 操作はしていない。

## 未確認

実 GitHub の競合・ラベル更新、実 UI、子孫全体の終了および Issue #6 全条件は未確認。統合後版の差分と証拠を別担当が独立レビューする。新たな操作失敗は認識していない。提出後は製品 movement の書込みを停止する。
