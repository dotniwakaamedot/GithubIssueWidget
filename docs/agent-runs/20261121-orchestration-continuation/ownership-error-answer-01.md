# 旧 worktree の Git 所有権エラー回答

2026-11-21、`D:\Projects\GithubIssueWidget` の PowerShell・既定 sandbox で `git -C 'C:/Users/semis/orca/workspaces/GithubIssueWidget/issue-4-board-resume' status --short --branch --untracked-files=all` を実行し、終了コード1、`fatal: detected dubious ownership` を受けた。Git は `.git` の所有者と現在のユーザーの SID が異なると報告した。これは Git が表示した情報であり、実際の担当者の活動状況や停止状態は示さない。具体的な SID を再掲しない。

続いて `git -c safe.directory=C:/Users/semis/orca/workspaces/GithubIssueWidget/issue-4-board-resume -C 'C:/Users/semis/orca/workspaces/GithubIssueWidget/issue-4-board-resume' status --short --branch --untracked-files=all` は終了0、`## dotniwakaamedot/issue-4-board-resume` のみを表示し、ユーザー ignore ファイルのアクセス拒否警告を伴った。変更は表示されなかったが ignore に関する制約が残る。設定はコマンド内だけで、永続設定・所有権変更は行っていない。元の結果はツール出力のみで別ログはなく、要約は `sync-ownership-result-01.md` にある。今回の回答作成に際して再実行していない。
