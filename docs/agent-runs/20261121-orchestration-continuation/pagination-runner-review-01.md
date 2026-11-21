# 大量ページ限定実行器レビュー 01

- 確認者 `/root/planning/plan_review`、作者 `/root/issue_survey` と別担当。対象 `pagination-test-execution-01/run-test.ps1` の開始・終了 SHA256 は `CD8705690A28D9F607BA596037307F92607B395BCD9687FEFD779639D58E1063` で依頼と一致。
- 判定: **実行器の固定版を承認**。blocking なし。これは実行前のコード確認で、ケース実行・試験成功・実 API を承認するものではない。開始前に入力と専用領域、ツール、競合とアクセスを依頼どおり再確認し、不一致なら実行しない。

## 確認結果

- `ValidateSet` が2件のみ受け付け、テスト名は `pagination_regression_tests::<Case>`、`--exact` と `--bin github-issue-widget` が選択される。固定版の隔離 main にモジュール宣言、テストファイルに2関数を確認。現在の実行用コピーの main `AEFF1E65AD93EFA04773CEBBB9736A6BDC5B9938DB9FAF937DD88B3D9CBFE86F` と test `BF690E818EB182FE16395E0AE3946B043D41DADD2B75235EFEACDC4873F4DE3D` は依頼値に一致。
- `--locked --offline`、専用 `CARGO_HOME`、`CARGO_TARGET_DIR`、TEMP、USERPROFILE、APPDATA と別ログを指定し、子環境をクリアして限定変数を渡す。ログの既存を拒否する。依存、Visual Studio の動作や各コマンドが OS 全体で通信しないことは静的確認だけでは証明しない。
- 直接 `cmd.exe` の終了コードを取得し、900秒超過は通知した上で同じハンドルを観測し続け、無関係なプロセスを終了しない。期限超過は直接終了・子孫の終了を意味しない。監督者停止時は結果不明として記録する。
- スクリプトは Cargo の終了コードを返すのみで、0件成功を自動拒否しない。実行担当はケースごとのログを読み、`running 1 test` と該当ケース成功・終了0を確認して初めて成功とする。0件、ログ欠落、期限超過、終了不明は成功扱いにしない。実行結果の独立レビューに具体的なログと件数を提出する。

実行器、Cargo、GUI、GitHub は実行していない。対象が変われば再レビューする。
