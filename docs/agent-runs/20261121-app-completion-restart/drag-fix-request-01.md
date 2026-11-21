# HTML ドラッグ設定の修正依頼

- 担当予定 `/root/drag_fix`。親 `/root` が起動。作業場所 `D:/Projects/GithubIssueWidget`、共有 run `D:/Projects/GithubIssueWidget/docs/agent-runs/20261121-app-completion-restart/`。
- 根拠: `drag-investigation-01.md`。現行 Tauri の Windows 向け設定既定値と HTML5 drag の競合が有力候補であるが、実 UI の直接原因は未確定。現行 `widget/src-tauri/tauri.conf.json` SHA256 `5C1F3A0E2E487DD4345A2DD335FE4AACA483FEEEA26C11AAB99B374278EC1364`。
- 作業: `app.windows[0].dragDropEnabled` を false とし、設定の妥当性・既存のネイティブファイルドロップ機能への影響を調べる。変更前の実体・ハッシュ・構造を確認し、相違時は変更せず質問。ほかの設定を保持する。目的は実 UI のカード移動の候補修正であり、実機検証前に解決済みと断定しない。
- 所有: 製品は `widget/src-tauri/tauri.conf.json` のみ、結果は `drag-fix-result-01.md` のみ。別担当が同じ checkout で調査中、他者の変更を戻さない。`.git` は読み取り専用。実 GitHub・資格情報を操作せず、アプリの同時起動をしない。
- 検証: JSON の構文・変更点とファイルドロップ参照の範囲を記録。独立ビルド・レビュー・ユーザーによる実 UI の検証は後続担当が実施する。対象 SHA、実行・終了結果、未確認を確定ファイルへ記録し、作業後は書込み停止・パスだけ通知。
