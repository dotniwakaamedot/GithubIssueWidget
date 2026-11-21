# GUI 診断スクリプトの独立レビュー依頼

ユーザー原文: 「issueを確認し、アプリを完成させてください。同じ失敗が2度続いた時は引き継ぎの資料を作成して、セッションを中断してください。」再開指示「続行」。

作業場所 `D:/Projects/GithubIssueWidget`、共有場所 `D:/Projects/GithubIssueWidget/docs/agent-runs/20261121-issue-completion/`。`gui_diagnostic.py` SHA256 `57D389E3262C882D32008FB6E9F2C14DD59097E22D78E45AED0484711F787ED0` と `gui_probe.py` SHA256 `C701915D7A81DB40EADDF9B73A1AE8F24E7616E2580380A3BB2C7E3DA96A127C` を確認する。

目的は前回の GUI 起動失敗を原因未確定のまま、空の設定先で一度の起動を安全に観測すること。標準出力・標準エラーから許可された固定分類だけを出し、秘密・自由文を出力しない。既存設定・資格情報・GitHub 書込みを保護し、失敗を再試行しない。`handoff-gui-stop-02.md` と起動時コードも参照する。

合成試験の妥当性、分類の漏出・境界、上限、プロセス終了・観測、fixture の隔離条件、実行時の安全性を検討し、重大な未確認または要修正を明示する。アプリ起動・外部通信・資格情報変更は行わない。他者の変更を保護し、共有場所 `gui-diagnostic-review-01.md` のみへ結果を記録して通知する。結果は対象 SHA の範囲に限る。
