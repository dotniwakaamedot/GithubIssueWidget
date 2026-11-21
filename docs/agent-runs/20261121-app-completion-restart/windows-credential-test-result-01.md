# Windows 資格情報ライフサイクルの事前確認

## 結論

**実 OS の資格情報試験は実施しなかった。** 指定の既存テストは一意の合成 service を使うが、書込み前にその UUID を外部へ記録しない。試験プロセスが作成後・削除前に異常終了すると削除対象を確実に識別できず、依頼の実行条件を満たせない。テストの成功・失敗や実資格情報の状態は未確認である。

## 対象・確認

- 担当 `/root/windows_credential_test`。作業ディレクトリ `D:/Projects/GithubIssueWidget`、branch `main`、HEAD `4961b25bec53a93439acdf5230de77f959908af0`。HEAD は未コミット・未追跡ファイルを識別しない。
- 確定依頼 `windows-credential-test-request-01.md` SHA256 `F84EC51202D1FE79E964BC93CF526E91B937FA8A62EF6337CEE0D23572C4FE9E`。対象 `widget/src-tauri/src/main.rs` SHA256 `AEFF1E65AD93EFA04773CEBBB9736A6BDC5B9938DB9FAF937DD88B3D9CBFE86F` で依頼値と一致した。
- `main.rs:1551-1567` の ignored test は `github-task-widget-test-<UUID>` とユーザー名 `synthetic`、固定の合成値を使用し、書込み・読取・更新・削除後の `NoEntry` を確認する。実アプリの `github-task-widget` と service 名は異なる。実 OS の credential に書き込む試験である。
- service はテスト内でランダム生成され、正常時には記録されない。削除が失敗した場合の assertion は service を表示するが、削除処理に到達しない異常終了の場合は識別情報が残る保証がない。固定の合成値であっても実資格情報領域に残留し得る。UUID の事前予約、書込み前登録、監督者停止時の識別・復旧証拠は確認できない。
- 依頼は「削除先を識別できない」場合に実行しないことを要求するため、この条件下では `cargo test` を実行せず、新規 sandbox も作成していない。ユーザーの実 PAT・既存 service・設定・アプリ・GitHub には触れていない。
- 限定した既存証拠 `docs/handoff-20261121-current.md`、`tasks/03/security-test-result-01.md`、`tasks/03/ipc-test-result-01.md` には当該テスト名の一致がなかった。これは全履歴の成功証拠が存在しない証明ではない。既存 `tasks/03/security-result-01.md` は当該試験の存在を説明し、成功を証明していない。

## 操作と結果

- `Get-Location`、`git branch --show-current`、`git rev-parse HEAD`、対象2ファイルの `Get-FileHash -Algorithm SHA256` は終了 0。
- `rg -n -C 18 -F 'fn isolated_windows_credential_lifecycle' widget/src-tauri/src/main.rs` は終了 0、上記実装を確認した。
- 既存証拠3ファイルに対するテスト名の限定 `rg -n -m 5 -F` は終了 1、該当文字列なし。検索の対象は明示した3ファイルのみ。
- 製品ファイル・試験・既存設定・資格情報・GitHub の書込みなし。試験・ビルド・アプリ起動は未実施。認識した操作失敗はない。

## 次の作業と残る制約

書込み前に合成 service 名を専用ファイルへ排他的に予約・記録し、その同じ識別子だけを試験と終了後の状態確認に使える監督方法を設計する。作成前、登録後、書込み後、削除中、監督者停止や異常終了の場合を区別し、対象の識別・所有権・終了・残留確認を独立レビューしてから再実行を判断する。単に既存試験を再実行してはならない。実 OS での保存・更新・削除、実アプリの PAT 保存・再起動復元・実 IPC は未検証のまま。

本担当の成果はこの結果ファイルのみ。書きかけは同じディレクトリの `.tmp` を使用し、UTF-8 実データ読戻し後に確定名へ公開する。結果自身の SHA256 は自己参照を避け公開後に管理が取得する。削除対象なし。結果確定後は変更しない。
