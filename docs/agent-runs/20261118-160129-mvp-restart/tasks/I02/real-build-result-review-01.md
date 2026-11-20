# 専用コピー release build 結果の独立レビュー 01

- 担当 `/root/process_test_review`、作業作者 `/root/build_verification`。対象 `real-build-result-01.md` SHA256 `D4056E537530E340B9556C82503D55FE40236FFE49471D429E1F3D2575E837D3` は依頼と一致。ログ公開 receipt の機密確認は本担当が行っており、その判断の独立再評価ではなく、build 作業作者とは別の結果照合である。
- 判定: **対象版の release build 結果に限って承認**。blocking 指摘なし。Windows の開発起動・正常終了や UI・Opener の実証、Issue #2 完了の承認ではない。

| 条件 | 実体・証拠との照合 | 限界 |
| --- | --- | --- |
| 対象版 | 依頼の31参照は全て存在し期待 SHA と一致。manifest の元・専用コピー各17ファイルは実体 SHA 一致。各3 stage の前後計6記録を別の JSON parser で照合し各17件・元とコピー双方一致。 | 前後一致は途中の一時変更や他環境からの非競合を完全には証明しない。HEAD のみでは対象を識別しない。 |
| 直接 process と EOF | rustup PID 30792、npm PID 5164、release PID 12016 は結果 JSON に exit 0、両 EOF RanToCompletion、開始・終了時刻順を記録。VS bootstrap PID 31708 も exit 0 と両 EOF を記録。 | 実行時の process handle 連続性や作者の外側 session exit を独立に再観測したものではない。npm→shell→Tauri→Cargo 各子孫の exit/伝播は直接未取得。 |
| ログ・receipt・exe | 各 stage の実ログ hash と receipt・結果 JSON の hash を照合。release ログの必要最小限の機械照合は release profile 終了と exe 名の存在を示す。release exe の実体 SHA `D5FD13D80B6FE7F72ACD0BA6CEA3E0E19F261E417F8F5331E81F94D09A379D44` は結果と一致。 | 保護ログの各 receipt は本担当によるパターン・文脈の限定的確認であり、秘密不存在の保証ではない。ログ本文・値は本記録に掲載しない。 |
| B03 と関連要件 | release build の専用コピー対象版と直接 process の成功に関する証拠は整合。 | **B03全体は未完了**。dev 起動・正常終了は別途必要。ウィンドウ移動・リサイズ・最前面・寸法・Opener の実 IPC 拒否も未検証。以前の別 build の exit 不明は今回で解消しない。 |

## 実施内容・失敗

- 依頼の31参照と対象17組は `Test-Path` / `Get-FileHash` で照合し全件一致。3 stage の実体・receipt・生成 exe と時刻・EOF・hash を Python JSON parser で照合、各 exit 0。承認済みログの限定読取も真偽値だけ出力し、秘密・本文は出力していない。
- 当初、PowerShell の `@(Get-Content -Raw ... | ConvertFrom-Json)` で stage 配列件数の判定が不成立となり停止。作者が同様の配列化で1回誤表示した履歴と合わせ同種2件として親に通知し、別担当の客観調査を受けた。原因は PowerShell の配列包み込みと整合。指示に従い同じ構文を再実行せず、別実装 Python `json.loads` で6記録を独立に照合し成功。同種3件目は全停止。
- 試験・build・画面・process 操作・GitHub 更新・製品変更・削除は行っていない。拒否された別対象の window metadata review 公開操作は再試行・回避していない。対象版変更後は承認を流用しない。
