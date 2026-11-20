# 隔離 IPC テストの別環境での検証計画

- `ipc-resume-decision.md` の条件に従う。前回 root は保持し、新しい UUID の試験専用 service、ID、正規化済み一時 root をテスト自身が生成する。作成前の不存在確認、製品設定と資格情報の分離、期限付き子監視、合成値一致後のみの cleanup は `ipc-diagnostic-review.md` で静的確認済み。識別子・fixture の記録、cleanup の成否を専用ログに保存する。実 credential、実 GitHub、GUI を使わない。
- 実行予定環境は `HOME_PC01\semis`、Windows session ID 2、管理者権限なし。実行直前に再確認し、不一致なら中止する。テストはサンドボックス外で実行するが、資格情報ストアの書込み利用可否はまだ不明。前回実行時と同一ログオンセッションかは未確認。
- 実行直前に `main.rs` SHA256 `91E544E355597D8A90EE0D26E9A5E12DD7433AB7F2191CE9F787B318AE8F2835` とテスト実行ファイル SHA256 `F850A786BEE2877C32252DA21EB3DE96A22D73F7CCF92E74B9132DDB44D919DE`、IPC context が未設定であることを確認する。同対象版の通常テストは50 passed、8 ignored。失敗時には固定カテゴリのみを記録し、同種失敗がこの指示と処理で2回となる場合は停止する。
- 実行後の値非露出・cleanup・ログと対象版を独立レビューし、実 WebView と実 GitHub が対象外であることを明記する。
- 実行対象は `D:\Projects\GithubIssueWidget\widget\src-tauri\target\debug\deps\github_issue_widget-8bc8fc9e0550a265.exe`、作業ディレクトリは `D:\Projects\GithubIssueWidget\widget\src-tauri`、引数は `--exact tests::windows_ipc_guard --ignored --test-threads=1 --nocapture`。専用ログは `%TEMP%\giw-ipc-retry-20261120.log` とし、既存なら上書きせず中止する。
- 実行直前に実行プロセス内で WindowsIdentity、session ID、管理者権限の有無を照合・記録する。操作は認可された合成資格情報検証の範囲に限定し、ツールの権限審査を通った環境だけを使う。試験内部の書き込み前の不存在確認と cleanup は同じユーザーの同じテストプロセスで実施する。
- 試験後は、結果ログから新規 service と ID だけを抽出する専用の読み取りスクリプトを、ログのハッシュに固定して静的レビューした後、同じユーザー・セッションで実行して状態だけを取得する。値は出力せず、存在・不明・セッション不一致・照会失敗なら資格情報と fixture を保持して独立分析する。ログ上の cleanup 表示だけを独立確認の代わりにしない。
- 純粋検証を含む同一テスト実行ファイルの通常テスト結果は `%TEMP%\giw-rust-diagnostic-20261120.log` に記録済み。実行前にハッシュと成功の対応を確認する。
