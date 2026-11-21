# 同期 UI の対象版合成試験

## 結果

`main.test.tsx` の対象６ケースを隔離コピーで実行し、**６件成功・失敗０件、対象外35件は未実行**。旧接続の遅延エラー、同期開始から60秒、時計変更２例、速い失敗から次回復旧、アンマウント後の応答抑制を対象とする。模擬 IPC と仮想時計の試験であり、Windows 実 WebView や実 GitHub の定期同期・実通信障害は証明しない。独立レビューは未実施。

## 対象と根拠

- 担当 `/root/sync_regression_test`、作業場所 `D:/Projects/GithubIssueWidget`、branch `main`、HEAD `4961b25bec53a93439acdf5230de77f959908af0`。HEAD だけで未コミットを識別しない。
- 依頼 `sync-regression-test-request-01.md` SHA256 `3F90516A80CD341BFF500A4DA28FD7DFDF442A92E8119C303E2286998E2D5C7B`、承認済み `plan-approved-03.md` `355CD3D07C9CEAD72576CACC02D31B27484F7C165AEAB967E2FBCC548DD175C1`。
- 元の `widget/src/main.tsx` `B22504C9DE48D275726C912820DA93E02F10C1838CCC68555C156D35030165D7`、`widget/src/main.test.tsx` `10AA28AD8D343F969C8BDB8C2D2232BF6007CB926619CB49850020DD53A40FBD`、`widget/package.json` `102235B5686088C87B3221DEC7AE3CBEE20F0C9B1EAE070FD730C8B631DFCEDF`、`widget/package-lock.json` `A5D65370135F83B08F02D5F1EE329706F08551743444264BCB10DFC7600B0C73` は開始時・終了時一致。隔離入力も個別 SHA 照合した。
- `docs/handoff-20261121-current.md` の既存 UI 39件成功は直前版、別途の対象版限定成功は移動・作成各１件。対象６ケースの対象版成功証拠は同文書にない。`git diff 4961b25^ 4961b25 -- widget/src/main.test.tsx` で対象の５テスト宣言が最新コミットで追加されたことを確認。時計変更の宣言は `it.each` により２ケース。関連検証の実行を選択したが、他の記録全体に存在しない証明ではない。
- 試験コードは `@tauri-apps/api/core` の `invoke` を `vi.fn()` に置換し、対象ケースは仮の owner/repo と結果、`vi.useFakeTimers()` を利用する。旧接続ケースも仮の Promise と模擬接続を用いる。実 PAT、実 GitHub、実アプリへの呼出しは対象コード内で行わない。
- Issue #4 の公開 REST 読取りを試みたが HTTP 403（レート制限）。今回直接本文を確認できず、承認済み計画と先行監査の要求記録に依拠した。失敗と出力制限の独立調査依頼は `questions/sync-regression-question-01.md`。

## 実行と証拠

- 専用領域 `sync-regression-sandbox-01/widget/` に `package.json`、lock、Vite・TypeScript 設定、`index.html`、`src`、`node_modules` をコピー。リンクを拒否して元とコピーの全 4,983 ファイルを SHA256 比較し不一致なし。`inputs.tsv` SHA256 `31EB08A122A9BA7E3048D6FD366BB6F42DE3DA900396EF670A2C432E1E2D5148`。入力を示し、将来の元ファイル不変性までは保証しない。
- 作業場所は専用コピーの `widget`。`node node_modules/vitest/vitest.mjs run src/main.test.tsx --pool=threads --maxWorkers=1 --no-file-parallelism --testTimeout=10000 --hookTimeout=10000 -t <対象５宣言の正規表現>` を `runner.py` が引数配列で実行。`runner.py` SHA256 `A25B4D96AB2C18B9DDC320C50AD4993ED6CCFCE561FD2101061B7D3C59516DFE` に正規表現の全体を記録。Node `v26.13.1`、Vitest `5.1.1`。テスト・フックは各10秒、直接子プロセスは120秒を上限とし、通常終了したためタイムアウト時の子孫停止は未検証。
- 監督セッション終了 0。`test-exit.txt` は `completed`、`exit_code=0`、SHA256 `6BE7E3591253C9D77DBB29E86536ABB77CB32B21922A88ABD2804FA4EC3F01E6`。`test.log` は `Test Files 1 passed`、`Tests 6 passed | 35 skipped`、SHA256 `74C1210F14BA4E73C62E22C63A4089F8AAA78B7F88BA409739DB257F09A6C960`。ログの集計はテスト名を個別表示しないため、対象名は runner の選択条件と対象ソース・件数に依拠する。
- 未追跡の専用領域内に入力、runner、ログ、実行後生成物を保持。全 4,991 ファイルの SHA256 は `outputs.tsv` に記録し、同ファイル SHA256 `AFFDBBC5C12B6C657BECAECB211818057B0D32D4BF1630649EEA51942288D3AA`。自己参照を避け同一覧は自身を含まない。結果ファイルのハッシュは確定後に管理側で取得する。削除対象なし。

## 失敗と未確認

1. ハッシュ一覧を `Format-Table` で表示した際、表示幅により SHA の末尾が省略された。その表示では全値を確認できず、別の完全な値の取得で照合した。
2. `Set-Content -NoClobber` は Windows PowerShell で未対応、終了 1、runner は作成されなかった。`FileMode.CreateNew` で新規作成してハッシュを確認した。
3. Issue #4 読取りは HTTP 403、終了 1。例外を絞らなかった結果、出力が上限で切り詰められた。元の完全出力は保存していない。表示省略との２事象を別担当へ調査依頼し、関連取得を中断。以上の記録・教訓反映を管理へ引き継いだ。

対象外のテスト35件は今回再実行していない。実同期・実 WebView、実 PAT、実通信障害、時計変更時の Windows 実動作は未確認。製品コード、既存ビルド、資格情報、GitHub に変更なし。各出力は専用領域に残し、他者素材を削除していない。
