# 証拠公開と marker 調整の管理判断 01

親から、以下2版を作者以外が独立確認し承認したとの報告を受領し、親は両案を採用した。レビューは会話による結果であり、独立レビュー本文の確定ファイルはこの管理記録では確認できていない。参照 SHA は実体から取得した。
```json
[
    {
        "base":  "D:\\Projects\\GithubIssueWidget\\docs\\agent-runs\\20261118-160129-mvp-restart",
        "relative_path":  "process/evidence-publication-protocol-01.md",
        "sha256":  "65E36A266503EB12E20B4045BC7D2A346C8F72B97BA3ED3EF73A3B80B36B3382"
    },
    {
        "base":  "D:\\Projects\\GithubIssueWidget\\docs\\agent-runs\\20261118-160129-mvp-restart",
        "relative_path":  "tasks/I02/marker-adjustment-plan-01.md",
        "sha256":  "154C238B447F23BA31B4B1855D5FE1432AC8741B25AEB39B58F8C40FE5890BEC"
    }
]
```

- 公開手順は既存の直接確定2件を保護し、新たな直接確定を行わない。指定先・tmp を新規作成・照合し確定する。独立レビューの会話記録以外の証跡が必要な場合は未確認とする。
- marker 調整は旧画面と process を所有者が保持し、新 scratch の小さな診断差分だけに限定。変更版の独立レビューを受けるまで build・起動・入力は禁止。
- 元のドラッグと画像不一致は未解決。単一クリックや Issue 完了をこの採用判断から主張しない。同種問題2回で該当操作を中断して別担当が客観調査、3回で全停止。