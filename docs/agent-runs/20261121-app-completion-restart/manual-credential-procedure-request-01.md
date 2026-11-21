# 実アプリ資格情報の安全な実機手順の調査依頼

- 担当予定 `/root/manual_credential_procedure`。親 `/root` が ID を確認して起動。作業場所 `D:/Projects/GithubIssueWidget`、共有 run `D:/Projects/GithubIssueWidget/docs/agent-runs/20261121-app-completion-restart/`。共有環境の他者変更を戻さない。
- 背景: Issue #3 と `completion-audit-01.md` に実アプリの Credential Manager 登録・更新・削除・再起動後取得の未検証がある。本人は既存接続で再起動できたと報告したが初回・更新・削除の証拠ではない。`windows-ipc-adapter-connect-result-01.md` は隔離 mock 接続を未実施と記録。親が本人に専用テスト用トークンでの協力可否を問い合わせ中で、回答はまだ未確定。
- `widget/src-tauri/src/main.rs` SHA256 `AEFF1E65AD93EFA04773CEBBB9736A6BDC5B9938DB9FAF937DD88B3D9CBFE86F`、README SHA256 `552827810A83303D430C143626E84ED4FAD5C94693A9CBBB3163071930778112`、現行 EXE と本人操作範囲を読み、設定と credential の service・保存先・更新・削除・復旧を実装と Issue #3 の一次要求に照合。専用 Windows ユーザー等で既存アプリの資格情報を保護できる実現可能性を検討し、本人だけがトークンを入力する最小操作、事前条件、各期待表示、保存・再起動・更新・削除の順序、終了後の安全な状態、失敗・不明時の停止と未検証を具体化する。専用アカウントが使えない場合は既存資格情報を削除・上書きせず保留し、必要な条件を示す。既存 PAT の値を読み出さない。
- 実施者は読取り調査のみ。実アプリ、Windows アカウント、Credential Manager、GitHub、設定を操作せず、製品・既存記録は変更しない。実 OS の排他や検証済みを推測しない。新規 `manual-credential-procedure-01.md` のみ所有。絶対パス・対象版 SHA・具体的手順と実施条件、未確認を記録。UTF-8 読戻し後一時ファイルから確定しパスのみ通知。出力を絞り同種失敗2回は停止・報告。
