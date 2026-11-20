# 競合テスト実装の実行前レビュー 05

- 担当: `/root/snapshot_review`。依頼 `race-code-request-05.md` の対象 `widget/src-tauri/src/main.rs` SHA256 `7052631058511BBBBC70477CE4A8D807444CF8E9DCC64828283C06CF78CA5AE2` は開始時と終了時に一致。
- 判定: **対象版の限定したテスト実行前レビューを承認**。前回の無期限 `child.wait()` は除去され、終了確認が不明な場合に合成資格情報を保持する構造を確認した。これは実行結果、製品の最終版、実 UI や GitHub との統合、親 Issue 完了の承認ではない。レビュー回数を承認理由とはしていない。

## 前回指摘の確認

- 子監視の 40 秒期限を超えた場合、`kill()` 後も最大5秒の `try_wait()` で終了を確認し、成功・失敗・期限超過が不明な場合は `None` とする。子または server / changer スレッドの終了を確認できないときは、`cleanup_allowed` を false のまま panic して合成 ID と fixture を保持する。
- テストは未実行のため、Windows の実プロセス終了・期限超過・保持時のログ内容と復旧は実証していない。強制終了等で親の後始末が走らない場合にも、事前 flush した合成 ID・作成段階のログを基に状態を確認する必要がある。

## 旧安全条件の照合

- `cleanup_allowed = false` は server / changer 起動前に設定され、子と両スレッドの終了確認後だけ true に戻る。明示 cleanup 前には再び false にし、成功時のみ `cleanup_done = true` とするため、明示 cleanup 失敗時に Drop が同じ cleanup を再実行しない。
- 親が未使用 UUID と一時 root を作成し、Credential Manager の合成値と一致するエントリだけを削除する。子の marker、許可されたディレクトリ内容、復旧記録不存在、settings path と隔離 APPDATA の照合が維持され、各 `finish_board` の前にも fixture を確認する。marker は同一ユーザーの故意の操作に対する権限境界ではない。
- 合成トークンの比較は固定文言の `assert!` であり、値を `assert_eq!` の panic 出力へ展開しない。ID と段階は credential 書込み前に flush する。
- ローカル loopback、proxy / redirect 無効、非表示の子、read/write・受信・スレッド待機の期限、ロック解放後の test-only フック、親の明示パスによる設定更新、正常系後の競合系を静的に確認した。実動作と実行後 cleanup は別途確認する。
- Cargo.toml SHA256 `82A440BA35FA24E315B3A458721021A43CC7CB2EEE39D1968B0C9B52BADA0540`、Cargo.lock `E81559E3D646FE6B25220590E64B5F3CAB3ACAAECDB120392E7EB8285E6B5AD9` は計画と一致する。

## 実行後に残る確認

実行する場合は承認版 SHA、単独テストの終了コードとログ、親・子・server / changer の終了、両合成 credential の不存在、未終了または cleanup 失敗時の保持・報告を確認し、結果を独立レビューする。`list_issues` の Tauri 入口全体、実 WebView、実 GitHub、`save_connection` 全体、最終照合後の別プロセス変更はこの試験の証明範囲外。

今回、製品と依頼・計画を読み取りで確認した。テスト・ビルド・GUI・資格情報・外部サービスへの操作、製品コードの変更は行っていない。認識した失敗はない。
