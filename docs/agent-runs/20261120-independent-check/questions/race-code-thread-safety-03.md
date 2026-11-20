# 補助スレッド開始前後の cleanup 条件

- 対象 `widget/src-tauri/src/main.rs` SHA256 `65524C85B0A9CE223AD6144736C17E34033812FCBF376D5133BE8AB02CDECEBF`。
- 事実: `server` と、隔離設定ファイルを書き換え得る `changer` は `credentials.cleanup_allowed = false` より先に起動する。`std::env::current_exe().unwrap()` や後続の準備がその間に panic すると、スレッドの終了確認前に `RaceCredentials::drop` が合成資格情報の cleanup を実行する可能性がある。
- 範囲: `changer` が書くのは親の一時 fixture の設定であり、生成値の照合もあるため、この事実だけから実ユーザーの資格情報への影響は確認できない。通常経路の不具合・実際の panic は未再現。ただし、補助スレッド停止前に credential を削除しないという計画上の条件は、この panic 経路では保証されない。
- 推奨: fixture に触れる可能性があるスレッドを起動する前に cleanup を禁止し、起動済みの全スレッドと子の終了を確認できた場合に限り cleanup を許可する。スレッド未起動を確認できる場合の cleanup は別に扱い、終了不明は合成 ID と fixture を保持して報告する。
- テスト・GUI・資格情報・外部サービスの操作、製品コードの変更は行っていない。
