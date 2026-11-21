# 合成資格情報テスト用ランチャーの独立レビュー

- 担当 `/root/interactive_launcher_review` は作者 `/root/interactive_launcher` と別。作業場所 `D:/Projects/GithubIssueWidget`。依頼された結果、cmd、PS1 の期待・実測 SHA256 はそれぞれ `14B7BAD098ABE1CFCEE9E5FFD787C0E2C2B0FFC4EE70A1B938782E65BE835F03`、`933288B08C80770C771A45ED07477A29B1F12BDCC9EFE7D5D66C429C95C34E41`、`8A6D3142B328B4BA08DD5B75D6565EDEB45A64628BEA22950CD16846D9A3A811` で一致した。未追跡の実体を確認した。
- **対象版を承認、blocking なし。** 静的照合と保存された作者の結果を対象とし、実行許可・実 Credential Manager の成功・Issue #3 の完了を承認しない。

## 完了条件と根拠

1. **対象・安全境界:** 承認済み `interactive-test-review-02.md` と `実行手順.md` を参照。PS1 は固定絶対ディレクトリと Python パスを使用し、ディレクトリ・実行対象４ファイルの実体と reparse 属性、ファイルの SHA256、one-shot marker の不存在、Python 3.9 以上を検査し、失敗時に停止する。各対象４ファイルの現在の実測 SHA は PS1 と手順の値に一致。対象検査と利用の間の変更、親ディレクトリや Python 実体の全安全性は保証しない。
2. **明示操作と停止:** 事前説明は PAT を使わない合成値の一時保存・更新・削除と、結果不明時の再実行・手動削除の禁止を記載。`Read-Host` で大文字の `YES` のみが実行へ進み、それ以外は終了2。supervisor の非0終了や例外は失敗として終了1、表示された state・exit・run ID を報告するよう案内する。supervisor のソースは対象 EXE の SHA・one-shot の予約、子登録後の grant、待機・終了記録を実装し、終了0には子の completed/exit0 を要求する。後続の競合や異常停止を完全防止する証拠ではない。
3. **cmd と文字コード:** cmd は固定 Windows PowerShell と PS1 の絶対パスを `-NoProfile -ExecutionPolicy Bypass -File` で呼び、終了コードを保持・表示・伝播し pause する。実体の PS1 に UTF-8 BOM `EFBBBF` を確認し、日本語文字列を `Get-Content -Encoding UTF8` で読めた。Windows PowerShell 5 の実際の対話表示・通常ユーザーの Python 利用可能性は未確認。
4. **合成試験:** `launcher-synthetic-01/test_launcher.py` と `test_cmd.py` の SHA は作者結果の値と一致。前者は PS1 の実 supervisor 呼出しを無害な到達マーカーに置換し、正常・拒否・対象不存在・SHA 不一致・marker 既存・Python 不在の７ケースを別プロセスで検査する。後者は偽 PS1 の終了7について cmd の表示と伝播を検査する。作者は結果を７ケース成功・cmd 成功と報告するが、実行ログは別ファイルに保存されず、当担当は元プロセスを監督せず再実行もしていない。実 supervisor・実 Credential API・対話画面の証拠ではない。
5. **変更と削除:** 限定 `git status --short` は cmd、PS1、作者結果を未追跡と示す。作者は削除対象なしと明記し、レビュー依頼にも削除対象の指定はない。全 workspace や ignored の削除有無を網羅確認したものではない。

## 未確認・作業結果

- 新しいランチャー版の実画面、通常の Windows ログオン環境での Python と Credential Manager の可用性、実 OS 試験と異常後の復旧は未検証。合成テスト成功でも実アプリの PAT 登録・更新・削除、実 WebView の非露出は証明しない。
- 作者が記録した合成ハーネスの初回 exit1 と別コマンドの引用誤りは、終了コードと原因未確定の区別を維持する。共有失敗記録への反映は別途必要。
- 適用指示と実体を読み、`Get-FileHash`、BOM のバイト確認、限定 `git status` を実行した。各コマンドは終了0。Git は個人 ignore 設定の読取拒否を警告した。実 supervisor・実 Credential API・製品テストの再実行は行わず、新たに認識した失敗はない。変更は本レビューのみ、削除対象なし。
