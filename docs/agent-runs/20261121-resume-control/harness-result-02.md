# 合成試験の変更結果 02

- 作者 `/root`、対象ディレクトリ `D:\Projects\GithubIssueWidget\docs\agent-runs\20261121-output-truncation`。前回停止記録 `stop-01.md` と `harness-review-01.md` を参照。実 GitHub 実行を無効のまま維持し、対象の変更を停止した。
- SHA256: `job_process.py` `A470126C3413A0F45DE52E08BC7AEFABAB9A640B79FEE8614F864159F1F59524`、`test_job_process.py` `5E5D66EE5CC6E68D508075DF7F9878C154D9E795B4C2AE62CF9745175C3892EA`、`run-live.py` `76DC6318E0E3C7B5D199EDD5DF1449793A8A8C1003090103CB9DF13B1E048B70`、`test_run_live.py` `357B300A8F4FD887B1DBD3D4D6698FFD2708AA081C7D07E28A6C798E85AB5B5B`。
- 子孫 PID ファイルを監視し、実行中のハンドルを先に開いて固定し、終了確認で最大3秒待つ合成試験に変更。前回の2対象はそれぞれ終了コード0。`python -m unittest -fv test_job_process test_run_live` は10件、終了コード0、ResourceWarning 0件。`synthetic-test-restart-03.log` SHA256 `5982A96BEDBF1B9E0D597F6A6A647FC1DF0D10EBE05762605E2752E85CF13FCD`。ログ自体は unittest の出力であり、終了コードは実行時に別途取得した。
- 前回の原因は未特定。PID ファイルとハンドルの対応を外部から完全に証明していない。Job API 失敗の模擬と実 OS 障害は異なる。生成・同期 API の全体待機上限、Job 外の子孫、実 GitHub の安全な実行は未検証。現時点でハーネスの承認を主張しない。
