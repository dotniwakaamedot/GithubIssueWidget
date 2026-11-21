# GUI 診断版の固定ステージ実測依頼

- 担当 `/root/gui_failure_investigation`。共有 run `D:/Projects/GithubIssueWidget/docs/agent-runs/20261121-orchestration-continuation/`。`gui-diagnostic-build-result-01.md` の診断 EXE SHA `57CFD705772F4AB2CBDD8BFC40EC555280B905872D347E046E4E57B07678055E`、固定 stage ソースと先行の `gui-release-observation-review-01.md` を確認し、新規専用領域・空設定・空診断出力を用意する。既存 EXE・ログ・旧領域・資格情報を変更しない。
- 既存の承認済み直接子保持方式を最小限再利用し、診断 EXE の固定 SHA と専用 `GIW_DIAGNOSTIC_DIR` 等の環境を指定する固有実行器を作成。固定 stage のみを保存し、一般の stdout/stderr は秘密を公開しない。対象と差分・入力 SHA・競合/実体確認・直接子の生存と終了の記録、期限時の保護・識別不能な窓や無関係なプロセスに触れない方法を結果に示す。
- まず固有実行器・実行手順と SHA を別担当へ確認し、管理通知後に**1回だけ**実行。ステージの到達、欠落（未到達・IPC未処理・書込み障害を区別できない）、直接の状態と画面が識別できるかを別々に記録。同じ DEVNULL 観測では得られなかった新しい固定 stage を主目的とし、実 PAT・GitHub 操作や既合格試験の反復をしない。
- 実行中は同じセッションとハンドルを保持。対象の通常閉鎖ができず終了が必要な場合は、先行の `gui-direct-exit-review-01.md` と同一性確認の限定条件を再確認し、管理の指示に従う。未知の子孫の終了・実機表示・旧症状の原因は推定で確定しない。結果を固有ファイルに確定して通知。
