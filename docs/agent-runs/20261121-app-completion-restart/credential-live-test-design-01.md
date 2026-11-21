# 合成資格情報の実 OS 検証経路 — 未完

担当 `/root/credential_live_test_design`。実 Credential Manager・実 PAT・実アプリ・GitHub には触れていない。依頼の専用領域 `credential-live-test-01/` に Rust の試験候補と runner を作成したが、**模擬テストも live 機能のコンパイル確認も未成立**であり、実 OS 試験を実行可能な承認済み成果ではない。

## 候補の構成と対象

- `src/main.rs` SHA256 `DB754FE76B46E6FE7C84F2EC021AD506611D91DDA74225398B97E5F7328D5B1E`。既存アプリの `keyring` 3 系・`windows-native` を想定し、専用 service と固定 account、合成値 A/B に限る候補。`LIVE_APPROVED=false` と Cargo の既定 feature 無効により、現版での通常起動は実資格情報へ進まない。未コンパイルの静的な記述であり、安全性を実行で証明したものではない。
- `Cargo.toml` SHA256 `38A06A4CCBAE1312682033DE8741DBFBB65C9BFAE40129F85B6C80A546583DA1`、`verify.py` `98CED489F50BB37EE559E9D2F417B834556D130B240117208B050008B5414575`、`verify-02.py` `329AED5D2380DFC81F31D5A9A31A2DEE54111E88778794C420E4DAAE11F61312`。親の対象版確認 `windows-ipc-feasibility-review-01.md`、`widget/src-tauri/Cargo.toml` と `main.rs` の既存 guard を参照した。本番コードは変更しない。
- service は専用 prefix と UUID、account は `synthetic`。候補は `create_new` で journal を作成し service/account を書き、flush・`sync_all`・読戻しを経てから状態遷移する。各外部操作の意図を journal に追加・同期し、`NoEntry` 以外の初期値は衝突として停止する設計。更新後 B の完全一致を確認してから削除し、削除後不存在を確認する。いずれかのエラーは再送・自動 cleanup せず停止し、journal に残る対象と段階を人が確認する。
- 存在確認と書込み・値確認と削除はそれぞれ別操作であり、UUID は衝突確率を下げるだけで原子的所有権や排他を保証しない。journal の `sync_all` は親ディレクトリの耐久性・ACL を保証しない。実行前には対象パスの workspace 内限定、同時書込みの排除、親・子孫停止、異常終了や対象不一致時の復旧手順を設計し独立確認する必要がある。

## 試験と失敗

1. `python .../credential-live-test-01/verify.py` は `cargo test --offline` の依存解決で終了 101。ログ `mock-test.log` SHA256 `C1133063DC6B4587EEABE9D0EF5180B0F7AEC8846610C15B549AA0AF26F9C3FD` は `no matching package named keyring found` を記録。試験バイナリは起動していない。
2. 先行隔離ビルドの registry を専用 `cargo-home` にコピーし、`verify-02.py` で再実行したが、同じ keyring 依存解決エラー・終了 101。ログ `mock-test-02.log` SHA256 は上記と同一。**同種失敗２回のため当該作業を中断し親に別担当による独立調査を依頼した。** 原因は未特定で、再試行していない。
3. 初回・再実行とも依存解決前後で試験本体・live 実行には到達せず、模擬8件と feature `live` のコンパイルは未実施。コピーした registry・ログ・生成物を削除せず保持する。先行 build/cargo-home は読取り元であり変更していない。

## 独立レビュー前に解消すべき点

- 現行 `main` は禁止時にメッセージを出して通常 return するため終了コードだけで成功と誤認し得る。実行可能な版にする前に fail-closed な非0終了と安全な監督条件を検討する。
- journal パスは現在のコードでは引数として任意に渡せ、workspace 内を強制しない。実行前に正規化・所有範囲・新規作成と ACL・復旧証拠を検証する必要がある。
- テストは単一プロセスの `Store` 模擬を想定し、実 keyring・実 WebView・本番 IPC や実 PAT を検証しない。現版は異常停止時の子孫監督、競合防止、OS 障害時の無条件終了・耐久性を証明していない。実 OS の試験は別の明示的な判断と独立レビューが必要。
- この結果文書・専用領域のみが本担当の成果。未追跡を含む既存資料・本番・他者の作業は変更していない。失敗記録と教訓への反映は親・独立分析担当へ引き継ぐ。
