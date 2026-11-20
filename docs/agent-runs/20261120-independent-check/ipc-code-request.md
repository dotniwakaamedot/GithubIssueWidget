# 合成資格情報 IPC テスト実行前レビュー依頼

- ユーザー原文・制約は `request.md` と `ipc-plan.md` を参照。ユーザーは離席中であり、ユーザー操作を必要としない確認を優先する。同じ失敗の停止回数は今回の指示と処理内で数える。
- 計画の承認: `ipc-plan-review-02.md`。今回の対象は `main.rs` SHA256 `804A285387730D80A27EA6416A42F4B2A15710E02FEA2F2F72010F61D6A60C65`、`Cargo.toml` `81A24FB82B063F7E89307808E22A554404544F181A1086A8F220EE28D614C2CE`、`Cargo.lock` `E81559E3D646FE6B25220590E64B5F3CAB3ACAAECDB120392E7EB8285E6B5AD9`。
- 確認事項: `cfg(all(test, windows))` 限定の service 検証と製品用 service への fallback 防止、UUID 形式、親子の隔離設定・復旧記録検査、合成 credential 以外を削除しない cleanup、子・期限、固定メッセージによる応答・エラー非露出、モック IPC の対象コマンドと限界、dev-only feature・lock の扱い。
- 方法: 読み取り専用。テスト、GUI、Credential Manager・外部サービスへの操作、製品ファイル変更は禁止。結果を `ipc-code-review.md` に対象 SHA と根拠・未確認範囲を記録する。
