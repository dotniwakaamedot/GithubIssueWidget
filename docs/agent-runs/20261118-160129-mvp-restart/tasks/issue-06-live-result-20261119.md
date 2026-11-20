# Issue #6 実 GitHub 移動検証

- 実行前に許可対象 `dotniwakaamedot/orcaDiversion` の新規テスト Issue #13 について、一意タイトル `github-issue-widget-test-20261119-create-01`、OPEN、`status:todo` を GitHub CLI で確認。
- 対象は Issue #6 の承認済みコミット `ab6122ef26bd62d5ec8dffbda1d35cd1a09606e0` と、独立レビュー済み ignored テスト追加 `widget/src-tauri/src/movement.rs` SHA256 `D7C987DB85F69482EF005FB95D9DA4C85DA5CBAACC2C1207B6A5F26EE5496E9F`。
- `authorized_live_move_six_directions_once` を1回実行し成功。todo→in-progress、in-progress→todo、todo→done、done→in-progress、in-progress→done、done→todo の各前後に GitHub API から取得し、状態と status ラベルを照合した。各方向でアプリと同じ `move_in_repo` を呼び出した。
- 別途 GitHub CLI で #13 が OPEN、`status:todo` に戻ったことを確認。対象 Issue と状態ディレクトリ `tasks/live-move-state-20261119-01` は保持。実行中の失敗なし。
- `gh` の認証情報を子プロセス環境で一時使用し、終了時に環境変数を復元。トークンは出力・保存していない。実 UI、Fine-grained PAT、非ステータスラベルを持つ実 Issue、通信障害や競合の実環境での再現は未検証。
