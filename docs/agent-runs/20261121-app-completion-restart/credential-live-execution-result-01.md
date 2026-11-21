# 合成資格情報の実 OS 試験結果 01

## 結論

試験は **未成功**。監督プロセスは終了コード 1、直接子は `state=completed`・終了コード 1 で終了した。journal は `before_write_a` まで進んだが、その書込みの成否・合成エントリの残留は不明である。保存・更新・削除や削除後の不存在は確認できていない。再実行・診断・削除は行わず、one-shot marker と記録を保護した。

## 対象と前提

- 担当 `/root/credential_live_execute`。作業場所 `D:/Projects/GithubIssueWidget`。親から UUID 衝突の非ゼロな残余リスクを認識した限定実行を依頼された。
- 対象 `credential-live-ready-02/supervisor.py` SHA256 `D0D9EFDDC4812B9D9F22C2A3E714F8C491825F3DCC386F4DADA385C2D047421D`、`recovery.py` `04A3911C622E09B1416962C388972443041916D6E06A14C0FB7F01646B86A302`、`RECOVERY.md` `F2F2D33AA2B3BC4CDC057384E38F2FBCF1BDAE10199D70946A5868C9A5B47E45`、`src/main.rs` `214DEF1642AAA6BE30F4E5A5C95CEF3E62083084EAC617527E9A10DC6C509F1B`、live EXE `211CAD118123794B3C15C27E14F88BEC60B4AD0906ED9145AEF71939A2706910` は、実行直前に承認済み結果と一致した。Python 3.11.15。
- `credential-live-ready-02` の絶対パスは workspace 内に解決され、実行前の `live-once.json` と本結果の確定先は存在しなかった。対象 EXE の同一パスに一致する実行中プロセスは実行前に 0 件。限定確認であり全プロセスの網羅や権限の証明ではない。
- `RECOVERY.md` と `supervisor.py` の実行順序・停止条件を確認。合成 service のみを対象とし、既存 credential・実 PAT・アプリ設定を意図的に取得・変更していない。対象外へのアクセスの完全性をこの記録だけで保証しない。

## 実行と証拠

- 作業ディレクトリ `D:/Projects/GithubIssueWidget/docs/agent-runs/20261121-app-completion-restart/credential-live-ready-02` で `python -u supervisor.py --execute` を１回実行。監督の終了コード 1、出力 `run_id=dcdc5dfdf4524ffa8d9139ee5a781ae4 state=completed exit=1`。子 stdout/stderr は承認版の設定により破棄され、失敗の直接原因は不明。
- 同一 run の outcome は `state=completed`, `exit_code=1`, PID 8708。これは supervisor の直接子を wait した結果であり、実 OS 一般の障害への終了保証ではない。
- journal の対象識別子は service `github-task-widget-synthetic-check-82e61bb1-c162-4a83-b28a-0d2cfafd2ebd`、account `synthetic`。stage は `awaiting_grant`、`grant_received`、`before_check`、`before_write_a`。最終 stage は書込み開始前の記録であり、書込み成功・失敗のいずれも証明しない。
- `live-once.json`、start、child、outcome、journal を保持。現時点で `recovery.py` の inspect・cleanup は実行せず、追加の読み書き・再送・削除は行っていない。親に不明状態と run ID を通知した。

## 未確認と次の判断

合成ターゲットの現時点の状態、書込みエラーの直接原因、失敗時の OS 状態は未確認。再開する場合は `RECOVERY.md` の停止条件に従い、対象版・直接子の終了・完全一致対象を独立確認したうえで、親が明示的に診断を判断する。失われた stderr を復元したとは扱わない。実アプリ、実 PAT、実 WebView による資格情報の保存・更新・削除・再起動後取得は未検証である。

## 失敗と変更

実 OS 合成試験の終了 1 は今回１回の失敗。同じ試験を繰り返していない。失敗記録・独立分析と後続の可否は親へ引き継ぐ。変更は対象専用ディレクトリの実行記録と本結果のみ。削除対象なし。
