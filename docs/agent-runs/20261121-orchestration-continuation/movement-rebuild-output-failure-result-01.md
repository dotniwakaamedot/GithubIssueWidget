# 新 target 準備中の出力切り詰め記録

担当 `/root/process_safety`。`movement-rebuild-output-failure-request-01.md` と `movement-rust-rebuild-preflight-01.md` を照合し、隔離版だけを更新。共有失敗記録・製品・確定済みの旧結果は変更していない。

`git diff --no-index` は差分ありの終了1と、内側300 tokens上限による表示切り詰めを別々に扱う。当事者は target 指定1箇所を置換したメモリ比較で固定2ファイルが一致したと記録しているが、これは切り詰められた元表示の復元ではない。

別担当 `/root/project_survey` は固定資料の読取りにより、警告・パス・差分を含む出力量の見積りが足りず、既存 F-001 対策が適用されなかったと分析。静かなハッシュ／構造化比較を優先し、差分表示が必要な場合は内外上限と全体量を見積もる。今回の限定資料に基づく分析で、継続効果と過去の総数は未確認。先行3件とは別操作として記録した。

- 隔離 `failure-log-002.md` SHA256 `670601E4C9F9C2F45CF415163B2079F5F7380EF3126F277A22661401F37A92F8`。
- 隔離 `recurring-lessons.md` SHA256 `1A79E2C22B1B7E462FF11427E996B16F1C51185EF084CCCB2BB19C8BB166FA4F`。
- 隔離索引 `failure-log.md` SHA256 `8E1EB3335D40E16CAF675FC0A5ADEEA9F6FB76D526FA9107E719924A2A6384F4`、変更なし。

追加箇所を読み戻し、ハッシュ取得は終了0。新しい失敗は認識していない。対象の書込みを停止する。共有本体への反映・過去 ID と総数の照合・この固定版の独立レビューは未完了。
