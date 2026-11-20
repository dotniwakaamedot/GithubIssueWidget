# I02 旧診断の安全な終了方法への回答 01

- 宛先 `/root/drag_design`。対象 `questions/I02-geometry-execution-question-01.md`。親は旧 app と server の所有者による穏当な終了を指示した。以前の画像から desktop のロックやユーザー操作の有無を断定しない。
- 操作直前に所有している各 session handle と現在の PID・exe path・実行コマンド・開始時刻・対象版を再照合し、PTY / process group と子孫の影響範囲を確認する。既存の報告値だけで PID の現在の同一性を仮定しない。所属・対象・信号到達範囲が不明、他者の process を含む、または安全性を確認できない場合は操作せず親に報告する。
- 対象と process group が担当所有の対象だけと確認できた場合、推測座標による UI 入力を使わず、その所有 session へ Ctrl+C を送る穏当な終了を検討する。旧 app を先、旧 server を後とし、各操作の後に対象 handle を poll して terminal と終了コードを確認する。timeout・終了不明は成功扱いにせず、同じ終了を盲目的に繰り返さない。
- 対象がコンソール信号を受けられないと確認した場合に限り、再照合した担当所有 PID の `CloseMainWindow` を穏当な代替として検討する。正確な対象 HWND / PID / exe を確認し、他の窓に作用しないことと実行可否が不明なら実施せず保留する。強制終了や他者の process への信号はこの回答では許可しない。
- 終了後は元の各 handle の terminal と対象 PID・port の状態を確認し、時刻、操作、終了コード、未確認、ログ・証跡の場所を記録する。所有外の process や port を停止しない。古い scratch、ログ、証跡、元製品と生成物は保護し、削除しない。新しい geometry 版は旧版の terminal と非競合を確認してから確定依頼どおり準備する。
- 今回の画像不一致は1回、browser inventory エラーは2回。以前の画像取得・getState / browser inventory、ドラッグや推測クリックは繰り返さない。同種問題2回で該当操作を中断し別担当へ客観調査、3回で全作業を中止する。