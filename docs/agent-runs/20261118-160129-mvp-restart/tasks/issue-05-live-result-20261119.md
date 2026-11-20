# Issue #5 実 GitHub 作成検証

- 許可対象 `dotniwakaamedot/orcaDiversion` の権限は admin/push を確認、必要な3ラベルが存在し、事前検索で `github-issue-widget-test-20261119-create-01` は0件。
- `issue-5-create-resume` の HEAD `98126d82ac20e256b6072b5abd8cba6a0a5d3020` に、レビュー済みテスト補助コードを追加した `widget/src-tauri/src/creation.rs` SHA256 `3982D55E2DA0B1B56FDEE6B8A9E3053C646DDDC00F826D27726C5F55C9E58166` を対象に、`authorized_live_create_once` を一度実行。結果は 1 passed、kind `created`、番号 13、operation ID `b04f8e69-4555-4024-8ca2-b8bd2f234475`。
- GitHub 読戻しで https://github.com/dotniwakaamedot/orcaDiversion/issues/13 、タイトルの識別子、OPEN、`status:todo` を確認。作成済みテスト Issue と pending 記録 `tasks/live-create-state-20261119-01/pending-issue.json` は保持し、既存 Issue の変更・削除は行っていない。
- `gh` の認証情報を子プロセス環境で一時使用し、終了時にプロセス環境変数を復元。トークンの値は表示・記録していない。Fine-grained PAT やアプリ画面を経由した検証ではない。
- 事前検索コマンドに無効な `--state all` を一度指定し、書込み前にエラーとなった。状態指定を省いた検索で open/closed を対象として結果0件を確認した。実 POST は1回。
