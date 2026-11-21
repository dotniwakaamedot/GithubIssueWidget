# 検証版 EXE のローカル受渡し準備依頼

- 担当予定 `/root/release_copy`。親が ID を照合して起動。共有 run `D:/Projects/GithubIssueWidget/docs/agent-runs/20261121-app-completion-restart/`、作業場所 `D:/Projects/GithubIssueWidget`。
- 対象は `selection-build-sandbox-01/cargo-target/release/github-issue-widget.exe` SHA256 `0D8AB47088B7A19233EF0D7D08CF033B681B20CA0E729FEAF73EAB74DCC98F55`。`selection-build-result-01.md`、`selection-build-review-01.md`、`release-readme-audit-01.md` と対象実体を確認する。前者の sandbox は read-only とし変更しない。
- 新規 `D:/Projects/GithubIssueWidget/widget/release-20261121-selection/` を受渡先候補とし、作成直前に解決済み絶対パス・対象が workspace 内であること・存在しないことを確認、専用ディレクトリを新規作成して EXE をコピーする。既存なら上書きせず停止して親に報告。元とコピーの SHA・サイズを照合し、同一でなければ未完として残す。既存 `widget/release-20261121*` や他者の素材は変更・削除しない。
- 所有: 上記新規受渡先と `release-copy-result-01.md` のみ。外部公開・Git 操作・ネットワーク送信・ユーザーの設定や資格情報・アプリ起動はしない。使用者への配布やインストーラー作成と誤認させない。実行コマンド、終了コード、対象版と SHA、未確認を記録。UTF-8 実データを読戻し一時ファイルから確定してパスのみ通知。別担当のレビューが必要。
