# I02 無害プロセス試験結果 01

- 担当 `/root/build_verification`。確定依頼 `tasks/I02/process-test-execution-request-01.md` SHA256 `F2AF7E3D6027C692288C0E40660FC98EBF100E1AC5A3805A4B04199E11359966`、対象設計 `process-test-design-02.md` SHA256 `31E6F1F35FD221DB21AFA35DFFD200F0D4E4E1DD979ABFA1A9DAE90D5B403894`、独立レビュー `process-test-design-review-03.md` SHA256 `95F3DADB834846F1EF2DE9559611DCE8FE9D70994EAE5017B0C5C54211041D51`。担当・対象・参照8件の実体を開始前に照合し一致。
- **4ケースは当該無害プログラムの期待どおりに観測された。** これは対象版の子終了コード、両ストリームの EOF と全文、timeout 後の待機・継続を確認した結果であり、元 build の待機原因・成功、Issue #2 の完了を証明しない。別ストリーム間の発生順序や実負荷での一般的な再現性は未確認。
- 作業場所 `D:\Projects\GithubIssueWidget-process-test-20261119-01`。対象 `child.cjs` SHA256 `55FFB888FAA4F5E2B93D0F806DF6892834258B15756535895BC68937C621276C`、driver `driver-04.ps1` `F24B68C56FA99A67685BFB864F9B39F768EBDBA0CC1D7B2E92A12CA4DF0C7FDD`、Node `C:\Program Files\nodejs\node.exe` `F5109DC868EC38703AD8F305E0ACEECE717CB60D4889873F434017AF4E96CCCB` は開始前後で一致。
- 元製品 worktree の branch `task/issue-2-foundation-20261118-160129`、HEAD `a4ac97286785a2a5ba0d070a909b58003fc49ff1`、README 変更と widget 未追跡16件は前の状態と同じ。製品・旧 scratch・既存証拠・GitHub は変更せず、GUI・外部通信・ポート・拒否された window metadata 公開操作も行っていない。OS 通常記録の副作用は残り得る。削除対象なし。

## 実行・観測

全ケースは scratch を cwd とし、`C:\Windows\System32\WindowsPowerShell\v1.0\powershell.exe -NoProfile -File <scratch>\driver-04.ps1 <case>` を順に実行した。各 driver の外側プロセス exit は0。非0ケースの実子 exit 7 は期待値であり、driver がケース成功として0を返した。各ログは UTF-8 で独立に読戻し、期待文字列全体を照合した。stdout/stderr 間の相対的な発生順序は未検証。

| case | 子 PID | 開始 UTC | 子の終了 UTC | 子 exit | stdout / stderr bytes | 各 EOF / 全文 |
| --- | ---: | --- | --- | ---: | --- | --- |
| normal | 30424 | 20:09:46.9703082 | 20:09:47.0116241 | 0 | 8 / 8 | true / 一致 |
| nonzero | 40820 | 20:10:03.7019091 | 20:10:03.7383113 | 7 | 8 / 8 | true / 一致 |
| flood | 57816 | 20:10:18.5527104 | 20:10:18.5917631 | 0 | 524,296 / 524,296 | true / 一致 |
| timeout | 52720 | 20:10:42.9599350 | 20:12:43.0138898 | 0 | 16 / 8 | true / 一致 |

日時はいずれも 2026-11-18 UTC。`timeout` は 20:10:43.4844428Z の 500 ms 経過時、exitCode null・EOF 未完了を記録し、driver セッション 61171 で同じ Process handle を保持。20:13:07Z の OS 読取で子 PID 不在を観測し、`questions/I02-process-test-execution-question-02.md` で親へ判断を求めた。管理回答 `I02-process-test-execution-answer-02.md` に基づき `continue` を1回送信し、同一 handle の `HasExited` 後に実子 exit 0・ExitTime・両 EOF を取得。20:14:29.3106241Z に driver が終了を観測し、セッション exit 0。強制停止・同一 case の再起動なし。

## 成果の SHA256

パスは全て scratch `D:\Projects\GithubIssueWidget-process-test-20261119-01` からの相対。各結果 JSON には子 PID、開始・終了時刻、終了コード、EOF、byte 数と判定がある。

```text
3F2FB78F91D49452DFCA7F0108005164F4D88CCA58695F1E19654ABDC5F2770E  normal.stdout.log
68567A24B0346B74EFC27DD091E259C56C12DF40458AAEDDD5F081AF8B003B4D  normal.stderr.log
E49A733829D28083DD2EBA7286C709AFC17366AF020C821D888227454F121F25  normal.result.json
3F2FB78F91D49452DFCA7F0108005164F4D88CCA58695F1E19654ABDC5F2770E  nonzero.stdout.log
68567A24B0346B74EFC27DD091E259C56C12DF40458AAEDDD5F081AF8B003B4D  nonzero.stderr.log
B010769416DDB4BE78682AC156CD3156470EFA56D0052680787C761AF67CCC03  nonzero.result.json
0FEA41D5EFF0721AE19F69CECA35C902BD4E4C0AB35207A0E3797763C19ACFC0  flood.stdout.log
2118D094CBDF2E05B68D8C98500A75CB5C996409A0E1CADE18DDBBDFCF675F8F  flood.stderr.log
6294C11624CC6787758AA9822E599DAE71D877003A1E5FC3304BFF09FD737233  flood.result.json
E49CB2D796CFD1358DE3FF22586B81670E401FD5EBA09FEE8398848391AE1FD7  timeout.stdout.log
68567A24B0346B74EFC27DD091E259C56C12DF40458AAEDDD5F081AF8B003B4D  timeout.stderr.log
9A215385E2A06C8412678C458AB4FEEC124DCBC31DC63BCAE053E4A2B911B669  timeout.result.json
3702CFA137B0027D23FAD2742DDF9D4AEBD03DDA4D2EB83E68E1F87987F9539D  timeout.timeout.json
```

主要コード・Node の hash は冒頭記載。scratch の旧 `driver.ps1`、`driver-02.ps1`、`driver-03.ps1` は今回変更せず保持。結果ファイルの対象版は確定後に別担当が hash を計算して承認する。

## 未確認・教訓

- Node が標準入力を閉じた場合・予期しない巨大出力・子孫を持つ場合の挙動、別種のツールによる実ビルドの終了コード、停止操作は検証範囲外。元 build は引き続き終了コード不明。独立レビュー後、別の専用領域・資源管理と元ソース17 hash の固定に基づき、本 build 検証の可否を管理が判断する。
- 今回の操作・検証・結果確定までのコマンド失敗なし。判定は対象版・観測値に限定し、成功文言や PID 不在だけで実終了コードを推測せず、同一 Process handle と EOF を別々に確認した。
