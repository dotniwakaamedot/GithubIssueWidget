# 合成資格情報の実 OS 試験失敗：独立調査

担当: `/root/credential_os_failure_analysis`。作業場所: `D:/Projects/GithubIssueWidget`。本調査は実 OS 書込み・削除・試験再送を行わず、対象１件の読取り診断のみ実施した。

## 結論

２回目の診断版は `operation=write`、`kind=no_storage_access`、Windows コード **1312**、子 `completed`・終了 1、最終 stage `before_write_a` を記録した。Windows の `ERROR_NO_SUCH_LOGON_SESSION` は指定されたログオンセッションが存在しない、またはそのセッションに資格情報セットが関連していない場合を示す。Microsoft の [CredWrite の公式文書](https://learn.microsoft.com/en-us/windows/win32/api/wincred/nf-wincred-credwritea) を参照。これにより診断版で記録されたエラーは識別できたが、セッションがその状態になった原因は未特定である。１回目は標準エラーが破棄されており、同じコード・同じ直接原因とは断定できない。

２回目の記録済み合成ターゲットに対する読み取り専用 inspect を１回実施し、`absent`、終了コード 0 を得た。観測時点で当該ターゲットは見つからず、資格情報の書込み・削除や元試験の再送は行っていない。absent は終了後の残留に限る証拠であり、過去の書込み成否・既存 credential 全体・別セッションを証明しない。

## 根拠と手順

- run `67c06448b9734163a1949e63d740c6e7`。実行担当の確定 `credential-diagnostic-execution-01.md` の SHA256 は `2F8930DC564331A52E009994959C513DE348025163A181CE050E428A465CD1FE`、親指定と一致。前回 `credential-live-execution-result-01.md` は標準エラー破棄・最終 `before_write_a` を記録する。
- 診断版 `diagnostic_summary.py`、`recovery.py`、`supervisor.py` の SHA256 はそれぞれ `25C683CB1057CC3E0C77EDC4FF1984EC13502F89160C80A3D4F64A576F499BB0`、`04A3911C622E09B1416962C388972443041916D6E06A14C0FB7F01646B86A302`、`D603D89B148A652AC0E54014E3AC63F78F75362D8554B07C09069E1D2FFA34F7` で診断版結果と一致。
- `python -B diagnostic_summary.py <run-id>` は終了 0、上記の固定分類と数値のみを出力。プログラムは `read_identity` で対象 EXE・one-shot・start・child・outcome・journal を照合し、Credential Manager を呼ばない。対象値・本文は出力していない。
- `read_identity` の限定出力は有効、子 PID 62776、`completed`・exit 1、stage `before_write_a`、inspect 記録なし。`Get-Process -Id 62776` は `pid_absent` を返した。PID 不在だけを過去の全子孫の終了保証とはみなさない。
- 承認済み `python -B recovery.py --inspect <run-id>` は target を識別後、値非表示で１回だけ CredReadW を呼び `absent`、終了 0。inspect outcome の SHA256 は `B6DF9AC5DD0CBF2E0E9E4CEF51523578CCB7AA0319A8092E8341C03202ED24BE`。記録以外への書込みや削除なし。

## 次の対応と限界

自動実行側の Windows ログオンセッションと、本人がアプリを操作した対話セッションが同一で Credential Manager を利用可能か未確認。セッションや権限の違いは候補であり、エラー1312だけから特定しない。本人操作による専用テスト環境、または必要な権限を持つ対話セッションにおける事前識別・安全な監督・復旧条件を整えて検証する。元の合成ターゲットの再書込み、未知ターゲットの削除、資格情報の値の共有は不要。実アプリの PAT 保存・更新・削除、WebView IPC の非露出、アプリ完成は未検証。

追加成果は本結果と上記 run の inspect-start / inspect-outcome のみ。削除対象なし。本調査中に認識した失敗はない。結果自身の SHA256 は公開後に別途取得する。
