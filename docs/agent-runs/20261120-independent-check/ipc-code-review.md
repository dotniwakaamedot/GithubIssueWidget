# 合成資格情報 IPC テスト実装の実行前レビュー

- 担当: `/root/snapshot_review`。正式依頼 `ipc-code-request.md` の対象 SHA256 は `main.rs` `804A285387730D80A27EA6416A42F4B2A15710E02FEA2F2F72010F61D6A60C65`、`Cargo.toml` `81A24FB82B063F7E89307808E22A554404544F181A1086A8F220EE28D614C2CE`、`Cargo.lock` `E81559E3D646FE6B25220590E64B5F3CAB3ACAAECDB120392E7EB8285E6B5AD9`。開始時と終了時に実体と一致。
- 判定: **要修正・実行保留**。隔離 service とモック IPC の主経路は確認できるが、承認計画が要求した「製品 service 名を指定できないこと」の実行時検証がない。現行の静的分岐が実際に誤削除するとの再現結果ではない。

## Blocking: service 拒否分岐の実行時検証がない

- 根拠: `entry` の `cfg(all(test, windows))` 分岐は `github-task-widget-ipc-test-` と正規 UUID の完全形式、正規 UUID の ID を要求し、不一致を固定エラーで拒否する。一方、`windows_ipc_guard_child` は親から生成された正常 service のみを使い、製品 service 名・不正 prefix・不正 UUID が実際に拒否される assertion を持たない。承認済み `ipc-plan.md` は製品 service を指定できないことの静的・実行時検証を要求している。
- 影響: 現行試験が成功しても、試験専用 service の拒否分岐が実行時に働くことは証明できない。実 credential を触らずに調べる必要がある。
- 対応案: service / ID の検証を Credential Manager に触れない純粋関数等として切り出し、正規 service と ID の受入れ、製品 service 名、不正 prefix・UUID の拒否を固定メッセージで確認する。拒否テストが失敗しても製品 service の credential 操作へ進まない構成とする。詳細は `questions/ipc-code-negative-service-01.md`。

## 静的に確認できた安全条件

- 子は `WIDGET_IPC_CONTEXT=1`、新規一時 root・APPDATA 一致、設定パス、marker、許可ディレクトリ内容と復旧記録不存在、合成 ID・値を検証する。IPC 前に fixture と現在 ID・値を再確認する。marker は同一ユーザーによる意図的な改変に対する権限境界ではない。
- `entry` の override は `cfg(all(test, windows))` 内だけで、context がある場合は service / ID が無効なら固定エラーを返し、製品の固定 service に fallback しない。親の初期登録と cleanup は明示的な一意試験用 service・ID に対して行い、値が合致しない credential は削除しない。
- mock runtime には `connection_info` と `delete_token` を登録し、前後状態と合成 token・credential ID の非露出を固定文言の assertion で確認する。mock IPC エラーや JSON 読取りエラーに生レスポンスを展開しない。
- `Cargo.toml` の `tauri` test feature は dev-dependency にのみ指定され、Cargo.lock は計画基準と一致。通常 release の feature 解決・生成物とテストの実際の成功は未検証。
- mock IPC の期限なし受信は別子プロセス内にあり、親は期限付きで終了・強制終了後も期限付きで確認する。終了不明なら credential cleanup を行わず記録・保持する。これらの実動作は未検証。

## 未確認・実行時の注意

- `WIDGET_IPC_CONTEXT` が外部環境に残った状態で他の Rust テストを起動すると、test-only `entry` の service が影響を受け得る。IPC fixture の親は自身の環境を書き換えない設計だが、通常・隔離競合テストを実行する際は該当変数を明示的に除外するか、環境不在を確認して対象の service を記録する。
- 実 WebView、IPC capability、実 GitHub、`save_connection` 通信、製品 service 名の実保存／削除、最終照合後の別プロセス変更は対象外。実行後には source・binary・ログ・合成 credential cleanup を独立レビューする。
- 読み取りのみを行い、テスト、GUI、Credential Manager・外部サービスへの操作、製品ファイル変更はしていない。対象ハッシュ再確認時に誤った相対パスを指定してエラーとなり、出力が切り詰められた。親に報告済みで、正しいパスは別途確認し対象 SHA と一致した。
