# 実 GitHub 検証の次段階に関する独立確認

ユーザー原文: 「Issue を確認しアプリを完成させてください。一回の処理の間に同じ失敗が２度続いたらそこまでの経過を引き継ぎ用ファイルに保存して、中断してください。」

`../20261120-current-review/live-plan-02.md`、`../20261121-resume-control/live-design-review-03.md`、`../20261121-resume-control/harness-review-02.md` と `repeated-failure-decision.md` を照合し、実行を有効化せずに次に進められる具体的な合成試験または設計作業を優先順に提案する。`probe-fixture-list.py` の安全性と証拠の範囲も確認し、OS 障害・終了保証、認証・秘密保護、作成結果不明と再送禁止を含める。元の実 GitHub 操作は行わず、トークンを取得・表示しない。成果は `live-next-review.md` に記録、パスのみ通知。製品や他担当ファイルを変更しない。
