# 実 GitHub 接続・ボード読取り検証

- `issue-6-move-resume` HEAD `ab6122ef26bd62d5ec8dffbda1d35cd1a09606e0` と、独立レビュー済みの ignored テスト追加を含む `widget/src-tauri/src/main.rs` SHA256 `C140670F7DCD568DEF33401EFE5B409973E44347D12F41D8CBC20AA611CCE4BC` を対象とした。
- 指定リポジトリ `dotniwakaamedot/orcaDiversion` のテスト Issue #13 が OPEN・`status:todo` であることを実行前に読み戻した。
- `authorized_live_connection_and_board_read` を1回実行し成功。アプリの `verify_connection` と `fetch_pages` を使って repository ID と名前を確認し、取得結果中の #13 を一意タイトル・todo 分類・更新日時ありと照合した。その他の Issue の内容は出力していない。
- `gh` の認証をプロセス環境で一時利用し、環境変数は終了後に復元した。トークンを出力・保存していない。保存済み資格情報、実画面、Fine-grained PAT、および #13 以外の全 Issue が正確に表示されることの個別照合は未検証。
