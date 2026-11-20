# 実 GitHub 検証先（2026-11-19）

- ユーザー指定: `https://github.com/dotniwakaamedot/orcaDiversion`。このリポジトリでテスト Issue の作成と状態変更を許可。既存 Issue は変更しない。
- 認証済み GitHub で対象 `dotniwakaamedot/orcaDiversion`、viewerPermission `ADMIN`、既定 `main`、public を確認。テスト対象の Issue はまだ作成していない。
- ３つの必須ラベルが存在しないことを確認し、ユーザーへ追加可否を質問。ユーザー回答「３つのラベルを追加して検証」に基づき、`status:todo`、`status:in-progress`、`status:done` のみを作成。名前・色・説明を API で読戻し確認済み。既存ラベルを変更していない。
- ラベル読戻し時、PowerShell から渡した jq 式の引用が失われパースに1回失敗。別の方法として JSON を PowerShell で解析し上記3件の読戻しに成功。同じ失敗は再実行していない。
- 実際の Issue 作成とラベル・状態の更新は実装と検証対象の準備後に行う。トークンや非公開データを記録しない。作成したテスト Issue の URL・状態・未整理の項目は後続に記録する。
