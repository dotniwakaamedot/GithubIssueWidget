# Issue #3 復旧処理の部分レビュー

- 判定: **R01 の修正方針と対象版の静的な部分確認に限り承認**。Issue #3 全体・実機動作の承認ではない。作者 `/root`、確認 `/root/connection_review` は別担当。HEAD `59578f8a4a775b4bd66c6acdde318b2e934abc08`、作業場所とブランチは依頼どおり。
- 修正版依頼 `issue-03-recovery-review-request-20261119-02.md` の8ファイルは SHA256 が全件一致。`git status --short --untracked-files=all` で依頼どおりの変更7件と未追跡テスト1件を確認。削除はない。古い SHA 転記誤りの依頼は対象にしていない。

## R01 の対応

- `main.rs` の `commit_settings` は新資格情報を書き込む前に旧・新 ID の復旧記録を保存し、設定変更後に復旧処理を呼ぶ。復旧は現在の設定 ID と記録を比較し、参照中でない世代だけを削除する。Credential Manager の削除成功前に復旧記録を消さず、記録の読取失敗・不一致や削除失敗をエラーにする。起動時の `connection_info`、保存、トークン削除は、プロセス内・ロックファイルの排他を取得してから未完了の復旧を試みる。設定参照の前後で終了した場合に未参照世代を検出できない、という前回 R01 のコード経路は対象版では是正された。
- `recovery_handles_interruption_on_both_sides_of_commit`、`recovery_survives_deletion_failure_and_can_be_retried`、`journal_failure_prevents_writing_credentials`、別プロセスのロック試験と HTTP 応答試験のソースを確認した。UI テストのソースは、設定エラーが無関係な最前面操作で消えない場合を確認する。これらは作者の実行結果の独立再現ではなく、ソースの存在・内容の静的確認である。
- 異常終了前に復旧記録が存在しない、旧版ですでに発生した孤立した Credential Manager エントリは、新方式では発見できない可能性がある。旧版実行で孤立エントリが生じた証拠は本レビューでは確認していない。新方式で実行された保存についても、実ストア、OS 異常終了、電源断時の永続性は未検証。

## 非 blocking の改善候補

1. `obsolete_generation` は両 ID の UUID 形式を検査するが、`old_id == new_id` を拒否しない。通常の新 ID は UUID v4 で生成されるため通常の衝突可能性は極めて低いが、記録が同一 ID になった場合は現在参照中の ID を削除する分岐となる。破損した復旧記録に対する防御として、非空の両 ID の一致をエラーにし、削除しないテストを追加するとよい。実際にその状態が発生したことは確認していない。

## 未確認と範囲

- 作者による `cargo test --locked` 11件成功、ignored 2件のうち合成 Windows credential lifecycle の個別成功、Vitest 1件・build 成功は依頼文の報告としてのみ扱う。独立したログ・再実行はない。子プロセス lock helper の実行も作者報告に限る。
- 実 GitHub 接続、実 UI、実 PAT、復旧処理全体の実 Credential Manager 検証、実行時 IPC・ログ・設定・復旧記録の漏洩検査、実際のプロセス中断・再起動は未実施。Issue #3 の完了条件に関する B03 は未達のままであり、今回の部分承認はこれを解消しない。
- 対象版と前回レビュー R01、Issue #3、独立調査 `issue-03-test-diagnosis-20261119.md`、コードとテスト定義を静的に照合した。テスト再実行、UI 操作、資格情報の読書、GitHub 更新、製品変更はしていない。

今回、新たなコマンド失敗や反省事項はない。
