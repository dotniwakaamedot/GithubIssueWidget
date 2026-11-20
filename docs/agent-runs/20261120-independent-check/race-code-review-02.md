# 競合テスト実装の実行前レビュー 02

- 担当: `/root/snapshot_review`。正式依頼 `race-code-request-02.md`、対象 `widget/src-tauri/src/main.rs` SHA256 `15501708A2B6F8158721A4AC17D1D903ABF55834E9C2D8124BED2314D2EFF0BD`。開始時と終了時に一致を確認。
- 判定: **要修正・実行保留**。旧版の資格情報値を `assert_eq!` で出力し得る問題と、ソケット書込み・スレッド待機の期限不足は対象版で対処されている。一方、子テストの fixture 保護と cleanup 失敗時の再実行について blocking 指摘が残る。実際の秘密漏出、資格情報の誤削除、cleanup 失敗は再現していない。

## Blocking 指摘

### B-01: 子テストが親の新規 fixture だけを受け入れる保証が不足

- 根拠: `windows_inflight_guard_child` は root が OS 一時領域内、`APPDATA` と設定パスが環境変数に一致すること、設定の ID が指定 UUID と一致し取得値が `synthetic-old` であることを確認する。しかし親が今回新規作成した root であることや、設定ディレクトリに `credential-recovery.json` がないことは確認しない。子は単独指定で起動可能で、正常系・競合系で呼ぶ `finish_board` は `recover_pending` を通り、復旧記録があれば固定サービスの資格情報を削除する。
- 影響: 通常の親経由では空の一意領域から準備されるが、単独起動や誤った一時領域の指定では、子自身の検査だけでは既存の復旧記録に基づく意図しない資格情報削除を排除できない。攻撃者による悪用を実証したものではなく、同一ユーザー権限内のテスト fixture 保護の問題。
- 対応案: 親が発行した一意な fixture を子が識別し、許可された内容・所有関係を検証する。最初の `finish_board` より前に少なくとも `credential-recovery.json` の不存在を確認し、想定外の記録やディレクトリ内容なら一切の回復・削除前に停止する。marker は同一ユーザーの攻撃に対する権限境界ではない点も明示する。

### B-02: 明示 cleanup の失敗後に Drop が同じ cleanup を再実行する

- 根拠: 親テスト終盤の `credentials.cleanup().expect(...)` が失敗すると panic により `RaceCredentials::drop` が走る。`cleanup_allowed` はその時点で `true` のため、Drop が `cleanup()` を再び呼ぶ。
- 影響: 資格情報読取り・削除の失敗が、別担当の調査やユーザーの停止判断より前に自動的に繰り返され得る。途中まで cleanup が成功した場合の残存確認も難しくなる。
- 対応案: cleanup の試行済み状態を保持し、明示 cleanup が失敗した場合に Drop が同じ操作を再試行しないようにする。残った合成 ID と不明状態を報告し、停止後の調査・再判断に引き渡す。成功時は削除後の不存在確認を維持する。

## 確認できた範囲

- 子の token と credential ID の比較は `assert!` と固定メッセージへ変更され、比較値自体を panic に展開しない。旧レビューで指摘した `assert_eq!(token, ...)` は対象版では使われていない。
- localhost `127.0.0.1`、`no_proxy`、redirect 無効、HTTP と制御ソケットの read/write timeout、受信・同期待ちの期限、子プロセスの期限およびスレッド完了確認を静的に確認した。終了未確認時は `cleanup_allowed` が false のまま fixture と合成 ID を保持する。
- 親の `process_lock_at` / `write_settings_at` は親が生成し正規化した隔離領域の絶対パスを使う。子は隔離 `APPDATA` と同じ settings path を使い、通常取得後に test-only フックで親へ通知、親が設定更新とロック解放後に応答本文を再開する構造。正常系と変更系の両方を検証する設計。
- 試験対象は `fetch_pages_checked` と製品の `finish_board` の共通経路であり、`list_issues` の Tauri 入口全体、実 WebView、実 GitHub、`save_connection` の一連の操作は通らない。最終照合後の別プロセス変更も未検証。
- 開始時に対象と Cargo 設定・lock の存在と SHA を確認。GUI、テスト・ビルド、資格情報の読取り・変更、外部通信、製品ファイルの変更は行っていない。認識した失敗はない。
