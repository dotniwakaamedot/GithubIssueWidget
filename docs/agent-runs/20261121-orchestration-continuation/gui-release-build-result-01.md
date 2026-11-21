# 現行版 release ビルド結果

担当 `/root/gui_failure_investigation`。実行依頼 `gui-release-build-finalize-request-01.md`、方式承認 `gui-release-build-review-01.md`、開始判断 `gui-release-build-decision-01.md`。作業場所 `D:\Projects\GithubIssueWidget\docs\agent-runs\20261121-orchestration-continuation\gui-release-build-01\widget`、元 checkout branch `main`、HEAD `5e74e1642fd18fe630dce0e684afd853721276ae`。未コミット変更を含む入力を下記マニフェストで固定した。

## 結果

専用複製で `python docs/agent-runs/20261121-orchestration-continuation/gui-release-build-01/run_build.py` を1回実行。保持したセッション `66951` は完了し、runner 終了0、直接 cmd の記録 `exit_code=0`。ログに `Finished `release` profile` があり、対象 EXE の存在とハッシュを確認した。GUI は起動しておらず、実機表示・資格情報・GitHub・開発モードは未検証。直接 cmd の終了は未追跡子孫の終了を証明しないため、ログが後続プロセスから絶対に更新されないことは未確認。

- EXE: `gui-release-build-01/cargo-target/release/github-issue-widget.exe`、12,883,456 bytes、SHA256 `23C383EB2E46CB205E052A9D4A24EE73B968EB2C776F7150826350685419B306`。
- ログ: `gui-release-build-01/build-output.log`、観測時 10,379 bytes、SHA256 `82E7D088500E604F157214A789A3404B24927349A932D1E470DAD53F425FC55E`。固定分類で release 完了文字列あり、`error:` なし。任意のログ本文は外部へ出力していない。ログは今回の確認と整理判断まで保持する。
- 生成版を固定する入力 manifest SHA256 `BE90B991F9B5B4B1460CED30ABE968E59A7E9C851B6B44B19DB7A0B27D6CE89A`、Cargo registry manifest `CDE20D556358AC7DB91DB5CE2B733AA608DAD656B886EBE386D8A66C4D097616`。開始直前に専用コピー 5740 ファイル・19製品ファイルと再解析点、専用出力先を確認し不一致0。実行後、元19ファイルのうち `src/main.test.tsx` が変更されていた。他担当による同時変更であり、元の変更を触っていない。製品実装入力 `src/main.tsx`、Rust 等は一致したが、完全な現チェックアウト版と本ビルドの入力が同一とは言えない。変更の影響と次の対象版を独立確認する。

## 保護と検証

`run_build.py` SHA256 `027C3800BAE10F7610AA2B89CF81199CDC9E3969115FF61B28E0E09A1C30B8F3`、`build.cmd` `BAE3C381D954B2AB243708605D05B3113636B83FACAFC18D98BB58F46321F2EA`。各値は開始直前に承認版と一致。Cargo/npm は offline 指定、APPDATA・LOCALAPPDATA・TEMP・Cargo home/target・npm cache/config は専用、必要な非機密環境のみを明示した。依存スクリプト自体の外部通信や credential store 非アクセスは保証していない。既存 rustup/MSVC は共有利用した。

ログ保存前に専用ルートの ACL に一般ユーザー読取りと認証済みユーザー書込みを確認したため、当方が新規作成した `gui-release-build-01` ルートのみを実行アカウント・SYSTEM・Administrators に限定し、新規ファイルの ACL に一般ユーザー・認証済みユーザーの許可がないことを確認した。別エージェントはこの制限付きディレクトリへのアクセスを拒否されることがあり、EXE・ログの独立検証に必要なアクセス方法は管理の再割当て・判断待ち。ログを無制限公開しない。元の製品・依存・既存 target、旧 fixture は変更・削除していない。ビルド専用コピー・専用 cache と target が新規生成物であり、所有・保持を保護する。

準備時に Python 複数行のリテラル改行を誤り SyntaxError で終了1、書込み前。here-string に変更した読み取りは終了0。同種1回。管理へ失敗記録の照合を依頼済み。以後の本ビルド・ハッシュ取得は終了0。実機 GUI 起動は独立した対象版・操作・終了方式の確認後に行う。結果提出後は対象を編集しない。
