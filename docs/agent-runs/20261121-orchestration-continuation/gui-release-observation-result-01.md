# release GUI 空設定観測結果

担当 `/root/gui_failure_investigation`。依頼 `gui-release-observation-request-01.md` と `gui-release-observation-decision-01.md`。固定 EXE SHA256 `23C383EB2E46CB205E052A9D4A24EE73B968EB2C776F7150826350685419B306`、実行器 SHA256 `F82AD7CADEC4F94B1E7832A32A701C0CE59CEE9E713AAD1833834FC839F48CF7`。対象は専用空設定で1回のみ直接起動した。

## 結果

**GUI 表示・通常終了は確認できなかった。** 同じ保持ハンドルから EXE パスと作成時刻を照合し、直接 PID 6004 は30秒後も生存。`Get-Process` は MainWindowHandle 39655226、SessionId 2 と記したが、CUA の窓一覧に対象 ID は2回現れず、画面内容・対象窓の可視性を判断できなかった。独立調査 `gui-window-identification-01.md` も対象を識別できず、原因は未特定。101 の再発とは観測していない。UI の誤操作・再起動・資格情報の入力・実 GitHub 操作はしていない。

管理判断と独立レビュー `gui-direct-exit-review-01.md` に基づき、空設定・無入力の自分が起動した直接子のみを限定して強制終了した。実行直前、同一の新規プロセスハンドルで EXE 実体パス・作成 FILETIME `134397021216357095`・生存状態を照合し、バイナリ SHA と一致することを確認してから、そのハンドルのみに `TerminateProcess` を実行、API は成功。元のセッション `83178` は `direct_exit_code=1`、runner 終了0を返した。この1は指定した強制終了コードであり、アプリの自発的な失敗や正常終了を示さない。未知の子孫・旧不明プロセスの終了は確認していない。

## 検証と制約

- 起動直前の５ディレクトリと親は再解析点0、５ディレクトリは空、広いユーザーグループの許可0、状態ファイル不存在。現行の条件付き空設定経路は `gui-empty-startup-audit-01.md` を参照。同一ユーザーの競合防止・WebView2 や依存の無条件の非通信・credential store 分離は保証しない。
- 直接起動実行コマンド `python docs/agent-runs/20261121-orchestration-continuation/gui-release-observation-01/observe.py`。stdout/stderr は DEVNULL のため panic 本文・原因は得られない。生存30秒後も通常閉鎖可能な窓を特定できず、終了確認は上記の強制終了に限定。実際のアプリ画面・操作・開発起動は未検証。
- ウィンドウ一覧への非出現が2回続いたため別担当の独立調査に委譲した。対象不在の一覧結果を実ウィンドウ不存在と断定しない。`gui-release-observation-status-01.md`、`gui-release-observation-status-02.md`、`gui-window-identification-01.md` と `gui-window-readonly-constraint-01.md` に観測・制約を記録。今後、識別に利用できる CUA の方法、起動環境・WebView2・タイトル未設定の因果を調査する。
- 作業場所 `D:\Projects\GithubIssueWidget`、branch `main`、HEAD `5e74e1642fd18fe630dce0e684afd853721276ae`。新規 `gui-release-observation-01/` と本結果等は保持し、削除対象なし。プロセス照合・終了コマンドは終了0、元セッションの終了も確認。実コード・既存設定・旧 fixture は変更していない。

本結果は成果の独立レビュー前。アプリ完成や F-078 の根本原因解決を主張しない。
