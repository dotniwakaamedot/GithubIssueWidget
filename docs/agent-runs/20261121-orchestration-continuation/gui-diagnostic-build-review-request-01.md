# GUI 診断版ビルド方式の独立確認

- 担当 `/root/plan_review`、実行者 `/root/gui_failure_investigation` と別。共有 run `D:/Projects/GithubIssueWidget/docs/agent-runs/20261121-orchestration-continuation/`。
- `gui-diagnostic-build-preflight-01.md` SHA `84CD3062CC3FE424AAC584AF96A2D38AE9D99B2DDDF44541B90B0FE89F2D79B6`、`gui-release-build-01/run_diagnostic_build.py` SHA `0A71A2FD9A3C083A072CE480987F0FB885E513FBAF54FFC5E7DD6BDD4B0315D3`、既存 `build.cmd` SHA `BAE3C381D954B2AB243708605D05B3113636B83FACAFC18D98BB58F46321F2EA` を開始・終了時に照合。
- 旧ビルドの EXE/ログ・既存入力の保存、承認済み診断ソース2版、旧 runner との差、実行時の競合・off-line・ログ・終了と未確認を確認。実行に必要な具体的条件・blocking に絞る。ビルド/GUI/外部操作を行わず `gui-diagnostic-build-review-01.md` に記録し通知する。
