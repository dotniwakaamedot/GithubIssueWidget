# 文字選択抑制の独立レビュー依頼

- 担当予定 `/root/selection_review`。親が起動。共有 run `D:/Projects/GithubIssueWidget/docs/agent-runs/20261121-app-completion-restart/`、作業場所 `D:/Projects/GithubIssueWidget`。作者 `/root/selection_fix` と別担当。
- 対象 CSS `widget/src/style.css` SHA256 `FFE871A15BDA8FAD0BB5E28E1717C93D2878ABB1F5885F730EC624DEC983FCFE`、結果 `selection-fix-result-01.md` SHA256 `A074FF9649AC53377EEEAD34C4CDAA784E5E0E18D0B0DCDFC5AD36BBE9C9FC88`、維持すべき `widget/src-tauri/tauri.conf.json` SHA256 `0132FCC92345F5D60DE84FD25197FCE853044AD63DAE28757AB4592CC72B991E`。
- ユーザーは実際には Issue は1件のみ動き文字列が複数選択に見えると報告。対象版の差分、実際の JSX・CSS、設定フォーム・作成フォーム・リンク・キーボード操作への影響と局所性を照合する。実 UI 効果を静的読取りだけで成功としない。既存ドラッグ設定を保護する。
- `selection-review-01.md` のみ所有。製品・設定・他者成果を変更せず、対象 SHA、blocking、未確認を記載。実データを UTF-8 で読戻して確定しパスのみ通知。必要な取得は出力を限定する。
