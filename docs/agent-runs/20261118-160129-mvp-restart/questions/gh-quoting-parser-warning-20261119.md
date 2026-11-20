# 調査中の PowerShell 構文エラー

- 担当 `/root/gh_quoting_diagnosis`、2026-11-19 UTC。依頼された3パスの存在確認を準備した際、`foreach(...) { ... } | Format-Table` を直接記述して PowerShell 5.1 の `ParserError: EmptyPipeElement` が1回発生した。パスの読取りや GitHub 操作は行われていない。パイプ対象を変数に受けてから処理する別の構文で存在確認に成功した。
- 共有 run `tasks/plan/resume-review-20261119.md` には補助集計中の PowerShell パイプ構文 `ParserError` 1回が過去に記録されている。ただし当時の元コマンド・具体的なパーサー識別子は同記録から未確認であり、今回との同一性は断定しない。これらは依頼で調査中の GitHub CLI 引用失敗とは直接の原因・実行段階が異なる。
- ユーザーの失敗回数・停止条件の適用は親に委ねる。以後は一つの単純な構文で操作し、予測パスを読まない。PathNotFound は発生していない。