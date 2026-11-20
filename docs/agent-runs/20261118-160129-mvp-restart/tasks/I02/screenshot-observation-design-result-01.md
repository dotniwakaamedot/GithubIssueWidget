# I02 診断窓の画像対応と安全領域の観測方法設計 01

- run / task / owner: `20261118-160129-mvp-restart` / `I02-screenshot-observation-design` / `/root/screenshot_design`。依頼 `tasks/I02/screenshot-observation-design-request-01.md` SHA256 `175BFB142F627939CCAAAB74BDA0A5B14518368638E0A4C21E326EAC0F6F78C5`。
- 結論: **既存資料だけで画像と対象窓の一致を保証できない。原因は未確定。安全領域確認前の入力は不可。** 本稿は手順の提案であり、UI 操作や capture の実行、結果・製品版の承認ではない。
- 対象: 質問08に記録された診断 exe、PID 21312、window ID 267184、exe SHA256 `2E7A6582B8F74E3E005788E4A0C5B311BCECCF956119726F4C8760477DCC23C4` は当時の観測値。質問09の handle 22902 / 17031 は 2026-11-18T18:42:30Z 頃に所有担当が生存を確認した記録であり、現在の生存・ID の再利用の不存在を本担当は確認していない。製品版と診断版を混同しない。

## 根拠と限界

- `questions/I02-drag-click-question-08.md` は対象の AX tree に `diagnostic-safe-area` と `diagnostic-ready:[]` を観測する一方、取得画像に対象窓ではなく背景が表示されたと記録。画像・取得範囲・時刻の詳細は保存されず、本人以外は画像を再確認できない。遮蔽、別 desktop、描画・取得先、座標／取得範囲のどれも仮説であり、画面ロックやユーザー不在を推定しない。
- ローカル `C:\Users\semis\.codex\plugins\cache\openai-bundled\computer-use\26.1111.21308\skills\computer-use\SKILL.md` は Windows.Graphics.Capture を使用し、同梱 `docs/guidance.md` は返却された window object の選択、観測と操作の分離、毎回の状態更新、画像の直接表示と screenshot ID の使用を説明する。同梱 `docs/api.md` の `WindowState.screenshots[]` は ID、origin、logical width/height、zIndex を定義するが、ID のみで描画内容・範囲が正しいことの証明にはならない。これは `@oai/sky` の API であり、現在の `cua` app の object や ID を推測して流用しない。
- Microsoft の [GetWindowRect](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getwindowrect) は矩形が DPI 仮想化され不可視枠を含み得ると明記し、DWM の extended frame bounds は別の DPI 取扱いである。[GetDpiForWindow](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getdpiforwindow) は DPI awareness に依存し、無効 HWND で 0 を返す。CSS px、logical screenshot px、window/client/screen px を根拠なしに同一視しない。

## 候補と実行前提（全て未実施）

| 候補 | 方法・副作用・限界 |
| --- | --- |
| A: 対象窓個別の状態 | 所有担当だけが既存2 handle を poll し、現在の PID・exe path / SHA・HWND・開始時刻と対象版・ポートを照合。`cua` の個別 app 状態と AX tree の公開 API を、各呼出し時に提供される文書に従って読む。起動・入力ではないが UI 状態取得とデスクトップ情報へのアクセスであり、他者の内容を記録しない。`getState` / browser inventory は禁止。tree と空 output は存在の一証拠で、画像・表示位置・安全領域の証明ではない。対象消失・同名複数・ID／PID／hash の差異・状態不明で停止。 |
| B: `@oai/sky` による対象窓の限定 capture | A で解決しない場合の代替案。適用される Computer Use の skill・guidance を事前に読み、別 session として `list_windows()` または `list_apps()` の返却 window から exe/path・title・一意性を照合し、返却 object で `get_window_state({window, include_screenshot:true, include_text:true})` を一度だけ取得。以前の CUA ID / object を異なる API の返却 object とみなさない。対象外に同名窓や PID 対応の不明があれば停止。画像は窓と関連 transient UI を含み得るため直接検査し、実際に対象 marker、header、button、resize 帯と空 output を識別できず他者の内容が写れば保存・入力せず停止する。キャプチャ ID・origin・サイズ・zIndex・時刻・対象 window object は当該観測限定で記録し、画像が取得できたという事実だけで正しい対象を保証しない。`sky.activate_window` や再取得、入力はこの候補の自動的な付随操作にしない。環境の対応と browser inventory への影響は未実証。既知のエラーが再発すれば3回目として全停止。 |
| C: OS 窓矩形の補助観測 | 承認済みの独立担当が新しい別依頼・別の所有領域で `GetWindowThreadProcessId`、`GetWindowRect`、`GetClientRect`、DWM bounds、`GetDpiForWindow`、DPI awareness、monitor・取得時刻を、成否と併記した読取り専用ハーネスで評価する案。実行コードや API の利用権限、Windows の DPI 仮想化・DWM 未対応・別 desktop 等により値の比較が不明なら停止。取得だけではその pixel が当該窓か、CSS element の矩形や覆いの有無、AX と画像の同時性は証明しない。process 内で状態を変更せず、対象アプリの start/stop は行わない。Computer Use が適用される場合は skill の混用禁止を順守し、UI 操作と同一ターンに直接 PowerShell UI Automation を行わない。 |

