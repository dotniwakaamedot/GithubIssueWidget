# 合成資格情報の残留確認の独立レビュー

- 担当 `/root/credential_residue_review` は検査作者 `/root/credential_residue_inspect` と別。対象 `credential-live-residue-inspect-01.md` の期待・実測 SHA256 は `603D8C1BC3BB8ABADA07E8B38EE0882CE978ED16ADC285E2E84635A1500C3A01` で一致。依頼は親からの会話で受領した。未追跡の対象実体を確認した。
- **対象版の記録を承認、blocking なし。** 以下の証拠に限定し、最初の保存成功、終了コード1の原因、実アプリの認証機能や製品完成を承認しない。

## 完了条件と照合

1. **対象の一致:** 実 journal の SHA256 は `777351B63A7951B145A692B6C6D36CCB17F3439C910D3211D27756FE2BD3B392`、service は `github-task-widget-synthetic-check-82e61bb1-c162-4a83-b28a-0d2cfafd2ebd`、account は `synthetic`、PID は 8708、末尾 stage は `before_write_a`。結果の記述と一致した。`recovery.py` と `supervisor.py` の実 SHA256 も結果に記載された値と一致し、`read_identity` は run、marker、start、child、outcome、journal、対象 EXE の SHA・PID・service UUID を照合し、`synthetic.<service>` を target とする構造である。実行前の照合結果そのものは作者の記録であり、本レビューでは実 credential API を呼び出していない。
2. **読み取りと分類:** `recovery.py` の `--inspect` 分岐は `CredReadW` を対象１件に呼び、Windows エラー 1168 のみを `absent`、それ以外の失敗を `error` と分類する。`inspect` は開始・結果の記録を専用ディレクトリに新規作成するが、credential の書込み・削除は `--cleanup` 側にある。結果に記載された inspect outcome の実 SHA256 は `3A80D847BCB0F95219182F196A7304DCC72C4FAF9D21D2AD2323075E9D774982`、実 JSON は対象 run、同一 journal SHA と `classification: absent` を記録する。作者は終了0と記録したが、その終了コードを独立監督した保存証拠は本レビューで確認していない。
3. **時点と限界:** outcome ファイルの最終更新時刻は `2026-11-21T12:26:40.1656360Z` と観測した。これはファイルシステム上の時刻であり、Credential API の厳密な呼出し時刻の独立証明ではない。`absent` は対象・観測時点の分類に限り、将来の状態や Windows ユーザー全体に拡張できない。journal は保存前 stage で停止、元 outcome は子 `completed`・exit 1 を記録する。保存が未実行、失敗、または一時的に成功したか、直接原因はこの証拠から判別できない。
4. **変更・削除:** 対象結果と inspect outcome の未追跡実体を確認。作者は本結果と inspect 記録以外に変更なし、削除対象なしと記載する。レビュー依頼にも削除対象指定はない。workspace 全体の全変更・削除の不存在は本レビューでは確認していない。

## 未確認・作業記録

- 実 credential の再読取り・元試験の再実行・削除・外部書込みはしていない。`absent` の確認は保存済み outcome と対象コードの照合であり、OS API を本レビューが直接監督した証明ではない。PID 再利用、同時変更、失われた stderr と作者操作時点の終了コードの独立証明も未確認。
- 対象限定の `Get-FileHash`、`Get-Content`、`rg --files`、`rg -n`、`git status --short` は終了0。Git の個人 ignore 設定はアクセス拒否警告を出した。作業中に新たに認識した失敗はない。対象成果物・製品・資格情報を変更していない。
