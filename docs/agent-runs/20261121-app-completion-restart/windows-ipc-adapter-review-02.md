# Windows IPC 隔離コピー安全境界レビュー 02 — 対象版を承認

- 担当 `/root/windows_ipc_adapter_review` は作者 `/root/windows_ipc_adapter` と別。依頼 `windows-ipc-adapter-review-request-02.md` の対象結果 `DA6C96F16A60DC7D433CD95E7AA39B210364A9833BF9266D7D0D79C75E61C1F2`、コピー `main.rs` `C160E15184C3651F3F17432DF9A5B14A40FEAB899EF8B6EADA7AACA48F7DC862`、`test_adapter.py` `8E9105F2D3B48B069F75DDFAA8636638297DEF2B5ED71DA97D6DAD97B7318F7F`、`adapter.py` `0A350F40863D3E80FA621195EAB8143E404026C594BF802CB382971EA6F421B0`、レビュー01 `1A70B564FDE8C4353A8E2860174D78F0B209B3960937F11F26E2EABBCEC603E5` は依頼値と実測が一致。
- **blocking なし。** レビュー01で指摘した親ゲート迂回を対象版で修正した。承認はこの版の隔離コピー安全境界と確認した証拠の範囲であり、実 OS 資格情報の操作や製品完成を承認するものではない。

## 完了条件と根拠

1. **親・子の独立入口:** コピー `main.rs` の `windows_ipc_guard` と `windows_ipc_guard_child` はそれぞれ関数本文先頭で固定文言を panic する `block_os_credentials()` を呼ぶ。いずれも `#[should_panic(expected = "isolated adapter: OS credentials disabled")]` が指定され、以前の子直接実行による親ゲート迂回は当該版では停止する。共有 helper `blocked_keyring_entry` も同じ停止関数を呼ぶ。限定検索ではコピー内に `keyring::Entry::new` の直接呼出しは残らず、５つの呼出しは helper に置換されている。依存ライブラリ・コピー外の全 OS API の保証ではない。
2. **実行証拠:** 隔離 crate の `main.rs` の SHA は対象コピーと一致。parent・child・central の3ログ SHA は作者記録と一致し、各々対象の単一テストが `should panic ... ok`、`1 passed` と示す。ソースの `expected` 属性と照らすと成功行は期待する禁止メッセージの panic が検査されたことを示すが、ログ自体に panic 本文は表示されない。各 exit 記録の SHA `13BF7B3039C63BF5A50491FA3CFD8EB4E699D1BA1436315AEF9CBE5711530354` と test binary SHA `B3C6054EAE6FEB4339616951665B2BE82381ABD450B5C7748CBC857499954D2D` も結果と一致。保存終了値は作者の記録で、当担当が元プロセスを監督した証拠ではない。再実行はしていない。
3. **Python 模擬:** `test_adapter.py` は直接 constructor の不在と両入口の先頭停止を静的照合する。`test-05.log` の SHA は記録と一致し10件 `OK`、exit 記録の SHA も上記0記録と一致。Python モデルは Rust guard を起動せず、実 OS・実 IPC の証拠ではない。親停止、OS 排他・ACL・子孫停止・障害復旧の実現性はこれで証明しない。
4. **実体・削除:** 限定 `git status --short` で隔離領域と結果は未追跡、本番 `main.rs` に変更表示なし。結果は削除対象なしと記載し、本依頼も削除を指定しない。個人 ignore 設定の読取拒否警告があり、workspace 全体・ignored の網羅確認ではない。実 Credential Manager・アプリ・GitHub は操作していない。

## 次作業・未確認

この版の無条件禁止を維持したまま Python 監督と Rust の接続、実行主体の識別、異常時の安全な停止・故障試験を別担当が具体化できる。実 OS 書込みの解禁は含まず、所有権・排他・記録・復旧・親子孫停止と実環境条件を別途検証して判断する。実 Credential Manager、実 WebView、実 PAT のライフサイクルは未検証。

## 作業上の失敗

依頼の読取り前に結果名を推測し、存在しない `windows-ipc-adapter-result-02.md` を読もうとして `Get-Content` が終了1となった（1回）。その操作は根拠とせず、依頼に実記載の `windows-ipc-adapter-fix-result-02.md` と SHA を照合した。対象ファイルを推測せず事前に存在と確定依頼を確認する既存手順を適用する。共有失敗記録への反映を管理に引き継ぐ。その他認識した失敗はない。
