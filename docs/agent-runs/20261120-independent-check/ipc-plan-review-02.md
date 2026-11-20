# 合成資格情報 IPC テスト計画の独立レビュー 02

- 担当: `/root/snapshot_review`。対象 `ipc-plan.md` SHA256 `6FDFCFEA0A587853FDA44DF1C2BC20A69B3A923B5A4B652D35E753A350551263`。
- 判定: **対象計画版に限り承認**。前回 B-01 の削除対象の混入リスクに対し、通常製品版で存在しない test-only の一意 service を使う方針が示され、B-02 の基準・変更範囲・実行方法も具体化された。実装後に下記の隔離条件を確認するまでテスト実行は承認しない。実 UI や実 GitHub、親 Issue 完了の承認ではない。

## 前回指摘と対象版

- B-01: 旧 fixture を直接拡張する方式を置き換え、IPC 用の一意 service・ID と一時 root を使う。子の設定が事前検査後に変化しても、厳密に分離された試験用 service しか指定できなければ、製品用 service の credential は削除対象にならない。service 名が製品と異なるため、製品 service 名そのものの実動作は本試験で証明しない。
- B-02: `main.rs`、`Cargo.toml`、`Cargo.lock` の基準 SHA と変更予定、単独 ignored テスト、通常テスト・release build、ログ・cleanup の確認が指定された。現在の各実体 SHA は計画の `5DE67CA065A45C3637B4DD0755D6ED9846E57C903ED93099214E9C03A16C0022`、`82A440BA35FA24E315B3A458721021A43CC7CB2EEE39D1968B0C9B52BADA0540`、`E81559E3D646FE6B25220590E64B5F3CAB3ACAAECDB120392E7EB8285E6B5AD9` と一致。

## 実装後・実行前の確認事項

1. Windows backend `keyring 3.6.3` の既定ターゲット名は `user.service` であることをローカルソースで確認した。試験用 service は固定 prefix と正規 UUID の**完全一致形式**から構成し、製品 service `github-task-widget` と区別する。任意文字列・空値・製品 service の指定を受け付けず、子が試験用 service を検証できないときは製品 service へ fallback せず失敗することを確認する。テスト用 ID も正規 UUID に制限する。
2. 親の環境変数を変えないため、親による未登録確認・登録・cleanup は、親が生成した同一の試験用 service と ID を明示して行う必要がある。子の `entry` と `recover_pending` を含む全資格情報操作もその同じ試験用 service に限定され、製品 service へ触れないことを静的・実行時に確認する。既存値の上書きや不一致値の削除はしない。marker は同一ユーザー権限の攻撃を防ぐ境界ではない。
3. Tauri 2.12.7 の `test` feature と mock API の存在をローカルソースで確認したが、`get_ipc_response` の内部受信は期限なしである。IPC 呼出しを期限付きの子プロセス内だけで実行し、子の停止や cleanup の成否が不明なら service・ID・fixture を保持して報告する。テストのコンパイル・実行成功は未確認。
4. response と error の検査は固定文言とし、失敗時に生 JSON・token・credential ID を `assert_eq!` や debug で出力しない。`--locked` が lock 更新を要求した場合は失敗を成功扱いせず、変更の妥当性と新 SHA を確認する。通常 release に test-only override が入らないことも実装・ビルドの証拠で確認する。

## 未確認・範囲

モック runtime は実 WebView の capability や画面操作、実 GitHub、製品 service 名、`save_connection` の通信、最終照合後の別プロセス変更を証明しない。実装コード、実 IPC の結果、cleanup と終了は実施後に対象 SHA とログを独立レビューする。

本担当は計画・対象の存在と SHA、ローカル Tauri / keyring の定義を読み取りで確認した。テスト、GUI、Credential Manager・外部サービス操作、製品ファイル変更は行っていない。認識した失敗はない。
