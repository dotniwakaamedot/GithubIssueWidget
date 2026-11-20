# ラベル削除応答の独立確認依頼 07

原文: 「現在の進行状況を確認しアプリの完成まで進行してください。一度の処理で2度同じ失敗をした時は停止してください」
対象: widget/src-tauri/src/movement.rs SHA256 ED49F4BA52C6887BCE427E4A6664D335092A1C3311E1B1ED61B4CE322791ABA5。変更前は review-06.md。
目的: GitHub の個別ラベル削除の成功応答に、移動先ラベルが存在し旧ラベルが存在しないことを確認。矛盾や不正応答時は後続の削除や state 更新を中止し部分適用を明示する。
一次資料: https://docs.github.com/en/rest/issues/labels?apiVersion=2022-11-28#remove-a-label-from-an-issue
ログ: 一時フォルダー giw-removal-confirm-e31206ac05fa422b8de392a193d98b7f.log、本体65 passed・9 ignored、URL範囲1 passed。
依頼: GitHub 応答との適合、異常時の処理、既存６方向と複数ラベルの模擬試験、未解消の競合を照合して review-07.md に記録。コードや設定、認証情報、GitHub は変更しない。出力は小さくし、同種の失敗２回で停止。
