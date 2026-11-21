# GUI 診断版ビルド方式の独立確認

- 確認 `/root/plan_review`、実行者 `/root/gui_failure_investigation` と別担当。依頼 `gui-diagnostic-build-review-request-01.md`。
- 期待・開始/終了 SHA256: preflight `84CD3062CC3FE424AAC584AF96A2D38AE9D99B2DDDF44541B90B0FE89F2D79B6`、診断 runner `0A71A2FD9A3C083A072CE480987F0FB885E513FBAF54FFC5E7DD6BDD4B0315D3`、`build.cmd` `BAE3C381D954B2AB243708605D05B3113636B83FACAFC18D98BB58F46321F2EA`。削除対象なし。
- 判定: **固定版の診断ビルド方式を承認**。blocking なし。旧プロセスの終了、固定入力と新ログ不存在を実行直前に確認し1回行う範囲。GUI 起動、診断結果、現製品全体の承認ではない。

| 条件 | 独立確認・限界 |
| --- | --- |
| 旧成果の保護 | 限定一覧で `diagnostic-backup-01/previous-release.exe` を確認、SHA `23C383EB2E46CB205E052A9D4A24EE73B968EB2C776F7150826350685419B306`。旧 main.rs `61595B68317E15FDD8F2BED88B920A468D5516740E3D3124710384709C52FC24`、旧 main.tsx `6FCF2A34145450C5029B7FF80DDB3EC1DA5E27173AAB84E57F16D6D54C0E6A3E` も保存。旧 build-output.log は既存結果の SHA `82E7D088500E604F157214A789A3404B24927349A932D1E470DAD53F425FC55E` と一致。旧 target の成果物は再利用により変わり得るので、保護対象を明示する。 |
| 診断入力 | 専用複製の main.rs `50692BB0FCEBB9B3B32D1FC88A43DCE1B0238EE681E534A6BC5D62E3EC87F4FD`、main.tsx `603473420E487660FB9B5467806268F8507BC00DE8AE7BF9DA93953A438B834C` は承認された隔離診断版と一致。旧版からの差を固定し、診断版の出力を現製品 EXE の実測と混同しない。 |
| 実行方法 | 旧 runner との差はログ・status 名2箇所だけ。新しいログ/status は現在不存在であり、同名を排他的に予約する。専用 Cargo・target・offline 等は先行の承認方式と同じ。再利用した cache/target は旧 manifest と同一とは主張しない。 |
| 終了・安全 | 直接 cmd の終了・ログ・新 EXE SHA を別々に確認する。旧ログ・旧 EXE backup を上書きしない。依存の通信・credential 非アクセスや未追跡子孫は保証されず、失敗や不明で再起動しない。GUI 起動は本方式の範囲外。 |

実行前に旧 build セッション終了と生成物の保護、診断2ファイル・runner・cmd・新ログ/ status 不存在を再確認する。実施後は終了・ログの完全性・新 EXE の SHA と旧 backup/ログの保持を記録する。既存データや実資格情報を使わず、変わらない同一ビルドを無理由に繰り返さない。

`D:/Projects/GithubIssueWidget` で依頼・preflight・旧結果と runner 差分、限定一覧と各ファイルの SHA を確認。`git diff --no-index` は差分あり終了1。初回の一括 `Get-FileHash` で EXE 名を存在確認前に誤推測し PathNotFound を表示したが終了コードは0で、該当出力は証拠とせず限定一覧・個別確認で正名を取得。管理と別担当へ調査を依頼した。ビルド・GUI・外部操作・製品編集はしていない。
