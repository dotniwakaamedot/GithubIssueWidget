# 起動診断の安全な代替案の調査依頼

ユーザー原文: 「issueを確認し、アプリを完成させてください。同じ失敗が2度続いた時は引き継ぎの資料を作成して、セッションを中断してください。」その後「続行してください」。

作業場所 `D:/Projects/GithubIssueWidget`、共有場所 `D:/Projects/GithubIssueWidget/docs/agent-runs/20261121-issue-completion/`。前回の `gui-diagnostic-review-01.md` と `handoff-diagnostic-stop-04.md` を踏まえ、既存の fixture、コードと検証資料を読み、Windows GUI の失敗を切り分ける実行可能で安全な方法を提案する。特に秘密出力、空の設定先、Credential Manager、プロセス・子孫・パイプの停止、異常時の扱いを具体化し、未証明な保証を主張しない。可能であれば既存資産やアプリの標準的な起動を利用した簡潔な方法と、その証拠・制約を示す。

読取りのみ。GUI 起動、外部通信、既存ファイル変更は禁止。他者の変更を保護し、共有場所 `gui-alternative-analysis-01.md` に結果と対象版ハッシュを保存しパスを通知する。
