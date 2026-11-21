# GUI 固定ステージ診断版ビルド結果

担当 `/root/gui_failure_investigation`。`gui-diagnostic-build-review-01.md` と管理開始判断に従い専用環境でビルドを1回実行。直接 cmd の `exit_code=0`、runner 終了0。EXE `gui-release-build-01/cargo-target/release/github-issue-widget.exe` SHA256 `57CFD705772F4AB2CBDD8BFC40EC555280B905872D347E046E4E57B07678055E`、12,840,960 bytes。新規保護ログは 1654 bytes、SHA256 `CD190F8102230C8D48AF104EE3C6F24902E9BA45F99774E032740535BD49CD47`、固定分類で `Finished `release` profile` あり `error:` なし。未知の子孫とログの以後の変更を無条件に否定しない。

旧 release EXE backup は `23C383EB2E46CB205E052A9D4A24EE73B968EB2C776F7150826350685419B306`、旧ログは `82E7D088500E604F157214A789A3404B24927349A932D1E470DAD53F425FC55E` とビルド後も一致。診断入力・実行器・旧バックアップを開始前にレビュー版と照合した。現製品・旧 fixture・認証・GitHub を変更していない。実行セッション `49332` の完了を確認。GUI は未起動、実測は別途レビューが必要。

ビルド対象・旧 main.rs からの test 限定の差、未追跡子孫・共有 Rustup/MSVC・依存の通信非遮断は `gui-diagnostic-build-preflight-01.md` の条件。作業場所 `D:\Projects\GithubIssueWidget`、branch `main`、HEAD `5e74e1642fd18fe630dce0e684afd853721276ae`。削除対象なし。準備中の1回のアクセス失敗は前記 preflight に記録済み。新たなビルド失敗なし。
