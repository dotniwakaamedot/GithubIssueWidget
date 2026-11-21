# Job 作成失敗の再開後調査

2026-11-21。前回の `job_list_probe.py` と `job_capability_check.py` は例外型を保存していない。同一原因や子の非生成は確定できない。

- 同じ `win32job.CreateJobObject(None, None)` を子を作らず固定分類した `job-create-error-check.py` は `argument_type_error`。この結果は Job 作成の権限拒否を示さない。
- ローカルで pywin32 のラッパーを確認し、[pywin32 の定義](https://github.com/mhammond/pywin32/blob/main/win32/src/win32job.i) は第2引数を `WCHAR *` と宣言している。今回の型エラーの正確な引数は未確認。
- `job-native-check.py` は子を作らず `CreateJobObjectW(NULL, NULL)` を呼び、`native_job=created_and_closed`、終了コード0。匿名 Job のネイティブ生成・ハンドルクローズが今回の環境で成功したが、所属、子孫、監督者停止、実アプリ起動を検証するものではない。
- `synthetic-residual-check.py` の観測では `python.exe` / `pythonw.exe` かつ引数 `-c pass` と完全一致する可視候補は0、名前を取得できないプロセスが1。`process-access-check.py` の限定照会でもその実行ファイルは取得できず、前回の子の不存在は未証明。プロセスは終了していない。
- 独立分析 `f081-analysis-01.md` に従い `docs/recurring-lessons.md` の F-081 を更新。安全性・予防効果は未確認。

実アプリや追加の合成子の起動前に、前回の対象の不在または終了を確認するという引き継ぎ条件が残る。未確認プロセスの識別子や名称だけで終了操作を行わない。
