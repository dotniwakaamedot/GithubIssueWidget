# Issue #2 作業場所・依頼の準備 01

- #2 は GitHub の親 Issue API で #1 のサブ Issue と確認。docs/issue-workflow.md はサブ Issue を親ごとの統合ブランチへ PR とし、標準連携時に `Sub-issue: #2` を単独行にする規約。`adapters/github/close-integrated-sub-issue.yml` は `integration/issue-1` を前提とするが配布例であり、導入・権限・外部 PR base は未確定。
- 2026-11-18 UTC、origin/main と main HEAD は a4ac97286785a2a5ba0d070a909b58003fc49ff1、Issue #2 の状態・担当・コメント・open PR を照合。ローカルとリモートに同名ブランチがないこと、作業先が未存在であることを確認。
- ローカル `integration/issue-1` を上記 commit から作成し、そのコミットから `task/issue-2-foundation-20261118-160129` と専用 worktree `D:\Projects\GithubIssueWidget-issue-2-20261118-160129` を作成。対象 worktree は clean、HEAD は基準 commit。親 checkout の未追跡共有 run は保護した。push / PR / merge は未実施。
- ブランチ作成の進行コメントは https://github.com/dotniwakaamedot/GithubIssueWidget/issues/2#issuecomment-6984639364 、識別キー `<!-- issue-progress:20261118-160129-mvp-restart:branch-01 -->`。本文を読戻し一致を確認。
- worktree 作成前検査では単一行の PowerShell 出力を配列と誤認し1回失敗（preflight-failure-01.md）。副作用が始まる前に停止し、明示的配列化で再検査・作成に成功。同種失敗の再発は確認していない。
- 担当 `/root/foundation` 向け事前確定依頼: `tasks/I02/request-01.md`、SHA256 1B5808B52E85573C6C2AA3EDDC6ABAA43CA599FF2D54A09E113F6C3F2EE586A4。開始前に変更・担当・競合を再照合する。