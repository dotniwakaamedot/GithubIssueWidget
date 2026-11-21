# 合成資格情報試験の文字列照合失敗の独立分析

- 担当 `/root/credential_supervisor_failure`。対象担当 `/root/credential_supervisor` の報告と既存資料を読み取りで照合した。実 Credential Manager、製品、対象コード、対象結果を変更していない。
- 1件目: 作者によると `python -m unittest -v test_supervisor` は終了1。`test_completed_has_durable_start_and_outcome` が Windows パスの生バイトを JSON エスケープ済みファイルの生バイトに対して `assertIn` した。Windows のバックスラッシュを JSON はエスケープするため両表現は異なる。これは値の不一致を意味しない。元の出力はツール応答のみで独立ログは未保存。現行 `test_supervisor.py` では JSON を decode して `record["binary"] == str(self.binary)` と比較している。作者は修正後に `py-test-02.log` の7件成功・終了0を報告した。本分析では失敗を再実行していない。
- 2件目: 作者によると結果 tmp を strict UTF-8 decode し「合成資格情報」、`Credential Manager`、置換文字なしを assert する Python コマンドが AssertionError・終了1。元のエラー出力は独立保存されず、複数の assert のいずれが失敗したかや、JS・PowerShell・Python をまたぐバックスラッシュの実引数は未確定。エスケープの二重化は原因候補であり断定しない。作者は失敗後に再実行・対象 tmp 書込み・公開を停止した。
- 現在の `credential-supervisor-result-01.tmp.md` の実バイトは 7,112 bytes、SHA256 `D4C86296A9670DE6B6D89C3C88BC68922FE08A627FC165C840935F73D2D8B946`。独立した ASCII-only Python コードで strict UTF-8 decode が終了0、「合成資格情報」1箇所、`Credential Manager` 2箇所、U+FFFD 0箇所を確認した。これは現在の一時ファイルに代表文字が存在する証拠であり、旧失敗時の引数、全文の意図との一致、テスト終了コード、実 OS 処理の成否を証明しない。
- 共通点は文字列の意味値と境界で符号化・エスケープされた表現を混同する危険。対象は異なるが一度の担当作業内で同種の照合失敗が２回と見なされ、関連作業停止後に別担当が分析した。未保存の表示は復元済みとしない。
- 安全な確定方法: 所有者の書込み停止を維持し、親または別の担当が現在の tmp の実体・SHA と内容を確認する。元のコマンドを復唱せず、ASCII-only のスクリプトで strict UTF-8 の実バイトを読み、Unicode escape を一段階だけ評価して代表文字と置換文字を照合する。元の作業指示・ログと全文の意味を別途照合し、確定先が未存在であることを確認して、同一ディレクトリ内で安全に公開する。公開後に SHA を再確認し、独立レビューへ渡す。表現の一致だけを安全性や実 Credential Manager の検証とみなさない。
- 実行結果: 対象 tmp の existence、元コードの限定読取り、上記 Python の strict decode・ハッシュと代表文字検査は終了0。秘密・実資格情報の出力なし。新たに認識した失敗なし。削除対象なし。共有失敗記録への反映と再発防止効果の検証は親の管理担当へ引き継ぐ。
