# ドラッグ修正版の隔離ビルド依頼

- 担当予定 `/root/drag_build`。親 `/root` が起動。共有 run `D:/Projects/GithubIssueWidget/docs/agent-runs/20261121-app-completion-restart/`。
- 対象: `widget/src-tauri/tauri.conf.json` SHA256 `0132FCC92345F5D60DE84FD25197FCE853044AD63DAE28757AB4592CC72B991E`、HEAD `4961b25bec53a93439acdf5230de77f959908af0`。先行 `drag-fix-result-01.md` を参照し、他の全ビルド入力の実体と SHA を固定・記録。conf が相違したら開始せず質問する。
- 作業: 依存が利用可能で安全に隔離できる場合、run 内の新規 `drag-build-sandbox-01/` に入力をコピーし、独立した target と出力・ログで release build を行う。既存 `widget/release-20261121`・`release-20261121-02` と既存 build/cache は上書きしない。依存取得・ネットワーク書込み・実アプリ起動・資格情報操作をしない。実行前にコマンド・生成物・子プロセスと終了方法を確認、環境制約で安全に成立しない場合は実行せず理由を記録。
- 結果: 新 EXE の絶対パスと SHA、ソース入力一覧・SHA、ビルドコマンド・作業場所・終了コード・安全なログとその SHA、未検証を `drag-build-result-01.md` に記録。EXE を勝手に配布先へ上書きしない。作者以外のレビューと実 UI が必要。専用領域と結果のみ所有し、他者の変更を戻さず、確定後パスのみ通知する。同種失敗2回は作業を止め調査へ引き継ぐ。
