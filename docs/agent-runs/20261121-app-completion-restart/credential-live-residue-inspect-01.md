# 合成資格情報の残留確認 01

担当 `/root/credential_residue_inspect`。作業場所 `D:/Projects/GithubIssueWidget`。親が指定した run `dcdc5dfdf4524ffa8d9139ee5a781ae4` だけを対象に、承認済み `credential-live-ready-02/recovery.py --inspect` を１回実行した。

## 結果

- 分類は **`absent`**、実行終了コードは 0。対象に対する読み取りの観測時点で、Windows Credential API は対象なしを返した。秘密や値は表示していない。
- 対象 service は `github-task-widget-synthetic-check-82e61bb1-c162-4a83-b28a-0d2cfafd2ebd`、account は `synthetic`。実 PAT やアプリの既存 service は対象にしていない。
- 削除・資格情報の書込み・元試験の再実行は行っていない。`before_write_a` の実行成否や exit 1 の直接原因は、この分類だけでは確定しない。実アプリの資格情報操作も検証していない。

## 事前条件と証拠

- `recovery.py` SHA256 `04A3911C622E09B1416962C388972443041916D6E06A14C0FB7F01646B86A302`、`supervisor.py` `D0D9EFDDC4812B9D9F22C2A3E714F8C491825F3DCC386F4DADA385C2D047421D` は `credential-live-execution-result-01.md` と一致。
- `python -B` で `recovery.read_identity` を読み取り専用で呼び、one-shot・start・child・outcome・journal・対象 EXE とその SHA の内部照合が成功。戻りの target は上記専用 service と account に完全一致、state `completed`、最終 stage `before_write_a`、journal SHA256 `777351b63a7951b145a692b6c6d36ccb17f3439c910d3211d27756fe2bd3b392`、inspect の start/outcome は未存在だった。終了コード 0。
- 元の監督記録は直接子 PID 8708 を wait して終了コード 1 と記録。追加の `System.Diagnostics.Process.GetProcesses()` の限定確認では PID 8708 はその時点で不在、終了コード 0。PID の再利用や OS 全体の保証ではない。
- 作業ディレクトリ `D:/Projects/GithubIssueWidget/docs/agent-runs/20261121-app-completion-restart/credential-live-ready-02` で `python -B recovery.py --inspect dcdc5dfdf4524ffa8d9139ee5a781ae4` を実行し、`absent` と終了 0 を確認した。inspect の start/outcome 記録は同ディレクトリに新規作成された。

## 失敗と残作業

- 事前のハッシュ取得で EXE のパスを推測して指定し、対象不存在の PowerShell 非終端エラーが１回発生した。複合出力の終了 0 をその EXE 確認成功として扱わない。実在する EXE は `supervisor.py` で確認し、上記 `read_identity` が固定 SHA を照合した。親へ失敗記録の反映を依頼した。同種２回目はない。
- 本結果と inspect の記録以外は変更していない。削除対象なし。記録された分類の独立レビューと、元試験失敗の原因調査は別途必要。
