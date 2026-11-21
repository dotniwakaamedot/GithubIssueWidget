# PowerShell の読戻しアンカー失敗の独立分析

担当 `/root/concat_failure_analysis`。依頼 `concat-failure-analysis-request-01.md` SHA256 `5032E32513E8943F2DB3401EF8473766BC197B1F069A05DE8097AAF508A767FC`。作業場所 `D:/Projects/GithubIssueWidget`。元操作は再実行していない。

## 結論

両事象は PowerShell の文字アンカーを構成・利用する照合中に `ArgumentNullException` が発生し、対象の照合を証拠として使えなかった。例外から対象文書の破損を推測できない。正確にどの式が null を受けたか、PowerShell のオーバーロード解決や元の入力バイトは、現時点の証拠だけでは確定できない。

## 確認した事実と差

1. `/root/readback_encoding_analysis` の報告では、`[string]::Concat([char]0x6587,[char]0x5B57,[char]0x5316,[char]0x3051)` を各ファイルに対し呼び、その後 `[regex]::Escape($needle)` と `Matches` で照合した。2ファイルそれぞれで例外が出て、`anchor=1482` と `anchor=1868` は無効とした。複合コマンド全体は終了0、各式の終了状態は記録されていない。既存 `readback-encoding-analysis-01.md` の SHA256 は `0489616AE166ED97F28883C0B69C7883DFEDD4DE2702CC9610AE0E60FB2A09DE`。本人からのコマンド全文・結果の説明も照合した。
2. `/root/credential_harness_review` の記録と追加回答では、`[string]::Concat([char]0x8CC7,[char]0x683C,[char]0x60C5,[char]0x5831)` の後に `$s.Contains($anchor)` を使い、`ArgumentNullException` と `パラメーター名:argument` が表示された。その後 `UTF8 Japanese readback OK` が表示されたが、それは失敗した照合の成功を示さない。個別の終了コード、複合処理の終了コード、完全な元コマンドと例外のスタックは提示資料では未確認。`credential-harness-review-question-01.md` の SHA256 は `BB6EE7AA06205A5B411F7C430B26D33786BB03C513EA57FD5A3E8E1E39D06E9D`、追加回答の SHA256 は `A22F74C94A90BCB05D328EC37638895416CF90656C520B491D00A45F31F6C17C`。
3. 前者は正規表現のエスケープと件数、後者は `Contains` を使う。どちらも4文字の `[char]` を `Concat` に渡しているが、例外時の変数値や正確な失敗式は両事象とも記録だけでは特定できない。同じ直接原因や文字コード破損と断定しない。
4. 両担当は別の ASCII のみの Python コマンドで strict UTF-8 と Unicode escape を用いる代替照合の終了0を報告した。前者の記録では代表語各1件・U+FFFD各0件。これは検査した代表文字と復号の証拠であり、文書の全体が元の意図と一致する証明ではない。後者の代替も対象範囲を超えて一般化しない。

## 対応と未確認

既存教訓 `docs/recurring-lessons.md` の F-057 / F-062 は文字コードの明示、実データ読戻し、短い日本語の往復確認を求める。これは文字コード経路の対策であり、今回の null 例外の直接原因を特定するものではない。比較は ASCII の Unicode escape を使った Python 等で対象ファイルを strict UTF-8 復号し、期待文字の存在と必要な原文を照合する。各確認を独立したプロセスで実行して終了コードを個別保存し、例外時は直ちに失敗として後続成功表示と混同しない。代替手順は既に各担当が対象データで成功を報告しているが、全環境の再発防止は未検証。

実際に null となった値、正確なオーバーロード、元のすべての出力と後者の複合終了状態は未確認。必要なら担当の保存済み完全ログを照合し、元の失敗操作を繰り返さず限定合成ケースで仮説を検証する。共有失敗記録の追記と件数・重複照合は管理担当へ引き継ぐ。

## 変更と操作

依頼・2件の記録・追加回答・既存教訓を対象限定で読み、SHA256 を取得した。`Get-Content`、限定 `rg`、`Get-FileHash`、`Test-Path` は終了0。製品・資格情報・GitHub・既存記録は変更していない。削除対象なし。新規成果は本結果のみで、UTF-8 実データを読戻し一時ファイルから確定する。作業中に認識した失敗はない。
