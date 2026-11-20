# I02 ウィンドウ移動の切り分け計画 独立レビュー 01

- run_id / task_id / review_revision / 回数: 20261118-160129-mvp-restart / I02-drag-plan / 01 / 1
- owner_role / owner_agent_id: verification_reviewer / /root/plan_review。計画作者 /root/restart_planner、実装作者 /root/foundation とは別担当。
- 対象 `tasks/I02/drag-plan-01.md` の依頼 SHA256 と実体: C6AC531B2AB8FB2BAAC401BFBEE09135356E7CA2E411A09D4C27198611A1B70A（一致）。
- 参照 `tasks/I02/result-01.md` F0AEAD322062EED3054B1A51B0A44DC05EB85E257A3A2EE59026F99C9112BE03、`tasks/I02/review-01.md` 7DB95B62CF8D23894AAEB4FCFAB51DD4A4634CE42C1C330421E4689E4FE6DEF9 は実体と一致。
- worktree `D:\Projects\GithubIssueWidget-issue-2-20261118-160129` の branch と HEAD は依頼と一致し、結果で列挙された未コミット・未追跡17ファイル全 SHA256 は実体と一致。`git status --short --untracked-files=all` の変更・未追跡は17件、削除対象なし。
- 判定: **対象 SHA256 の読み取り・計画に限り承認**。実アプリのドラッグ、入力注入、製品変更、その他の実機操作の承認ではない。後続操作の可否と停止条件の適用は親判断。

| 確認項目 | 独立確認・根拠 | 判断・未確認 |
| --- | --- | --- |
| 過去の2回と同種失敗の停止 | result-01 と review-01 は座標不変2回・原因不明を明記。対象計画は原因が確定するまで再試行せず、同種3回で中止と明記 | 適合。移動は未実証であり成功を主張しない |
| 仮説と事実 | ツールの保持、旧 drag-region、OS 等は未確定の仮説と明示。暫定報告を確定資料・実バージョンと照合する段階を設ける | 適合。仮説の真偽は本レビューでは判断できない |
| 入力保持と座標・イベント・Tauri・OS | 既存ツールドキュメントと過去実行記録を読み、押下・保持・解放の契約、要素境界・座標・DPI・呼出し・OS 応答を区別する計画 | 適合。API 契約の実際の内容や過去の保持は未確認であり、診断作業で出典を確認する必要あり |
| 安全性・対象・独立性 | 新規のイベント注入・マウス移動・ドラッグ・製品変更を禁止。観測装置は設計案までとし、副作用、復元、停止条件を評価、観測計画を超える操作は新版依頼と親判断。別担当の独立確認を要求 | 適合。読み取り自体に副作用がないことは各手順の実施前に個別確認すること |
| 証跡と版変更 | 保存先、時刻、作業場所、対象 hash、機密除外を計画。診断中の版変更で停止して新版を固定・再評価 | 適合。具体の所有パス、担当 ID、コマンド・終了コード、確認済み版ハッシュは後続依頼と成果に記録する必要あり |
| Issue #2 への適合 | Issue #2 はドラッグによる移動と Windows での実測を要求。対象計画は診断のみで移動実証を完了扱いしない | 適合。診断だけでは Issue 完了条件を満たさない |

## 指摘

blocking および non-blocking 指摘なし。計画は原因を推定のまま扱っており、後続の操作・観測手順や実装成果の承認を代行しない。

## 実施した確認・限界

- cwd `D:\Projects\GithubIssueWidget`。`Get-Content -Encoding UTF8` で依頼、対象計画、前版結果・レビューの関係文書、I02 management-02 と request-03 を確認。`Get-FileHash -Algorithm SHA256` で対象・参照を照合。終了コード 0。
- cwd 対象 worktree。`git branch --show-current`、`git rev-parse HEAD`、`git status --short --untracked-files=all` と結果記載17ファイルの `Get-FileHash -Algorithm SHA256` を実行し、全一致を確認。終了コード 0。
- Issue #2、適用 AGENTS.md、agent-workflow は直前の I02 成果レビュー時に読取。Issue 最新状態、実入力、アプリ、OS イベント、過去ツールの API 実装・操作ログは今回再取得・実行していない。
- 保存先の親ディレクトリ存在と確定先・tmp 不存在を確認し、tmp から確定。レビュー中の失敗なし。製品、Git、GitHub、入力・デスクトップ操作は変更していない。