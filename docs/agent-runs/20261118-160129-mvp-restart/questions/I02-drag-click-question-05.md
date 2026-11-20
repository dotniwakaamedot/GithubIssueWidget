# I02 診断版ビルド準備の状況 05

- 担当 `/root/drag_design`、宛先 `/root`。`questions/I02-drag-click-answer-03.md` と `answer-04.md` に基づき進行。
- 回答03の9参照すべての存在と SHA は一致。Computer Use の inventory では複数ウィンドウを観測したが、ユーザーの不在は断定しない。管理は具体的対象と干渉が観測されなければ起動準備を進め、入力前に対象窓・空記録・安全領域を確認すると判断した。
- 診断 scratch 内に専用 Cargo/Rustup home・npm cache・TEMP・profile等を準備。`rustup toolchain install 1.90.0 --profile minimal --no-self-update` は exit 0。既定の共有 Rustup への変更ではない。公開 registry を利用して `npm ci` は exit 0（esbuild install script の未承認警告あり）。`npm run build` は TypeScript 検査と Vite の build が exit 0。既存の worktree/B03 scratch に書込みなし。専用 cargo target で `cargo build --locked` はセッション 93236 が実行中、終了コード未取得。
- npm の初回起動は dedicated userconfig と globalconfig に同じファイルを指定し double-loading config のエラーで exit 1。個別の空 config ファイルに分離した2回目は exit 0。同種エラーは今回1回。2回同種問題で該当操作中断と別担当調査、3回で全中止。秘密をログに出していない。
- 診断アプリの表示・入力は未実施。コンパイル成功後、target exe と対象版 SHA を固定し、対象窓・空表示・安全領域・座標・競合を確認する。判定不能なら入力せず停止。元ドラッグは禁止。
