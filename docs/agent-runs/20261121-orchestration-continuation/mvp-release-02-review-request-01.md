# 最新 MVP 再ビルド固定版の独立確認

- 担当 `/root/plan_review`、作者 `/root/gui_failure_investigation` と別担当。共有 run `D:/Projects/GithubIssueWidget/docs/agent-runs/20261121-orchestration-continuation/`。
- `mvp-release-02-preflight-01.md` SHA `0A2F9AF6DD0F0DAAA310DCE4B9BEDD972582D25824AEF0CA7CDDB87FF39644A2`、`gui-release-build-01/run_mvp_build_02.py` SHA `804316F47E161BD79B2C21D5A95B96006129BB292AFE3FA78C7B621F68A62267`、`gui-release-build-01/mvp-input-manifest-02.json` SHA `2EB56D635C9CCC132D46BA80B641B8A81444DFE79CA9D75E37D58349302B586C` を開始・終了時に照合。
- 前回承認・成功方式との変更箇所、現行製品の承認済み入力、旧 EXE・ログの保護、専用ビルドと新規配布先、実行/アクセス・未確認を必要最小限で確認する。ビルド・GUI・実 credential・外部書込みは行わず `mvp-release-02-review-01.md` に blocking と開始条件を記録し通知する。
