# ハーネス変更と合成試験結果 01

- 作業者 `/root`。作業場所 `D:\Projects\GithubIssueWidget\docs\agent-runs\20261121-output-truncation`。旧ハーネス確認担当は interrupted、製品ファイルは変更していない。
- 対象 SHA256: `job_process.py` `FDC78F0BC62176159E80E4B9EE90EA47C33AE80DF2567AB2143DF3183ED92721`、`test_job_process.py` `9B3EC52FFFC30D99648998E1D777DEB1659033851731CD24AA42A3FE634DD550`、`run-live.py` `76DC6318E0E3C7B5D199EDD5DF1449793A8A8C1003090103CB9DF13B1E048B70`、`test_run_live.py` `357B300A8F4FD887B1DBD3D4D6698FFD2708AA081C7D07E28A6C798E85AB5B5B`。
- Job のアクティブ数確認、終了要求後の待機、親の回収、パイプの後片付けを追加。保持・非保持子孫の PID 確認、API 異常、合成子プロセスの出力からの結果判定を試験した。
- `python -m unittest -v test_job_process test_run_live` は9件、終了コード0、ResourceWarning 0件。ログ `synthetic-test-resume-02.log` SHA256 `1EDD56D0C21D8C0F1A5637466E9FF346493E8D6DC826D0E05A436C0C34AC3F13`。初回は警告が発生し F-069 に記録。
- 不明・制約: `subprocess.Popen` の生成呼出し自体に期限がなく、OS・API 停滞時の全体時間上限は保証できない。Job 外への breakaway や既存 Job の制限等は未検証。模擬 API 失敗は OS 自体の失敗を再現しない。正常時の全子孫確認と例外時の終了要求・待機の妥当性を独立レビューする必要がある。実 GitHub 実行は無効のまま。対象版を変更停止した。
