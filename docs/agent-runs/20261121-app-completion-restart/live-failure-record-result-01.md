# 実 OS 合成資格情報試験と関連失敗の記録結果

- 担当 `/root/live_failure_recorder`。作業場所 `D:/Projects/GithubIssueWidget`、共有 run `D:/Projects/GithubIssueWidget/docs/agent-runs/20261121-app-completion-restart/`。ユーザーの実資格情報・アプリ・GitHub を変更していない。
- `docs/failure-log-002.md` SHA256 `DB6A4733D8A35DD6E67F915945F39DE15D77F214D37F0F86597FD4BECE695264` に、実 OS 合成試験失敗を新規 F-087 発生1 として記録。終了1・`before_write_a`・元 stderr 未保存・直接原因と書込み成否未確定、後の対象限定 `absent` と独立レビューを区別した。F-001 関連切り詰め、F-012 関連推測パス、文字照合および表示文字化けもそれぞれ一回として記録。F-057/F-086 の直接原因同一性と過去累計は未確定。試験・調査・診断を別発生に数えていない。
- 独立 `/root/live_failure_recorder/independent_analysis` の `live-failure-record-analysis-01.md` SHA256 `A22FB93A21A5DDFD2BE0A18DC72DD881CF794D7EFF313BF8D24B216AE5FF9E6D`、検査担当回答と記録担当回答を根拠に、既存の F-001・F-012・F-057 の適用を整理。`docs/recurring-lessons.md` SHA256 `96CF6237C43D051F03B3D6D6F2BFEEBB8B233C55F003D8CE764B730683E0A79E` に該当各節への反映、`docs/failure-log.md` SHA256 `DA4586BC914E974D009E10911F86834F180880329E0CFBF7B59A606B93388052` に F-087 までの索引を反映。過去欠番・重複と全累計は未照合で確定していない。
- 対象実体を変更直前にハッシュ・文脈確認、適用後に strict UTF-8 と `F-087 発生1` の一意性・関連文字列を確認。`git diff --check -- docs/failure-log.md docs/recurring-lessons.md` は終了0、改行に関する Git 警告を出した。分冊002は未追跡のため実体と SHA・UTF-8 を別途確認。削除対象なし。
- 記録作業中、Python の日本語出力が表示上１回文字化けした。元ファイル破損は示さず、変換境界は未特定。これも独立分析・分冊・教訓へ反映。失われた表示、原因の一部と予防策の効果は未確認。
- 対象３ファイルの独立レビューは未実施。実 OS 試験失敗の直接原因、実 PAT・実 WebView の検証、アプリ完成を本結果は証明しない。確定後の本ファイル SHA は別途取得する。
