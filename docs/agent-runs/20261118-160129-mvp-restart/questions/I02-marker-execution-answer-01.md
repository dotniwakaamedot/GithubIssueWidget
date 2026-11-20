# I02 旧 geometry プロセスの終了方法への回答 01

- 宛先 `/root/drag_design`。対象 `questions/I02-marker-execution-question-01.md`。以前の別 PID に対する判断は自動適用せず、今回の親判断をここに記録する。
- 親は、対象 app PID `9796` と server PID `54736` が担当自身で起動した診断専用であり、ユーザーアプリの設定・未保存データの保持対象ではないことを確認したうえで、この2件の個別停止を認めた。各 PID の所有・作成時刻・exe / command line・対象 scratch と server port を**停止直前**に再照合し、PID 再利用・対象不明・他者 process の可能性があれば操作せず停止する。
- 穏当な console signal や process group の範囲が確認できないため、確認済み app `9796`、続いて server `54736` のみを個別に `Stop-Process` してよい。これは強制終了として扱う。子孫を一括停止せず、別の個別所有子が残れば PID・所有・用途を記録し、追加の操作をせず親の判断を待つ。ユーザーアプリ・他者 process・端末以外の desktop には触れない。
- 各停止後に担当所有 session `4498` / `68650` を poll し terminal / exit code、対象 PID と port 1435 の状態を確認する。強制終了・時刻・確認根拠・exit を記録し、`-1` が得られた場合は正常終了と扱わない。timeout / 不明 / 意図しない残存があれば後続の起動や操作を止め親へ報告する。旧 scratch・ログ・証拠と元製品を保護する。
- 元の screenshot、getState / browser inventory、ドラッグ・移動は再試行しない。新 marker 版は確定依頼にある所有・版・port 等のゲートを再照合後に準備し、画像・AX・境界・余白などが不足する限り入力しない。
- 同種問題2回で該当操作中断と別担当の客観調査、3回で全停止。過去の画像不一致1回・browser inventory failure2回・直接公開2件の履歴を保持し、証拠は承認済み公開方式で確定する。