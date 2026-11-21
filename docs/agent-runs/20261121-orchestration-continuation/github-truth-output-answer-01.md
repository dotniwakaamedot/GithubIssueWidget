# 出力切断についての回答

2026-11-21、作業場所 `D:\Projects\GithubIssueWidget` の PowerShell で、`rg -n --max-columns 150 --max-columns-preview -e 'retry\(' -e 'uncertain' -e 'pending' -e 'partial' widget/src-tauri/src/creation.rs` を実行した。`exec_command` の内側上限は1800 tokens、出力は元1878 tokens と報告され、`Warning: truncated output` と途中省略が表示された。終了コード0。検索結果は実装と多数のテスト行を混在し、150文字を超える行も preview 省略された。内部で省略された部分の正確な行番号と全内容は分からない。

事前に `rg` の関数名一覧は取得したが、一致件数・合計量と複数パターンにまたがる結果量は見積もらず、実装とテストを分割しなかった。直前に他ファイルの短い検索2件を並列表示しており、共通の外側出力も共有していた。意図的な `--max-columns-preview` 省略と、内側上限による切断を別の不完全性として扱う。検索対象は1ファイルで他のディレクトリや秘密の列挙はしていない。

前に `movement-rust-rebuild-preflight-01.md` で同担当が300 tokens の差分表示を切断させたため、同じ失敗と認識し追加調査を止めた。今回の検索を再実行していない。`github-truth-followup-01.md` の既述は読めた範囲に限り、creation の全経路についての結論ではない。
