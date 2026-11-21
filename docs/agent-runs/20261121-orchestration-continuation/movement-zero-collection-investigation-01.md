# 移動競合テスト0件収集の独立調査

2026-11-21、`/root/issue_survey`。`movement-rust-execution-result-02.md` と実行ログ、当該 fixture のコード・バイナリ・依存情報・時刻を読取り。製品・既存 target・ログは変更せず、テストや対象バイナリは実行していない。

## 確認された事実

- 直前の実行ログ stderr は `Finished test profile` とテストバイナリ `github_issue_widget-fc25f1eb6ae85386.exe` の起動を記録。stdout は `running 0 tests`、0 passed、79 filtered out。終了コード0はテスト成功を意味しない。
- fixture の `src/movement.rs` SHA256 `88BF21BB45C077DFDA592D906C753D168C5641E35EB12324D4ACF1652374695B` は新しい3テストを含む。`movement::tests` 内の関数名は `post_write_competing_status_addition_reports_partial`、`post_write_competing_target_removal_reports_partial`、`post_write_competing_state_change_reports_partial`。実行器のフィルタ `movement::tests::post_write_competing_` はソース上の3名に整合する。
- 実行ログで指定された exe SHA256 `AE685AF16375B83CD9881D6B1923D0D53C44E760B9D6349F0C96CC3CAE7A1D02` を実行せずバイト検索すると、新しい3関数名は各0件。一方で旧 movement の `all_six_transitions_preserve_labels_added_after_the_initial_get` 等と以前の pagination テスト名は各1件含む。文字列有無は実行時一覧の直接出力ではないが、旧テストを含み新3件を含まないバイナリが動いたという強い根拠になる。
- 同じ basename の `.d` SHA256 `4EF31ED6A52D4CACED097BFDFC805A11FA81E91E0AAC13208F3F026D2AE6FA78` は `src/main.rs`、`src/creation.rs`、`src/movement.rs`、`src/pagination_tests.rs` を依存として列挙。`movement.rs` は依存から漏れていない。
- 読取り時の fixture movement の最終更新 `2026-11-21T02:21:31.5468075Z`、exe `02:30:20.2196118Z`、同 fingerprint の `invoked.timestamp` `02:30:12.0106495Z`、0件時の stdout 更新 `02:50:47.3195435Z`。コピーによって既存ファイルの過去時刻が維持され、Cargo が旧バイナリを fresh と扱った可能性と整合する。ただし実際のコピー操作の時刻保存挙動と Cargo 内部の判断ログは未確認であり、原因を断定しない。

## 具体的対処

新規の空 `CARGO_TARGET_DIR` を確保し、fixture と Cargo.lock・依存を固定した上で同じ3件の接頭辞フィルタを **一度だけ**実行することを推奨する。既存 target を消さず、旧 exe・dep-info とログを保持でき、古い fingerprint・mtime に依存しない。一方、時刻のみ更新する対処は時計のずれ・時刻分解能・比較実装に依存するため、確実性は新規 target の方が高い。コンパイル所要時間・追加容量、専用 cache の同時利用、アクセス権とログの新規保存は事前確認が必要。

実行方式は既承認の runner を変更するため、新規 target と新規ログを指定した固定版を別担当がレビューする。子環境から秘密を渡さず、実 GitHub を呼ばず、loopback mock のみを使う。`cargo test --locked --offline --bin github-issue-widget movement::tests::post_write_competing_ -- --test-threads=1` 後、`running 3 tests`、3つの完全名それぞれの `ok`、0 failed、直接終了コード、ログ完結と期限を確認する。filter は `--exact` ではないので予想外テストが混ざらないことも検査。既存の成功済み pagination や全テストは再実行しない。実ネットワーク、Credential Manager、全子孫終了は別の未確認事項として維持する。

## 調査記録

限定 `Get-Content`、`Get-Item`、`Get-FileHash`、Python でバイナリ内の既知の合成テスト名の件数だけを出力した。いずれも終了0。新規テストの静的名前確認と実行ログの対象照合を行い、秘密や任意のバイナリ内容を出力していない。調査中の操作失敗は認識していない。確定後に書込みを停止する。
