# 移動テスト起動拒否の対応

`movement-execution-policy-investigation-01.md` は当事者と別担当が過去の同種拒否と、pagination 試験で成功したプロセス限定 `powershell.exe -NoProfile -ExecutionPolicy Bypass -File` を確認。今回の実効ポリシーの詳細は不明であり、永続設定変更はしない。

`/root/existing_work_review` は固定版 `run-movement.ps1` SHA `AF6C6C39AC166180D0B7D7B750562F5366CFA4797A0D65E907C03791314E07D3`、専用 fixture・保存版・ログ不存在と所有を再確認できた場合、`powershell.exe -NoProfile -ExecutionPolicy Bypass -File <当該実行器の絶対パス>` で未実行の3ケースを1回開始する。前回は実行ポリシーで拒否され Cargo/テストは未起動。直接終了・3件の実行と各成否・ログ・子孫の未確認を区別する。同じ拒否が再発したら繰り返さず、別担当が調査する。共有ポリシーや他者資産は変更しない。
