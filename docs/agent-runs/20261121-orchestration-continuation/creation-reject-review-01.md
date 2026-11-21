# 作成応答喪失時の再取得修正レビュー

- 確認者 `/root/plan_review`、実装・試験担当 `/root/project_survey` と別担当。親の指定対象 main.tsx `B22504C9DE48D275726C912820DA93E02F10C1838CCC68555C156D35030165D7`、main.test.tsx `10AA28AD8D343F969C8BDB8C2D2232BF6007CB926619CB49850020DD53A40FBD` は開始・終了時に一致。削除対象なし。
- 判定: **固定版の限定差分を承認**。blocking なし。実 GitHub の POST・UI・アプリ全体の完了承認ではない。

作成 IPC が reject した後、pending があるか読み取れない場合だけダイアログを閉じ、既存の `boardEpoch` による一覧取得を開始する。pending が存在すると作成ボタンが無効になり、読み取り失敗時は `pendingReady=false` で無効になる。pending がない確定的失敗では従来どおりダイアログ・エラーを保つ。既存の `creatingRef` と pending 等のガードがあり、変更自体は作成 IPC を再送しない。Rust の `creation.rs` では POST 前に pending 記録を書き、結果不明時に保持する構造を静的確認した。保存状態が実際に維持されるかは実環境・障害時の別検証が必要。

追加の1テストは pending あり・読取不能・null の3分岐で、前2つは一覧が更新され作成ボタンが無効、null はダイアログが残り余計な一覧更新がないことを検査し、各分岐で `create_issue` IPC が1回だけと確認する。これは UI モックであり、実 HTTP POST の回数を直接計測したものではない。`creation-reject-test-01/logs/stdout.log` SHA `AFD3D21876182FC77D2E8BD40090090B59FA924BE9F0897022A5AAF6C5934D64` は1 passed・40 skipped と完了を表示、stderr SHA `E3B0C44298FC1C149AFBF4C8996FB92427AE41E4649B934CA495991B7852B855` は空。実行 fixture の main/test は現行製品の指定 SHA と一致。対象選択・直接終了0は実行者の報告に依存し、独立した確定結果ファイルは本レビュー時に提示されていない。

`git diff` で新たな変更が catch 時の条件付き一覧更新と新規テストであることを、先行同期・移動テスト差分と区別して確認した。実 GitHub の応答喪失・実 UI・実認証・子孫終了や統合後の製品ビルドは未確認。成功済み同一ケースの再実行は行わず、ビルド対象を更新する場合は現行固定版を反映する。

`D:/Projects/GithubIssueWidget` でコード・差分・テストログ・実行 fixture を読取り SHA を照合。テスト・GUI・外部書込み・製品編集は未実施。対象が変われば再レビューする。
