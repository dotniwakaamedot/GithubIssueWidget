# 実 OS 合成資格情報試験の独立調査

担当 `/root/credential_live_failure_investigation`。作業場所 `D:/Projects/GithubIssueWidget`。依頼は実行記録の読取りと診断方法の調査に限定し、実資格情報の照会・書込み・削除、試験や診断の再実行は行っていない。

## 結論

対象 run `dcdc5dfdf4524ffa8d9139ee5a781ae4` の監督側記録は直接子の `completed`・終了コード 1、journal は `before_write_a` を最終段階としている。合成エントリの書込みが行われたか、残留しているかは未確認。子の stderr は破棄されており、終了 1 の直接原因は特定できない。**再実行や削除を行わない。**

## 根拠と切り分け

- 実行担当の `credential-live-execution-result-01.md` は実測 SHA256 `7C72FB0497934AE43A92015BDD38FA36BBD011807FF2A8B0AFA7B45E5429E5ED` で親の指定と一致。監督側が直接子を wait した結果を報告し、親子孫全般や OS 障害の無条件保証とは区別する。
- `credential-live-ready-02/src/main.rs` SHA256 `214DEF1642AAA6BE30F4E5A5C95CEF3E62083084EAC617527E9A10DC6C509F1B` は NoEntry 確認の後に `before_write_a` を同期し、A 書込みを呼んだ後に `before_read_a` を記録する。最終 stage のみでは API 呼出し前の停止、書込みエラー、書込み成功後の journal 更新エラーを区別できない。`set_password` のエラーは一般化され、監督側は stdout/stderr を `DEVNULL` にしている。後続 stage 不在と exit 1 は成功や失敗の直接原因を証明しない。
- `supervisor.py` SHA256 `D0D9EFDDC4812B9D9F22C2A3E714F8C491825F3DCC386F4DADA385C2D047421D`、`recovery.py` `04A3911C622E09B1416962C388972443041916D6E06A14C0FB7F01646B86A302`、`RECOVERY.md` `F2F2D33AA2B3BC4CDC057384E38F2FBCF1BDAE10199D70946A5868C9A5B47E45` は実行記録と一致した。journal SHA256 は `777351B63A7951B145A692B6C6D36CCB17F3439C910D3211D27756FE2BD3B392`、outcome SHA256 は `DF9BD782C56927353B2B66BAC2B0CB819B4010C4F3FC69A3E0830D38ECDFD439`。
- 実行ディレクトリで `python -B` により `recovery.read_identity` だけを呼び、終了 0、`identity_validation=passed`、`state=completed`、`last_stage=before_write_a` を得た。これはローカルの one-shot・start・child・outcome・journal・EXE の識別整合をコードの条件で確認するだけで、Windows credential API は呼ばず、実際の対象値・残留・ユーザーの既存資格情報の状態は確認していない。`-B` により bytecode を生成しない。

## 安全な次の判断

`RECOVERY.md` と対象 `recovery.py` では、識別証拠と直接子の wait 済み outcome を照合した後、親が明示的に判断した場合に限り `python recovery.py --inspect dcdc5dfdf4524ffa8d9139ee5a781ae4` で、記録された合成 target のみを１回読取り、値を出さず absent／synthetic_A／synthetic_B／other／error に分類し、create-new の開始・結果記録を作る。実 OS の読取りと記録書込みを伴うため本担当は実施していない。呼出し前に対象版・記録の不変性、終了根拠、既存 inspect 記録の不存在、作業権限と停止条件を再確認し、失敗や error は不存在とみなさず再試行しない。`CredReadW` の実環境動作と分類は未確認。

現在の最終 stage `before_write_a` は書込みの戻り値が確定した `before_read_a` ではなく、`cleanup` の許可 stage に該当しない。inspect が synthetic_A を返したとしても既存 cleanup は拒否し、推測で別の削除や再送をしない。absent は読取り時点の対象不在に限り、other／error／不明は状態未解決として保護する。別操作が必要なら対象の所有・プロセス終了・値の扱い・read/delete 間の非原子競合を評価し、専用手順と独立レビューを経て親が判断する。実 PAT・実アプリ・WebView の要件はこの合成試験で証明できない。

## 実行・失敗

`Get-FileHash`、対象範囲を限定したソース読取り・`rg` と上記 `python -B` は終了 0。run 全体のファイル名を最初に列挙した表示は内側上限で１回切り詰められ、欠落箇所は未確認。その後は run ID を指定した限定列挙に切替えた。同種２回目は認識していない。共有の失敗記録は所有範囲外のため親へ引き継ぐ。既存ファイルや資格情報を変更・削除していない。
