# 同期確認と復旧後の検証 Issue #11 読取り依頼

- 担当予定 `/root/issue_state`。親 `/root` が ID を照合して再開。共有 run `D:/Projects/GithubIssueWidget/docs/agent-runs/20261121-app-completion-restart/`、作業場所 `D:/Projects/GithubIssueWidget`。先行結果 `issue-11-state-04.md`、`issue-11-sync-state-01.md` と `sync-user-observation-01.md` を読む。
- 親からの報告: ユーザーが60秒同期の確認を報告し、#11 を Done に戻して closed を確認するよう依頼中。本人の復旧操作が完了したとの通知を受けてから、公開 `dotniwakaamedot/GithubIssueWidget` の #11 を番号指定の読取りで確認し、状態、status・非 status ラベル、更新時刻、同期検証以降のイベントを安全な範囲で記録。応答のページ終端と同時更新・非原子性の制約を明記。公開 API と本人の実 UI 報告を区別し、API だけで60秒同期や復旧操作の完了を推定しない。
- 読取りのみ。Issue・ラベル・PAT・アプリ・既存成果は変更しない。不要な本文・秘密を出力しない。取得失敗を不存在扱いにしない。実行前にページ処理・終了・出力量を確認し、取得した状態が中間・不明なら変更せず親へ報告する。
- 新規 `issue-11-sync-state-02.md` のみ所有。対象・観測日時・コマンド・終了状態・未確認を記録。UTF-8 実データ読戻し後に一時ファイルから確定しパスのみ通知。共有 workspace の他者変更を戻さない。同種失敗2回では停止し親に報告。作者以外のレビューを要する。
