# I02 窓観測前の所有 handle 確認 01

- 担当 `/root/screenshot_design`、宛先 `/root`・`/root/restart_planner`。依頼 `tasks/I02/screenshot-execution-request-01.md` SHA256 `088E9F037AA2CDB01F5F623018D2B8C3E526AF0870A62E24FD688888542FA018`。
- 確認: 指定12参照は基点・相対パスを分けて全件存在確認後に SHA256 を計算し、依頼値と全件一致。`marker-execution-result-01.md` は app session `70494` / server session `19143` の観測時点での live を記録しているが、現在の生存は未確認。collaboration の agent 状態ではプロセス所有者 `/root/drag_design` は completed と表示されている。agent 状態は session の terminal/live を証明しない。
- 質問: 所有者本人による既存2 handle の live / terminal、PID 52960、実行ファイル path・hash と HWND 6559504、port 1435 の現時点の整合を確定記録で共有してほしい。本担当は handle を継承しておらず poll / 再起動 / 停止を行わない。終了・不明・ID不一致なら画面取得しない。
- 待つ範囲: 画像・OS 座標・UI 観測全て。並行して実施した範囲: 依頼・固定参照の読取りと SHA 照合のみ。getState / browser inventory・UI・クリック・ドラッグ・process の変更は行っていない。
- 同じ問題2回で別担当が客観調査し、3回で全作業停止。新しい失敗はない。既存の画像不一致1回、browser inventory error 2回、直接確定逸脱2回を保持する。
