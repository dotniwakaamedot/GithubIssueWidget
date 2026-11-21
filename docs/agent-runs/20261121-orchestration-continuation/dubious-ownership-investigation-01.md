# 旧 worktree の dubious ownership 反復失敗の独立分析

2026-11-21。担当 `/root/issue_survey`、当事者 `/root/existing_work_review` と `/root/project_survey` とは別担当。依頼 `dubious-ownership-investigation-request-01.md` SHA256 `066B9E3E2B449CD1BF7081A905CACBB7D70B4B7F96DA626E21B7E28752D9F208`。既存資料と失敗記録を読取り、当事者に詳細を照会した。旧 worktree の Git、設定、所有権、プロセスは変更せず、失敗を再現していない。

## 観測と影響

1. `sync-ownership-result-01.md`（SHA256 `A8C2F73D84CFEF4B8374EDA15512EACF33B227F1350A07C452E7572A36DBA9C7`）によると、調査担当が旧 #4 worktree `C:/Users/semis/orca/workspaces/GithubIssueWidget/issue-4-board-resume` の通常 `git status` を試み、dubious ownership で失敗。その後、当該パスをその呼出しだけ `-c safe.directory=...` に指定した読取りは終了0、branch 以外の変更表示なし。Git のユーザー ignore 読取り警告あり。通常の失敗の正確なコマンド・終了コードは結果資料に記載されていない。
2. `sync-integration-result-01.md`（SHA256 `69DB6A14164D63CC0FCAC7929ED154BBE9F0FE7D99583965B9AD1EABED6CAF2E`）によると、別担当が同じ旧 worktree の通常 `git status` を実行し、dubious ownership、終了1。その後、`git -c safe.directory=<該当場所> -C <該当場所> status --short --branch` の単回限定設定で終了0、branch はクリーンと報告。グローバル設定と旧 worktree を変更していないと記録される。両事象の元の完全なツール出力は本担当未確認。
3. どちらも失敗自体を状態がクリーンな証拠とはしない。後続の成功はその実行時点・対象範囲の読み取りを示すだけであり、旧担当の停止・所有権移管・他セッションの不存在や未完了 PR の不存在を証明しない。

## 原因と既存記録

Git が指定 worktree を dubious ownership と判断したことは両引継ぎ記録から確認できる。実際のファイル所有者、プロセスの実行 ID、SID、ACL、Git が比較した正規パスと、その差を生んだ具体的原因は未確認。異なる実行ユーザー・サンドボックス・所有者の可能性は仮説であり、特定しない。ユーザー指示の2回条件は別担当による同種の失敗2件として扱い、旧履歴の累計とは分ける。

`docs/failure-log-001.md` の F-003 発生1は、2026-11-20 の他 worktree に対する同種警告と、所有・利用状況の未確認、Git 安全設定を勝手に変更しない方針を記録している。今回2件との同一性・累計・重複は共有ログ担当が照合する。既存指示 `docs/adapters/git-github.md` は他担当の所有・未統合作業を保護し、移管不明を保留する。単回の status 成功だけで引き継ぎ条件を省略しない。

## 具体的解決と再発防止

- 既に当事者は単回の `safe.directory` 指定で限定 status を読み取っている。再試行や恒久設定が解決のために必要とは認められない。対象が信頼できることと正規パスを確認できる場合だけ、必要な一回の読取りコマンドで当該 worktree の正確な絶対パスを `git -c safe.directory=<絶対パス> -C <絶対パス> status --short --branch` として明示する。`*` や親ディレクトリの一括信頼、global/system/local 設定への登録、所有権変更はしない。単回指定も対象の Git 設定を信頼する判断であり、信頼性が不明なら無理に実行せず未確認として報告する。
- その前に worktree 一覧、既存記録、実在・正規パスを照合し、所有者・利用状況を利用可能な読取り権限で確認する。アクセス拒否や不明は空・停止・移管済みと扱わず、結果と終了コードを記録する。既に関連 worktree に警告が出たことを次担当に渡し、既知の同一失敗を通常 status で繰り返さない。
- 単回の安全設定を使っても ignore 読取り警告・未追跡ファイルの扱い・並行変更があれば、必要な範囲を限定して再確認し、状態の確証が不足することを記載する。実際に信頼・所有と移管を確認するまで統合・削除や既存 worktree の変更をしない。
- 当事者結果の読戻しは成功しているが、以後の担当にこの策が継続適用されるかは未検証。共有失敗記録・教訓の更新は担当外として親に依頼する。正確なコマンド・環境・Git 所有者判断の追加根拠は当事者から未受領なら不明のまま保持する。

## 調査証拠

`docs/failure-log-001.md` SHA256 `3C6E6A21BF10270A5917C5FE12386935E276BE81DD22A64FFE37B8EC1DA3FFDD`、`docs/recurring-lessons.md` `78A83DCAE885F26A4107EAB29476E68014B31D1CCACF9BF15448EEB36E006F54`（読取時点）。`Test-Path`、限定 `Get-Content`・`rg`、`Get-FileHash` は終了0。本調査中に操作失敗は認識していない。
