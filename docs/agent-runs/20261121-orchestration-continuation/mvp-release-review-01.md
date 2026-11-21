# 現行 MVP release ビルド固定版レビュー

- 確認 `/root/plan_review`、作業者 `/root/gui_failure_investigation` と別。依頼 `mvp-release-review-request-01.md`。
- 開始・終了時 SHA256: preflight `BC81031242E366E938E7E5296D00BFB219ACA8067A3335602EBC16E7588E3BEC`、runner `E2F83ADFEEDEE21FE7E3889A81BBE8EC99A75ED8326B5D9F2753255BC4A263F7`、manifest `673FDABA5A91336866FB4A8F073F773E071FF67146F2F04516CD9DD337B89024`、build.cmd `BAE3C381D954B2AB243708605D05B3113636B83FACAFC18D98BB58F46321F2EA`。期待値と一致。削除対象なし。
- 判定: **固定版の MVP ビルド方式を承認**。blocking なし。既存のビルドセッションと入力・出力の直前確認後、1回実行してよい。ビルドの成功、GUI 起動や Issue 完成の承認ではない。

| 項目 | 結果 |
| --- | --- |
| 現行入力 | manifest 20件について現 `widget`、MVP 保存入力、ビルド用コピーの全 SHA を独立照合し不一致0。main.rs・main.tsx は診断コードを含まない現行製品版と一致。これは照合時点の証拠。 |
| 既存方式との差 | 既承認 `run_build.py` との diff はログ名と status 名だけ。既存 build.cmd、offline と専用 Cargo/target・設定・temp を利用。過去 build の cache/target は再利用するため旧 manifest と同一とは主張しない。 |
| 保護と出力 | 旧診断 main・UI・EXE と先行 movement・test のバックアップを限定一覧で確認。新 MVP ログ・status と配布先 EXE は現時点で不存在。配布先・親の実体と再解析点、同名不存在、旧ログ・バックアップの保持を直前に再確認し、既存ファイルを上書きしない。 |
| 成功と限界 | cmd の直接終了・ログの完了・新 EXE と配布コピーの SHA・読取/実行アクセスを区別して記録。これだけでは Windows 上の GUI 表示、認証、GitHub や子孫全体の終了を証明しない。失敗・不明は再実行する前に原因を確認する。 |

現 checkout の変更と専用コピーとの照合後、他の担当による同一 target 使用がないことを確認し速やかに実行する。成功済み同一版のビルドを新たな懸念なしに繰り返さない。ログ・個人情報・認証情報を不要に公開せず、実 GUI は別途確認する。

`D:/Projects/GithubIssueWidget` で依頼・preflight・manifest・runner 差分と20ファイルを読取り照合、対象ファイルは存在を個別確認して SHA を取得（終了0）。差分表示の終了1は差分あり。ビルド・GUI・外部操作・製品変更はしていない。対象版が変われば再評価する。
