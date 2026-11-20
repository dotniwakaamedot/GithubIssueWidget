# 分冊ルール変更後の整理確認

- 確認担当: `/root/cleanup_check`。対象: `D:/Projects/GithubIssueWidget` の失敗記録分冊ルール変更。対象 Issue・PR は指定されていない。調査は読み取り専用で、削除や GitHub 変更はしていない。
- 作業場所は Git リポジトリ、`origin` は `dotniwakaamedot/GithubIssueWidget`。現在の checkout は `main`、HEAD は `6b8e2fcfa40b6fb0bc9116e9458362a0ff48de63`。GitHub CLI の認証状態確認と `gh repo view` は成功し、GitHub 既定ブランチは `main`。認証情報は表示・保存していない。
- 現在の checkout に `docs/failure-log.md` と `docs/recurring-lessons.md` の変更、`docs/failure-log-001.md` と共有 run の未追跡ファイルがある。今回の成果物や証跡、既存の変更を含むため保持する。共有 run とその結果も保持する。
- `git worktree list --porcelain` で現在の checkout を含む７件を確認し、ロック指定は表示されなかった。各 worktree の ignored を含む状態件数はいずれも 8,000 件以上で、内容・所有者・使用状況・統合状況は未確認。現在の checkout は使用中、他の６件も不明なためすべて保留する。件数だけを根拠に削除候補としない。
- オープン PR は `gh pr list --state open --limit 100` で空配列だった。100件の上限を超える場合は未確認。対象 Issue/PR と完了目的の対応関係は指定されず、クローズ候補は判定できない。リモート全 branch、保護設定、最新統合 SHA、open Issue 一覧は未確認であり、branch 削除候補も判定しない。
- 親から削除可能な一時物の一覧、所有者、利用終了の情報は渡されていない。削除候補は確認できず、存在しないと断定しない。必要な場合は親が対象と認可を個別に特定し、最新状態を再確認する。
