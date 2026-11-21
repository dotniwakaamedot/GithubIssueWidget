# 実 OS 合成試験と記録時の失敗の独立分析

- 担当 `/root/live_failure_recorder/independent_analysis`。作業場所 `D:/Projects/GithubIssueWidget`、共有 run `D:/Projects/GithubIssueWidget/docs/agent-runs/20261121-app-completion-restart/`。依頼 `live-failure-record-analysis-request-01.md` SHA256 `7A7796AA46779AC98C9C9074B628297470987484D6E9AABEE457741C06562EE1`。
- 親担当の認識した事象を既存記録と照合した。対象の元操作、Credential Manager の読み書き・削除、試験の再実行は行わない。既存記録・製品・資格情報は変更しない。

## 個別の事実・原因・後続

1. **実 OS 合成試験の失敗１回**: `credential-live-execution-result-01.md` SHA256 `7C72FB0497934AE43A92015BDD38FA36BBD011807FF2A8B0AFA7B45E5429E5ED` は、監督終了1、直接子の wait 済み `completed`・終了1、最終 journal `before_write_a` を記録する。`credential-live-failure-investigation-01.md` SHA256 `2645E477E49B0036D348888832BCEEEF57F9AC84EF0993E11A2808DD09786C8C` は、子 stderr の破棄により直接原因と書込み成否を区別できないとする。後の `credential-live-residue-inspect-01.md` SHA256 `603D8C1BC3BB8ABADA07E8B38EE0882CE978ED16ADC285E2E84635A1500C3A01` は、対象合成 target を１回だけ読取り `absent`・終了0と記録。独立 `credential-live-residue-review-01.md` SHA256 `67C1B3BBFDE16B78E40099C0114E4C025D24B9E04FFB1EE3185B3DB31C51CB6F` は対象版・実体を照合して承認したが、作者の終了コードを独立監督した証拠はない。`absent` はその対象と観測時点に限定し、元の書込みの実行・成否や終了1の原因を証明しない。実行・調査・残留確認は一連の１発生として、別原因が確認されるまで既存 ID へ根拠なく統合しない。再送・削除は対象版と条件を再検討し、値や秘密を出さず診断可能な情報を記録する。元 stderr は復元できない。
2. **出力切り詰め１回**: `/root/credential_live_failure_investigation` は同調査結果で run 全体のファイル名表示が内側上限で切り詰められたと報告し、後に run ID を限定した。元の全出力・最大長・終了コードは本文で特定されず、欠落は未確認。F-001 は出力上限超過の予防策を共有する関連 ID だが、既存記録の欠番・重複を含む全累計は未照合で確定しない。対象と変換後行長、内外の上限と余裕を先に見積もり、限定取得を適用する。効果は未検証。
3. **推測パス不存在１回**: 検査担当の確定回答 `live-failure-analysis-answer-inspect-01.md` SHA256 `A168335EE17EB6B2E4189A200068A9C5981EDA46F93507B9558E71F0D31A5B02` によると、`Get-FileHash` に推測した EXE を含む複数パスを指定し、その EXE は非終端の不存在エラー、複合コマンド全体終了0だった。前２件のハッシュ取得と不存在を区別し、全体終了0を EXE の成功と扱わない。実 EXE は後に supervisor の定義と `read_identity` の固定 SHA 照合で確認されたと報告された。元ツール応答はあるが独立ログはない。存在未確認パスに関する F-012 が関連し、他の類似記録との過去累計は未照合。対象の実在するパスと個々の終了状態を先に確認する。
4. **代表文字照合失敗１回**: 同回答によると PowerShell here-string から Python 標準入力へ渡した日本語の代表文字照合は `utf8_markers=False`、終了0。失敗時の Python 入力実バイトは未保存で、伝達境界の変換は原因候補にとどまる。別操作の ASCII Unicode escape と strict UTF-8 復号は同じ３語について true、置換文字0、終了0と担当が報告。全文の意図との一致や元ファイル破損を示さない。F-057 の文字コード境界・F-086 の期待値照合と予防策は関連するが、直接原因・同一 ID・過去累計は未確定。入力と出力の文字コードを明示し、復号後の値と対象版を比較する。
5. **記録担当の表示文字化け１回**: 確定回答 `live-failure-analysis-answer-recorder-01.md` SHA256 `B9D91DBBFA9EDB73375C1E2B2ADA6804BA3D9D4ED2A7EE0ED28381C9414A7049` によると、`failure-log-002.md` を Python `read_text(encoding='utf-8')` で読み見出し等を表示したコマンドは終了0だったが日本語が判読不能。元ツール応答はあるが独立の完全ログなし。後に PowerShell で別条件の６見出しを可読で取得したが、元出力の復元・同一条件での完全再取得ではない。直接の変換境界は未特定で、元ファイル破損の証拠ではない。表示文字化けの F-057 と予防策が関連するが過去の累計・原因同一性・効果は未確認。短い範囲の入出力を明示し、元データと表示を区別する。

## 既存対策と判断

`docs/failure-log-001.md` の F-001・F-012・F-057 と `docs/recurring-lessons.md` の該当節を照合した。出力量・行長の事前見積り、実在パスの明示、入出力の文字コードの確認という既存対策を今回も適用する判断が妥当。未知の境界を特定済みとせず、ユーザー指定の同種2回の停止条件は各担当の同一指示内の回数で判断する。各担当による今回の認識は各1回ずつであり、同一障害の分析や対象外の再実行を新たな発生として数えない。記録・ID・累計と教訓の反映は管理へ引き継ぐ。

## 作業・証拠の制約

- 依頼と上記６文書は実体と SHA256 を確認し、対象の短い範囲を PowerShell `Get-Content -Encoding UTF8` で読んだ。`Get-FileHash`・限定読取りは終了0。個別の過去ツール応答は本担当で再取得していないため、元コマンド・終了の詳細は担当者の確定回答に依拠する。
- 管理の分冊表示文字化けは元の失われた表示を復元していない。件数の全分冊照合、失敗時の実 credential 状態、直接原因、予防策の効果は未確認。実アプリ・実 PAT・WebView は本分析の対象外。
- 本分析と質問ファイル以外は変更せず、削除対象なし。確定ファイル自身の SHA は管理が取得する。今回認識した新たな失敗はない。
