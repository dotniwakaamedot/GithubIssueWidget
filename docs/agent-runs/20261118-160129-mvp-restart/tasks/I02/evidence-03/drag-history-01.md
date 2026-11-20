# I02 過去のドラッグ試行に関する観測

- 対象: 共有 run `tasks/I02/result-01.md` に記した2回。新たなドラッグは行っていない。担当 `/root/foundation`、宛先 `/root/drag_diagnosis`。
- 利用ツール: Computer Use の `mcp__node_repl__js` 上の `@oai/sky`。対象は `sky.list_apps()` の結果から一意に選択した `github-issue-widget`、debug exe、window ID `12129262`。直前 `sky.get_window_state` は screenshot ID `screenshot-0`、対象領域 902×522、originX 1477 / originY 796 を返した。
- 第1回: 初期の `data-tauri-drag-region` 実装。観測直後に次の呼出しを実行した。例外なし。後続 state の screenshot メタデータでも 902×522、originX 1477 / originY 796 で、座標変化を観測できなかった。

```js
await sky.drag({window:o.window,screenshotId:o.screenshots[0].id,from_x:300,from_y:27,to_x:180,to_y:80});
state=await sky.get_window_state({window:o.window,include_screenshot:true,include_text:false});
```

- 第2回: ソースを `header` の `onMouseDown` から `appWindow.startDragging()` を呼ぶ方法に変更し、Vite HMR 後に新たな `get_window_state` の screenshot を取得。次の呼出しは例外なし、直後の state でも同寸・同 origin だった。

```js
await sky.drag({window:o.window,screenshotId:o.screenshots[0].id,from_x:300,from_y:27,to_x:160,to_y:80});
state=await sky.get_window_state({window:o.window,include_screenshot:true,include_text:true});
```

- 対象版: 第1回のソース内容の SHA は記録していない。第2回以降の `widget/src/main.tsx` は SHA256 `07318B40089C01EEB897EFDB5A933DD15982C51E51CDAEABC097AC219C867AFE`。いずれも HEAD `a4ac97286785a2a5ba0d070a909b58003fc49ff1` に対する未追跡・未コミット作業状態。
- 画面観察のツール結果は会話履歴に残るが、当時の画像・独立ログは保存していない。各 mouse-down イベントが JS に到達したか、OS にドラッグが渡ったか、非同期遅延や snap 等の影響は測定しておらず不明。ウィンドウ位置が実際に変わらない原因を確定しない。
