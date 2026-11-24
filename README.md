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

## ビルド済みアプリの利用

Windows 11 用の実行ファイルは `widget/releases/github-issue-widget.exe` にあります。起動には WebView2 Runtime が必要です。インストーラーや署名済み配布物ではありません。対象ファイルの SHA256 は `6380F51818DEE3D9971162B9FB0AB79417BED5DF8A62B52677538A5CBBA29299` です。

初期状態は最前面、枠なし・ダークテーマです。タイトルバーの固定ボタンで最前面を切り替え、タイトル部のドラッグで移動し、外周のドラッグでサイズを変更します。

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

実行ファイルは `widget/src-tauri/target/release/github-issue-widget.exe` に出力されます。ビルド後、このファイルを起動して使用できます。これは上記の検証版とは別の出力先です。

## 接続と使い方

GitHub の Fine-grained personal access token は対象リポジトリだけを選び、`Metadata: Read` と `Issues: Read and write` を付与します。必要な期間に合った有効期限を設定し、期限切れになったら新しいトークンを設定してください。トークンの値はアプリの設定画面に直接入力し、チャットや Issue、ソースコードなどで共有しないでください。設定画面にオーナー名、リポジトリ名、トークンを入力し、接続確認に成功すると保存されます。接続確認は Issue の書き込み権限を保証しないため、作成や移動で権限エラーが出る場合は GitHub 側の設定も確認してください。

接続先は `%APPDATA%/github-task-widget/settings.json` に保存し、トークンは Windows Credential Manager に保存します。保存済みトークンは画面へ読み戻しません。既存トークンを使う場合は入力欄を空のまま接続でき、設定画面からトークンを更新・削除できます。

設定画面のホスト URL は既定で `https://github.com` です。GitHub Enterprise Server では `https://github.example.com` のようにサーバーの HTTPS URL を入力します。API には `/api/v3` が使われます。ホストを変更する場合は、そのホスト用のトークンを新たに入力してください。サーバー証明書を信頼できない環境では接続できません。

同じ設定領域を共有する別ウィンドウで接続先や資格情報が変更された場合、古い画面からの一覧取得・作成・移動や、設定の保存・トークン削除は中止されます。エラーが表示されたら設定を再読み込みし、対象を確認してから操作してください。アプリを介さず資格情報を直接変更する操作との完全な排他は保証されません。

ボードは約60秒ごとに同期します。`status:todo`、`status:in-progress`、`status:done` ラベルを事前に GitHub 上で作成してください。存在しないラベルをアプリが自動作成することはありません。Issue 作成が途中で失敗したり結果が不明になった場合は、表示された結果と GitHub の状態を確認してから確認操作を行い、重複作成を避けてください。

カードをクリックすると GitHub で Issue を開きます。新しい Issue は上部の「Issue を作成」または各列の「＋」から作成できます。「＋」は移動ではなく新規作成です。状態を変更するには、カードを移動先の列へドラッグ＆ドロップします。ドラッグ中の文字選択を防ぐため、カードと列見出しの文字は選択・コピーできません。Done に移すと Issue は Closed、Todo または In Progress に移すと Open になります。

Closed の Issue はステータスラベルにかかわらず Done に表示します。Open の Issue はステータスラベルを Todo、In Progress、Done の順に優先し、該当ラベルがなければ Todo に表示します。ラベルと Open / Closed が一致しない場合は、GitHub 上の状態を確認してください。

作成後に他のステータスラベルが同時に付いた場合も、作成済み Issue を示して確認を促します。GitHub 上でラベルと Open / Closed の状態を確認してください。

Issue 作成フォームには作成先を表示します。接続先を変更する場合は、フォームの入力を確認して送信するか、キャンセルしてから設定を開いてください。

## エラー時の確認

通信エラーが出た場合はネットワーク接続と対象リポジトリを確認してください。認証・権限エラーではトークンの期限、対象リポジトリ、上記の権限を GitHub 側で確認します。レート制限が表示された場合は案内に従って時間をおいてから再試行してください。

作成結果が不明な場合は、同じ Issue を再送する前に GitHub 上で題名と作成時刻を確認してください。作成済みか判断できないときは再送せず、対象と状態を確認できるまで待ってください。移動や Done への作成が途中で失敗した場合も、一部だけ反映されている可能性があります。GitHub 上の Issue とラベル、Open / Closed の状態を確認し、画面の案内に従って対応してください。
