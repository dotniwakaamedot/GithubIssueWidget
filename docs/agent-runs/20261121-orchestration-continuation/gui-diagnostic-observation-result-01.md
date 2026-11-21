# GUI 固定ステージ診断の実測結果

担当 `/root/gui_failure_investigation`。依頼 `gui-diagnostic-observation-request-01.md`、固定方式レビュー `gui-diagnostic-observation-review-01.md` と管理開始判断に従い、診断 EXE SHA256 `57CFD705772F4AB2CBDD8BFC40EC555280B905872D347E046E4E57B07678055E` を専用の空設定で1回起動した。元製品とは異なる診断バイナリである。

## 観測

- 直接 PID 49384 は保持ハンドルにより実行パスと作成 FILETIME `134397042396495387` が確認され、30秒後も生存。専用 marker を固定 enum で分類した結果、30秒時点および終了後とも `before_run` 1件、`setup_entered`・frontend/IPC の各 stage 0件、未知行0。終了後の marker SHA256 `3C5A45772EC6798E02A4BF874E504881A621ED9B64C22D91A4370AD1C5D7F331`。
- 固定 Tauri 2.12.7 のコードでは設定窓の `build()` が戻った後に `setup` が呼ばれる。今回の固定記録は `before_run` は書き込まれたが `setup_entered` は観測されなかったことを示す。stage の書込み失敗・callback 未到達をこの観測だけで区別できず、窓生成の内部位置・WebView2・環境のいずれが原因かは未確定。診断版の結果を元の release EXE へ自動的に一般化しない。
- `Get-Process` の MainWindowHandle は 49157026、タイトル空、SessionId 2。CUA の一覧に対象 HWND は出ず、実際の画面表示・可視性を識別できなかった。他の窓には触れず、通常終了の操作はできていない。標準出力・標準エラーは DEVNULL のため panic 内容や以前の101との因果は得られない。
- 管理の指示と `gui-direct-exit-review-01.md` の限定方針に従い、空設定・無入力の自分が起動した直接子だけを対象に、実行パス・EXE SHA・作成 FILETIME・生存を同じ新規保持ハンドルで照合後 `TerminateProcess` を実行し API 成功。元セッション `5563` は `direct_exit_code=1`、runner 終了0。1は明示した強制終了コードでありアプリの自然な終了結果ではない。未追跡子孫や旧不明プロセスの終了は未確認。

## 実行条件・限界

起動前に6専用ディレクトリと親の再解析点0・空・広いグループへの許可0、status 不存在、EXE と観測器 SHA の一致を確認。入力なし、実 PAT・実 GitHub 操作なし。Credential Manager と WebView2 の OS 共有、同一ユーザーとの競合、依存の通信・子孫の無条件終了は保証していない。旧設定・fixture・製品は変更していない。新規診断領域を保持し、削除対象なし。

作業場所 `D:\Projects\GithubIssueWidget`、branch `main`、HEAD `5e74e1642fd18fe630dce0e684afd853721276ae`。起動器・固定分類・直接終了照合は終了0。UI 表示・通常終了・根本原因・MVP 完成は未検証。これ以上同条件での再起動を繰り返さず、段階の欠落と実際の窓 build を別担当が調査し、次の情報を得る方法へ進む。
