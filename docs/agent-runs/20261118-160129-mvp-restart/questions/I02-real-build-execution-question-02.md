# I02 実ビルド rustup stage 確定と npm 準備 02

- 担当 `/root/build_verification`、宛先 `/root`。rustup の別担当 receipt `tasks/I02/real-build-rustup-log-approval-01.json` SHA256 `0A008E59EFC56D5FD2B69A8D295ABB9F479FCB2EFA5B622E491880D2FA7F238E` の stage・担当・driver・ログ hash と自動走査を照合し、親指示に基づき同一 session 85009 に `publish` を1回送信。session exit 0。
- `rustup.result.json` SHA256 `58E4EFDCB6E212E0BD2F640D2CB015EA7DC896CB055C38C25B290E9FC94FAEBE` は PID 30792 の実子 exit 0、stdout/stderr の `RanToCompletion`、receipt hash 一致を記録。専用 rustup 1.90.0 の rustc SHA256 `1A9F8C489A90009CE5DB828D0A7436A1354789C42590D73FF5CC8B4E53695612`、cargo SHA256 `5615085F39570E13B4ABD6F23E3302521A0EC570B2DD5182E39A98A87E5E9584`。ログ内容は記載しない。
- `rustup.before.json` と `rustup.after.json` は元・コピーの各17パスに対応し、全 hash が manifest と一致。別途の現在の17ファイル実体・ハードリンク・hash も一致。npm 専用出力・receipt とコピー側 node_modules は存在せず、同一 driver process 不在。空き D: 約125.6 GB、空きメモリ約39.3 GB。追加の固定参照と toolchain を最終照合してから、認可された npm stage を同じ driver の別 session で起動する。
- 初め before/after の件数をパイプと配列化の組合せで1と表示したが、JSON 実体の配列を直接取得して各17件・期待値一致と確認。元データや成功判定は変更していない。今回の stage 実行・読戻しに失敗なし。元製品・GitHub・他者作業は変更していない。
