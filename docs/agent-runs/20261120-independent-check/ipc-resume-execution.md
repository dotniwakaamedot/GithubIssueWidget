# IPC テスト再開時の検証記録

- 対象: `widget/src-tauri/src/main.rs` SHA256 `91E544E355597D8A90EE0D26E9A5E12DD7433AB7F2191CE9F787B318AE8F2835`。診断のためテスト専用のエラー種別出力を追加した。診断変更は `ipc-diagnostic-review.md` で静的承認済み。
- Windows Rust テストのコンパイルは成功。診断変更後の通常テストは 50 passed、8 ignored。製品版 release ビルドは成功し、実行ファイル SHA256 は `DED93B43736E261CA732118A4475BD228CEBA0ABBCBB47310BB35B3DC455F10E`。この版の GUI 実行は未確認。
- 前版 `B293846185F74D23F8F6371B5F4081A5E330DCC74CEB7F3E6AB3E246036E2201` で安全な service 検証の純粋テストは成功。隔離 IPC テストは1回実行し、合成資格情報の書き込み時に失敗した。詳細は `ipc-write-failure-analysis.md` と失敗記録 F-027。再実行していない。
- 失敗時 fixture の一時 root と marker は存在し、root 自体は reparse point でなく、設定ファイルは存在しなかった。レビュー済み `inspect-ipc-credential.ps1` SHA256 `645D3C30EDE6AB838BD246DD717BB38BF37BB2E67435A283AB0A66F5663A40E0` の読み取りで、ユーザー `HOME_PC01\CodexSandboxOnline`、Windows session ID 2 の領域では対象資格情報は見つからなかった。失敗時と同一ログオンセッションかの独立記録はなく、他の領域での不存在は未確認。root は削除せず保留。
- 同じレビュー済みスクリプトをサンドボックス外の `HOME_PC01\semis`、Windows session ID 2、管理者権限なしでも実行し、同領域では対象資格情報が見つからないことを確認した。失敗時との同一ログオンセッションや他の資格情報領域は未確認。
- 承認された `ipc-retry-plan.md` に従い、source `91E544E355597D8A90EE0D26E9A5E12DD7433AB7F2191CE9F787B318AE8F2835` と実行ファイル `F850A786BEE2877C32252DA21EB3DE96A22D73F7CCF92E74B9132DDB44D919DE` を実行直前に照合し、`HOME_PC01\semis`、session ID 2、管理者権限なしで `tests::windows_ipc_guard` を単独実行。終了コード0、1 passed。ログ `%TEMP%\giw-ipc-retry-20261120.log` SHA256 `CE3D2D242F7FD2410A9992863793B898F19A44C547ADC7D5BA30A204343E88D3` は応答・削除・cleanup の到達を記録する。ログ自体に実行ユーザー等は含まれず、環境と終了コードは実行時のツール出力に基づく。
- 試験後、静的承認済み `inspect-ipc-retry-credential.ps1` SHA256 `3D083D0A0BE0EF82D058DC5EF79B79FF7F916BA6B8C823A21B406BFB5EA4F48B` を同じユーザー、session ID 2、管理者権限なしで照会し、対象の合成資格情報は見つからなかった。他のログオンセッションは対象外。fixture は保持する。
- 実 GitHub、実 WebView の IPC、実認証情報の保存と削除は未検証。
