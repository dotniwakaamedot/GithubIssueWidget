# 対話試験準備結果の SHA 訂正

- 担当 `/root/interactive_test_prepare`。対象は `interactive-test-prepare-01.md` SHA256 `AA6256419EE9E91A71BDB1D8C7AC40B5CD50CC00898C5BE081E7E04A924B3285`。独立レビューで `RECOVERY.md` の記載値と実体が一致しないと判明したため、元結果を変更せず訂正する。
- `widget/credential-check-20261121/RECOVERY.md` の正しい SHA256 は `F2F2D33AA2B3BC4CDC057384E38F2FBCF1BDAE10199D70946A5868C9A5B47E45`。元結果に記載した末尾 `3C3EED` の値は誤りであり、対象の証拠として使用しない。
- コピー元 `docs/agent-runs/20261121-app-completion-restart/credential-error-capture-01/RECOVERY.md` と新規コピーを `Get-FileHash -Algorithm SHA256 -LiteralPath` で再計算し、ともに上記正しい値、終了コード 0 であることを確認した。
- 最初の作業では SHA の出力は正しい値を表示していたが、結果文書への転記時に誤った値を書いた。対象ファイルの実体が変更された証拠ではない。元結果の UTF-8 読戻しは代表文字と EXE SHA を対象にしており、この転記誤りを検出しなかった。
- 元結果と両 `RECOVERY.md` は変更していない。実 Credential Manager、実 PAT、既存資格情報、GitHub、アプリは操作していない。今回の転記失敗１件について共有失敗記録への反映と、結果・本訂正の対象版に対する独立レビューを親に依頼する。削除対象なし。
