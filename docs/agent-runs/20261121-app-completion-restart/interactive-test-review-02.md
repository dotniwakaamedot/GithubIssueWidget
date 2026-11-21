# 対話セッション用合成資格情報テストの独立レビュー 02 — 対象版を承認

- 担当 `/root/interactive_test_review` は修正作者 `/root/interactive_test_prepare` と別。作業場所 `D:/Projects/GithubIssueWidget`。対象 `widget/credential-check-20261121/実行手順.md` の期待・実測 SHA256 は `A6F45828F142FDDF7B29F2BEEE7AC9FDCC9428E3A7281BC4CF1A29236DF0EB97`、`interactive-test-guide-fix-02.md` は `E76C09AC0123C84F72D44C06EC5689E9367169767890CEF58A600AEF278164F6`。前回レビュー `interactive-test-review-01.md` の対象とは別版。
- **対象版を承認、blocking なし。** 前回の `Set-Location` 失敗後に旧ディレクトリの相対スクリプトを実行し得る問題は、絶対パスと停止条件を設けた本版で解消された。承認はこの手順版と照合範囲に限り、実 OS テストの成功・Issue #3 の完了を承認しない。実行可否は親が判断する。

## 条件と根拠

1. **実行対象と失敗時停止:** 手順の PowerShell ブロックは固定絶対ディレクトリ・Python・supervisor を使い、`Set-Location` や `./supervisor.py` を使用しない。ディレクトリ・対象ファイルの実体、各 SHA、reparse point 属性、one-shot の不存在、Python 3.9 以上を確認し、失敗は `catch` で止める。実行前確認と利用の間に他者がファイルを変更する競合や、親ディレクトリを含む全環境の安全性を保証するものではない。
2. **対象版:** supervisor、recovery、diagnostic summary、EXE の実測 SHA は手順内の各値と一致。試験結果 `guide-safety-01.log` は記載 SHA `5A129AEC71F09E67B252D5167F66985AC3145C356A04FCC69FF1637862ADFAE9` と一致し、正常ケースのみ到達、ディレクトリ不存在・Python 不在・ハッシュ不一致・marker 既存は未到達と記録する。作者による無害マーカー置換試験であり、自分は元のプロセスを監督・再実行していない。対象 one-shot は確認時点で存在しなかった。
3. **以前のレビューと差分:** `interactive-test-review-01.md` で照合した supervisor・Rust・EXE・recovery 等は同じハッシュを維持。元結果の `RECOVERY.md` のハッシュ転記誤りは `interactive-test-prepare-correction-01.md` で訂正され、実体・コピー元とも `F2F2D33AA2B3BC4CDC057384E38F2FBCF1BDAE10199D70946A5868C9A5B47E45` と一致する。手順ファイルの旧 SHA を本版に流用しない。
4. **本人への案内と限界:** 手順は PAT を使わず一意な合成値を使い、正常候補を `state=completed` と `exit=0`、異常・不明時は再実行と推測削除をせず記録を保持すると説明する。削除後 NoEntry は対象テスト内の確認であり、その後の同時変更の不存在を保証しない。本人の通常 PowerShell から指定 Python を使用できるか、Credential Manager が利用可能か、Windows 1312 が再発するかは未確認。合成 OS 試験は実アプリによる PAT の登録・更新・削除や実 WebView IPC の代用ではない。

## 実施・未確認

対象２ファイルと対象実体の `Get-FileHash`、UTF-8 の手順・結果・ログ読取り、one-shot 存在確認、対象限定 `git status` を行い、各処理は終了0。Git は個人 ignore 設定の読取拒否を警告したが対象状態を返した。未追跡ファイルの実体を確認し、作者は削除対象なしと記載。本レビューでも削除していない。実 credential API、アプリ、テスト起動、GitHub 書込みは実施していない。今回認識した新たな失敗はない。
