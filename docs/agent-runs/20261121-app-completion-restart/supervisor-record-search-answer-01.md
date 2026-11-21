# 検索失敗の追加回答

- 担当 `/root/supervisor_failure_record`。質問 `supervisor-record-search-question-01.md` に回答する。
- １件目: `rg -n -l --glob 'failure-log-*.md' 'JSON|AssertionError|代表文字|文字列照合|バックスラッシュ' docs` は `docs` 全体に再帰し、対象２分冊のほか run 配下のアクセス拒否を表示した。元ツール応答の終了コードは 2 で、完全一覧とは扱わない。独立ログは保存していない。
- ２件目: PowerShell の here-string を `python -X utf8 -` に渡し、`docs/failure-log-001.md` の `### F-` 見出しを日本語リテラル「文字」「照合」「比較」「値」と ASCII `JSON` のいずれかで分類した。終了 0・空出力で、該当見出しが得られなかった。ツール応答以外の独立ログや実際に Python が受けた引数は保存していない。
- 後続の確認は、同じファイルを strict UTF-8 で読み、スクリプト内を ASCII の Unicode escape とし、見出しを `ascii(line)` で出力した。終了 0、該当する F-071・F-062・F-057・F-056・F-049・F-046・F-041・F-018・F-021・F-028 の見出しを得た。後続の手段の成功は元のパイプ引数を復元しない。
- ２件目は PowerShell パイプ境界の日本語変換が原因候補だが、実引数は未保存のため確定しない。１件目は対象を docs 全体に広げたことがアクセス拒否を含む結果の直接の条件であり、拒否の詳細理由は未確認。類似性と別事象の判断は独立分析に委ねる。
