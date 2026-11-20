# 競合テスト実装の実行前レビュー 03

- 担当: `/root/snapshot_review`。依頼 `race-code-request-03.md` の対象 `widget/src-tauri/src/main.rs` SHA256 `65524C85B0A9CE223AD6144736C17E34033812FCBF376D5133BE8AB02CDECEBF` は開始時と終了時に一致。
- 判定: **要修正・実行保留**。前回の fixture と cleanup 再実行に関する指摘は対象版で対応されている。一方、補助スレッド開始後・cleanup 禁止前の panic 経路では、スレッドの終了確認前に合成資格情報が削除され得る。

## Blocking 指摘

### B-01: 補助スレッド開始後、終了確認前に cleanup が実行される経路

- 根拠: `windows_inflight_guard` は HTTP の `server` と、隔離設定ファイルを更新する `changer` を起動した後、`Command::new(std::env::current_exe().unwrap())` などの準備を経て `credentials.cleanup_allowed = false` とする。その間に panic すると、スレッド終了を確認する前に `RaceCredentials::drop` が cleanup を呼ぶ可能性がある。
- 影響: `changer` は親の一時 fixture へしか書かず、cleanup も合成値が一致する ID に限るため、この事実だけで実ユーザー資格情報への影響は確認できない。だが所有スレッドが残っている可能性を確認する前に合成資格情報を削除し、fixture が後から変更され得る。実際の panic・競合は未再現。
- 対応案: fixture に触れ得る補助スレッドを開始する前に cleanup を禁止し、開始済みのすべてのスレッドと子の終了が確認できた場合だけ再び許可する。起動前に誰も開始していないと確定する場合の cleanup は分け、終了不明の場合は合成 ID と fixture を保持して報告する。詳細は `questions/race-code-thread-safety-03.md`。

## 前回指摘の確認

| 指摘 | 対象版の対応 | 判定 |
| --- | --- | --- |
| 子が親の fixture だけを受け入れる確認 | 一意 marker と ID、正規化済み root・設定パス、root / settings ディレクトリの許可内容・シンボリックリンク、復旧記録の不存在を確認。各 `finish_board` 前にも fixture を再確認 | 通常の親による新規 fixture と誤指定の切り分けとして静的に対応。marker は同一ユーザーによる故意の偽造を防ぐ権限境界ではなく、検査後の変更も完全には防げない。 |
| 明示 cleanup 失敗時の Drop による自動再試行 | 明示 cleanup 前に `cleanup_allowed = false` とし、成功時に `cleanup_done = true`。失敗時の Drop は固定警告を出し、cleanup を再呼出ししない | 静的に対応。実際の cleanup 失敗は再現していない。 |

## ほかの確認と未確認事項

- ローカル `127.0.0.1`、proxy・redirect 無効、非表示の子、HTTP ヘッダー後の一回限り test-only フック、親の明示パスでのロック・設定更新、正常系後の競合系、期限と結果の固定文言を確認した。各保護の実行結果は未検証。
- 対象の Cargo.toml / Cargo.lock は計画指定 SHA と一致。実 WebView、実 GitHub、`save_connection` 全体、最終照合後の変更はテスト範囲外。
- 読み取りだけを実施し、テスト、GUI、実資格情報、外部サービス、製品ファイルの変更は行っていない。認識した失敗はない。
