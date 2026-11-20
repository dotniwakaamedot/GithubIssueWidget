# I02 旧診断の安全な終了方法の確認 01

- 担当 `/root/drag_design`、宛先 `/root`・`/root/restart_planner`。対象依頼 `tasks/I02/geometry-execution-request-01.md`。
- 構造化参照14件は各基点と相対パスを分け、全件存在後に SHA を照合し一致。所有 session `22902`・`17031` は空 poll で session ID が返り終了していない。read-only 確認では PID `21312`、HWND `267184`、旧診断 exe path および SHA256 `2E7A6582B8F74E3E005788E4A0C5B311BCECCF956119726F4C8760477DCC23C4` は一致し、port `1435` の所有 node PID `22956` の command line は旧専用 scratch と一致。観測時刻 2026-11-18T18:58:57Z 前後。handle と PID の寿命や外部の並行状態の保証ではない。
- 旧アプリを穏当に終了する必要があるが、前回は対象 app の AX tree と screenshot の内容が一致せず、安全領域と現在の desktop 状態を確認できなかった。`computer-use` guidance は Windows desktop が locked の場合に操作を停止するよう指示する。現時点でロックと断定はできないが、画像からは対象窓を確認できず、安全な UI 操作先の確証が不足している。前の画像取得・browser inventory・getState は禁止されており再実行していない。
- 質問: 同じ画像取得を繰り返さず、所有アプリの穏当な終了先と desktop 状態をどう確認すべきか。強制終了、推測による UI 入力、他アプリ変更は行わず停止中。代替が安全に利用可能なら具体的操作と根拠を確定回答に示してほしい。ユーザーによる画面状態の確認が必要なら親が判断する。
- 旧 app・server の停止は未実施、新 geometry 診断の依存導入・起動・入力は未実施。削除なし。今回の新たな失敗はない。既往の画像不一致1回、browser inventory error2回を保持し、同種2回で該当操作中断・別担当調査、3回で全中止。
