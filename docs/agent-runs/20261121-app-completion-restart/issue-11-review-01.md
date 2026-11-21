# Issue #11 の実機・公開履歴の独立レビュー 01 — 対象版を承認

- 担当 `/root/issue_11_review` は結果作者 `/root/issue_state` および観測記録の作成担当とは別。依頼 `issue-11-review-request-01.md` の対象3件は未追跡の実体を確認し、SHA256 は依頼と一致した。`issue-11-state-04.md`: `44F629CB8B2E14ECB0FB8FC82A541D17A97022F4BD549734BB82D4ABAC5D2DF3`、`user-observation-07.md`: `F2B593C7B5093AA225F4DCF85395D117C902319ACDA3FEAF847ACD09391BBC67`、`user-observation-08.md`: `261EC6EDAF2A30209D4646FB7FE28CEB4DBE4D2847E5A980DDFAACCF3EF5B483`。承認はこの版に限る。
- **blocking なし**。実機観測と公開履歴の対象記録に対する承認であり、アプリや Issue #6 全体の完了承認ではない。

## 要件と独立確認

1. 公開 API の `GET /repos/dotniwakaamedot/GithubIssueWidget/issues/11` は HTTP 200、`closed`、ラベル `status:done` のみ、`closed_at` は `2026-11-21T06:13:25Z`。非 status ラベルは観測時点ではない。
2. `/issues/11/events?per_page=100&page=1` は HTTP 200、26件、`Link` ヘッダーなし。今回の匿名公開応答上は1ページで終端した。隣接する status ラベル変更から、Todo→In Progress、In Progress→Todo、Todo→Done、Done→Todo、In Progress→Done、Done→In Progress の全6方向に対応する変化を照合できた。Done との往復には close/reopen も観測された。
3. 事前の合成確認では正常終端、正しい次ページ、不正ホスト、逆行の各リンクを確認した。実応答には次ページがないため、実後続ページ取得・途中通信失敗は検証していない。各 API GET は別時点であり、取得中や取得後の同時変更、匿名利用者に見えない対象は保証しない。
4. `selection-build-sandbox-01/cargo-target/release/github-issue-widget.exe` の実体 SHA256 は `0D8AB47088B7A19233EF0D7D08CF033B681B20CA0E729FEAF73EAB74DCC98F55` で本人報告の対象記録に一致。画面の現象改善・操作は本人報告であり、当担当の直接観測ではない。EXE の現在の実体一致は操作時のメモリ内実体の直接証明ではない。

## 制約・未確認

イベントはラベル追加・削除・状態変更が別々のため、原子的な単一 UI 操作の直接記録ではない。非 status ラベルは対象 Issue に存在せず、その保持は実機で検証できていない。実認証失敗、通信障害、競合、全 Windows 環境やキーボード・フォーム操作は本レビューの証明範囲外。対象記録はいずれも未追跡で、削除対象は指定されていない。

## 実施結果

作業場所 `D:/Projects/GithubIssueWidget`。`Get-FileHash` と限定 `git status --short` は終了0。Git 個人 ignore 設定の読取りは権限拒否の警告が出た。Python 標準ライブラリによる匿名 HTTPS GET、型・リンク確認と必要項目のみの抽出は終了0。実アプリ、資格情報、Issue、GitHub、既存資料を変更していない。対象結果と EXE を読み取り、削除対象なしの記載を確認した。認識した作業失敗なし。
