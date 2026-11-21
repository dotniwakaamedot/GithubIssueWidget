# 検証 Issue #11 の状態確認

公開 GitHub REST の番号指定 GET で #11 は **open**、ラベルは `status:todo` のみと確認した。非 status ラベルは取得時点でない。作成から本観測までのイベント一覧は `status:todo` の追加1件のみで、移動や close のイベントは観測されなかった。ただし UI の操作内容や失敗原因を証明するものではない。

- 担当 `/root/issue_state`。依頼 `issue-11-state-request-01.md` SHA256 `00075E1A830E9570BE7112184E8536F6EF772735CEBD4C812C015F51AC078995`。
- 作業ディレクトリ `D:/Projects/GithubIssueWidget`、branch `main`、HEAD `4961b25bec53a93439acdf5230de77f959908af0`。HEAD は未追跡・未コミットの状態を識別しない。
- リポジトリ `dotniwakaamedot/GithubIssueWidget`。Issue の `created_at` と `updated_at` はともに `2026-11-21T05:11:05Z`、`closed_at` は null。
- `/issues/11/events?per_page=100&page=1` は HTTP 200、JSON 配列1件、Link ヘッダーなし。`labeled`、`status:todo`、`2026-11-21T05:11:06Z`。公開 API の観測範囲と観測時点に限定し、同時更新や API 対象外の履歴は保証しない。Issue 本体とイベントの GET は別リクエストであり原子的なスナップショットではない。
- 検証: Python 標準ライブラリ `urllib.request` で Issue 本体とイベントを GET し、両 HTTP 200、プロセス終了コード0。必要な状態と日時だけを出力し、本文・資格情報を表示・保存しなかった。`git branch --show-current`、`git rev-parse HEAD`、依頼の `Get-FileHash` は終了コード0。
- 未確認: 実 UI のイベント到達・ドラッグ不能の原因、操作後に発生した変更、実資格情報の権限。推測で再送・Issue 更新を行っていない。
- 変更: 本結果ファイルのみ。未追跡を含む結果ファイルの SHA256 は自己参照を避け、確定後に管理側で取得する。削除対象なし。認識した失敗なし。Issue・ラベル・資格情報・アプリ・Git は変更していない。
