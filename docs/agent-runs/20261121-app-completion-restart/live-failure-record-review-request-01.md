# 実 OS 合成試験の失敗記録レビュー依頼

- 担当予定 `/root/live_failure_recorder/record_review`。管理 `/root/live_failure_recorder` と記録作者・独立分析担当とは別。共有 run `D:/Projects/GithubIssueWidget/docs/agent-runs/20261121-app-completion-restart/`、作業場所 `D:/Projects/GithubIssueWidget`。
- 対象 `live-failure-record-result-01.md` SHA256 `DC53CE43D5F4D3051A98AA9E67549F93F70BA7ECA3732AEB669A99AFCD9176D4`、`docs/failure-log-002.md` `DB6A4733D8A35DD6E67F915945F39DE15D77F214D37F0F86597FD4BECE695264`、`docs/failure-log.md` `DA4586BC914E974D009E10911F86834F180880329E0CFBF7B59A606B93388052`、`docs/recurring-lessons.md` `96CF6237C43D051F03B3D6D6F2BFEEBB8B233C55F003D8CE764B730683E0A79E`。実体と SHA を照合し、違えば停止・質問する。
- 実 OS 失敗、独立分析、対象 absent の読み取りとレビュー、関連出力切り詰め・推測パス・文字照合・表示文字化けを一次記録と照合。直接原因・元出力・過去累計・予防効果の未確認、同一事象の二重計上防止、対象外の既存変更保護と索引を確認する。実 credential の再操作、実験の再実行は行わない。
- 新規 `live-failure-record-review-01.md` のみ所有。版・根拠・blocking・未確認を記録し、UTF-8 実データを読戻して一時ファイルから確定。出力を絞り同種失敗2回は停止して管理へ通知。共有環境の他者の変更を戻さない。公開後パスのみ通知する。
