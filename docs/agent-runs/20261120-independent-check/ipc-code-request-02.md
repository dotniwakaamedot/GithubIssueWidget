# IPC テスト実装の再レビュー依頼

- 対象: `widget/src-tauri/src/main.rs` SHA256 `B293846185F74D23F8F6371B5F4081A5E330DCC74CEB7F3E6AB3E246036E2201`、`widget/src-tauri/Cargo.toml` SHA256 `81A24FB82B063F7E89307808E22A554404544F181A1086A8F220EE28D614C2CE`、`widget/src-tauri/Cargo.lock` SHA256 `E81559E3D646FE6B25220590E64B5F3CAB3ACAAECDB120392E7EB8285E6B5AD9`。
- 前回レビュー `ipc-code-review.md` の実行保留事項を踏まえ、資格情報に触れない純粋な service・ID 検証と受入れ／拒否テストを追加した。元の安全条件と合わせて独立確認し、合成資格情報を使用する ignored テストを実行可能か判定する。
- 資格情報、GUI、外部サービスの操作は禁止。作業ディレクトリとパスの存在を確認してから参照し、出力は小分けにする。レビュー前後の対象 SHA を記録し、結果を `ipc-code-review-02.md` に確定する。
