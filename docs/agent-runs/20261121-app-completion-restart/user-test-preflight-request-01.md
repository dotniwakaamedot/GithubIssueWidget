# 実 GitHub 検証の前提確認依頼

- 担当予定 `/root/test_preflight`。親 `/root` が起動。共有 run `D:/Projects/GithubIssueWidget/docs/agent-runs/20261121-app-completion-restart/`。
- 対象: 既承諾の公開 `dotniwakaamedot/GithubIssueWidget` に新規の検証 Issue 1件を作成し、列移動後クローズする検証。`user-test-procedure-01.md`、`docs/handoff-20261121-current.md`、Issue #5・#6 を参照。
- 作業: 読取りにより対象 repo の公開状態、必要な `status:todo`、`status:in-progress`、`status:done` の各ラベルの存在と読み取り完了、既存の承諾の正確な範囲、テストに必要な本人の PAT 権限と書込み前に確認すべき事項を整理する。既存 Issue の保護、1件作成、応答不明時に再送せず照合・部分失敗時の復旧方法を確認する。取得不能は不存在としない。アプリに設定された PAT の権限はコネクタの権限から推測しない。
- 操作は読取りのみ。実 Issue・ラベル・資格情報・Git・製品は変更しない。本人に秘密の共有を求めず、出力を限定する。所有範囲は `D:/Projects/GithubIssueWidget/docs/agent-runs/20261121-app-completion-restart/user-test-preflight-01.md` のみ。他担当の変更を戻さず、確定後はパスのみ通知。
