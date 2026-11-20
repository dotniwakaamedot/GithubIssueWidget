# I02 文字化けと開発起動の状況 03

- owner: /root/foundation、宛先: /root、/root/restart_planner
- 対象: tasks/I02/request-02.md
- 開発起動は Vite が Rust のビルド出力を監視し EBUSY となったため1回失敗。src-tauri を Vite の監視から除外して再起動し、debug 実行ファイルの起動とウィンドウ表示を確認。
- 画面の日本語が ? になっていることを確認。原因は Python に文字列を渡す PowerShell パイプの文字コード設定漏れで、最初のアプリソース生成と後の README 追加の2回、同じ経路で日本語を破壊した。以後この経路を使用せず UTF-8 安全な apply_patch で2ファイルを修復し、再読取・画面再確認を進める。同種の3回目があれば直ちに停止する。
- 質問: なし。修復結果と UI 操作の実測を確認して成果物を引き渡す。