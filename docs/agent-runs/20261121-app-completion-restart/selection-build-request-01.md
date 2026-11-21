# 文字選択抑制版の隔離ビルド依頼

- 担当予定 `/root/selection_build`。親が起動。共有 run `D:/Projects/GithubIssueWidget/docs/agent-runs/20261121-app-completion-restart/`、作業場所 `D:/Projects/GithubIssueWidget`。ビルド作者は修正作者と別担当。
- 対象 CSS `widget/src/style.css` SHA256 `FFE871A15BDA8FAD0BB5E28E1717C93D2878ABB1F5885F730EC624DEC983FCFE`、`widget/src-tauri/tauri.conf.json` `0132FCC92345F5D60DE84FD25197FCE853044AD63DAE28757AB4592CC72B991E`、`widget/src/main.tsx` `B22504C9DE48D275726C912820DA93E02F10C1838CCC68555C156D35030165D7`、基準 HEAD `4961b25bec53a93439acdf5230de77f959908af0`。開始前後に対象版を確認、異なるなら停止・質問。
- 新規 `selection-build-sandbox-01/` の独立した入力・target・ログ・成果物で Windows release をビルドする。前ビルド `drag-build-sandbox-01/`、既存 release と他担当素材は読み取りに限り上書きしない。依存は必要なものを分離して offline で利用、実アプリ・資格情報・GitHub を操作しない。コマンド・子プロセス・ログ・入力・終了方法と空き領域を先に確認。源・コピーと EXE の SHA、終了コード・ログ、必要な全ビルド入力の対応を記録する。状況不明を成功扱いせず、同種失敗2回で停止・報告。
- `selection-build-result-01.md` と専用領域だけを所有。共有環境で他者の変更を戻さない。UTF-8 実データ読戻し後に確定しパスのみ通知。独立レビューと実 UI 検証まで問題解決を主張しない。
