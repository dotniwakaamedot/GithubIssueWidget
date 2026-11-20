# 参照パスの失敗の独立調査結果

- 担当: `/root/path_diagnosis`。対象依頼: `tasks/I02/path-diagnosis-request-20261119.md`。対象製品は読み取り専用。
- 確認された事実: `tasks/I02/verification-design-result-01.md` は、前回、製品 worktree を cwd として共有 run の相対パスを読もうとし PathNotFound が1回発生したと記録している。以前の実行ログと具体的な対象パスは同結果にはない。
- 今回の親の実行記録では、cwd が `D:\Projects\GithubIssueWidget-issue-2-20261118-160129` の `Get-Content` に `docs/agent-runs/20261118-160129-mvp-restart/tasks/I02/combined-review-01.md` を指定し、PathNotFound が発生した。親 checkout の同名の絶対パスは存在し、worktree 内の同名の絶対パスは存在しないことを `Test-Path -LiteralPath` で確認した。
- 共通点: worktree を cwd とし、親 checkout にある共有 run を相対パスで参照したこと。今回のエラーはその解決先に対象がないことで説明できる。前回の個別ログを確認していないため、報告を超えて細部の同一性は断定できない。
- 対応: 共有 run と製品 worktree の絶対基点を明確に分け、共有記録の参照は共有 run の絶対パスで指定する。異なる基点からの参照を1コマンドに混在させない。読み取り前に `Test-Path -LiteralPath` で実際の対象を確認し、存在しなければ実行しない。生成した一覧をパスとして処理する場合は注記やハッシュを分離する。
- 未確認: 前回の元コマンドと当時のファイル配置。2回の事象は記録に基づいて照合しており、前回の独立ログの再生ではない。失敗回数の運用上の判定と作業再開は親が行う。
- 検証: 依頼と前回結果を UTF-8 で読取。対象2パスについて `Test-Path` の結果は親 checkout が `True`、別 worktree が `False`。アプリ実行、入力、Git 操作、外部更新、既存ファイルの変更は行っていない。
