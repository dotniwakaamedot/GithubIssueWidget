# #4 大量ページと分類の試験範囲調査依頼

- 担当 `/root/issue_survey`。共有 run `D:/Projects/GithubIssueWidget/docs/agent-runs/20261121-orchestration-continuation/`、作業場所 `D:/Projects/GithubIssueWidget`。読み取り探索のみ、結果は固有 `pagination-coverage-investigation-01.md`。製品・テスト・既存出力・外部状態を変更しない。他担当も作業中であり、他者の変更を戻さない。
- Issue #4 の原文と現行 `widget/src-tauri/src/main.rs` の実体 SHA を取得して固定。100件超・1,000件超、Open/Closed・PR除外、複数ページ、途中失敗・レート制限の実装と既存合成試験を照合。テストが扱う件数・ページ数・HTTP 入力と期待結果、含まれない条件を具体的に記し、コードの存在と実行成功、合成応答と実 GitHub の証明を区別する。
- 不足が見つかれば、製品・他の未コミット変更を保護した所有範囲、実行可能な具体的回帰ケースと必要なテスト環境・期待値を示す。実行はしない。存在するテストが必要ケースを満たす場合も現行版の実行証拠と不足を区別する。
- コマンド・終了結果・対象 SHA、失敗、未確認を記録し、結果確定後は書込みを停止してパスを通知。製品修正やプロセスを伴う試験は後続の承認済み個別依頼で実施する。