## 入力前に必要な整合ゲートの案

1. 管理が上記のいずれを実施するか、API の実際の利用可能性・担当・画面への影響・証拠の保存方法を固定し、独立レビューを経て別依頼とする。状態取得の度に `window ID / PID / exe SHA / 診断コード4件の SHA / UTC / session / port` を紐付け、元版との違いを明記。所有担当の poll は本人が行い、別担当は handle を継承したと仮定しない。
2. 個別窓から `diagnostic-ready:[]` を取得して証拠として最低限の AX 対象文字列・時刻・窓識別を保存できることを入力前に確認。対象内容以外の text、URL、Issue、他窓を記録しない。上記値・観測メタデータを UTF-8 で専用証拠に保存し、実体 SHA256、取得 API と成否、取得者と対象版を記録。画像は skill guidance の「検査目的のみで再保存しない」を守り、成果として画像保存が必要な場合は適用 tool の保存可能性と個人情報排除を別途判断。許可された対象内の画像が得られなければ画像なしと記載。
3. 現時点の観測が同一の target window であり、同一時点に近い AX と capture の境界・origin と実ウィンドウの visible/client rect が一致し、選択した DPI awareness・単位換算の根拠が確認できることを確認。複数モニター・DPI 仮想化・枠・取得の時間差・レイアウト変更・ウィンドウの移動で対応が不明なら採用しない。各値と変換式、差、取得時刻、許容誤差とその理由を証拠化し、閾値は未評価のまま固定しない。
4. 診断版の `diagnostic-safe-area`・header・全 button・全 resize 帯の矩形を **同じ版・同じ観測時点・同じ座標系** で取得できる方法を別途確定し、marker 中心が全 button / resize 帯から 12 CSS px 以上の実余白を満たすことを算出する。AX tree が要素名だけで矩形を返さない場合は推測座標で代用しない。画像の視認だけで CSS px の閾値を自動証明しない。適切な客観的境界取得が未検証なら入力禁止。
5. 上記と親判断が満たされた場合だけ、実行者が有効な最新観測から、対象の非対話 marker 中心に **単一押下・解放** を行う別依頼へ進める。所有者以外の対象や既存アプリには触れない。観測後に移動・再配置・表示更新・対象不一致・干渉があれば座標を破棄し停止。診断版のイベント成功を旧2回の事象、製品版の drag・IPC・OS 移動や Issue 完了の証明としない。

## 停止・保管・次担当

- 画像不一致は既往1回。代替観測で同種不一致2回目ならその観測を止め、別担当が客観的に調査する。3回目で全作業停止。browser inventory error は既往2回、`getState` / browser inventory を再実行せず、同じ error 3回目で全停止。失敗時は別 API の連続試行で閾値を回避しない。
- 予期しない他人情報・対象窓消失・プロセス不一致・画面ロック・権限要求・収集不可・座標不明・想定外の通信や書込みが生じたら入力せず停止。所有するプロセスの取扱いは親が判断し、ログ・証拠・既存成果の削除や上書きをしない。
- 個別 API の選択が browser inventory を内部実行しない保証、`sky` の対象 window 選択・capture 成功、AX 境界、DPI 変換、診断版の単一入力、製品版の実移動は全て未検証。Windows 11 実機での Issue #2 完了条件を満たしたとは扱わない。独立レビュアーが出典・対象版・副作用・停止条件を確認し、親が次の担当と実施の可否を判断する。

## 実施記録・失敗と対象

- cwd `D:\Projects\GithubIssueWidget`、branch `main`、HEAD `a4ac97286785a2a5ba0d070a909b58003fc49ff1`。`git status --short --untracked-files=all` exit 0 で共有 run の未追跡を含む状態を確認、出力は長く一部切り詰められたため全件を検証したとは扱わない。他担当の成果は変更しない。
- `Get-Content -Encoding UTF8` による AGENTS.md、agent-workflow、依頼、質問08、レビュー02、computer-use の skill と bundled guidance/API 読取は exit 0。構造化依頼の9参照は基点を分離し全件実在確認後 `Get-FileHash -Algorithm SHA256` で実体と期待値が全件一致。Microsoft 文書は上記の公式 URL を開き関連記述を確認。
- 最初の web 検索 orchestration JavaScript に構文エラーが1回あり、検索は実行されなかった。親へ即報告し同じスクリプトを再実行せず、別の正しい `open` 呼出しで2公式資料を取得した。今回の他のコマンド失敗なし。ステータス出力の切り詰めは表示上の制約として記録し再検索しない。
- 本担当の変更はこの結果ファイルのみ（未追跡、自己 SHA はファイル確定後に親が照合）。削除対象なし。実UI、画像取得、アプリ／process の操作・poll、コピー・build・外部更新は未実施。結果の公開前に親ディレクトリ存在、確定先・tmp 不在を確認し、UTF-8 を検査して同じディレクトリで確定する。
