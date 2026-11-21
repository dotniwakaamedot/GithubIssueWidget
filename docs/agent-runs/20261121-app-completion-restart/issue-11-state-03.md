# 検証 Issue #11 の状態と移動履歴確認 03

公開 GitHub REST の観測では #11 は **closed**、ラベルは `status:done` のみで、非 status ラベルはない。イベントからは Todo→In Progress、In Progress→Todo、Todo→Done、Done→Todo、In Progress→Done の5方向に対応するラベル変更が見える。**Done→In Progress に対応するイベントは観測されず、イベントだけでは全6方向の完了を証明できない。** ユーザーの操作報告と API で観測した履歴は区別する。

- 担当 `/root/issue_state`。依頼 `issue-11-state-request-02.md` SHA256 `43BC76323607DC6ACC0D309D11944A16478BFD7CABA63E03D404784164FCC270`。作業ディレクトリ `D:/Projects/GithubIssueWidget`、branch `main`、HEAD `4961b25bec53a93439acdf5230de77f959908af0`。HEAD は未追跡・未コミットを識別しない。
- `GET /repos/dotniwakaamedot/GithubIssueWidget/issues/11` は HTTP 200。観測日時 `2026-11-21T06:11:47Z`、`updated_at` と `closed_at` はともに `2026-11-21T06:10:06Z`。
- `GET /repos/dotniwakaamedot/GithubIssueWidget/issues/11/events?per_page=100&page=1` は HTTP 200、20件、Link ヘッダーなし。今回の公開 API 応答では1ページで終端。表示は status ラベルと close/reopen に限定した。
- 時系列上、05:39:56–57 に todo→in-progress、05:40:36–37 に in-progress→todo、05:46:09–11 に todo→done と close、05:46:17–19 に done→todo と reopen。06:08:54–55 に todo→in-progress、06:09:04–05 に in-progress→todo、06:09:55–06:10:06 に todo→in-progress→done と close が観測された。時刻は UTC。ラベル追加と削除が別イベントであるため、この対応は隣接イベントに基づく読み取りであり、UI の各ドラッグの原子的記録ではない。
- 検証: Python 標準ライブラリで匿名 HTTPS GET、プロセス終了コード0。合成リンクの終端、正しい次ページ、不正ホストの3ケースを先に検査した。実応答に継続リンクはなく、実際の後続ページ取得・途中通信失敗の試験はしていない。本文・資格情報は表示・保存していない。別々の GET の間や取得後の同時変更、匿名利用者に見えない情報、API が記録しない操作は未確認。
- 変更は本結果ファイルと、版を質問した `issue-11-state-question-01.md`（SHA256 `EB3B21063B165F2EE2012D0AA442B9C04D5A6AB035D2A3196905742C8B9D8C60`）。本結果の SHA256 は自己参照を避け確定後に管理が取得する。削除対象なし。Issue・ラベル・資格情報・設定・Git を変更せず、認識した失敗なし。
