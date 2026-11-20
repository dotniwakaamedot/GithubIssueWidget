# ハーネスの独立確認依頼

- 担当 verification_reviewer `/root/harness_check`。共有 run `D:\Projects\GithubIssueWidget\docs\agent-runs\20261121-resume-control`。対象と SHA256 は `harness-result-01.md` を参照する。
- `plan-02.md`、前回の `../20261121-output-truncation/execution-rereview.md` および上記結果を照合し、実ファイルとテスト証拠を独立に確認する。特に子孫終了、異常経路、待機上限、Job の制約、合成結果判定、未確認を検討する。テスト実行時は機密を扱わず、合成データだけを対象とする。
- 製品、対象ファイル、実 GitHub と資格情報を変更しない。他者の変更を戻さない。合否と対象ハッシュ・証拠・不足を `harness-review-01.md` に確定し、パスのみ通知する。無条件停止の解除や書込みを承認しない。
