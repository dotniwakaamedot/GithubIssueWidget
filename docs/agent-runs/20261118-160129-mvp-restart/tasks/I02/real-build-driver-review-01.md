# 実ビルド driver 設計の独立レビュー 01

- 担当 `/root/process_test_review`、作者 `/root/build_verification`。対象 `real-build-driver-design-01.md` SHA256 `F4D173AAEF4964FB8C1B46B2A9BD4D71E16B4DDCD7EDAFB45651B79D6BA9E50D`、`build-driver-02.ps1` SHA256 `6B0E8CEB7C86C52E06982A8FB83A024E34EEE18C226B285319777F68608EC078` は依頼と一致。依頼の10参照、元と専用コピー各17ファイルの SHA は全件一致。
- 判定: **要修正**。実 build は実施せず、本版を承認しない。実行可能性・元の待機原因・Issue #2 の完了は未確認。

| 対象 | 確認 | 未確認 |
| --- | --- | --- |
| 無害試験との対応 | 直起動 Process の exit を HasExited 後に読む構造。stdout/stderr は別タスクで並行読み取り。timeout は同じ handle を保持。 | 無害試験は文字列 ReadToEndAsync、今回は BaseStream→FileStream の別経路。実 npm/Cargo 子孫と終了伝播は未実証。 |
| 対象・環境 | stage と固定引数、17対象の元・コピー hash、専用 home/cache/target/temp、公開取得先を記録。環境変数は秘密の遮断境界ではないと明記。 | 子孫や依存 script による任意の別場所アクセス、実行時干渉と OS 通常副作用は否定できない。 |
| 出力・時間 | 100 MiB は停止ではなく観測閾値、EOF と timeout は区別。 | 下記 B01/B02 を修正し、実行前条件を版に結び付ける必要がある。 |

## 指摘

| ID | 種別 | 根拠・影響 | 是正条件 |
| --- | --- | --- | --- |
| B01 | blocking | 設計は「出力に機密が混入した場合は公開せず保護」とするが、実コードは両 stream 完了後に内容を確認せず `.stdout.log.tmp` / `.stderr.log.tmp` を確定名へ Move する。取得先や依存 build script の出力に秘密が含まれる場合、約束した公開前停止を実施できない。 | 両 stream を保護領域に置いたまま必要な秘密混入確認・不明時停止・所有者への報告を実装または実行手順として確実なゲートにする。確認前に確定名への公開や内容の不要な表示をしない。検出の限界も明示する。 |
| B02 | blocking | `CopyToAsync` の2 task は process が動作中に fault しても監視ループで検査されず、後の `WhenAll.Wait` や `GetResult` が例外を投げ得る。`Hold-ForDecision` は `IsCompleted` のみを見て fault と正常 EOF を区別せず continue を受理する。例外処理が handle と証拠を保持して管理判断を待つ構造ではないため、要求された fault 時の追跡・保持を保証できない。VS bootstrap の非同期 task も同様に fault の分類・保持を明示しない。 | 各 task の fault/cancel と正常 EOF を区別し、故障・ログ書込み失敗・起動成否不明の各場合に所有 process/handle・一時ログの状態を保護し管理へ報告、同じ stage を自動再起動しない具体的処理を用意する。wait/continue も成功 EOF でない fault を明示し、安全な終了や状態の追跡を独立レビューできるようにする。 |

## 実施内容と限界

- 依頼・設計・driver・無害試験版を UTF-8 で読み取り、構造化依頼の10参照は全件存在後 SHA256 照合。元およびコピーの各17件は manifest の各基点・相対パスで存在確認後、SHA256 照合（cwd `D:\Projects\GithubIssueWidget`、各 exit 0）。
- 依頼の対象版は plan-review と別の作業。元の未コミット README と未追跡 widget は保護、対象設計に削除提案なし。作業外の全 ignored ファイル・全外部実行の非競合は未証明。
- driver・npm・Cargo・GUI・プロセスを一切実行せず、外部通信や GitHub 更新もしていない。以前拒否された window metadata の公開操作は再試行・回避していない。今回の読取コマンド失敗なし。修正版は新しい hash に対し独立レビューする。
