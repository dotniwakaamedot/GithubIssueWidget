# Windows 1312 の対話セッション確認案

担当 `/root/logon_session_resolution`。作業場所 `D:/Projects/GithubIssueWidget`。本調査は読取りのみで、Credential Manager・PAT・アプリ・GitHub へ操作していない。

## 判明した事実と未確認

- `credential-os-failure-analysis-01.md` は２回目の合成試験で `write`、`no_storage_access`、Windows 1312 を記録し、対象を１回だけ読み取った結果 `absent` としている。１回目のエラー本文は保存されておらず、同じ原因とは断定できない。
- 1312 はログオンセッションまたは資格情報セットに関するエラーである。自動実行側と本人の通常対話セッションの違いは候補だが、同一性や具体的な原因は未確認。本人の対話セッションで成功する保証はない。
- `credential-error-capture-01/supervisor.py` の SHA256 は `D603D89B148A652AC0E54014E3AC63F78F75362D8554B07C09069E1D2FFA34F7`、独立レビュー `credential-error-capture-review-01.md` の対象と一致。`--execute` は恒久的な `live-once.json` を予約し、対象バイナリ SHA と子登録を確認後に開始、終了を待つ。現在の専用領域には前回の marker と結果が残るため、消去・上書き・再使用しない。
- この実行環境で `python --version` は `3.11.15`、`python -c "import sys; print(sys.executable)"` は `C:\Users\semis\AppData\Local\hermes\hermes-agent\venv\Scripts\python.exe`、それぞれ終了 0。`py -3 --version` は `No installed Python found!`、終了 1。この失敗は今回１件であり、本人の通常 PowerShell の PATH・実行権限を証明しない。秘密は表示していない。

## 実行前に用意する具体的な対象

1. 管理者が workspace 内に未存在の新規専用ディレクトリを１つ作成し、旧 run の marker・journal・outcome をコピーせず保護する。承認済み診断版から `supervisor.py`、`recovery.py`、`diagnostic_summary.py`、`RECOVERY.md` と、相対位置 `target/debug/giw-credential-synthetic-check.exe` に EXE をコピーする。古い領域の `live-once.json` は削除しない。既存先なら上書きせず停止する。
2. 各実体を元とコピーでハッシュ照合する。承認版の supervisor は上記 SHA、`recovery.py` は `04A3911C622E09B1416962C388972443041916D6E06A14C0FB7F01646B86A302`、`diagnostic_summary.py` は `25C683CB1057CC3E0C77EDC4FF1984EC13502F89160C80A3D4F64A576F499BB0`、EXE は `9AEC7CDC8FCC5AB3109153C930CE45900CA5290B284D1F03B052C33B72CF845F`。対象と依存ファイルの対応、one-shot 不在、書込み先と user profile、保存権限、終了・復旧手順を別担当がレビューし、親が実行を判断する。コピー後にファイルを変更しない。
3. 本人は Windows に通常ログオンしたユーザーの PowerShell を開き、管理者権限への切替や別アカウントの使用はしない。先に `& 'C:\Users\semis\AppData\Local\hermes\hermes-agent\venv\Scripts\python.exe' --version` を実行し Python 3.9 以上であることと、親が用意した新規ディレクトリ・EXE へアクセスできることを確認。利用不能なら実行せず報告する。上記実体は調査環境での観測であり、本人環境での存在は未確認。

## 本人が１回だけ実行する候補

親が準備・確認した新規ディレクトリの**実際の絶対パス**を `<新規ディレクトリ>` に置き換える。既存の `credential-error-capture-01` では実行しない。

```powershell
Set-Location -LiteralPath '<新規ディレクトリ>'
& 'C:\Users\semis\AppData\Local\hermes\hermes-agent\venv\Scripts\python.exe' -B '.\supervisor.py' --execute
```

この手順は一般利用者が明示的に操作する対話端末であり、バックグラウンド起動を提案するものではない。本人は画面や入力を他者と共有せず、PAT を入力しない。試験は UUID 付きの合成 service と固定合成値のみを使う。NoEntry→write と確認→delete は非原子的であり、同名競合をゼロとは保証しない。親が許容リスクを判断し、本人が実行する前に対象版レビューを完了する。

- `state=completed exit=0` のみを正常候補とし、run ID、出力の状態と終了時刻を記録。journal の `stage=completed` はテスト内の削除後 NoEntry 確認を示す。親が保存ログと直接子の終了を独立に照合する。
- `exit=1`、timeout、表示なし、コマンド例外、記録欠落や終了不明では**再送しない**。ウィンドウを閉じずに、表示された run ID と一般的な状態のみを親に伝え、marker・journal・outcome を保持する。識別情報に疑義があれば対象を推測して照会・削除しない。診断は親の判断と対象版照合後に `diagnostic_summary.py`、必要なら別途承認された target 限定 read-only inspect を１回行う。値・例外本文は表示しない。
- 合成 target が残った場合の cleanup は `RECOVERY.md` の許可対象 stage、子の終了、journal・inspect・合成値の完全一致を満たすことを別途確認してから判断。該当しない場合は自動削除せず保護する。正常・失敗いずれも one-shot を消去しない。

## 証明できる範囲

成功しても別名・合成値での OS/keyring 経路と、その対話セッションに限る。実アプリの PAT 登録・更新・削除、実 WebView IPC、他の Windows 環境を証明しない。既存 PAT を取り出す・共有する必要はない。`py -3` の失敗は共有失敗記録への反映を親へ引き継ぐ。
