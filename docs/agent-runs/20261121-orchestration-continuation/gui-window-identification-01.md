# release GUI の窓識別に関する独立調査

担当 `/root/existing_work_review`。起動担当 `/root/gui_failure_investigation` とは別。2026-11-21、共有 run の固定記録と起動担当の回答を照合し、CUA の読み取りのみを行った。PID 6004、HWND 39655226、セッション 83178 の対象への入力・終了・再起動や、他の窓操作は行っていない。保持中のセッション 83178 はこの担当から照会・操作していない。

## 結論

**現在の証拠では対象ウィンドウを CUA 上で一意に選べず、見えているとも見えていないとも判定できない。** 原因は未特定。`MainWindowHandle` の値、空タイトル、`Responding=true`、一覧にない事実は、それぞれウィンドウの可視性や画面内容の証明にならない。窓一覧外の HWND を推測して `getApp` に渡すこと、名前だけで別窓を操作することは行わない。

## 読取り結果

- 起動担当の `gui-window-identification-answer-01.md` によると、保持ハンドルで対象 EXE のパスと作成時刻を照合し、PID 6004・SessionId 2、少なくとも30秒後も HWND 39655226・空タイトル・応答中・未終了。正確な最初の HWND 観測時刻と desktop 名、CUA ホストの session は未取得。HWND の所有 PID を別の API で確認した証拠と実際の可視性はない。
- この担当の `cua.getState()` では対象を含む app entry がなく、続けて `cua.listWindows({emit:false})` は8窓・対象 ID との一致0を返した。ウィンドウ一覧には対象以外の個人画面情報が含まれ得るため、それらの詳細は保存しない。ツールの一覧取得自体は正常で、この結果は**その観測時点・CUA の列挙範囲に該当 ID がない**ことだけを示す。列挙対象のフィルタや CUA ホストの desktop/session はこの情報では不明。
- この担当の PowerShell 自身は `Get-Process -Id $PID` で SessionId 2。対象の SessionId 2 と数値が同じでも、CUA 実行主体の Windows session・window station・desktop の同一性を証明しない。`Get-Process` の結果を用いて独自の窓 UI 操作はしていない。

## 原因候補と安全な次手

候補は、対象 HWND が実際に表示対象ではない、表示初期化が完了していない、CUA 列挙側の条件や desktop/session が異なる、HWND が別の役割であること等。いずれも現時点では仮説であり、`MainWindowTitle` が空な原因、Tauri/WebView2 の失敗、製品側の不具合と断定しない。

1. 起動担当が保持ハンドルで同一対象・直接子の生存を観測し続け、管理に限定した状態を伝える。以前の2回の識別失敗と今回の独立照合で対象を得ていないため、同じ一覧取得を根拠なく反復しない。
2. CUA が識別可能な対象を返す条件とホストのセッション・desktop を、利用可能なツール文書・環境の権威ある情報で調べる。同一の ID が列挙に現れた場合も、保持ハンドルと窓の所有関係を確認できる根拠が必要。CUA の Windows API は一覧から返った ID を選ぶ手順であり、列挙外の値を直接選ぶ方法は文書で確認できていない。
3. 使用可能な read-only の PID/HWND 所有関係、visibility と desktop を確認できる正式な手段がある場合、その権限・実行環境・対象・出力の機密性を先に固定し、保持ハンドルの PID と作成時刻との対応を確認する。Win32 `IsWindowVisible` と `GetWindowThreadProcessId` は候補だが、この担当は呼んでおらず、CUA の「UI 操作は cua_repl を使用」という制約との適合も未評価。プロセスや他窓への書込み・画面変更はしない。
4. 正式な読み取り方法でも CUA と対応できない場合、起動担当と管理は期限・保全・終了方法を別途検討する。ユーザーに確認を頼むことは、対象を一意に示せず、誤操作や私的画面の露出の危険があるまま先行しない。独立した固定入力のビルド・試験・静的監査は継続可能。

## 証拠・未確認

- `gui-release-observation-status-01.md` SHA256 `31F4F0BDDF7DE3A389D98D7B9081554D530EEEC4ACCFC1235C451E37C4290F3B`。
- `gui-release-observation-status-02.md` `4EF067CC86F2FB02EEC2981113DEDC924AF2083B05564ADAF07B329EC77ED0D4`。
- `gui-window-identification-answer-01.md` `BC97BB1B9C2D891E229D5B0F69BA26616ACE9E1685F29D41548B37C45470AEB7`。
- `gui-release-observation-preflight-01.md` `8F17A28BDA7B98121F63F03C3B9835BE7B150EE850EF03400C5ADD020876C20E`。

CUA は対象IDなしの正常な一覧応答で、ブラウザーインベントリにホスト利用不可の別エラーが含まれていたが今回の Windows 窓識別との因果は未確認。`Get-Process` と SHA の読み取りは終了0。対象 UI の表示、終了、退出コード、未追跡子孫、窓とプロセスの現時点での完全な対応はいずれも未確認。この担当の追加照合は元の２回とは別担当による独立観察であり、元の失敗を解消したとは主張しない。
