# 合成資格情報モデルの独立レビュー 01 — 対象版を承認

- 担当 `/root/credential_harness_review` は作者 `/root/credential_harness` と別。依頼 `credential-harness-review-request-01.md` の対象について期待・実測 SHA256 は、結果 `457544D7E19EAAC080D151C769539BCF9DFD01F05963A1F0151A81E4E56CC93C`、`harness.py` `76DCF04A4A4B3827DC50860F75E9221F7E344FED3EF1BC95CB9418F8EB5CD51C`、`test_harness.py` `D960994042FF0D4750E073F0BA549B65178730075196585969E3A1028A3B5D67` で一致。承認はこの版とモデルとして確認した範囲に限る。blocking はない。
- 一次設計 `credential-safety-design-01.md` の SHA256 は `76057E5039F7BA536545B4F0C0247B000B32C24B7C9CD5D6CF51CBEFF789E6ED`。その独立レビューを参照。モデルは後続の実 OS 書込みの認可・安全性・アプリ完成を証明しない。

## 完了条件と確認

1. 対象変更: `git status --short` は結果と `credential-harness-01/` を未追跡として表示し、実ファイルと SHA を確認した。作者結果は削除対象なしと明記。レビュー依頼に個別の削除対象指定はない。workspace 全体の削除有無を網羅確認したという意味ではない。
2. 故障範囲: ソースとテストを照合し、正常な保存・更新・削除、事前 gate と journal 障害、同名衝突、保存・更新・削除の前後障害、読取り不明、指定した前後停止、登録・監督者・実行体の模擬状態変化、再開と cleanup の拒否を確認した。停止や gate は同期的な模擬で、実プロセスの強制停止ではない。初回 journal の排他作成、flush、fsync、読戻しは実ファイルを使うが、ACL、ディレクトリの永続性、電源断での原子性は確認しない。
3. 対象範囲と秘密: `MemoryBackend` は辞書への操作であり、対象の二つの Python ファイルに OS credential API、ネットワーク、子プロセスの起動経路は見当たらない。合成値 A/B は journal に含まれないことを正常ケースで検査する。全例外・依存実装・実ログについての秘密非露出を保証しない。実アプリ、実 PAT、Credential Manager、実 IPC に触れていない。
4. 作者の元実行はツール出力のみで保存ログがないため、独立した新規 `credential-harness-review-sandbox-01/` に対象の2ファイルをコピーし、SHA が対象版と一致することを確認。作業ディレクトリをその専用領域、TMP/TEMP も同領域に設定し、`python -B -m unittest -v test_harness` を実行。終了コード 0、9 tests、OK。ログ `review-test.log` SHA256 `1ECA4F8FA70381169F120CCD5B38EC680CBAEFDC986477EF0C62569FF6E19801`、終了記録 `review-exit.txt` `5FECEB66FFC86F38D952786C6D696C79C2DBC239DD4E91B46729D73A27FB57E9`。終了コードは当レビューの実行直後に保存した。テストの一時領域は標準ライブラリが作成・削除した。作成したレビュー領域と証拠は保護し、削除対象に指定しない。

## non-blocking と後続

- モデルの `stop_at` は save/update/delete の前後だけを列挙し、全 read 段階の前後や実監督者停止、タイムアウトの OS 挙動は試していない。結果文書は同期的模擬であると限定している。実 OS 試験に進む前に、独立実装、対象識別・排他・ACL・実プロセス監督と故障時の復旧証拠を別途確認する。
- `Journal.record` の後続段階は通常の上書きであり、異常停止後も完全な journal が残ると断定できない。結果文書はこの限界を明示する。既存資格情報や実アプリへの影響を防ぐ隔離条件が証明できなければ実 OS 書込みを開始しない。

## 作業と失敗

作業場所 `D:/Projects/GithubIssueWidget`。対象の `Get-FileHash`、限定 `git status --short`、設計・モデル・試験の読取りと上記専用領域の試験を実施。Git の個人 ignore 設定読取りに permission denied の警告が出たが対象状態を返した。認識した作業失敗はない。製品・Git・GitHub・資格情報・他者の成果を変更せず、実アプリを起動していない。
