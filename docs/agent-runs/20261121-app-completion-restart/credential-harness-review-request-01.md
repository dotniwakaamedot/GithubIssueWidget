# 合成資格情報の安全境界モデル・試験の独立レビュー依頼

- 担当予定 `/root/credential_harness_review`。親 `/root` が ID を照合して起動。作者 `/root/credential_harness` と別。共有 run `D:/Projects/GithubIssueWidget/docs/agent-runs/20261121-app-completion-restart/`、作業場所 `D:/Projects/GithubIssueWidget`。
- 対象 `credential-harness-result-01.md` SHA256 `457544D7E19EAAC080D151C769539BCF9DFD01F05963A1F0151A81E4E56CC93C`、`credential-harness-01/harness.py` `76DCF04A4A4B3827DC50860F75E9221F7E344FED3EF1BC95CB9418F8EB5CD51C`、`credential-harness-01/test_harness.py` `D960994042FF0D4750E073F0BA549B65178730075196585969E3A1028A3B5D67`。実体と SHA を照合し、`credential-safety-design-01.md`・レビューと照らして各故障ケースの実際の検証範囲、ログ・終了証拠、実 OS 資格情報に触れる経路がないこと、OS 排他・ACL・子プロセス監督・耐久性が未実装であることを確認する。
- テスト出力が作者のツール結果だけの場合は再確認可能な証拠と区別し、必要な場合に限りモックのみを別専用領域で再実行する。実 Credential Manager、実 PAT・設定・アプリ・GitHub を操作しない。モデルの合成結果を OS 実試験または実アプリの完成の証拠としない。
- 新規 `credential-harness-review-01.md` と必要な新規専用レビュー領域のみ所有。他者のコード・記録・製品を変更しない。対象版、根拠、blocking、未確認と後続を記録し、UTF-8 実データ読戻し後に一時ファイルから確定、パスのみ通知。出力を絞り同種失敗2回で停止して親へ報告。
