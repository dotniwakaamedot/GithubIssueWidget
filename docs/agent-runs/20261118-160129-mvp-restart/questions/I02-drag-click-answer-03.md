# I02 診断版の実行前判断 03

対象 `questions/I02-drag-click-question-03.md` と修正後の診断版。担当・対象版の参照、履歴と閾値を以下に固定した。各 SHA256 は実体から取得し、確定前後に照合する。
```json
{
    "recipient":  "/root/drag_design",
    "reviewer":  "/root/plan_review",
    "port":  1435,
    "thresholds":  {
                       "independent_investigation":  2,
                       "stop_all":  3
                   },
    "history":  {
                    "drag_no_movement_before_current_restart":  2,
                    "process_self_match_failure":  1
                },
    "references":  [
                       {
                           "base":  "D:\\Projects\\GithubIssueWidget\\docs\\agent-runs\\20261118-160129-mvp-restart",
                           "relative_path":  "tasks/I02/drag-click-request-01.md",
                           "sha256":  "83D63BA78D6CEEC7E07BDF9834583D0274BCADCEFC5DB5BCAB6B25377880CB6C"
                       },
                       {
                           "base":  "D:\\Projects\\GithubIssueWidget\\docs\\agent-runs\\20261118-160129-mvp-restart",
                           "relative_path":  "tasks/I02/drag-instrumentation-revision-result-01.md",
                           "sha256":  "43AE521C9934642B3CA71553BACAA7C7E0129C4DD8421D59B4713A84000DCEE1"
                       },
                       {
                           "base":  "D:\\Projects\\GithubIssueWidget\\docs\\agent-runs\\20261118-160129-mvp-restart",
                           "relative_path":  "tasks/I02/drag-instrumentation-review-02.md",
                           "sha256":  "191A8B66C672DF8C1794FB08312B390BC55402561B4FE189B6CD5ECAAB2ABBF4"
                       },
                       {
                           "base":  "D:\\Projects\\GithubIssueWidget\\docs\\agent-runs\\20261118-160129-mvp-restart",
                           "relative_path":  "tasks/I02/drag-count-correction-01.md",
                           "sha256":  "71637C9E9ADF66DD9094A9E9C53A1AEE01761F5EA9917530C29D9EBC57FFBC86"
                       },
                       {
                           "base":  "D:\\Projects\\GithubIssueWidget\\docs\\agent-runs\\20261118-160129-mvp-restart",
                           "relative_path":  "tasks/I02/build-verification-result-01.md",
                           "sha256":  "7A8AF3B60D32D2AC94888801067C73031DB8A84974DA95E6373D03C03E68A098"
                       },
                       {
                           "base":  "D:\\Projects\\GithubIssueWidget-drag-diagnostic-20261118-160129",
                           "relative_path":  "widget/src/main.tsx",
                           "sha256":  "820F164EF8947DA6B87F6ACE8B74D787E09ED9AF7A6610E2C07678DFA2C4AE5C"
                       },
                       {
                           "base":  "D:\\Projects\\GithubIssueWidget-drag-diagnostic-20261118-160129",
                           "relative_path":  "widget/src/diagnostic.ts",
                           "sha256":  "56A62632E8D63AEE5DD0AB636C954E30FBB76562C90C88CD9C0D53AB497B6D37"
                       },
                       {
                           "base":  "D:\\Projects\\GithubIssueWidget-drag-diagnostic-20261118-160129",
                           "relative_path":  "widget/vite.config.ts",
                           "sha256":  "194D17A739D0F626DDAF94B5D2B6F9335DBF50BB571AB4F7739DA727ED4F772A"
                       },
                       {
                           "base":  "D:\\Projects\\GithubIssueWidget-drag-diagnostic-20261118-160129",
                           "relative_path":  "widget/src-tauri/tauri.conf.json",
                           "sha256":  "EDE0EA3E1EE5C6FE067A5E127EEAAE02A19110E8CD8E057C8B7FCB2B826A55D5"
                       }
                   ]
}
```

- 前回レビュー B01/B02 と N01 は、修正版コードと実行前ゲートの設計上は是正された。独立レビューは型・起動・UI の実動作や製品完成を承認していない。親はこの診断版の実行準備を承認した。
- 管理の読取時点で TCP 1420 / 1435 に該当行なし、対象アプリ名と専用 scratch に一致する process は非検出、interactive session 2 を確認。これは一時点のスナップショットであり、別名プロセス、ユーザーのデスクトップ操作や実際の対象窓が競合しない保証ではない。B03 は担当の確定結果において terminal、終了コード未取得で未完了。
- 起動前に構造化参照の各 SHA、元とコピーの対象版、担当・scratch・port・profile・process・実デスクトップの非競合を再確認し、ユーザー操作や対象が不明なら開始しない。承認済み専用コピー方式に従い、起動・依存の副作用と既存資産保護を維持する。
- 入力前に同一の main window / PID / exe と対象版を識別し、空の `diagnostic-ready:[]` を同窓から確実に読み取って保存できることを確認。marker・header・全 button・resize 帯の実境界と DPI / 座標系を実測し、非対話領域が各境界から最低12 CSS px 離れていることを確認する。空表示・保存・対象識別・座標・余白・ユーザー操作の非競合のいずれかが不明なら入力せず停止する。
- すべてのゲートを満たした場合に限り、同じ対象窓の marker 中央へ単一の押下・解放を1回のみ実施し、同窓のイベント記録と対象版・PID・時刻を専用 evidence に保存。結果が空・不達・誤対象・読取失敗・不明なら反復せず停止し報告する。ドラッグ・移動・リサイズ・button 操作は禁止。診断版の観測は過去のドラッグや製品の完了を証明しない。
- 同種問題の2回目は該当操作を中断し別担当へ客観調査、3回目は全作業を中止。プロセス照合の自己一致は今回1件。過去のドラッグ不成功2回と文字破損・lockfile の失敗経路も再試行しない。保護対象・scratch・生成物は削除せず、最終の証拠 SHA と未確認を報告する。