# GithubIssueWidget

GitHub Issue を Windows デスクトップのウィジェットとして表示するアプリの開発プロジェクトです。

## 現在の状態

現在のファイルは別プロジェクトから持ち込まれた AI ワークフローの資料と連携素材です。アプリ本体はまだ含まれていません。対応 OS は Windows 11 です。技術構成と要件は GitHub の Issue に整理しています。ビルドと起動手順は実装後に追記します。

## 開発時の資料

- [開発指示](AGENTS.md): プロジェクトの前提と作業上のルール。
- [Issue 運用](docs/issue-workflow.md)、[複数担当の手順](docs/agent-workflow.md)、[完了後整理](docs/task-cleanup-check.md)。
- [Git/GitHub 連携](docs/adapters/git-github.md)・[Windows 連携](docs/adapters/windows.md): 任意の自動化素材の前提。
- [プロジェクト設定例](examples/github-issue-widget.toml): 人とエージェント向けの記入例。実設定や権限の付与ではありません。

`skills/`、`.codex/agents/`、`adapters/`、`tests/` は移入した AI ワークフローの素材とテストです。Windows の定期 Issue レビューはアプリ本体ではありません。配置しただけでは Skill の登録、GitHub Actions の有効化、定期実行は行われません。使用前に設定、認証、権限と副作用を確認してください。
