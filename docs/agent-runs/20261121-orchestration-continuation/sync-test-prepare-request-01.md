# 同期テストの隔離入力準備依頼

- 担当 `/root/project_survey`。共有 run `D:/Projects/GithubIssueWidget/docs/agent-runs/20261121-orchestration-continuation/`。他担当と共有 checkout は書込み競合するため製品・既存依存・Git・旧 worktree・既存 cache に変更しない。
- 所有先は新規 `sync-test-fixture-01/` と結果 `sync-test-prepare-result-01.md`。既存なら中断し上書きしない。`sync-test-runtime-audit-01.md` の固定版と制約を読み、実行しない限定的な準備を担当する。実行方式は独立レビュー中であり、テスト、ビルド、GUI、外部接続、プロセス終了は行わない。
- 対象の `sync-isolated-01/main.tsx` SHA256 `6FCF2A34145450C5029B7FF80DDB3EC1DA5E27173AAB84E57F16D6D54C0E6A3E`、`main.test.tsx` `66E118A5A7062854C23BE405D2E9578C974FA283BEA332F2A7C51DCA1BFFD13C` と、監査に示す CSS・設定・package/lock の出典 SHA を検査する。不一致なら中止する。新規領域内の `project/src` に上記2版と CSS、`project` に package/lock・tsconfig・vite.config を独立ファイルとして複写し、出典とコピーの一致を確認する。
- 必要な依存コピーについては実行前に親・全階層に再解析点がないこと、容量と実体・アクセスを確認。リンク・ジャンクション・ハードリンクを作らず新規の通常ファイルとして独立複写し、キャッシュ `.vite`、`.vite-temp`、`.vitest-cache` は元から持ち込まない。正規化した先が当該新規領域内にあることを確認する。コピー前後に対象一覧・件数・内容ハッシュの一致を検査し、取得不能・変化・リンク等があれば不明とし中止する。無関係な既存領域は変更しない。問題があれば準備途中の資産を保持して報告し、推測で削除しない。
- 隔離出力・TEMP/TMP 等の新規ディレクトリを固定し、全所有ファイル一覧と実行対象ハッシュ・コピー検査・結果・失敗・未確認を記録。実行は後続のレビューと確定依頼を待つ。完了後は書込みを停止し、確定結果パスを通知する。
