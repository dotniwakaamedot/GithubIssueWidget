# GUI 診断版 build の固定条件

担当 `/root/gui_failure_investigation`。`gui-diagnostic-build-request-02.md`、親の早期実測指示と、旧 release 観測との比較を対象とする。旧 release の build セッションは終了し、旧 `main.rs`・`main.tsx`・EXE を保護領域 `gui-release-build-01/diagnostic-backup-01/` に別名で保存し各 SHA 一致（61595B...、6FCF2A...、23C383...）。旧 `build-output.log`・status・manifest は上書きしない。target 再利用の結果、他の生成物と従来の EXE パスは上書きされ得るため旧 exe は事前退避した。旧 exe の共有コピーも `gui-release-observation-01/` に残す。

診断固定版2ファイルを専用複製のみに配置し、対象 SHA を照合: `main.rs` `50692BB0FCEBB9B3B32D1FC88A43DCE1B0238EE681E534A6BC5D62E3EC87F4FD`、`main.tsx` `603473420E487660FB9B5467806268F8507BC00DE8AE7BF9DA93953A438B834C`。版は `gui-diagnostic-isolated-review-01.md` が静的に承認。現製品の main.rs は test 限定モジュール追加後で異なるが、旧 release 観測と同じ旧製品版からの差であり `gui-diagnostic-input-answer-proposal-01.md` に詳細を記録。診断版の結果を現製品版の自動的な証拠とは扱わない。

候補 `python docs/agent-runs/20261121-orchestration-continuation/gui-release-build-01/run_diagnostic_build.py` は前回承認された `run_build.py` のログと status 名だけを `diagnostic-build-output.log`・`diagnostic-build-status.txt` に変更した版。SHA256 `0A71A2FD9A3C083A072CE480987F0FB885E513FBAF54FFC5E7DD6BDD4B0315D3`、`build.cmd` は前回版 `BAE3C381D954B2AB243708605D05B3113636B83FACAFC18D98BB58F46321F2EA`。専用 CARGO_HOME/target、offline、設定・temp と認証のない明示環境、保護されたログ、直接 cmd の終了確認は前回と同じ。Cargo の source/cache/target は前回ビルドにより更新済み、内容は旧 manifest と同じ状態ではなく再利用対象である。ログ以外の出力を共有しない。未追跡子孫や dependency の通信・credential への絶対的な非到達は未確認。

実行前に診断版・旧 backup・新ログ/ status 不存在を再照合。実行器変更と build 対象を別担当が独立確認し、管理から開始通知後に1回実行する。新 EXE はハッシュを固定し、新規観測領域へコピーする。GUI 起動は別途 stage 出力先・終了観測方式のレビュー後に限定する。既存プロセス・旧 fixture・credential は操作しない。

準備中の `git diff --no-index` は旧領域にアクセスできず終了1、PowerShell/Python の対象読取りは成功。原因未確認、１回の失敗として `gui-diagnostic-input-question-01.md` に記録済み。既存の製品・旧 EXE/ログ・status は削除していない。
