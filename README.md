# GithubIssueWidget

GitHub Issue を Windows デスクトップのウィジェットとして表示するアプリの開発プロジェクトです。

## 現在の状態

Windows 11 向けアプリの開発版は `widget/` にあります。GitHub リポジトリへの接続、Issue ボード、Issue 作成、カード移動を備えています。Windows での一部の統合検証は未完了です。技術構成と要件は GitHub の Issue に整理しています。

カード移動では更新直前に Issue を読み直し、表示時の `updated_at` と異なれば更新を中止します。ステータスラベルは個別に追加・削除するため、他のラベルを全置換しません。これらの操作と Open / Closed の変更は一括ではなく、途中で失敗すると一部だけ反映される場合があります。読み直しから各更新までの同時変更も完全には検出できないため、エラー時や共同編集時は GitHub 上でステータスラベルと状態を確認してください。

## 開発時の資料

- [開発指示](AGENTS.md): プロジェクトの前提と作業上のルール。
- [Issue 運用](docs/issue-workflow.md)、[複数担当の手順](docs/agent-workflow.md)、[完了後整理](docs/task-cleanup-check.md)。
- [Git/GitHub 連携](docs/adapters/git-github.md)・[Windows 連携](docs/adapters/windows.md): 任意の自動化素材の前提。
- [プロジェクト設定例](examples/github-issue-widget.toml): 人とエージェント向けの記入例。実設定や権限の付与ではありません。

`skills/`、`.codex/agents/`、`adapters/`、`tests/` は移入した AI ワークフローの素材とテストです。Windows の定期 Issue レビューはアプリ本体ではありません。配置しただけでは Skill の登録、GitHub Actions の有効化、定期実行は行われません。使用前に設定、認証、権限と副作用を確認してください。

## アプリの開発とビルド

Windows 11、Node.js / npm、Rust 1.90.0、Visual Studio C++ Build Tools（MSVC と Windows SDK）、WebView2 Runtime が必要です。Rust は `widget/rust-toolchain.toml` により rustup で指定バージョンが選択されます。Visual Studio の x64 Native Tools Command Prompt、または `VsDevCmd.bat -arch=x64` で設定した環境を使用してください。

```powershell
cd widget
npm.cmd ci
npm.cmd run tauri -- dev
```

リリース用実行ファイルの作成:

```powershell
npm.cmd run tauri -- build --no-bundle
```

実行ファイルは `widget/src-tauri/target/release/` に出力されます。初期状態は最前面、枠なし・ダークテーマです。タイトルバーの固定ボタンで最前面を切り替え、タイトル部のドラッグで移動し、外周のドラッグでサイズを変更します。外部リンクは `https://github.com/` とその配下に限定し、GitHub Enterprise 等の別ホストは対象外です。

## 接続と使い方

GitHub の Fine-grained personal access token は対象リポジトリだけを選び、`Metadata: Read` と `Issues: Read and write` を付与します。設定画面にオーナー名、リポジトリ名、トークンを入力し、接続確認に成功すると保存されます。接続確認は Issue の書き込み権限を保証しないため、作成や移動で権限エラーが出る場合は GitHub 側の設定も確認してください。

接続先は `%APPDATA%/github-task-widget/settings.json` に保存し、トークンは Windows Credential Manager に保存します。保存済みトークンは画面へ読み戻しません。既存トークンを使う場合は入力欄を空のまま接続でき、設定画面からトークンを更新・削除できます。

同じ設定領域を共有する別ウィンドウで接続先や資格情報が変更された場合、古い画面からの一覧取得・作成・移動や、設定の保存・トークン削除は中止されます。エラーが表示されたら設定を再読み込みし、対象を確認してから操作してください。アプリを介さず資格情報を直接変更する操作との完全な排他は保証されません。

ボードは約60秒ごとに同期します。`status:todo`、`status:in-progress`、`status:done` ラベルを事前に GitHub 上で作成してください。存在しないラベルをアプリが自動作成することはありません。Issue 作成が途中で失敗したり結果が不明になった場合は、表示された結果と GitHub の状態を確認してから確認操作を行い、重複作成を避けてください。

作成後に他のステータスラベルが同時に付いた場合も、作成済み Issue を示して確認を促します。GitHub 上でラベルと Open / Closed の状態を確認してください。

Issue 作成フォームには作成先を表示します。接続先を変更する場合は、フォームの入力を確認して送信するか、キャンセルしてから設定を開いてください。
