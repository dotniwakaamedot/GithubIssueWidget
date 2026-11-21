# 同期確認中の検証 Issue #11 の状態読取り依頼

- 担当予定 `/root/issue_state`。親 `/root` が ID を照合して再開。共有 run `D:/Projects/GithubIssueWidget/docs/agent-runs/20261121-app-completion-restart/`。先行 `issue-11-state-04.md` は #11 を closed・`status:done` と観測。
- ユーザーは #11 のみを GitHub 上で reopen・`status:in-progress` に変更しアプリ手動更新なしで最大2分の同期を確認する依頼を受けているが、回答待ち。公開 `dotniwakaamedot/GithubIssueWidget` の #11 を番号指定の読取り GET で、観測時刻、状態、status・非 status ラベル、更新時刻を確認。必要なら公開イベントを読み、ページ終端・観測範囲・同時更新の限界を記録する。本人の表示・60秒同期・操作完了や復旧の成功を API 状態だけから推定しない。
- 新規 `issue-11-sync-state-01.md` のみ所有。読取りのみであり、GitHub やラベル・アプリ・資格情報・既存ファイルは変更しない。ユーザーの同期試験に干渉しない。本文・トークンを出力せず、取得失敗を不在とせず、UTF-8 実データ読戻し後に一時ファイルから確定、パスのみ通知する。他者の変更を戻さない。出力を絞り同種失敗2回は停止・報告。
