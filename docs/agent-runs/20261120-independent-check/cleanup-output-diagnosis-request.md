# 整理手順の出力切り詰め分析依頼

親が `Get-Content -LiteralPath 'docs/task-cleanup-check.md' -Encoding utf8 -TotalCount 45` を出力上限 1300 tokens で実行し、約2070 tokens の出力が切り詰められた。提示情報だけから原因・未確認事項・防止策を分析し、`cleanup-output-diagnosis.md` に保存する。コマンド、ファイル、ネットワーク、GUI、再試行は禁止。通知は結果ファイルの絶対パスのみ。
