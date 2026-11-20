# 権限不足と隔離競合の独立レビュー

- 担当: `/root/snapshot_review`。対象記録 `permission-execution.md` SHA256 `AC32F7D7936731EA22F0994798F524075133201671E958CB86A8222702C30F6C`。
- 判定: **指定版のローカル HTTP テストと隔離競合テストの結果を限定承認**。通常の権限不足 403 と二次レート制限 403 の両方で、合成 token と Issue 名を含む本文が返却エラーへ露出しないことを対象テストは確認する。実 UI・実 GitHub やアプリ全体の完成を承認するものではない。

## 対象と証拠

| 対象 | 実測 SHA256 |
| --- | --- |
| `widget/src-tauri/src/main.rs` | `5DE67CA065A45C3637B4DD0755D6ED9846E57C903ED93099214E9C03A16C0022` |
| `widget/src-tauri/target/debug/deps/github_issue_widget-263288f3cb360ecd.exe` | `1A4DD972A85C5135286313427436B8B7320E9E4DDAEDFB60FF41E056226B5339` |
| `%TEMP%/githubissuewidget-permission-tests-20261120.log` | `F5518D054C374393F4D39EBD0B1AE00B94AC84DC2788B932100756D2A42A1003` |
| `%TEMP%/githubissuewidget-permission-isolated-20261120.log` | `027C56517F1E86A80AECCE148222222F134DFCC0BC79D2B283759C5C3CFBF656` |

開始時と終了時の SHA は親の実行記録と一致。通常テストログに当該テスト `ok` と `49 passed / 6 ignored`、隔離競合ログに `1 passed` と正常・旧応答拒否、credential cleanup の stage がある。ソース更新後に実行ファイル、通常テストログ、隔離競合ログの順で更新されている。ログ自体にソース SHA は含まれず、親の記録する終了コード 0 は本担当が直接観測していない。

## 確認結果

- 対象テストは 401 / 通常の 403 / 二次レート制限の 403 / 404 / 429 / 500 の合成本文に合成 token と Issue 名を入れる。通常 403 は `HTTP 403`、二次レート制限 403 は `レート制限` を含むことを分けて確認し、両者を含む各エラーに合成値がないことを確認する。ローカルの HTTP と proxy 無効クライアントを使う。対象コードの該当箇所は読み取りで確認した。
- 隔離競合ログの正常・旧応答拒否 stage は対象コード上、子と server / changer 終了・各結果確認後に出力される。cleanup stage は合成 credential 2件の値照合・削除・削除後不存在確認後に出力される。記録された子 PID `30768` は本レビュー時点で存在しなかった。Credential Manager への直接照合は行っていない。
- 両ログを文字列照合し、対象の合成 token 値・合成 Issue 名が含まれないことを確認した。実トークンや未知の個人データが全ての経路に露出しないことを、これだけで保証するものではない。

## 未確認とレビュー中の失敗

- `fetch_pages` 経由の合成応答に限定した結果であり、実 GitHub の権限・レート制限、その他の API 応答、実 WebView と Rust の結合、`save_connection`、最終照合後の別プロセス変更は未検証。timeout・panic・cleanup 失敗も本成功ログでは実行されていない。
- レビュー中、`Select-String -Context` の結果を重複出力したため表示が上限で切り詰められた。`questions/permission-output-01.md` に報告し、親から別担当の独立分析と F-001 記録済みの連絡を受けた。同じ検索は再実行していない。該当テストの必要な行は表示された範囲で確認し、欠落部分を根拠にしていない。
- テスト再実行、GUI、Credential Manager・外部サービスへの操作、製品ファイル変更は行っていない。
