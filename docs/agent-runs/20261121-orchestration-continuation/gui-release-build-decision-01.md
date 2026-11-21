# release ビルドの実施判断

`gui-release-build-review-01.md` は `run_build.py` SHA `027C3800BAE10F7610AA2B89CF81199CDC9E3969115FF61B28E0E09A1C30B8F3` と `build.cmd` `BAE3C381D954B2AB243708605D05B3113636B83FACAFC18D98BB58F46321F2EA` の固定版を承認。`/root/gui_failure_investigation` は `gui-release-build-finalize-request-01.md`・レビューの実行前条件を再確認できた場合、専用領域で現行版の release ビルドを1回実行する。未知の追加・リンク・入力差、出力先・権限・容量・ログ保護が不適切なら開始せず通知する。

継続中は同じ実行セッションを観測し、時間切れ・子孫の状態不明を終了と扱わない。直接終了・終了コード・ログ完全性・生成 EXE SHA と未確認を結果に記録。GUI の起動、実 credential・GitHub 操作はこの判断に含まない。同じ失敗2回時は別担当へ調査を依頼する。
