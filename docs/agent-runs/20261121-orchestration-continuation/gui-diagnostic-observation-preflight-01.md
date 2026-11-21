# GUI 固定ステージ診断の観測方式

担当 `/root/gui_failure_investigation`。対象 EXE SHA256 `57CFD705772F4AB2CBDD8BFC40EC555280B905872D347E046E4E57B07678055E` を新規 `gui-diagnostic-observation-01/github-issue-widget.exe` にコピーし同 SHA を確認。ビルドの入力と結果は `gui-diagnostic-build-result-01.md`、固定コードの静的承認は `gui-diagnostic-isolated-review-01.md`。診断 EXE は製品 EXE と異なり、段階の観測は自動的に製品版に拡張しない。

観測器 `gui-diagnostic-observation-01/observe.py` SHA256 `5A89F1E3832ADA05E4892C5C50C72873490E78B4EEF2FFA0C2E06C204677A662` は承認済み `gui-release-observation-01/observe.py` との差が対象 SHA、空チェック対象に `markers` を追加、`GIW_DIAGNOSTIC_DIR` の専用 `markers` 指定の3箇所のみ。構文 AST は終了0。`appdata`・`localappdata`・`temp`・`profile`・`webview2`・`markers` の6ディレクトリは新規空、所有者・SYSTEM・Administrators だけに ACL を制限。実行時には6領域と親の実体・再解析点・空・ACL・同時作業、EXE と実行器・入力版・status 不存在を再照合する。Credential Manager と WebView2 実行基盤は OS 側の共有資源であり環境変数だけでは隔離されない。

候補は workspace から `python docs/agent-runs/20261121-orchestration-continuation/gui-diagnostic-observation-01/observe.py` を1回。直前まで空設定と同一ユーザーの書込み競合なしを確認し、実資格情報・実 GitHub・フォームを操作しない。実行器は start 前に status を排他的に記録、明示的な環境で直子起動、同一保持ハンドルの実行パス・作成時刻、PID・30秒生存・終了コードを記録し続ける。標準出力/エラーは DEVNULL、一般 panic 本文は失われるが新しい固定 stage が得られる。対象 HWND を CUA の一覧とプロセスにより一意に結び付けられる場合のみ、空設定画面を観測し通常閉じる。紐付けできなければ他窓に触らない。

30秒経過時と終了後の `markers/stages.log` は**既知固定行の分類のみ**読取り、内容を無加工で表示しない。上限・ファイルの実体・アクセスを確認し、固定 enum `before_run`、`setup_entered`、`frontend_mount`、`connection_requested`、`connection_succeeded`、`connection_failed`、`pending_requested`、`pending_succeeded`、`pending_failed`、`invalid_stage` の有無と未知行・読み取り失敗を区別する。到達順は非同期 IPC の順序と同一とは限らず、欠落は未到達・未処理・書込み失敗を区別できない。ログの不在を製品バグと断定しない。排他制御は同一ユーザーの別プロセス全体の保証ではない。旧領域やそのログは触らない。

30秒後も対象の窓を CUA が列挙できず通常終了できない場合、対象の空設定・未入力を維持し、管理に状況を報告する。終了は以前の `gui-direct-exit-review-01.md` と同じ限定方針を今回の固定版について別途確認し、自己起動の保持子のみ、パス・作成時刻・生存を同じ新規ハンドルで照合した上で行う。PID や HWND のみで停止しない。直接子と未知の子孫の終了は別、観測器停止・照合失敗は不明として領域を保護し再実行しない。追加の GUI 取得・診断コード実行はまだ行っていない。

この方式と対象版を独立レビューし、管理開始通知を得るまで診断 EXE を起動しない。診断は1回、実 credential・GitHub 操作や旧不明プロセス停止は対象外。過去の101と今回の非表示の原因が判明する保証はない。
