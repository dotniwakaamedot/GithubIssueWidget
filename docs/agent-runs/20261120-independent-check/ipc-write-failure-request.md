# 隔離 IPC テストの書き込み失敗の分析依頼

- ユーザー指示により、担当者間の依頼と結果はファイルで行い、通知はファイルパスだけとする。
- 対象: `widget/src-tauri/src/main.rs` SHA256 `B293846185F74D23F8F6371B5F4081A5E330DCC74CEB7F3E6AB3E246036E2201`。コンパイルと `tests::ipc_test_identity_rejects_product_and_noncanonical_values` は成功。
- `tests::windows_ipc_guard` を1回実行し、終了コード101。ログは `%TEMP%/giw-ipc-isolated-20261120.log`。`stage=before-credential-write` の後、`synthetic credential write failed` で停止。具体的な OS エラーは assertion が出力しない。ログに cleanup 未確認の警告はないが、これだけで独立した不存在確認にはならない。
- コード・ログを読み、確認事実、原因の候補と不明点、合成 fixture と資格情報を安全に確認する方法、再実行前に必要な条件を分析する。テストの再実行、資格情報ストアへの操作、GUI 操作、変更は禁止。出力に合成値や秘密を含めない。結果を `ipc-write-failure-analysis.md` に保存し、通知は絶対パスのみとする。
