# I02 子プロセス終了コード・出力の無害試験設計 02

- 担当 `/root/build_verification`。前版 `tasks/I02/process-test-design-01.md` SHA256 `C39FE60DFEDA356AC5C39C86A1052BCD26E5C2DEADF3CB90F4F0922392E5DC48` の方式・開始条件・所有範囲・timeout 制御・停止条件を維持する。ただし本版と前版が異なる場合は本版を適用し、**実行対象は driver-04.ps1 のみ**とする。旧ドライバーは保護して使わない。
- 親からの独立レビュー結果: 前版 driver は各ストリームのバイト数と末尾のみで成功を判定するため、同じ長さ・末尾の内容の誤りや timeout の `WAITING` の順序を検出できない。レビュー記録の公開に関する拒否された操作は再実行・回避しない。
- 修正対象 `D:\Projects\GithubIssueWidget-process-test-20261119-01\driver-04.ps1` SHA256 `F24B68C56FA99A67685BFB864F9B39F768EBDBA0CC1D7B2E92A12CA4DF0C7FDD`。前版 `driver-03.ps1` から別版を新規作成し、UTF-8・PowerShell parser エラー0・確定前後 hash 一致を確認。**子や driver の試験実行はしていない。**
- 子 `child.cjs` SHA256 `55FFB888FAA4F5E2B93D0F806DF6892834258B15756535895BC68937C621276C`、Node `C:\Program Files\nodejs\node.exe` SHA256 `F5109DC868EC38703AD8F305E0ACEECE717CB60D4889873F434017AF4E96CCCB`。各実行前に絶対パス、存在、hash、親・出力先・所有者を確認する。対象の runtime ・driver の実動作は未検証。

## 是正内容と期待値

各ストリームの UTF-8 復号後文字列を、期待する ASCII 文字列**全体**と大文字小文字を区別して比較する。検証コードでは `-ceq` で比較し、同じ長さでも文字違いや同じストリーム内の順序違いがあれば `pass=false`。同時に終了コード、UTF-8 byte 数、EOF と timeout の有無も比較する。

| case | stdout の完全な期待内容 | stderr の完全な期待内容 | 実子の期待 exit |
| --- | --- | --- | --- |
| normal | `OUT_END\n` | `ERR_END\n` | 0 |
| nonzero | `OUT_END\n` | `ERR_END\n` | 7 |
| flood | ASCII `O` 524,288個の直後に `OUT_END\n` | ASCII `E` 524,288個の直後に `ERR_END\n` | 0 |
| timeout | `WAITING\nOUT_END\n` | `ERR_END\n` | 0（終了後のみ） |

前版で示した 8 / 8、8 / 8、524,296 / 524,296、16 / 8 の byte 数は変わらない。完全比較はそれぞれのストリーム内部の内容と順序を検証するが、**stdout と stderr の間での発生順序や真の同時性は証明しない**。両方を同時に非同期消費し、大量書込みが詰まらず両 EOF とコードを取得できるかは実行後に初めて判定する。

## 実行計画と待機

前版の4つの例示コマンドは、それぞれ対象ファイル名を `driver-03.ps1` から `driver-04.ps1` に読み替える。正常0、意図した7、大量出力、timeout は独立した4ケースを直列に行い、各 case の完了・証拠・未解決の問題を確認してから次に進む。timeout 500 ms と子の120秒自然終了、非 timeout の20秒、EOF の5秒、同一 handle 保持、管理判断待ち、出力先・採取項目は前版を維持する。

管理・独立レビューが承認し、対象 SHA・既存作業の停止・出力先の不存在、Node/PowerShell 実行可能性を確認するまで一切起動しない。実行開始に失敗・timeout・標準入力不明等があれば同じ case を再起動せず質問。子・外部操作・元 build は再実行しない。

同じ問題が初回を含め2回で該当操作を止め別担当へ客観調査を依頼、3回で全作業停止。共有 run の直接確定名への公開逸脱2件、拒否されたレビュー記録の公開操作、過去の停止条件は前版を維持する。実際の試験成功は元ビルド・Issue 完了の証明ではない。

## 成果と未確認

この版での変更は scratch に新規作成した `driver-04.ps1` と本設計ファイルのみ。製品・GitHub・旧証拠・旧ドライバーの変更や削除はない。driver の構文検査は exit 0 だが、実際の文字列生成、Process API、EOF、timeout の期待動作は未確認。独立レビューと親判断を待つ。今回の修正時のコマンド失敗はない。
