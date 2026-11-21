# GUI の識別に関する回答

担当 `/root/gui_failure_investigation`。起動時は `gui-release-observation-preflight-01.md` の専用 APPDATA/LOCALAPPDATA/TEMP/profile/WebView2 と、許可された最小 Windows 環境を `observe.py` に渡した。child は `Popen` の保持ハンドルから実行パス一致、作成 FILETIME `134397021216357095` を確認。PowerShell と後続 `Get-Process -Id 6004` は SessionId 2。Windows desktop 名・CUA ホストの session ID は取得していないため、同一 desktop/session の実証はない。

起動後の初回 `Get-Process` は HWND 39655226、title を未取得、未終了、SessionId 2。取得の正確な時刻は記録していない。少なくとも30秒経過後に同じ `Get-Process` を再取得し、同じ HWND、MainWindowTitle 空、Responding true、未終了。Win32 の `IsWindowVisible`・`GetWindowThreadProcessId` は使用しておらず、これらによる可視性・所有 PID は未確認。`Get-Process` の MainWindowHandle は該当プロセスのプロパティだが、HWND の可視性や CUA による同一ウィンドウ識別を証明しない。

CUA `listWindows` の２回とも HWND 39655226 の項目なし。インベントリには他アプリのみ表示され、対象選択や画面操作は行っていない。CUA の詳細な対象取得は inventory に現れた Windows ID を前提とするため、対象未選択。保持セッション `83178` は最後の poll で live、直接終了コードなし。CUA の終了サマリーを各観測後に記録済み。再起動・停止は行っていない。
