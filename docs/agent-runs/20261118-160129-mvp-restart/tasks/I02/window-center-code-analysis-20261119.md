# 中央位置の静的コード調査

- 対象: `dotniwakaamedot/issue-widget-mvp-resume` の基盤 `59578f8a4a775b4bd66c6acdde318b2e934abc08`、`widget/src-tauri/tauri.conf.json` は `decorations: false`、`center: true`、幅900・高さ520を指定。観測値は `tasks/I02/window-center-result-20261119.md`、その限界は `tasks/I02/window-center-review-20261119.md` を参照する。
- ローカル依存 `tauri-runtime-wry 2.12.5` の `src/lib.rs` では、初期配置で要求された inner size を使って `window_size` を初期化し、Windows の `AdjustWindowRect` による補正は `decorations` が true の場合に行う。`center` 指定時、monitor の work area で計算した座標を window builder の position に渡す。`src/window/mod.rs` の `calculate_window_center_position` は作業領域と渡されたサイズから左上座標を算出する。
- 推測: 装飾なしでも Windows の実際の不可視枠などにより、計算対象サイズと実際の表示位置が異なる可能性があり、記録された差を説明する候補となる。これは静的ソースと1回の観測からの仮説であり、個別の処理が実行された証拠や DPI context の同時記録はなく、原因確定ではない。
- 次に必要: 同一実行・同一座標系で DPI/context、実クライアントとフレーム・作業領域を固定し、修正案の Windows 11・他 DPI・複数モニターを含む検証を設計する。現状では center 設定の存在や観測差だけで仕様適合／不適合を確定しない。新たな UI 入力・アプリ起動・製品変更は行っていない。
