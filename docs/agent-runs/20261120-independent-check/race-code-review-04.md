# 競合テスト実装の実行前レビュー 04

- 担当: `/root/snapshot_review`。正式依頼 `race-code-request-04.md` の対象 `widget/src-tauri/src/main.rs` SHA256 `54A32EE8E7E9434A4135CE94885C7B65EB9FFFAF24734E538F2E990D64165719` は開始時と終了時に一致。
- 判定: **要修正・実行保留**。前回の補助スレッド開始前の cleanup 制御は対象版で対処されたが、子プロセスの強制終了後に期限のない待機が残る。実際のハングは未観測。

## Blocking 指摘: 強制終了要求後の待機に期限がない

- 根拠: 親テストの子プロセス監視は 40 秒の期限を設けるが、期限超過後に `child.kill()` が成功すると `child.wait()` を期限なしで呼ぶ。計画は子とサーバーの待機に上限を設け、終了が確認できない場合は fixture と合成 ID を保持して報告するよう求めている。
- 影響: OS が強制終了要求後のプロセス終了を直ちに確定できない場合、終了不明の報告に到達せず待機し続ける可能性がある。OS や本テストの実行でハングが発生したことは確認していない。
- 対応案: `kill` 後も期限付きで `try_wait` を行い、終了未確認なら `cleanup_allowed` を false のまま fixture と合成 ID を保持して報告する。詳細は `questions/race-code-child-wait-04.md`。

## 前回 B-01 と旧安全条件

- `cleanup_allowed = false` は server / changer の `thread::spawn` より前に設定され、子および両スレッドの終了を確認した後だけ `true` になる。スレッド開始後・子起動前の panic における Drop の cleanup は抑止される構造。実行時の panic は未再現。
- 子側の fixture marker・ルートと設定領域の許可内容・シンボリックリンク・復旧記録不存在の確認、各 `finish_board` 前の再確認は維持されている。marker は同一ユーザーの攻撃に対する権限境界ではなく、検査後の変更も完全には防げない。
- 明示 cleanup 前に自動 cleanup を禁止し、成功時のみ `cleanup_done` を設定するため、明示 cleanup が失敗したとき Drop は同じ cleanup を再実行しない。
- ループバック・proxy / redirect 無効、合成値の固定文言比較、設定ファイルの明示パス、HTTP・制御ソケット read/write と補助スレッドの期限は静的に確認した。テストが実際に成功すること、実 WebView・GitHub・`save_connection` 全体は未検証。
- Cargo.toml と Cargo.lock は計画指定 SHA と一致。読み取り以外のテスト・GUI・実資格情報・外部サービス操作および製品変更は行っていない。認識した失敗はない。
