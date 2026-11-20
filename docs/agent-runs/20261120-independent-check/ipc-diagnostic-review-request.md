# IPC テストの失敗診断変更のレビュー依頼

- 対象 `widget/src-tauri/src/main.rs` SHA256 `91E544E355597D8A90EE0D26E9A5E12DD7433AB7F2191CE9F787B318AE8F2835`。Cargo.toml は `81A24FB82B063F7E89307808E22A554404544F181A1086A8F220EE28D614C2CE`、Cargo.lock は `E81559E3D646FE6B25220590E64B5F3CAB3ACAAECDB120392E7EB8285E6B5AD9`。
- `ipc-write-failure-analysis.md` の指摘を受け、合成資格情報書き込み失敗時に keyring エラーの固定カテゴリのみを出すよう変更した。エラー内部や値を表示しないこと、失敗時の cleanup と既存安全条件を変えないことをレビューする。前回の失敗原因は引き続き未特定で、今回の変更は原因の証明ではない。
- 失敗済み fixture と資格情報の独立確認前に再実行を承認しない。コードの読み取りとレビューに限り、資格情報操作、テスト実行、GUI 操作、秘密・合成値の出力は禁止。出力時に検索行の内容を事前に検討し、値を表示しない。対象 SHA を確認し `ipc-diagnostic-review.md` に結果を保存する。通知はファイルパスのみ。
