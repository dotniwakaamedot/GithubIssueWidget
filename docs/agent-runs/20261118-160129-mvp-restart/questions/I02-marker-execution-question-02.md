# I02 旧 geometry 終了と新版準備の進捗 02

- 担当 `/root/drag_design`、宛先 `/root`。対象 `tasks/I02/marker-execution-request-01.md`、終了方法の判断 `questions/I02-marker-execution-answer-01.md`。
- 依頼構造化参照16件はすべて SHA 一致。旧 app PID `9796` と server PID `54736` の exe/command line・親・開始時刻・port・所有を停止直前に再照合し、親判断に従って各 PID だけを 2026-11-18T19:22:12Z、19:22:25Z に `Stop-Process`。**強制終了**であり正常終了とは扱わない。各操作 exit 0、所有 app session `4498` と server session `68650` は terminal `-1`。対象 PID・既知 app child PID はその後非検出、port 1435 listener 0件。全部の別名子孫不存在は証明しない。
- marker scratch の対象18ファイルを copy-manifest と新版 main.tsx の指定 SHA に対して照合。専用 cache・target・temp・profile・証拠領域と npm 設定を準備。専用 Rust 1.90.0 toolchain install、公開 registry 指定 `npm ci`、`npm run build`、VS 18 環境の `cargo build --locked` は各 exit 0。npm は esbuild install script 未承認警告。旧 scratch・元製品・B03 と既存証拠は保護し、OS の通常 cache/network 副作用の可能性は残る。
- 次は専用 port と対象 process を再確認後に marker 診断版を起動し、公開 API の個別窓で空 output と geometry output の可否のみ観測する。画面・DPI・余白・非遮蔽が不明なら入力しない。過去の失敗した screenshot / getState / browser inventory と元ドラッグは再実行しない。
- 今回新たな失敗なし。直接公開2件、browser inventory2回、画像不一致1回の履歴を保持し、同種2回で独立調査、3回で全停止。今回の結果ファイルは同一ディレクトリ `.tmp` → UTF-8 検査 → 非上書き rename で公開。
