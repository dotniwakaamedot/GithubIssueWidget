# 現行版 release ビルド実行器レビュー 01

- 確認担当 `/root/plan_review`、作者 `/root/gui_failure_investigation` と別担当。依頼 `gui-release-build-review-request-01.md`。
- 期待・開始/終了 SHA: `run_build.py` `027C3800BAE10F7610AA2B89CF81199CDC9E3969115FF61B28E0E09A1C30B8F3`、`build.cmd` `BAE3C381D954B2AB243708605D05B3113636B83FACAFC18D98BB58F46321F2EA`。入力 manifest `BE90B991F9B5B4B1460CED30ABE968E59A7E9C851B6B44B19DB7A0B27D6CE89A`、registry manifest `CDE20D556358AC7DB91DB5CE2B733AA608DAD656B886EBE386D8A66C4D097616` は準備記録値と一致。`concrete-build-01.md` SHA `2924EDFA1FA84920A2B422FF4F397875A90158379E3B3539C4223F59F1ABDA00`、`preflight-01.md` `9E5C3A1F7269E6C2B358E39C939268B551D75B56D74B2090AAF981170158C733`。削除対象なし。
- 判定: **固定版のビルド方式を承認**。blocking なし。実行直前に入力・出力と権限を確認し、管理の開始通知を受けて1回実行する範囲に限る。ビルド成功・GUI 起動・実サービスの承認ではない。

| 項目 | 独立確認と未確認 |
| --- | --- |
| 入力と既存資産 | 専用 widget の manifest 4,994件、専用 Cargo registry の746件を実ファイル SHA と照合し不一致0。manifest 中の node_modules 以外19製品入力を現 checkout と比較し不一致0。開始直前にも最新対象との一致、未知の追加・リンク・変更を確認。既存未コミットを含む製品を変更しない。 |
| ビルド実行 | `run_build.py` は固定 `build.cmd` を専用環境・cwd、DEVNULL stdin、CREATE_NO_WINDOW で起動し直接終了を待つ。cmd は MSVC 設定・移動の失敗を判別し `npm ... tauri -- build --no-bundle` の終了を返す。実際の MSVC・Cargo 依存やビルド成功は未検証。 |
| 専用領域・offline | TEMP、APPDATA、LOCALAPPDATA、Cargo home/target、npm cache/config を専用先に設定し、Cargo と npm の offline を指定。既存 rustup ツールチェーンは共有読取り想定。build script や依存の実通信・Credential Manager 非アクセス、共有ツールの副作用は未証明。実 PAT と明示的な GitHub 操作を含まないビルドであり、残余リスクを記録する。 |
| ログ・終了 | ログ/status は実行前に存在せず、ログは排他的に作成。出力を会話へ出さず専用領域に保持する。ログはサイズ無制限で容量・ACL・保持期間の確認を要する。直接 cmd の終了と子孫の終了は別、監督停止や不明の場合は領域・証拠を保持して再実行や不明 PID の終了をしない。 |
| 成功判定 | 本ビルドだけを一度実行し、実行者は直接終了、ログの完全性、生成 EXE の存在と SHA を記録し別担当が照合。失敗や未生成を成功としない。GUI 起動や実機操作は別の方式と検証が必要。 |

非 blocking: 実行担当は最新入力、ファイルの追加・再解析点、出力領域の不存在、空き容量とログ ACL・保持方針を直前に確認し、不一致や不適切な保存先なら開始しない。ログは環境パスを含み得るため内容を無差別に公開せず、必要な安全な診断のみを抽出する。実際の `status`・終了コード・EXE SHA を記録し、同一版の成功を理由なく再実行しない。

`D:/Projects/GithubIssueWidget` にて依頼、具体案、準備結果とスクリプトを読取り、manifest 記載全5,740ファイルと現行製品19ファイルの SHA を比較（不一致0）。SHA の最終照合は終了0。ビルド・GUI・外部操作・製品編集は行っていない。対象版や条件が変われば再評価する。
