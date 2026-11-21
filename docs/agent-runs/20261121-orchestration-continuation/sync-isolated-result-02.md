# 同期隔離案 B1 修正結果

担当 `/root/project_survey`。独立レビュー `sync-isolated-review-01.md` の B1 に対応。修正前の隔離版 SHA256 は main `02EF79F6671D9F3F416B4D8B57AEBE4D52A0040932C8B2348CB14BF7E22BFD37`、test `C0E248CF39C71C34AE9A8485CB29BD8AE9FAB82EE7636D7BE8E8D22C41A54D35` と照合一致。

- 隔離 `sync-isolated-01/main.tsx` は経過時間を `Date.now()` から単調時計 `performance.now()` に変更。新版 SHA256 `6FCF2A34145450C5029B7FF80DDB3EC1DA5E27173AAB84E57F16D6D54C0E6A3E`。
- 隔離 `sync-isolated-01/main.test.tsx` は取得開始10秒後に壁時計を前後1時間変更し、開始から経過60秒で次の取得を開始する2ケースを追加。新版 SHA256 `66E118A5A7062854C23BE405D2E9578C974FA283BEA332F2A7C51DCA1BFFD13C`。テスト用偽時計の `setSystemTime` と単調時計の挙動は実行確認していない。
- 出典製品 `widget/src/main.tsx` は `01A7E8A6961BE2DC4F00D96F9369098321EF5137F6959650931E36ABFB357658`、`widget/src/main.test.tsx` は `6D1F45AFDBFC3BD66071C81FD7BA0025A186BD6A1CC0CE89022D06FB19B7D1E1` を再確認。製品ファイルや他担当の作業は変更していない。

PowerShell ハッシュ取得と `rg` による該当箇所の確認は成功。テスト・ビルド・GUI・実 API は未実行。スリープ時の時計とタイマーの挙動、実機の周期、既存接続・API制限を含む統合動作は未検証。新版の独立レビューと承認済み方式での実行が必要。作業中の失敗なし。
