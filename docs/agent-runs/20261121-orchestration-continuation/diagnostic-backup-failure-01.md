# 診断バックアップ EXE の推測パス失敗

`/root/plan_review`、作業場所 `D:/Projects/GithubIssueWidget`。`gui-release-build-01/diagnostic-backup-01/github-issue-widget.exe` を推測し存在確認前に複数対象の `Get-FileHash -LiteralPath` に含めた。PowerShell は PathNotFound を表示し、コマンド終了コードは0だったが一部の対象は取得できず、出力表はパスが省略表示されたため照合の証拠としない。`Get-ChildItem -File -LiteralPath .../diagnostic-backup-01` で正名 `previous-release.exe` を特定した。既存の「推測せず存在確認」ルールを読んでおり前回も別担当に指摘を受けていたが、文書にディレクトリ名だけがありファイル名を勝手に補った。対象個別一覧を取らず複数を一括指定したことが原因。製品・バックアップへ書込みなし。管理に報告、独立分析を依頼した。
