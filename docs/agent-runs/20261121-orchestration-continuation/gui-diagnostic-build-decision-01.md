# GUI 診断版ビルドの実施判断

`gui-diagnostic-build-review-01.md` は `run_diagnostic_build.py` SHA `0A71A2FD9A3C083A072CE480987F0FB885E513FBAF54FFC5E7DD6BDD4B0315D3` の固定版を承認。`/root/gui_failure_investigation` は旧セッション終了・旧 EXE/ログの保護、診断入力2ファイル・実行器・batch の SHA、新ログ/status 不存在と同時使用なしを直前に確認できた場合、診断版ビルドを1回行う。不一致・不明なら開始しない。

直接終了・ログ・新 EXE の SHA・旧成果保持と未確認の子孫/共有資源を区別して固有結果に記録する。GUI 起動・実 credential/GitHub は別条件。既に成功した無関係なテストを繰り返さない。
