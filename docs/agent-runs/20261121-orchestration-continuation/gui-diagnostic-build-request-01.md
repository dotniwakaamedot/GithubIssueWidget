# GUI 固定ステージ診断版のビルド・実測依頼

- 担当 `/root/project_survey`。親による担当指定後に開始する。共有 run `D:/Projects/GithubIssueWidget/docs/agent-runs/20261121-orchestration-continuation/`。診断版 `gui-diagnostic-isolated-01/main.rs` SHA `50692BB0FCEBB9B3B32D1FC88A43DCE1B0238EE681E534A6BC5D62E3EC87F4FD`、`main.tsx` SHA `603473420E487660FB9B5467806268F8507BC00DE8AE7BF9DA93953A438B834C` と承認 `gui-diagnostic-isolated-review-01.md` を確認。
- `gui-release-build-01/` とそのログ・出力・前回 EXE は前担当の成果として保護。対象の書込み停止・保存・所有移管とハッシュを当該担当から明示確認し、親が許可した場合だけ専用 build 環境を再利用する。移管までは触らない。無意味に大きな依存を再複写せず、再利用不能なら必要分の新規場所を用意する。診断ソースの配置前に旧入力のバックアップ・SHA を保存し、既存結果・ログを上書きしない。
- 現行製品の対象版・他入力の SHA、専用 Cargo/target・TEMP・環境・オフライン・ログの所有を固定して、診断版を一度ビルド。実行前に具体的コマンド・対象・保存先を確定し、既存の承認された release ビルド方式との差分を別担当が確認する。失敗・未確認の終了や不明な子孫を成功にしない。既合格の無関係な試験は反復しない。
- ビルド済み診断 EXE の SHA と専用の空 APPDATA・診断記録先、既存ユーザー credential や GitHub に触れない前提、固定 marker と秘密を含み得る一般出力の扱い、同一プロセス・ウィンドウ観測・終了と未確認を固定し、診断実行はその方式の独立確認・開始通知後に一度だけ行う。既存の DEVNULL 観測を同条件のまま繰返さない。
- 固有の結果ファイルに対象版・コマンド・直接終了・ログの範囲・実際のステージ・限界を記録し、停止後パスを通知する。実際の Windows UI、実 credential/GitHub と製品版の完成は別に検証する。
