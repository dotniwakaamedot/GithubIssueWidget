# 文字選択抑制版ビルドの独立レビュー依頼

- 担当予定 `/root/selection_build_review`。親 `/root` が担当 ID を照合して起動。作者 `/root/selection_build` とは別担当。共有 run `D:/Projects/GithubIssueWidget/docs/agent-runs/20261121-app-completion-restart/`、作業場所 `D:/Projects/GithubIssueWidget`。
- 対象 `selection-build-result-01.md` SHA256 `471185044A0D807DE1AF2D8799CE0C9095D27334ABCB7A1E2E69314CB43E371D`、EXE SHA256 `0D8AB47088B7A19233EF0D7D08CF033B681B20CA0E729FEAF73EAB74DCC98F55`。入力一覧、CSS・設定と隔離コピー、ビルドログ、終了記録、生成 EXE の実体と SHA を結果に基づき照合。対象 CSS・設定が修正対象版であること、実際の成果との対応、未追跡の素材を確認する。
- 既存 release・前ビルド・他者の成果を変更しない。ビルドやアプリを再実行せず、成功ログと保存された終了コードの証拠範囲を明示。実 UI のドラッグ像・入力・リンク等は未確認とする。必要な対象と出力量を絞り、同種失敗2回は停止して親へ報告。
- 新規 `selection-build-review-01.md` のみ所有。対象 SHA・blocking と未確認・確認結果を記録。UTF-8 実データ読戻し後に一時ファイルから確定し、パスのみ通知。他者の変更を戻さない。
