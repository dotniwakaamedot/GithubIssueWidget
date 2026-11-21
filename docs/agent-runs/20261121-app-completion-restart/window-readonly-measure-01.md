# 現行ウィンドウの読取り専用計測

## 結論

対象ハッシュと一致する実行ファイルを使うプロセス１件を確認したが、`EnumWindows` が `Access is denied` で失敗したため、可視ウィンドウの識別と矩形・DPI・モニター・最前面属性の計測は**未実施**である。失敗した列挙の後に出た `visible_count=0` は PowerShell が非終端エラーの後に続行した結果であり、ウィンドウが存在しない証拠ではない。対象の初期サイズ、最小サイズ、中央表示、最前面動作も未検証。

## 対象と一次要求

- 担当 `/root/window_readonly_measure`、作業場所 `D:/Projects/GithubIssueWidget`、branch `main`、HEAD `4961b25bec53a93439acdf5230de77f959908af0`。HEAD のみでは未コミット・未追跡を識別しない。
- 依頼 `window-readonly-measure-request-01.md` SHA256 `46F699B6D586A21F5D0FCE41FD1696710FE848B9C0442C15D45FF1D778087446`。Issue #2・#7 を GitHub コネクタの番号指定で取得し、Windows ウィンドウの表示・移動・サイズ変更・最前面などの実機確認の要求を確認した。
- `runtime-gap-procedure-01.md` と `user-observation-07.md`〜`10.md` を確認。本人による新版のドラッグとフォーム操作・再起動の報告と、旧版でのウィンドウ操作報告を区別する。

## 観測と失敗

1. `Get-Process -Name 'github-issue-widget','GitHubIssueWidget' -ErrorAction SilentlyContinue` と明示的な Id・SessionId・Path・MainWindowHandle の抽出は終了コード 0。１件、PID `49512`、session `2`、パス `D:/Projects/GithubIssueWidget/widget/release-20261121-selection/github-issue-widget.exe`、MainWindowHandle `0` を取得した。MainWindowHandle が 0 であることは可視ウィンドウの不存在を証明しない。
2. 同パスの `Get-FileHash -Algorithm SHA256` は終了 0、`0D8AB47088B7A19233EF0D7D08CF033B681B20CA0E729FEAF73EAB74DCC98F55` で依頼値に一致。`[Environment]::UserInteractive` は `True`、計測側 PowerShell の session は `2`。実ファイルの現在値とプロセスの実行開始時の内容の完全な同一性はこの証拠だけでは保証しない。
3. `Add-Type` で user32 の `EnumWindows`、`IsWindowVisible`、`GetWindowThreadProcessId` を定義し、PID `49512` の可視トップレベル HWND のみを列挙する読取り専用操作を行った。`EnumWindows` は失敗し、`Win32Exception` は `Access is denied` を示した。PowerShell は非終端エラー後に続行し、外側の終了コードは 0 だった。結果を成功や 0 件と扱わない。失敗は１回で、同じ列挙を繰り返していない。
4. 列挙失敗により HWND を識別できず、後続の `GetWindowRect`、`GetDpiForWindow`、`MonitorFromWindow`、`GetMonitorInfo`、拡張スタイル取得は実行しなかった。物理座標・DPI 仮想化、`rcMonitor` と `rcWork` はいずれも未取得。画面、フォーカス、位置、サイズ、最前面、資格情報、GitHub を操作していない。

## 残作業・失敗の引継ぎ

- 現在のデスクトップで可視 HWND を許可された読取り経路で一意に識別できる環境が必要。識別できた場合も計測値はユーザー操作後の現時点の値であり、初期値・最小サイズ・中央表示・最前面切替の実動作の証明にはならない。アクセス拒否の根本原因は未特定。
- 対象に関する失敗は `EnumWindows` のアクセス拒否１件。失敗記録の共有ファイルは所有範囲外のため変更せず、管理担当へ記録・既存の該当失敗との照合を引き継ぐ。既存 F-045 の教訓（実行環境と API の利用可否を確認）を参照したが、Windows API の存在だけで利用権限は証明できなかった。再発防止の効果は未確認。
- 変更は未追跡の本結果のみ。確定後の SHA256 は自己参照を避け管理が別途取得する。削除対象なし。他者の変更・製品・実アプリの状態は変更していない。
