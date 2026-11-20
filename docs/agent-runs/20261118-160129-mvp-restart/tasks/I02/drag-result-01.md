# I02 ウィンドウ移動の読み取り診断結果 01

- run / task / 担当: 20261118-160129-mvp-restart / I02-drag / `/root/drag_diagnosis`。対象依頼 `drag-request-03.md` SHA256 `46C1451583A57D0D71EC2A607F409C92CE0F7FA7D1E81790CA2FBA83CE5E7BEE`。
- 結論: **原因は未確定。ドラッグによる移動は未実証。** 既存証拠は操作 API の呼出しと直後の不変座標を示すが、入力が WebView の対象要素に届き、左ボタンが OS 側の処理開始まで保持され、Tauri コマンドが実行され、Windows が移動処理を開始したことは示していない。新たなドラッグ・入力・対象製品の変更は行っていない。
- 対象: `D:\Projects\GithubIssueWidget-issue-2-20261118-160129`、branch `task/issue-2-foundation-20261118-160129`、HEAD `a4ac97286785a2a5ba0d070a909b58003fc49ff1`、未コミット17ファイル。開始と終了時に `result-01.md` の全17ハッシュと status を照合し一致。計画・独立レビュー・既存結果・既存レビューは依頼にある各 SHA256 と一致。

## 一次情報と確認事実

- `tasks/I02/evidence-03/drag-history-01.md` SHA256 `49233DFDBC3A610E1FE99670607463C26E917F496022D9756EAC3A07E74B7B02` は実装担当が会話履歴に基づき作成した事後記録。対象は debug exe、window ID `12129262`、902×522、origin `(1477,796)`。2回とも `@oai/sky` の `drag` 呼出しは例外なしで、直後の画面状態で origin 不変と報告。開始点 `(300,27)`、終了点は初回 `(180,80)`、次回 `(160,80)`、いずれもウィンドウ相対。元のスクリーンショット・独立ログ・各段階のイベントは保存されていない。旧版のコード SHA は未記録。2回目の現在ファイル `widget/src/main.tsx` SHA256 は `07318B40089C01EEB897EFDB5A933DD15982C51E51CDAEABC097AC219C867AFE`。初回・2回目は実装方式が異なる。
- ローカルの Computer Use `26.1111.21308` の `docs/api.md` は `sky.drag` にウィンドウ相対の開始・終了座標および任意の screenshot ID を定義する。`SKILL.md` は SendInput・UI Automation・Windows.Graphics.Capture を使用すると説明するが、参照した API 文書は押下継続時間や OS 到達の実測を保証しない。呼出し成功と移動成功は別。実際の入力時系列は残っていない。
- [Tauri Window Customization](https://v2.tauri.app/learn/window-customization/)（2026-11-18 UTC 閲覧）は `data-tauri-drag-region` が属性を直接指定した要素にのみ作用し、子要素に継承しないこと、およびカスタム処理として mousedown から `startDragging` を呼べることを記載。旧版の実際の target と属性配置は不明であり原因の証明ではない。現在版は `header` の左ボタン `onMouseDown` を使い、button 以外の子要素からのバブリングを想定している。
- `widget/src-tauri/Cargo.lock` が固定する `tauri` 2.12.7、`tauri-runtime-wry` 2.12.5、`tao` 0.37.2 とローカル取得ソースを照合。`@tauri-apps/api/window.js` の `startDragging` は `plugin:window|start_dragging` を呼ぶ。`tao-0.37.2/src/platform_impl/windows/window.rs` の `handle_os_dragging` はカーソル位置取得、`ReleaseCapture`、`WM_NCLBUTTONDOWN` / `HTCAPTION` の `PostMessageW` を実行する。戻り値の成功のみで OS による最終移動までは証明できない。対象 capability には `core:window:allow-start-dragging` があるが、過去の許可判定結果は未記録。

## 仮説と区別に必要な証拠

| 層 | 候補と未確認 | 識別のための計画案（いずれも今回未実行） |
| --- | --- | --- |
| 押下・座標 | 相対座標と実座標の対応、DPI、実際の DOM target、ボタンに重なるか、旧属性の位置は不明 | 既存スクリーンショットと DOM 配置の保存済み証拠があるか確認。対象版・window ID・取得時刻・スケール・座標系を付記。画像や target の過去証拠がなければ不足とする |
| 入力・保持 | `sky.drag` の呼出し結果だけでは左ボタンの時系列・実対象への送達は不明 | ツールの実装または入力履歴の既存ログが入手可能かを読み取りで調べる。観測装置の設置や入力の再送は別の計画・承認が必要 |
| WebView と権限 | `mousedown` が発生したか、button 判定、Tauri invoke の開始・成否は不明 | 実施前に非機密の event type / target 種別 / button / 時刻・コマンドの成否だけを記録する計測方法を別途設計。計測用変更は対象 SHA が変わるため新規版として固定し、秘密を記録しない |
| Windows | OS へのメッセージ配送、押下状態、snap・移動座標・DPI の影響は不明 | OS イベント・矩形・カーソル情報の取得方法を静的に評価し、計測の副作用とウィンドウ識別、収集範囲、ログ保存先・時刻・撤去方法を先に審査。呼出し成功は移動の証明と区別する |

## 次の判断と停止条件

- 保存済み事後記録からは原因を一意に特定できない。上の観測手順は設計案であり、入力注入、実行中アプリへの接続・計測、OS 操作や製品変更の認可ではない。安全性、個人情報・秘密の混入、観測による挙動変更と復元方法を管理・独立レビューで確認した上で新版の依頼を要する。
- 原因が確定するまで同じドラッグ／移動を再試行しない。新たな計測が許可されても、同種失敗が初回を含め3回に達した時点で全作業を停止し親へ報告する。版変更・証跡不一致・ログへの機密混入・対象ウィンドウ不明時も停止し、記録を保護する。
- Issue #2 は移動だけでなく `review-01.md` B02〜B04 の実機測定・ビルド証跡・URL 許可拒否も未達。今回の読み取り診断は完了の証明ではない。

## 実施記録と限界

- 2026-11-18 16:59 UTC 前後、cwd `D:\Projects\GithubIssueWidget` で `Get-Content -Encoding UTF8` により依頼・計画・レビュー・結果・事後記録と Computer Use の文書を読み、`Get-FileHash -Algorithm SHA256` で依頼に挙げた4資料と事後記録を照合。各終了コード 0。公式資料は上記 URL の `web.open` / `find` で参照。
- 同 UTC 時刻帯、cwd 対象 worktree で `git branch --show-current`、`git rev-parse HEAD`、`git status --short --untracked-files=all` と `result-01.md` の一覧に対する `Get-FileHash -Algorithm SHA256` を開始時・終了時に実行し、各終了コード 0、17件一致・削除なし。変更ファイルや対象版の変更は観測しなかった。
- 静的読取として `rg` で依存の実装と Computer Use API 文書を検索し、`Cargo.lock` の版と照合。OS イベント、入力の押下・解放、実際の target は未観測。開始・終了時の照合は間の一時的な変更がなかった完全な証明ではない。
- 診断の前回ターンに `workdir` 誤入力で OS error 267 が1回発生し、正しいパスで読み取った。同種再発なし。今回の再開以降、本担当の実行失敗はない。履歴上の文字破損2回・移動失敗2回は既知で、同経路を実行していない。
