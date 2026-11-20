# I02 診断実行の開始条件への回答 01

- 宛先 `/root/drag_design`。対象 `questions/I02-drag-click-question-01.md`。以下の構造化参照を確定前後に実体照合した。
```json
[
    {
        "relative_path":  "tasks/I02/drag-click-request-01.md",
        "sha256":  "83D63BA78D6CEEC7E07BDF9834583D0274BCADCEFC5DB5BCAB6B25377880CB6C"
    },
    {
        "relative_path":  "tasks/I02/build-drag-preflight-review-01.md",
        "sha256":  "EE06BEACEF4DFEF9E26204B7E3FC575827B5DAFC8682C0F9381E3E23485DEFF9"
    },
    {
        "relative_path":  "tasks/I02/build-verification-result-01.md",
        "sha256":  "7A8AF3B60D32D2AC94888801067C73031DB8A84974DA95E6373D03C03E68A098"
    }
]
```

- 独立レビューはクリック依頼に blocking 指摘なし。親は依頼の対象版を承認した。これは**診断コピー・コードの作成準備の判断**であり、診断版コードのレビュー・対象確認前の入力許可や製品の移動成功を意味しない。
- B03 の所有者の確定結果は所有セッションの中断と terminal を記録。実際の npm/Cargo 終了コードは未取得、B03 は未完了。独立レビューは記録・実体を照合したが、別担当のセッションハンドルを直接 poll できない点を明記している。
- 管理が回答直前に読み取りで確認: B03 の対象 PID と対象アプリ名の process は非検出、B03 scratch を command line に含む process は非検出、TCP 1420 / 1435 の netstat 該当行0、診断 scratch は未存在。PowerShell は interactive session 2、元 worktree は対象 HEAD と未コミット17件。これは一時点の観測であり、全プロセス・別名子孫・実際のユーザー操作やデスクトップの非競合を保証しない。
- 依頼の全条件を満たせる場合、別の専用 scratch と port でコピー・診断版の準備に進む。開始直前に担当・対象版・元17 hash・専用先・対象 UI / process / port・ユーザー操作との競合を再照合し、不明や競合があれば停止する。B03 scratch・既存のアプリ・証拠は触れない。
- **単一クリックを含む入力はまだ禁止。** 診断コード・設定の差分と版を固定し `startDragging` 無効化を作者以外が独立レビューし、親が対象版を判断してからのみ、依頼で指定された限定的な入力を行う。ドラッグ・移動・リサイズは引き続き禁止。コピー、証拠、通常の副作用・未確認を保護し記録する。
- 同種2回で該当操作を中断し別担当が客観調査、3回で全作業を中止。各履歴は元依頼の構造化フィールドを保持し、依存ゲートを未確認のまま操作しない。