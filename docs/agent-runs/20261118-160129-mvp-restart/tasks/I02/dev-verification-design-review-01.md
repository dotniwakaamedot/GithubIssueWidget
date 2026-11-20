# I02 開発起動・正常終了計画の独立レビュー 01

- 担当 `/root/process_test_review`、作者 `/root/build_verification`。対象 `dev-verification-design-01.md` SHA256 `B8563D57E131E8D70CD6307270BAF394084B4C1B297BA1D29A19EF694190F037` は依頼値と一致。依頼の8参照と元・release コピーの各17対象は全件の存在確認後に SHA256 を照合し一致。
- 判定: **対象版の実行計画に限り承認**。blocking 指摘なし。具体 driver・overlay の承認、実行許可や開発起動の成功、Issue #2 完了を示さない。

| 確認点 | 独立確認・判断 | 限界と実施前の条件 |
| --- | --- | --- |
| 対象と競合 | 元17件と専用コピーの SHA は一致。元 Vite は127.0.0.1:1420・strictPort、Tauri devUrl は localhost:1420。計画は既存1435と別とし直前確認を要求。 | localhost の名前解決・IPv4/IPv6、他者の新規起動や共有デスクトップの非競合は未確認。依存取得・生成物・OS/network の通常副作用は残る。 |
| 設定と WebView | 元設定は identifier `com.githubissuewidget.app`、window 個別 dataDirectory の指定なし。Tauri の公式設定資料と固定 tauri 2.12.7 source は appDirectoriesOverride が既定 WebView data directory に影響すると説明する。計画は専用 root と `WEBVIEW2_USER_DATA_FOLDER`、registry policy の影響確認を区別。 | overlay の実際のマージ・WebView2 の実効 profile・registry override の有無と実際の分離は未確認。環境変数は権限境界ではない。判別できない場合は成功扱いせず停止する。 |
| dev 起動・終了 | 直起動 Node/npm、対象 app と Vite を別 process とし、PID/作成時刻/exe SHA/window 所有を結ぶ。CloseMainWindow true のみで成功とせず同じ app handle の終了・exit 0、Node/npm の exit と両 EOF を別々に取得。 | 実際の Tauri CLI・Vite の連鎖終了や close の受理は未実施。Vite の独立 exit を取れなければ未確認。ウィンドウ可視属性は非遮蔽や表示確認の証明ではない。 |
| 保護・停止 | 新規専用 copy、cache、profile、target、証跡、保護ログ・独立確認・receipt を指定。画像取得の反復・既存1435 process への干渉を禁止。timeout・不明・同種2回で調査、3回で停止。 | 具体 driver・設定・ログ取得・profile 判定と UI/proc 関連の操作方式は別版で固定・独立レビューし、適用される Computer Use の制約も実施前に確認する。正常な他アプリを変更しない。 |
| 完了条件 | 成功しても overlay を使った debug 版に限り、既定 data directory・release exe・移動やサイズ・Opener まで拡張しない。 | **B03 全体や Issue #2 は未完了**。既定 profile の挙動は未検証として残す。 |

## 実施内容と限界

- 依頼の8参照を全件存在確認後 SHA256 照合。manifest の元・専用コピー各17件も存在確認後 SHA256 照合、全件一致（PowerShell、cwd `D:\Projects\GithubIssueWidget`、exit 0）。元設定・package・Vite を読み、固定 tauri 2.12.7 の WebView/path source と Tauri 公式設定資料を静的照合。
- 参考: [Tauri Configuration](https://v2.tauri.app/reference/config/)、[Process.CloseMainWindow](https://learn.microsoft.com/en-us/dotnet/api/system.diagnostics.process.closemainwindow)。前者は app directory override と WebView への適用例、後者は close 要求と実終了の区別の確認に使用。現在の環境での成功を示す証拠ではない。
- 実機、UI、画像、build、起動、process の停止・操作、依存導入、製品/GitHub 変更はしていない。旧 window metadata review 公開操作の再試行・回避も行っていない。削除提案なし。出力が長い source 検索は表示が切り詰められたため必要箇所だけを根拠とし、完全検索の証明には使わない。コマンド失敗なし。
