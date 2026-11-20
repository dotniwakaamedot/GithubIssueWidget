# I02 dev 検証計画の進捗 01

- 担当 `/root/build_verification`、宛先 `/root`。依頼 `tasks/I02/dev-verification-design-request-01.md` SHA256 `6267BAA463643AB6C80D279844CD2AAAE818F8504D8A68F13134BABE359B38F1` と7固定参照は実体 SHA 一致。元と release コピーの各17ファイルは全件一致、release exe SHA `D5FD13D80B6FE7F72ACD0BA6CEA3E0E19F261E417F8F5331E81F94D09A379D44` も一致。
- 2026-11-18T21:11Z の読取では診断 server PID 32908 が 127.0.0.1:1435、診断 app PID 52960 が既知専用 exe path で存在。1420 の listener は観測されなかった。既存 handle の生存や他環境・今後の非競合の保証ではなく、停止・操作は行っていない。
- 対象と診断版は共に app identifier `com.githubissuewidget.app`。固定された Tauri 2.12.7 の Windows 実装はデフォルト WebView2 data directory を `app_local_data_dir` とし、ローカル依存 `dirs-sys` は Windows Known Folder を使う。従って LOCALAPPDATA 環境変数だけで profile が分離されるとは断定できず、同一の既存 profile を共有するリスクがある。実際の profile path は未測定。
- 現在、独立した dev scratch / target / profile のための config overlay と、プロセス・正常終了を確認する手順を検討中。Tauri の `appDirectoriesOverride` を使用する案は実行版の設定に差があるため、元の既定設定での起動成功とは区別する。起動・build・入力・process 停止は行っておらず、今回のコマンド失敗はない。
