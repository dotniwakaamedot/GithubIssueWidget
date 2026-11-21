# ドラッグ設定修正の独立レビュー 01 — 対象版を承認

- 確認担当 `/root/drag_fix_review`、作者 `/root/drag_fix` とは別。対象 `widget/src-tauri/tauri.conf.json` の依頼・実測 SHA256 は `0132FCC92345F5D60DE84FD25197FCE853044AD63DAE28757AB4592CC72B991E` で一致。作者結果 `drag-fix-result-01.md` の実測 SHA256 は `9566E9DFE51A3B8266E6875B8210B30B78619349BE396DDD925D001EE70A54E4`（依頼に期待値なし）。承認は設定ファイルの上記版に限る。
- **blocking なし**。ユーザーが報告した障害の解決や Issue #6 全体の完了を承認するものではない。

## 完了条件と根拠

1. 対象版・差分: 作業場所 `D:/Projects/GithubIssueWidget` で対象実ファイルをハッシュし、Git 差分が `app.windows[0].dragDropEnabled: false` 追加のみであることを確認。HEAD 上の JSON と新 JSON から追加プロパティを除いた構造は一致。HEAD の原バイトは LF、Windows 作業版相当の CRLF に変換した基準 SHA256 は依頼の `5C1F3A0E2E487DD4345A2DD335FE4AACA483FEEEA26C11AAB99B374278EC1364` と一致。Git 状態は対象ファイルを変更済みと示す。
2. 妥当性: ローカル Tauri CLI の `config.schema.json` の WindowConfig は本項目を boolean、既定 true とし、Windows の HTML5 ドラッグには無効化が必要と説明する。JSON の構文と window が1件で当該値が false であることを確認。Issue #6 の公開 API を番号指定で取得し、カードの3列間移動を要求することを確認した。現行ソースのイベントや症状と整合する候補だが、直接原因は確定していない。
3. 影響: `widget/src`、`widget/src-tauri/src`、当該設定の TS・TSX・Rust・JSON に対する限定検索で Tauri ネイティブファイルドロップの参照は設定以外に見つからない。依存ライブラリや検索対象外は未確認。`dragDropEnabled: false` はネイティブファイルドロップ機能の利用には影響しうる。
4. 削除: 作者は削除対象なしと記録。対象差分にも削除されたファイルはない。未追跡の作者結果は実体と SHA を確認した。

## 未確認と次の確認

- 本担当はビルド、アプリ起動、ユーザー操作、GitHub 書込みを行っていない。新しい識別済み EXE のビルド結果と、実 Windows での #11 のドラッグ、移動後の GitHub 状態、ウィンドウ移動・サイズ変更・リンク等の回帰を確認するまで解決済みとしない。
- Issue #6 の全要件や検索対象外のコード・依存関係を本レビューは承認しない。

## 実施と失敗

`Get-FileHash`、JSON 読取り、`git status`・`git diff`、対象限定 `rg`、基準と新 JSON の Python 比較、および Issue #6 の公開 REST GET を実施。各コマンドは終了0。Git の個人 ignore 設定は読取拒否の警告が出たが、対象状態と差分は返った。

Issue API の最初の表示では Python と PowerShell の文字コード不一致により日本語が表示上文字化けした（1回）。API 応答のデータ破損とは判断していない。同じ読取を ASCII エスケープ JSON として再取得し、HTTP 200 と Issue 内容を確認した。文字化けの失敗記録と教訓反映は管理へ引き継ぐ。変更は本レビューのみ。削除対象はない。
