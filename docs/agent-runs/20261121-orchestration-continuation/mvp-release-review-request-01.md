# 現行製品 MVP ビルドの固定版確認

- 担当 `/root/plan_review`、作業者 `/root/gui_failure_investigation` と別。共有 run `D:/Projects/GithubIssueWidget/docs/agent-runs/20261121-orchestration-continuation/`。
- `mvp-release-preflight-01.md` SHA `BC81031242E366E938E7E5296D00BFB219ACA8067A3335602EBC16E7588E3BEC`、runner `gui-release-build-01/run_mvp_build.py` SHA `E2F83ADFEEDEE21FE7E3889A81BBE8EC99A75ED8326B5D9F2753255BC4A263F7`、input manifest `673FDABA5A91336866FB4A8F073F773E071FF67146F2F04516CD9DD337B89024`、`build.cmd` `BAE3C381D954B2AB243708605D05B3113636B83FACAFC18D98BB58F46321F2EA` を照合。
- 現行製品の固定入力、診断変更が含まれないこと、旧 EXE/ログと他者変更の保護、既承認 runner との差、出力先・終了・アクセスと未確認を、起動可能な MVP に必要な最小限で確認。ビルドを実行せず `mvp-release-review-01.md` に blocking・条件・未確認を確定し通知する。
