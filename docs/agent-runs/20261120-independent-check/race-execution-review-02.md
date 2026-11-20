# 隔離競合テスト再確認の独立レビュー

- 担当: `/root/snapshot_review`。対象記録 `race-execution-02.md` の SHA256 は `7F37E61DA788D539821B04C835975D2556E0DD5AE40ED052D53DA065F3D4A638`。
- 判定: **対象版の隔離競合テスト結果と、合成エラー本文の非露出テスト結果を限定承認**。前回承認した Rust の共通取得・取得後検証経路における別プロセス競合は、新版でも再確認された。実 UI・実 GitHub を含むアプリ全体の完成を承認するものではない。

## 対象・ログの照合

| 対象 | 実測 SHA256 |
| --- | --- |
| `widget/src-tauri/src/main.rs` | `D4C7C833AA003967D34B994033B74189EF7F4D42FDE8F910C6A16569EC63799C` |
| `widget/src-tauri/target/debug/deps/github_issue_widget-263288f3cb360ecd.exe` | `FE3E32180DB7371B802D7727F8B4FBFF11AE88F5F791BAC6C96A19D53A35D59E` |
| `%TEMP%/githubissuewidget-real-lock-recheck-20261120.log` | `0951D31EB3E603DBF456F0DBCECE0EDA221E0CFEAF2AD641E025F5587AA8EE6F` |

上記は開始時と終了時に存在と SHA を照合し、実行記録の値と一致した。Cargo.toml / Cargo.lock も計画指定 SHA と一致。ログに `1 passed; 0 failed`、`stage=normal-and-stale-cases-confirmed`、`stage=credential-cleanup-confirmed` がある。対象コード上、前者は子の正常終了・正常系と競合系・server / changer の終了と結果確認後、後者は合成資格情報2件の値照合・削除・削除後の不存在確認後に出力される。この担当は Credential Manager にアクセスしていない。

記録された子 PID `36020` はレビュー時に存在しなかった。PID は再利用され得るため将来の操作根拠にはしない。親の実行記録は終了コード 0 と記載しているが、この担当はプロセスの終了コードを直接観測していない。ログ自体にソース SHA は含まれないが、ソース更新後に実行ファイルが更新され、続いてログが更新されていることを確認した。

## 追加されたエラー本文テスト

- `api_error_bodies_do_not_leak_tokens_or_issue_titles` はローカル HTTP の 401 / 403 / 404 / 429 / 500 応答本文に合成トークンと合成 Issue 名を入れ、`fetch_pages` から返るエラーにそれらが含まれないことを確認する。`%TEMP%/githubissuewidget-redaction-tests-20261120.log` SHA256 `74581B318233066F88FA8E56A466C873D31F1CBFE7F86832C1294E04FA4A844C` は当該テスト `ok`、全体 `49 passed / 6 ignored` を記録する。
- 再確認ログと通常テストログを読み取り、該当する合成トークン値と合成 Issue 名が含まれないことを確認した。テストは限定された本文・ステータスと `fetch_pages` 経由のエラーについての証拠であり、全ての API・ログ・UI 経路や実秘密の非露出を証明しない。403 の本文は `rate limit` を含む例であり、秘密を含む通常の非レート制限 403 本文はこのテストの対象外。

## 競合テストの範囲と未確認

- 対象コードでは子が実 `RateCooldown(..., true)`、設定ロックと Credential Manager の合成 credential を使う。HTTP ヘッダー後の test-only フックで通知を受けた親が別プロセスから同じ隔離ロックを取得し設定を変更、ロック解放後に本文を再開する。正常系を先に、旧応答拒否を後に検証する。
- `list_issues` の Tauri 入口全体、React / 実 WebView への応答配送、`save_connection`、実 GitHub、最終照合後の別プロセス変更、接続先の切替や token 削除の実プロセス競合は対象外。既存画面テストログは `20 passed` だが、モックを使うため実 WebView と Rust の結合証拠ではない。
- timeout、panic、cleanup 失敗時の動作は今回の成功ログでは実行されていない。この担当はテスト再実行、GUI、Credential Manager・外部サービスへの操作、製品ファイル変更を行っていない。認識した失敗はない。
