# Windows の rg パス指定失敗の独立分析

担当 `/root/project_survey`。元担当から直接聴取し、`wildcard-failure-answer-01.md` と報告を照合。製品・共有既存ファイルは変更していない。

## 事実と原因

- `/root/process_safety`: PowerShell で `rg -n 'describe\(|test\(|it\(|spawn|exec|process\.|fetch\(|localStorage|sessionStorage|listen\(' widget/src widget/vite.config.* widget/package.json`。`widget/vite.config.*` が OS error 123、終了1。一部は表示されたが全体の検索成功ではない。実在する `widget/vite.config.ts` を個別に読み直し成功。書込みなし。
- `/root/planning`: `rg -n '質問|発生元|記録対象' docs/agent-runs/20261121-orchestration-continuation/*request*`。OS error 123、終了1。検索結果は得られず、既取得情報から回答したが検索範囲全体は未確認。書込みなし。
- 両者とも `*` を含むパスが Windows のファイル操作に渡り、無効な名前として拒否されたとエラーから判断できる。PowerShell のネイティブコマンドへの引数展開を期待してパスを渡したことが共通原因。rg の `-g` フィルタとは別物。環境ごとの引数伝達の内部詳細の追跡や元コマンドの再現は行っていない。

## 解決と予防

既知のファイルは存在を確認して正確なパスを指定。複数ファイル検索なら存在を確認した親ディレクトリを指定し、`rg -n -g '*request*' '質問|発生元|記録対象' docs/agent-runs/20261121-orchestration-continuation` のように `-g` に glob を渡す。必要なら先に `rg --files -g '*request*' docs/agent-runs/20261121-orchestration-continuation` で対象一覧を確認し、終了コードを判定する。無結果は不存在の証明にしない。両コマンドは是正案であり、実検索や実行結果は未確認。

既存の `docs/recurring-lessons.md` F-012 には未展開 `*` の予防として親ディレクトリ確認が既記載。初稿の F-015 は番号の誤記で、別担当の指摘を受け現行文書で見出しを再確認して訂正した。上記の rg `-g` と部分出力・非ゼロ終了の扱いを補足候補として親に引き渡す。両発生の累計記録と反映は共有ファイル担当が実施する。回帰・再発防止効果は未検証。
