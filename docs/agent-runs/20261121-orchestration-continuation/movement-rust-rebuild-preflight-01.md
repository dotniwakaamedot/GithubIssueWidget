# 移動競合の新 target 再ビルド preflight

担当 `/root/existing_work_review`。`movement-rust-rebuild-request-01.md` に基づき、0件収集の証拠と別担当調査 `movement-zero-collection-investigation-01.md` を保護し、新規 `movement-rust-test-execution-02/` にランナー案を作成した。コンパイル・テスト・既存 target の変更は行っていない。実行前に固定版の独立レビューと親の開始通知が必要。

## 固定対象

- 依頼の現行製品/fixture main `AEFF1E65AD93EFA04773CEBBB9736A6BDC5B9938DB9FAF937DD88B3D9CBFE86F`、fixture movement `88BF21BB45C077DFDA592D906C753D168C5641E35EB12324D4ACF1652374695B`、製品 movement と保存元 `ED49F4BA52C6887BCE427E4A6664D335092A1C3311E1B1ED61B4CE322791ABA5` は確認時に一致。
- 新 `run-movement.ps1` SHA `0583FA81C1F27753E4BDC6A8BD0DCF59714FE387FD9CB0266CA0186B3429E1BA`、`run-movement.cmd` `321976316709C7B3AC2742D203F4E508179E739A44D9C73E9CE261719179B19E`。旧承認版とのメモリ内比較で、PowerShell は `CARGO_TARGET_DIR` を既存 fixture target から新ランナー配下 `target` へ変更した1箇所以外同一。cmd は同一 SHA。環境、direct cmd 終了・900秒超観測、専用ログ、loopback・対象フィルタは前回同様。
- 新 target・stdout/stderr は不存在。D ドライブ利用可能空き容量は `DriveInfo.AvailableFreeSpace` で 99,446,681,600 bytes。従来の target、ログ・証跡、専用 registry は削除・上書きせず、registry は既存専用 CARGO_HOME を共有する（Cargo 自身の読書き・OS 全体の通信非使用は未検証）。同時利用の有無を開始直前に再照合する。

## 実行方法と判定

承認後、上記・元入力・ログ不存在を直前再確認し、`powershell.exe -NoProfile -ExecutionPolicy Bypass -File <新 run-movement.ps1 絶対パス>` を一度実行。これは以前の実行ポリシー拒否に対するプロセス限定指定であり、永続設定を変更しない。Cargo は新空 target で `--locked --offline --bin github-issue-widget movement::tests::post_write_competing_ -- --test-threads=1`。3件の完全名と各成功・件数、直接終了、stdout/stderr の終端を確認。前回の終了0・0件は未実行であり、過去に成功した別テストを反復しない。ビルド中の process tree 全体の終端や OS 失敗時の無条件停止は未保証。失敗・不明ではログと対象を保持し、無関係なプロセスに触れない。

## 本作業の失敗

`git diff --no-index` の出力が内側上限300 tokensで切り詰められた。差分あり終了1は期待結果だが、省略された出力を完全な証拠にしない。別のメモリ内文字列比較で「旧ファイルの target 指定1箇所を置換した結果が新ファイルと一致」と確認した。出力量を警告・パス込みで見積もらなかった点を F-001 候補として管理に記録・独立分析を依頼する。今回の同種失敗はこの作業で1回。`Get-PSDrive` の Free は空値で利用可能容量は得られなかったが、`DriveInfo` の結果と区別する。調査結果・修正の効果は未確認。
