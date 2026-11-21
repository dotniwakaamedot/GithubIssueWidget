# 同期 UI 回帰試験の独立レビュー 01 — 対象版を承認

- 担当 `/root/sync_regression_review` は作者 `/root/sync_regression_test` と別。依頼対象 `sync-regression-test-result-01.md` の期待・実測 SHA256 は `5BD3B33CEB1484A5A4BFCBD8C1CCA26F4B790E57B6BECBBC0BF6914EC687E527` で一致。承認は当該結果版と以下の確認範囲に限る。blocking なし。
- 元・隔離コピーと証跡: `inputs.tsv` SHA256 `31EB08A122A9BA7E3048D6FD366BB6F42DE3DA900396EF670A2C432E1E2D5148`、`outputs.tsv` `AFFDBBC5C12B6C657BECAECB211818057B0D32D4BF1630649EEA51942288D3AA` は記録と一致。前者 4,983 行の元・コピー両方、後者 4,991 行の成果実体を独立に SHA256 再計算し欠落・不一致 0、終了 0。対象 `main.tsx`、`main.test.tsx`、package・lock も一覧内の対象版に対応する。一覧に存在しない入力や実行期間全体の不変性は証明しない。
- 試験と結果: `runner.py` SHA `A25B4D96AB2C18B9DDC320C50AD4993ED6CCFCE561FD2101061B7D3C59516DFE` の引数配列と正規表現、対象テスト宣言を照合。旧接続の遅延エラー、開始から60秒、時計変更の2例、速い失敗後の回復、unmount 後の応答抑制の5宣言・6ケースに対応する。対象テストは模擬 IPC と仮想時計を使用する。保存ログ SHA `74C1210F14BA4E73C62E22C63A4089F8AAA78B7F88BA409739DB257F09A6C960` に 6 passed、35 skipped を確認。終了記録 SHA `6BE7E3591253C9D77DBB29E86536ABB77CB32B21922A88ABD2804FA4EC3F01E6` は completed・exit_code=0。ログは個別の試験名を表示せず、終了値は作者が保存したもので元プロセスを本担当が独立監督した証拠ではない。
- 実ファイルと未追跡: 限定 `git status` は結果と隔離領域を未追跡と表示し、対象製品4ファイルには差分を表示しなかった。結果は削除対象なしと明記。対象レビューにも削除指定はない。workspace 全体の削除・差分を網羅した確認ではない。

## 未確認・失敗

- 対象外35件、実 WebView・実 GitHub の通信障害・Windows の時計変更挙動は本試験の証拠外。試験の再実行、実アプリ起動、資格情報操作、外部書込みはしていない。ユーザーが後に報告した実同期確認は本合成試験とは別の証拠である。
- Issue #4 のコネクタ番号指定取得を試みたが引数が不正と返り、本文の直接取得は未完（1回、再試行なし）。結果の要件対応は依頼と既存承認計画の範囲に限り、今回のレビューは Issue #4 全体の承認ではない。失敗記録・集計を管理担当へ引き継ぐ。Git の個人 ignore 設定読取りは権限警告を出したが限定 status は終了 0。
- 作業場所 `D:/Projects/GithubIssueWidget`。実施は対象限定の `Get-FileHash`、Git status、対象ソース・ログ読取りと Python SHA 比較であり、試験・ビルドの再実行なし。対象結果や製品は変更していない。
