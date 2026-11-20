# IPC テスト実装の再レビュー

判定: 対象版の静的レビューを承認する。合成資格情報を使用する ignored テストの前に、資格情報に触れない `ipc_test_identity_rejects_product_and_noncanonical_values` を Windows で実行して成功を確認すること。テスト自体の成功は現時点では未確認。

- 担当 `/root/snapshot_review` は実装者と別担当。レビュー前後に対象 SHA256 の一致を確認した。`main.rs`: `B293846185F74D23F8F6371B5F4081A5E330DCC74CEB7F3E6AB3E246036E2201`、`Cargo.toml`: `81A24FB82B063F7E89307808E22A554404544F181A1086A8F220EE28D614C2CE`、`Cargo.lock`: `E81559E3D646FE6B25220590E64B5F3CAB3ACAAECDB120392E7EB8285E6B5AD9`。
- 前回 blocking の対応: `validate_ipc_test_identity` は固定 prefix と正規形 UUID の service および ID を要求し、`entry` の試験コンテキストは検証失敗時に製品 service へ戻らない。純粋関数のテストは正規値の受入れ、製品 service `github-task-widget`、不正 prefix、UUID、非正規表記と不正 ID の拒否を確認する構造。資格情報 API を呼ばない。実行結果はまだないため実行時の拒否は未証明。
- 既存の安全条件: 親は一意な試験用 service、合成 ID と値、分離 root を使い、子は APPDATA、fixture、設定と合成値を事前確認する。期限内に子の終了を確認できない場合は資格情報を保持し、cleanup は試験用 service と一致する値に限定される。mock IPC 応答には合成値と ID が含まれないことを検査する。これらは静的確認であり動作は未確認。
- `tauri` の test feature は dev-dependency のみ。通常 release の解決結果、実 WebView・capability、実 GitHub、製品 service での保存削除は未確認。試験実行後に対象 source、binary、ログと cleanup を独立確認する。
- Non-blocking: 外部環境に `WIDGET_IPC_CONTEXT` が残ると他のテスト内の service 選択に影響する。通常テストの実行時は不在を確認するか子プロセスの環境から明示的に除外する。
- 今回はテスト、GUI、資格情報、外部サービス操作、製品変更を行っていない。新たな失敗はない。
