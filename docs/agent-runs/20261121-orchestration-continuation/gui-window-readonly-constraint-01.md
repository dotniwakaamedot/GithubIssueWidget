# 窓の追加照会に関する制約

`gui-window-identification-01.md` を確認。CUA のツール指示は Windows のコンピューター操作に `cua_repl` の記載された API のみを使用し、他技術を使用しないよう求めている。Win32 `IsWindowVisible` と `GetWindowThreadProcessId` の直接呼出しは CUA の記載 API にないため、この担当は追加の UI 照会を実行しない。CUA の HWND 選択は在庫に返った ID を前提とし、対象 ID は出ていない。可視性・所有 PID・desktop の同一性は未確認のまま。対象のウィンドウに操作する根拠は未成立。

保持セッション `83178` は直近の poll で live、終了は未確認。開始・停止・強制終了を追加していない。CUA が提供する情報の範囲や既存情報の安全な扱いについて、管理からの判断を待つ。
