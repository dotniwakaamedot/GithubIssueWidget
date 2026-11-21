# Issue #11 非ステータスラベル保持の照合

- 担当: `/root/label_retention_check`。作業場所 `D:/Projects/GithubIssueWidget`、基準 HEAD `4961b25bec53a93439acdf5230de77f959908af0`。HEAD は未コミット・未追跡の内容を識別しない。
- 観測時刻: 2026-11-21 15:30 UTC 頃。ユーザーは #11 に `enhancement` を追加後、アプリで Done → In Progress → Done と移動し、`enhancement` が残り closed と報告した。その後、追加したラベルを外し `status:done` と closed が残る旨を報告した。実画面を本担当が直接観測したわけではない。
- GitHub コネクタの `fetch_issue` は番号指定で成功し、#11 は `closed`・`status:done` のみ、`enhancement` は付いていなかった。`closed_at` は `2026-11-21T15:27:32Z`、`updated_at` は `2026-11-21T15:28:05Z`。この状態は取得時点に限る。
- 匿名 GitHub REST の `/repos/dotniwakaamedot/GithubIssueWidget/issues/11/events?per_page=100&page=1` は HTTP 200、配列48件、Link ヘッダーなし。観測した公開応答では１ページで終端。`enhancement` の追加は `2026-11-21T14:10:27Z`、削除は `2026-11-21T15:28:05Z`。同じ一覧には `status:in-progress` 追加 `15:27:23Z`、`status:done` 削除 `15:27:24Z`、reopen `15:27:25Z`、`status:done` 追加・`status:in-progress` 削除 `15:27:31Z`、close `15:27:32Z` がある。これらは本人のラベル保持・移動・復元報告と整合する。イベントは別操作で記録され、アプリの内部処理や原子的な一括更新を証明しない。
- イベント取得用の限定 Python スクリプトは正常終端と、短いページにも次リンクがある例、不正なホストを合成チェックした。実応答に次リンクはない。途中失敗・循環の合成試験は今回未実施であり、ページング一般の完成を主張しない。2回の GET は各終了コード 0、両方とも HTTP 200、Link なし。連続リクエスト間・取得後の同時更新、匿名利用者に見えないイベントは保証しない。コネクタと REST の取得は別で原子的なスナップショットではない。
- 製品・Issue・ラベル・資格情報・Git は変更していない。結果ファイルのみ新規作成。削除対象なし。今回認識した失敗はない。内容は UTF-8 として実データを読み戻し、確定後の SHA256 は親が取得する。
