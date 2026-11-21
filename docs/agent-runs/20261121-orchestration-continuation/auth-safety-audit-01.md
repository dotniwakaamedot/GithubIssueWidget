# Issue #3 接続・認証の静的監査

担当 `/root/project_survey`。GitHub コネクタで Issue #3 本文を取得。対象 SHA256: `widget/src-tauri/src/main.rs` `61595B68317E15FDD8F2BED88B920A468D5516740E3D3124710384709C52FC24`、`widget/src/main.tsx` `6FCF2A34145450C5029B7FF80DDB3EC1DA5E27173AAB84E57F16D6D54C0E6A3E`、`widget/src/main.test.tsx` `66E118A5A7062854C23BE405D2E9578C974FA283BEA332F2A7C51DCA1BFFD13C`。静的照合であり、今回テスト・外部通信・Credential Manager・GUI は実行していない。

## 要件との照合

| 要件 | 現行コードと証拠の範囲 |
| --- | --- |
| 接続確認後保存、空欄時既存資格情報、再起動なしの切替 | `save_connection` は入力検証、既存設定・トークンと revision の確認後、空欄なら既存 token を選び、20秒 timeout・redirect 無効の GitHub repository GET 成功後に `commit_settings`。成功後 UI は新設定を適用しボードを表示。画面テストは保存待ち・失敗・遅延旧接続応答をモック。実接続切替・空欄既存使用の実操作は未検証。 |
| 保存・更新・削除・再起動 | 設定は APPDATA 配下 settings.json、token は UUID 世代の keyring entry。保存は新世代と recovery journal、設定置換、旧世代の整理順。削除は該当 entry を削除し has_token false を返す。通常テストの合成設定や `isolated_windows_credential_lifecycle`（ignored）は実機資格情報テスト用。後者は同一プロセスの登録・更新・読取・削除であり再起動を証明しない。現行版実 PAT・再起動後取得は未確認。 |
| token を画面に戻さず平文設定・ログ・ソースに記録しない | `ConnectionInfo` は owner、repo、has_token、revision のみ。保存設定は credential_id を持ち token を持たず、UI は入力値を save command に渡し成功後消す。`mutation_revision_rejects_changed_settings_and_credentials` は合成 token・id を response JSON が含まないことを検査。HTTP failure は固定メッセージと status を返し、合成秘密を含む応答本文の非露出を Rust テストが検査。実行時全ログ・実設定・全ソースの秘密不存在は未検証。合成値のテスト文字列を実 PAT と混同しない。 |
| 失敗・認証/権限不足・復旧 | 接続または資格情報書込み前の失敗は保存を進めず、journal・credential・settings 各段階の失敗で以前の設定を維持するか復旧未完了を通知する設計。`failed_settings_commit_keeps_previous_credential`、`journal_failure_prevents_writing_credentials`、`recovery_survives_deletion_failure_and_can_be_retried`、`recovery_handles_interruption_on_both_sides_of_commit` は合成処理を検査。HTTP 401/403 などは固定の HTTP 番号で通知し、レート制限は別表示。実 OS の書込み成功後にエラーが返る曖昧な障害・突然の停止と復旧は未実測。 |
| 権限案内 | 設定画面に Metadata Read、Issues Read and write と、接続確認が書込み権限を保証しない旨がある。実 UI 表示とアクセス可能性は現行版未確認。 |

## 証拠と次の検証

`product-test-result-01.md` は対象 `main.rs` で通常 Rust テスト67成功、10 ignored、opener 1成功、release build 成功を記録し、現行版 Windows UI・実 GitHub・Credential Manager・ignored 試験は未実施とする。`sync-practical-test-result-01.md` は対象 UI テスト39成功、モック・隔離環境であり実認証は対象外。双方の実行者記録を参照し、実機試験の代替としない。

今回の範囲で明確なコード不具合は確認していない。次段階は、独立の一時設定ディレクトリと unique service/ID、合成資格情報だけを用いた現行版の登録・更新・削除と別プロセス再取得の試験設計を作成し、削除対象の同一性、停止・結果不明時の保持と漏洩防止を独立レビューすること。実 Credential Manager は環境変数だけでは隔離されず、現時点では実行しない。`commit_settings` の注入関数のみを使う合成試験として put が書込み後に失敗・設定置換失敗・cleanup 失敗を返すケース、現参照世代以外だけが消され journal が残るかを別途追加可能。ただし設定の読み戻しと API 経由の結果、プロセス停止や Windows ストアは別途必要。

## 操作・失敗・未確認

作業場所 `D:/Projects/GithubIssueWidget`。Issue #3 取得、SHA 照合、関数・テストの限定読取りを実施。2回の広い `rg` は出力が切断され、欠落を根拠とせず対象関数と短い試験記録を個別取得。2件について親に報告し別担当 `/root/process_safety` に独立分析を依頼済み。両 rg の元終了コードは出力に保持しておらず不明。テストや現行版実行は未実施。製品・実データ・他者ファイルは変更せず、削除対象なし。
