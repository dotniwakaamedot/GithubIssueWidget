# PowerShell 読戻しアンカー失敗の独立分析依頼

- 担当予定 `/root/concat_failure_analysis`。親 `/root` が ID を照合して起動。共有 run `D:/Projects/GithubIssueWidget/docs/agent-runs/20261121-app-completion-restart/`。
- 対象は `/root/readback_encoding_analysis` の `readback-encoding-analysis-01.md` と `/root/credential_harness_review` の `credential-harness-review-question-01.md` に記録された PowerShell `String.Concat` の `ArgumentNullException` 各1件。ユーザーの同じ失敗2回の指示に従い別担当が独立分析する。元の失敗操作は再実行せず、各コマンド・引数の具体的な相違・失敗境界・終了状態・代替確認と未確認を本人の記録・必要に応じた質問に基づき区別する。
- ファイル破損を例外から推測しない。複合コマンドの終了コードと個別処理の成否を区別し、既存教訓、代替経路、再発防止の検証可能な方法を明記。合成試験を実施する場合は既存ファイルを変更せず対象を限定し、本来の実データ破損有無の証拠と混同しない。
- 新規 `concat-failure-analysis-01.md` のみ所有。共有失敗記録、製品・資格情報・実アプリ・GitHub は変更しない。他者の変更を戻さない。UTF-8 実データを読戻し一時ファイルから確定してパスのみ通知。出力を絞り同種失敗2回で停止し親へ報告する。
