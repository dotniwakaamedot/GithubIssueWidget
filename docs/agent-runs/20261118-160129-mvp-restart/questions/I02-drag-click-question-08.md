# I02 診断画面の安全領域が確認できない状況 08

- 担当 `/root/drag_design`、宛先 `/root`・`/root/restart_planner`。回答 `questions/I02-drag-click-answer-06.md` に従って既存 handle と対象 ID を確認。
- 所有する dev server session `22902`・診断アプリ session `17031` はそれぞれ poll 時に session ID を返し終了していない。`Get-Process` は対象 exe path 一致の PID `21312` を1件、MainWindowHandle `267184` を返した（cwd `D:\Projects\GithubIssueWidget`、exit 0）。別の同名 exe は上記対象 path では検出されなかった。実 exe SHA256 は `2E7A6582B8F74E3E005788E4A0C5B311BCECCF956119726F4C8760477DCC23C4`。
- 同一 CUA session で `cua.getApp({windowId:267184})` を1回のみ実行し、`http://localhost:1435/` の main WebView、`diagnostic-safe-area` と `diagnostic-ready:[]` を accessibility tree で観測した。ブラウザー inventory のエラーはこの呼出しでは発生していない。対象窓での実空表示の証拠ファイルはまだ保存していない。
- 続けて対象 app の `getAXStateAndScreenshot()` を1回行った結果、tree は同じ空記録と marker を示したが、返された画像には診断ウィンドウではなく背景画像が写り、marker・button・resize帯を視覚的に識別できなかった。画像内容の原因は未確認であり、ロック画面やユーザー状態を断定しない。画像は個人情報保護のため保存していない。対象画像の予期した観測が得られなかった問題は今回1回。空記録の証拠保存、DPI・座標・12 CSS px の実境界・ユーザー操作非競合は未達。
- **単一押下・解放もドラッグも未実施**。安全領域と記録取得ゲートが満たせず、依頼どおり同じ画面取得や入力の再試行をせず停止した。CUA の部分作業は completion summary で終了。所有するアプリと dev server はまだ終了しておらず、再起動・停止は管理判断まで実施しない。
- 質問: 画像と accessibility tree の不一致について、どの証拠と方法で対象画像・DPI・境界を確認すべきか、また所有プロセスの安全な扱いを判断してほしい。同種2回で操作中断・別担当による客観調査、3回で全作業停止を維持。ブラウザー inventory エラー2回はすでに独立調査済みであり、`cua.getState()` は再実行していない。
