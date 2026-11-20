# I02 B03 コピー前の照合失敗 05

- 担当 `/root/build_verification`、宛先 `/root`、`/root/restart_planner`。
- 専用スクリプト `copy-source.ps1` で対象一覧セクションを抽出する際、`Product section missing` で exit 1。専用コピーのディレクトリ作成より前に停止し、製品・GitHub・元 worktree は変更していない。
- 推測: PowerShell 5.1 が BOM なし UTF-8 の ps1 に含まれる日本語リテラルを既定コードページで解釈し、照合文字列が実際の見出しと異なった可能性。直接原因は未確認。
- 今回同種のセクション抽出失敗は1回。2回目で別担当へ調査、3回目で全停止。失敗した方式を繰り返さず、読み取り専用で使用可能な ASCII 識別条件を分析し、親の判断を待つ。
- コピー、依存導入、build/dev は未実施。
