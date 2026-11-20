# ハーネスの独立確認 01

判定: **要修正。実 GitHub 検証は実行しない。** 作者 `/root` と確認者 `/root/harness_check` は別担当。対象は未追跡ファイルで、実体の SHA256 は `harness-result-01.md` の4件とログの値にすべて一致した。本確認はハーネスの確認であり、製品の完成や無条件停止の解除を承認しない。

## Blocking

1. **異常経路で全子孫の終了確認を証明できない。** `job_process.py` の `TerminateJobObject` または `QueryInformationJobObject` が失敗すると、`finally` で Job を閉じた後、親プロセスだけを待つ。`CloseHandle(job)` の戻り値は検査せず、パイプを閉じた子孫の終了は照会できない。エラーとして結果不明を返す点は確認したが、それは子孫終了の証明ではない。`test_termination_failure_is_not_success` と `test_accounting_failure_is_not_success` は `pass` の単一プロセスのみで、生きている子孫を伴う失敗時に子孫が終了したか独立観測していない。実行中の書込みが親の終了後も続くおそれを排除できない。終了失敗・照会失敗・Job close 失敗時の状態を結果不明として扱いながら、別の生存確認方法または安全な停止設計を示し、パイプ保持・非保持子孫を伴う異常系を期限付きで独立観測する。検証不能なら実行を止める。
2. **全体の待機上限が未確立。** 結果に明記されたとおり `subprocess.Popen` 自体に期限がなく、Job API や `CloseHandle` も同期呼出しであり、生成から終了回収まで全体の期限は保証できない。承認済み `plan-02.md` は上限を検討し、保証できない場合は実行を止めると定める。外側の監視も含めた実行方式と制約を設計・検証するか、実環境実行を無効のまま保つ。`run-live.py` の無条件 return は現時点で維持されている。

## 条件と証拠

- 正常とタイムアウト時: `wait_empty` は期限付きで Job の ActiveProcesses を照会する。保持・非保持子孫の試験は PID を記録し、`OpenProcess` / `WaitForSingleObject` または不在のエラーで終了を検査する。PID 再利用や OS の Job 外への breakaway 等は未検証。
- API 模擬: 代入、再開、終了、照会の失敗時に例外を要求する試験はあるが、前述のとおり全異常経路で子孫終了を独立に示す証拠はない。実 OS API 障害ではない。
- 合成出力: `test_run_live.py` は子プロセス経由の正常、0件、途中・異常終了を `checked_results` に通す。管理側ログは9件 OK を示す。終了コード0は `harness-result-01.md` の申告であり、ログ自身には元プロセス終了コードが含まれない。今回の独立テストは、`TemporaryDirectory` が管理外の一時領域に書き込むため実施していない。
- `run-live.py` の無条件停止を静的に確認。停止解除は対象版の変更であり、別途対象差分・ハッシュと実行計画の再確認が必要。実 GitHub、Credential Manager、実 UI は確認していない。

## 対象版・制約

`job_process.py` `FDC78F0BC62176159E80E4B9EE90EA47C33AE80DF2567AB2143DF3183ED92721`、`test_job_process.py` `9B3EC52FFFC30D99648998E1D777DEB1659033851731CD24AA42A3FE634DD550`、`run-live.py` `76DC6318E0E3C7B5D199EDD5DF1449793A8A8C1003090103CB9DF13B1E048B70`、`test_run_live.py` `357B300A8F4FD887B1DBD3D4D6698FFD2708AA081C7D07E28A6C798E85AB5B5B`。ログ `synthetic-test-resume-02.log` `1EDD56D0C21D8C0F1A5637466E9FF346493E8D6DC826D0E05A436C0C34AC3F13`。依頼 `6ADDEE95F0A95259D692BAB8A9FCF84406E7CA25F3BAA33646C87BFFC016986F`、結果 `3F941776F310D74A8196B06156E6B7FA25B84A37C6E132BFA8E6AB48AD59DCDF`、計画 `F592EBF669048B2C4F9DDEB4B94559BDB6E49A019ABB9D482783D1CCC6113CED`。削除対象の指定は結果にない。対象ファイルの編集や実 GitHub 操作は行っていない。
