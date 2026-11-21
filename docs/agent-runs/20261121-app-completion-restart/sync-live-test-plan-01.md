# 60秒同期の実機確認案

## 結論と前提

既存の本人報告は #11 のアプリ操作による移動と再取得を示すが、アプリ外での変更が手動更新なしの定期同期で反映された証拠ではない。#4 の60秒同期の実機確認は未完了である。本人が公開 `dotniwakaamedot/GithubIssueWidget` の検証用 #11 のみを GitHub 画面で変更し、アプリを開いたまま待つ方法を提案する。実施は親が既存承諾・権限と現在状態を再確認してから案内する。本人のトークンや Issue 本文を聞かない。

対象は本人が利用した `selection-build-sandbox-01/cargo-target/release/github-issue-widget.exe` SHA256 `0D8AB47088B7A19233EF0D7D08CF033B681B20CA0E729FEAF73EAB74DCC98F55`。`issue-11-state-04.md` は観測時点で #11 が closed・`status:done`、他ラベルなしを記録する。実行時は GitHub 上の現在値を本人が再確認し、違う・不明なら作業を止める。手順は既存の「検証 Issue 1件を移動し最後に close」承諾の範囲として提案するが、GitHub 画面からの一時的な reopen とラベル変更の扱いは親が最終確認する。別 Issue・PR、資格情報、共有ラベル定義を変更しない。

## 根拠とタイマーの限界

対象 `widget/src/main.tsx` SHA256 `B22504C9DE48D275726C912820DA93E02F10C1838CCC68555C156D35030165D7` の 103–121 行では、ボード取得を開始し、終了後に `max(0, 60000 - 経過時間)` を待つ。通常は取得**開始から**60秒ごと、60秒以上かかった取得は完了後に次回を開始する。処理は重複しないが、実通信や負荷によって反映時刻の上限は保証できない。設定画面の開閉、接続切替、手動更新、アプリ内作成・移動は再取得・タイマー再設定を招くため観察中は行わない。失敗時は一覧を空にしてエラーを表示する。

`widget/src-tauri/src/main.rs` SHA256 `AEFF1E65AD93EFA04773CEBBB9736A6BDC5B9938DB9FAF937DD88B3D9CBFE86F` の 91–97 行では closed はラベルにかかわらず Done、open は todo→in-progress→done のラベル順、該当なしは Todo。GitHub の reopen とラベル変更は別操作なので途中に不整合があり得る。閉じたまま in-progress にしても Done に表示され、open かつ `status:in-progress` のみになった後の取得で In Progress を期待する。変更途中の中間表示は正常・異常の判定に使わない。

Issue #4 は60秒同期と旧接続の遅い応答を、#7 は実環境統合を要求する（番号指定の公開 REST GET、各 HTTP 200）。この手順は通常同期１例であり、接続切替競合、通信・レート制限、100/1000件超の実 GitHub を証明しない。

## 本人が行う最小手順案

1. 修正版を起動し、接続先が `dotniwakaamedot/GithubIssueWidget`、#11 が Done に表示、GitHub でも #11 が closed・`status:done` のみと確認。接続先・書込み権限・ラベルが不明、同期中表示が終わらない、一覧エラーがある場合は変更せず停止。GitHub 側の #11 ページを別ブラウザで開き、アプリは一覧のまま維持する。手動更新しない。
2. GitHub の #11 だけで `status:done` を `status:in-progress` に交換し、Issue を reopen する。順番による一時不整合は想定内。各操作が成功したことを GitHub の表示または状態読取りで確認し、最終状態が **open・`status:in-progress` のみ**になった時刻を記録する。途中で失敗・結果不明なら再送や次操作をせず、GitHub の実状態を読み直して親に報告する。前提と異なるラベルは勝手に削除しない。
3. 最終状態を GitHub で確認した時刻から、アプリの設定・手動更新・作成・移動・再起動を行わず、同期中表示が終わることを見ながら最大120秒程度観察し、#11 が In Progress に現れた時刻、表示されたエラーと同期中の有無を記録する。直前に開始済みの取得が変更前の状態を返しても、その次の取得を待つため120秒は目安である。取得が長い・エラー・反映しない場合は推測で成功やバグ確定にせず、待機時間と画面の状態を報告し停止する。同期の自動更新そのものが新しい状態を示した場合のみ当該一例を成功とする。
4. 観察成功後、アプリ内で #11 のみを In Progress → Done にドラッグし、GitHub で **closed・`status:done` のみ**に戻ったことを確認。観察失敗時も手動再送を重ねず実状態を確認し、復旧の操作を親と判断する。外部変更の応答不明・状態不一致では既存 Issue を触らず停止。終了時に最終状態、各時刻、成功・失敗・未実施だけを報告し、秘密や他 Issue 内容を共有しない。

## 対象・実施・未確認

- 担当 `/root/sync_live_plan`、作業場所 `D:/Projects/GithubIssueWidget`、branch `main`、HEAD `4961b25bec53a93439acdf5230de77f959908af0`。HEAD は未コミットを識別しない。依頼 `sync-live-test-plan-request-01.md` SHA256 `5770726F56EEEDC6919148A43275879DD62EE186EB4CB74E43A3896E3C30C7B8`。`docs/handoff-20261121-current.md` SHA256 `9D855AF53D956D0CA0F58A92F1B294428B00DFCE216579F14E9D37C719F756B5`、`issue-11-state-04.md` `44F629CB8B2E14ECB0FB8FC82A541D17A97022F4BD549734BB82D4ABAC5D2DF3`、`selection-build-result-01.md` `471185044A0D807DE1AF2D8799CE0C9095D27334ABCB7A1E2E69314CB43E371D`。本人報告 07/08/09/10 の SHA256 は順に `F2B593C7B5093AA225F4DCF85395D117C902319ACDA3FEAF847ACD09391BBC67`、`261EC6EDAF2A30209D4646FB7FE28CEB4DBE4D2847E5A980DDFAACCF3EF5B483`、`83D7461C97B278354C3480361738D8AD6841DAC873B7E709E1F11CFD3037A896`、`AF186CDF5DF6533FD1A74E56A3C5BB1C0CBA7FA5CB38F90C21B7535772741548`。
- 対象限定 `Get-Content`、`rg`、`Get-FileHash`、`git branch --show-current`、`git rev-parse HEAD`、公開 REST 番号指定 GET は終了0。Git 個人 ignore 設定は読取拒否の警告を出したが限定 status は終了0。長い検索行はツールによるプレビュー省略があり、その行を根拠とせず該当範囲を別途読んだ。実機・実 GitHub 書込み・実資格情報操作はしていない。記録すべき失敗は認識していない。
- 新規成果は本ファイルのみ。確定版の SHA は自己参照を避け公開後に管理が取得。削除対象なし。他者の変更は変更していない。実施前の #11 の状態・認可・本人の権限、GitHub UI、同期中の画面・通信状態、実試験結果は未確認。
