# 隔離 IPC テスト再実行の条件確認

`ipc-resume-execution.md`、`ipc-write-failure-analysis.md`、`ipc-diagnostic-review.md`、`inspect-ipc-credential-review-03.md` を参照。前回の失敗後、試験 root と marker の存在、設定ファイル不存在、レビュー済みの照会により現ユーザー `HOME_PC01\CodexSandboxOnline`・Windows session ID 2 での対象資格情報の不存在を確認した。失敗時のログオンセッションの独立記録はない。試験自体は前回後に固定カテゴリだけを出す診断変更を行って静的承認されている。

未確認のセッションを明示した上で、異なる新規 service・ID・一時 root を作るテストについて、前回 fixture を保持したまま安全に別環境で1回実行できる条件と、実行前に不足する確認を分析してください。環境の変更や実行を指示・実施せず、根拠と限界を `ipc-resume-decision.md` に記録してください。対象ログやコードから値・識別子を出力せず、資格情報操作・テスト・GUI を実施しないでください。通知は結果の絶対パスのみ。
