# 合成資格情報を使った Tauri IPC テスト計画

## 目的

実 UI を自動操作せず、Tauri のテスト用モック runtime から製品の `connection_info` と `delete_token` コマンドを呼ぶ。合成資格情報を読み戻した場合に応答へトークン値や credential ID を返さないことと、削除後の保存状態を確認する。

## 方法と隔離

- 既存 `windows_inflight_guard` の新規一意 fixture・UUID・合成資格情報・親子プロセス・後始末の仕組みを再利用する。子の `APPDATA`、marker、許可されたファイル、復旧記録不存在と設定パスを確認し、ユーザーの既存設定と実トークンは使用しない。
- Tauri の `test` feature は dev-dependency にのみ指定する。`mock_builder` と `mock_context` に製品の `connection_info` / `delete_token` と新規 `ConnectionLock` を登録し、実 WebView や OS の認証画面を操作しない。
- 既存の正常系の前に `connection_info` を呼び、owner/repo と `has_token` は合成設定に一致し、応答に合成トークン・credential ID が含まれないことを固定メッセージの assertion で確認する。
- 既存の競合系の後に、現在の ID が親が生成した新しい UUID と一致し、その資格情報の値が想定した合成値であることを、値を表示しない条件で確認する。fixture を再検査した後に `delete_token` を1回だけ呼び、合成資格情報が存在せず `connection_info.has_token` が false となることを確認する。親の cleanup は残る合成エントリだけを値照合後に削除し、不存在を確認する。
- 子と補助スレッド、ソケットと cleanup の既存期限・停止条件を維持し、失敗時は合成 ID と fixture を記録する。トークン値はログへ出さない。同一ユーザー権限の marker は攻撃者に対する権限境界ではない。

## 検証と限界

- 対象ソース・Cargo.toml・Cargo.lock の SHA と、単独 ignored テスト・通常テストの終了コード・ログを記録し、合成 credential が残らないことを確認する。
- モック runtime は実 WebView の IPC 許可設定や画面操作、実 GitHub、`save_connection` の通信、最終照合後の別プロセス変更を証明しない。test feature の具体的な依存とモック runtime の動作は実装前に独立レビューする。

## 独立レビュー後の変更

- 基準: `main.rs` SHA256 `5DE67CA065A45C3637B4DD0755D6ED9846E57C903ED93099214E9C03A16C0022`、`Cargo.toml` `82A440BA35FA24E315B3A458721021A43CC7CB2EEE39D1968B0C9B52BADA0540`、`Cargo.lock` `E81559E3D646FE6B25220590E64B5F3CAB3ACAAECDB120392E7EB8285E6B5AD9`。変更対象は `main.rs` と `Cargo.toml`。Cargo.lock が変わる場合は差分・新 SHA を確認し再レビューする。
- 上記の既存 fixture をそのまま使う方式に代え、IPC 試験専用の一意な service 名と ID、新規一時 root を持つ独立した親子テストを作る。service の切替は `cfg(all(test, windows))` のみで有効にし、一意なテスト用 prefix と UUID を検証する。通常製品版では環境変数による切替自体が存在せず、固定の `github-task-widget` service を使う。製品と service 名が異なるため、この点の実動作の証明範囲を限定する。
- 親は service/ID が未登録であることを確認し、書込み前に非秘密の ID・stage を flush する。合成値を登録してから隔離 settings を作り、試験専用 service と root を持つ非表示の子を起動する。子は fixture marker・設定内容・復旧記録の不存在・想定 service/ID/合成値を固定文言で確認する。IPC の前に再確認し、実アプリ用 service を指定できないことを静的・実行時に検証する。marker は同一ユーザー権限での攻撃に対する境界ではない。
- 親は対象 service/ID と合成値が一致するものだけを cleanup し、未知の値は削除せず保持する。子の終了を確認できない場合、または cleanup が失敗した場合も再試行せず記録・保持する。子の監視は期限付きで、期限超過時の終了要求後も期限付きで観測する。親の環境変数は変更せず、子にのみ `APPDATA` と試験用環境を渡す。
- `widget/src-tauri` から `cargo test --locked -- --exact tests::windows_ipc_guard --ignored --test-threads=1 --nocapture` を単独で実行し、専用一時ログに終了コード、対象 SHA、正常な情報取得・値非露出・削除後状態・cleanup を記録する。通常 `cargo test --locked` と release build も行い、対象版を照合する。
