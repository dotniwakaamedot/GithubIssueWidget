# 隔離競合テストの実行結果レビュー

- 担当: `/root/snapshot_review`。依頼 `race-execution.md` と実行前レビュー `race-code-review-05.md` を参照。
- 判定: **対象版の隔離 Rust 競合テストの結果を承認**。共有ロックを使う取得中に別プロセス側が隔離設定を変更した場合、実ストアを読み戻す共通処理が古い Issue を返さなかったという範囲の証拠がある。親 Issue #1 や実 UI・実 GitHub の統合完了を承認するものではない。

## 対象とログの照合

| 対象 | 実測 SHA256 |
| --- | --- |
| `widget/src-tauri/src/main.rs` | `7052631058511BBBBC70477CE4A8D807444CF8E9DCC64828283C06CF78CA5AE2` |
| `widget/src-tauri/target/debug/deps/github_issue_widget-263288f3cb360ecd.exe` | `F4E38714CE92D34B8FDC98DC7C107927F26BED89D6383C67A9FB428AE4FF8F54` |
| `%TEMP%/githubissuewidget-real-lock-test-20261120.log` | `3B5388B4BF36F8F82640A615D73DF2E4FD735A141FE9E0AE3BEC5E80BADB528D` |

いずれも開始時と終了時に存在と SHA を確認し、実行記録の値と一致した。ログは上記実行ファイルを指定して `1 passed; 0 failed` を記録し、親が保存した起動スクリプトには依頼どおりの単独テストコマンドがある。親の確定記録は終了コード 0 と記載する。この担当はプロセスの終了コードを直接観測していない。ソース更新、実行ファイル更新、ログ更新はその順だが、ログ自体にソース SHA は含まれない。

## 同期・cleanup・終了の証拠

- 対象コードでは、子が実際の `RateCooldown(..., true)` を使ってローカル HTTP を取得し、ヘッダー取得後の test-only フックで親へ通知する。親は子と別プロセスとして同じ隔離 `settings.lock` を取得し、設定を新しい合成 credential ID に書き換え、ロックを解放してから応答本文の送信を再開する。正常系を先に行い、その後に競合系を行う。
- ログの `stage=normal-and-stale-cases-confirmed` は、対象コード上、子の正常系・旧応答拒否の確認に加え、子の正常終了と server / changer スレッドの終了・結果確認を通過した後に出力される。PID `26608` は本レビュー時点でも存在しなかったが、PID 再利用の可能性があるため将来の操作根拠にはしない。
- `stage=credential-cleanup-confirmed` は、対象コード上、親が生成した2件について保存値との一致、削除、削除後の `NoEntry` を確認できた後に出力される。この担当は Credential Manager を操作・直接照合していない。ログには合成 ID と fixture path が記録され、合成トークン値と合成 Issue タイトルは含まれなかった。

## B-01 の解消範囲と未確認

- 前回の合成メモリストアだけのテストと異なり、実ファイルロック、設定ファイル、Windows Credential Manager の合成 credential、および別プロセス間の同期を通る証拠が得られた。B-01 の **Rust の共通取得・取得後検証経路におけるこの競合の証拠不足**は、対象版・対象ケースについて解消した。
- `list_issues` の Tauri 入口全体、React への応答配送・実 WebView、設定画面からの `save_connection`、実 GitHub、最終照合後の別プロセス変更は対象外。接続先自体の切替や credential 削除の同様の実プロセス試験も、この1件では証明しない。timeout・panic・cleanup 失敗の実行結果も未確認。
- 実行前レビューが限定した marker の性質、失敗時の合成 credential 保持・追跡の制約も残る。対象ソース・ログ・実行ファイルの照合とコードの読取りのみを行い、テスト再実行、GUI、Credential Manager、外部サービスの操作や製品ファイル変更は行っていない。認識した失敗はない。
