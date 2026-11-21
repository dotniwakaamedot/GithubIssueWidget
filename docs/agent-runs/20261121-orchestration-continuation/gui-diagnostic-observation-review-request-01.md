# GUI 診断版の固定ステージ実測方式の確認

- 担当 `/root/planning/plan_review`、実行者 `/root/gui_failure_investigation` と別担当。共有 run `D:/Projects/GithubIssueWidget/docs/agent-runs/20261121-orchestration-continuation/`。
- `gui-diagnostic-observation-preflight-01.md` SHA `16D84AFE1012956D423F5E180FDD63EEF8FB55ABF582DDB72E48294BFB283873`、`gui-diagnostic-observation-01/observe.py` SHA `5A89F1E3832ADA05E4892C5C50C72873490E78B4EEF2FFA0C2E06C204677A662`、診断 EXE `57CFD705772F4AB2CBDD8BFC40EC555280B905872D347E046E4E57B07678055E` を開始・終了時照合。
- 旧承認方式との差分、空設定と専用 marker、固定値のみの収集、直接プロセスと通常閉鎖不能時の扱い・秘密保護・未確認を確認。実測へ進むため必要な blocking と条件に絞り、GUI を起動せず `gui-diagnostic-observation-review-01.md` に記録し通知。
