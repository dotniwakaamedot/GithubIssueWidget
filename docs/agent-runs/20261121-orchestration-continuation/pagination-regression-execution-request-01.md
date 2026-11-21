# 大量ページ回帰の隔離実行依頼

- 担当 `/root/issue_survey`。共有 run `D:/Projects/GithubIssueWidget/docs/agent-runs/20261121-orchestration-continuation/`。新規 `pagination-test-execution-01/` と結果 `pagination-regression-execution-result-01.md` を所有。既存なら上書きしない。他者の変更・既存 target・Git・旧 fixture を保護する。
- `pagination-regression-review-01.md` は試験案の静的承認。製品 main SHA `61595B68317E15FDD8F2BED88B920A468D5516740E3D3124710384709C52FC24`、隔離 main `AEFF1E65AD93EFA04773CEBBB9736A6BDC5B9938DB9FAF937DD88B3D9CBFE86F`、`pagination_tests.rs` `BF690E818EB182FE16395E0AE3946B043D41DADD2B75235EFEACDC4873F4DE3D` を直前に照合。異なれば実行せず通知する。
- 実行に必要な Cargo プロジェクトと設定・素材の固定版を新規領域へ独立コピーし、main と試験ファイルは上記レビュー済み版を使用。ビルドターゲット・TEMP・出力ログを専用領域に分離し、既存 target やユーザー設定を更新しない。依存は lock とオフラインの既存キャッシュを利用、秘密・PAT は引き継がず実 GitHub に接続しない。入力・出力先・実行ファイル・アクセス権を実行前に確認し、不足や予期せぬ変更があれば中止し記録。
- 対象は追加の `mixed_pages_preserve_exact_issue_set_and_classification` と `later_failures_never_return_the_first_page_as_success` の2テストのみ。それぞれ正確なテスト名フィルタと `--locked --offline` を指定し、テスト数0を成功にしない。loopback mock の有限時間、実行全体の期限・直接プロセスの終了コード・ログ・テスト実行数と失敗を別々に記録する。実行が継続中なら同じセッションを待ち、時間切れ・未知の子孫を終了済みとせず、無関係なプロセスを推測終了しない。
- 実行結果・対象 SHA・正確な作業ディレクトリとコマンド・依存/target の範囲・終了・失敗・未確認を結果に記録し、証跡を保持し書込み停止後にパスを通知。既存合格試験を新たな根拠なしに再実行しない。別担当が結果をレビューする。実 API・GUI・全 Issue は本試験で証明しない。
