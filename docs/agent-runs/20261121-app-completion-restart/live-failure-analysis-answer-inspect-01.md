# 残留確認中の失敗に関する回答

回答者 `/root/credential_residue_inspect`。質問 `live-failure-analysis-question-01.md` の対象１について、当時のツール出力に基づき記録する。元操作および資格情報操作は再実行していない。

1. **EXE パス不存在:** `Get-FileHash -Algorithm SHA256 -LiteralPath` に `credential-live-ready-02/recovery.py`、`supervisor.py` と、推測した `credential-live-ready-02/target/release/credential-live-test.exe` を同時指定した。後者は存在せず `Resolve-Path` の非終端エラーが表示された。複合コマンドは終了 0 で、前２ファイルの SHA は返ったが、推測した EXE のハッシュは得られなかった。元出力は当時のツール応答に残るが、独立ログファイルには保存していない。後に実在する EXE パスを `supervisor.py` の `BINARY` で確認し、`recovery.read_identity` が EXE 実体の固定 SHA と各記録を照合して成功した。不存在エラーを EXE の照合成功とは扱わない。
2. **文字照合失敗:** `credential-live-residue-inspect-01.md` の SHA と UTF-8 読戻し確認を行う PowerShell の here-string を Python 標準入力へ渡した際、Python 側で日本語リテラル `残留確認`・`対象`・`失敗` を代表文字に指定した比較は `utf8_markers=False` を表示した。処理の終了コードは 0。PowerShell→Python 間の日本語の伝達が候補だが、失敗時に Python が受け取った文字列の実バイトを保存しておらず直接原因は確定できない。元出力はツール応答に残るが別ログはない。別操作で ASCII の Unicode escape を用いてファイルの実バイトを strict UTF-8 復号し、同じ３語について `utf8_markers=True`、置換文字数 0、終了 0 を確認した。これは代表文字の確認であり全文の意図との一致は保証しない。

両事象は対象・性質が異なる各１回として親へ報告済み。実データ破損や元の試験の原因をこれらから推定しない。元データの削除・変更、Credential Manager の再照会は行っていない。
