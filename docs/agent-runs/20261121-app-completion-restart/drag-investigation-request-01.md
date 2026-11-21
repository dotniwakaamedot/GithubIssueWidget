# 実 UI のドラッグ不能の独立調査依頼

- 担当予定 `/root/drag_investigation`。親 `/root` が起動。共有 run `D:/Projects/GithubIssueWidget/docs/agent-runs/20261121-app-completion-restart/`。
- 事実: ユーザーが現行指定 release で検証 Issue #11 のドラッグ＆ドロップができないと報告。対象 `widget/src/main.tsx` SHA256 `B22504C9DE48D275726C912820DA93E02F10C1838CCC68555C156D35030165D7`、対象 release SHA256 `D19A72750006DF5BB6CBCA4FB15F87F69BFF48A5785F5569C5B05E40C9B93F26`。ユーザーの具体的な操作方法・イベント到達・内部原因は未確認。実 GitHub の書込み状態は別担当が読取り調査。
- 作業: Issue #6 の原文、現行 UI と CSS の drag 開始・drop・ウィンドウドラッグ領域の干渉、テストと現行配布版の対応、既存 F-015/F-034（ウィンドウ操作と Issue カード移動の区別）を読取りで確認。再現を推測せず、想定原因・識別に必要な短い確認操作・コード修正案と検証を明記。別担当が UI 操作の案内を調査中なので結果を尊重。
- 所有 `D:/Projects/GithubIssueWidget/docs/agent-runs/20261121-app-completion-restart/drag-investigation-01.md` のみ。アプリ起動・実 Issue の変更・既存ファイルや他者の変更をしない。秘密は出力しない。検索対象と上限を限定し、同じ失敗2回は該当作業を止め別担当へ引き継ぐ。確定後パスのみ通知。
