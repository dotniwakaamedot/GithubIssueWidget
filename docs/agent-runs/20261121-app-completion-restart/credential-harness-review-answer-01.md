# 文字連結失敗の詳細

- 対象コマンドの該当部分は `$anchor = [string]::Concat([char]0x8CC7,[char]0x683C,[char]0x60C5,[char]0x5831)`。直後に `$s.Contains($anchor)` を検査した。
- ツール出力は `ArgumentNullException`、`パラメーター名:argument`、PowerShell の該当行情報を表示し、その後 `UTF8 Japanese readback OK` も表示した。個別失敗の終了コードは取得していない。ツール出力はエラー発生と後続出力を示すのみであり、照合成功の証拠にはならない。
- 元操作は再実行せず、別の Python コマンドで Unicode escape による UTF-8 読戻しを行い、終了コード0を確認した。後続の成功は前の失敗の非発生を示さない。
