# 失敗記録の隔離準備と担当指定

- 担当 `/root/process_safety`。共有 run `D:/Projects/GithubIssueWidget/docs/agent-runs/20261121-orchestration-continuation/`。製品変更や実行・外部書込みなし。対象経緯は `failure-record-request-01.md`、`failure-record-answer-01.md`、`output-failure-independent-01.md`、`output-failure-second-agent-01.md` と、追加の別担当分析を参照。
- 現在の共有ファイルの版は `docs/failure-log-001.md` SHA256 `3C6E6A21BF10270A5917C5FE12386935E276BE81DD22A64FFE37B8EC1DA3FFDD`、`docs/recurring-lessons.md` SHA256 `78A83DCAE885F26A4107EAB29476E68014B31D1CCACF9BF15448EEB36E006F54`。どちらも既存の未コミット変更があり、当該変更の旧担当が他セッションで書込み停止したかは証明されていない。現時点で本体を上書きしない。
- 進捗を止めず、本 run の新規かつ固有 `failure-isolated-01/` に両ファイルを複写し、上記版と複写版の SHA 一致を確認する。一致しない・領域が存在する場合は上書きせず中断して通知。複写だけを編集し、各失敗・既存記録との重複・反復分析・既存ルールの適用・効果未確認を正確に反映する。改善提案の有無を区別し、他者の変更を削除・上書きしない。
- 本 run に `failure-record-result-01.md` と隔離コピー以外は所有しない。結果には出典と変更後 SHA、追記対象・重複判断・操作結果・未確認を記録し、確定後に書込みを止めてパスのみ通知する。独立レビューと旧担当の停止・所有権確認後、親が本体への適用を判断する。実ファイルへの適用は別依頼とする。
