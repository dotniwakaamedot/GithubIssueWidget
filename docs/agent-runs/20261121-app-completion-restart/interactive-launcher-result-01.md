# 合成資格情報テスト用ランチャーの作成

担当 `/root/interactive_launcher`。作業場所 `D:/Projects/GithubIssueWidget`。既存の `widget/credential-check-20261121/実行手順.md` を基に、新規 `run-credential-check.cmd` と `run-credential-check.ps1` を同ディレクトリに作成した。既存の supervisor・recovery・EXE・資格情報・GitHub は変更していない。

## 変更と対象版

- `widget/credential-check-20261121/run-credential-check.ps1`: SHA256 `8A6D3142B328B4BA08DD5B75D6565EDEB45A64628BEA22950CD16846D9A3A811`。Windows PowerShell 5 用 UTF-8 BOM。PATを使わず実OSに合成値を一時的に保存・更新・削除すること、結果不明時の再実行と手動削除をしないことを表示し、`YES` の明示入力以外は終了2。既存手順と同じ固定絶対パス・実体・ハッシュ・Python版・one-shot を確認し、例外時は終了1。正常時は supervisor の state・exit・run ID の報告を求める。
- `widget/credential-check-20261121/run-credential-check.cmd`: SHA256 `933288B08C80770C771A45ED07477A29B1F12BDCC9EFE7D5D66C429C95C34E41`。ASCII。固定の Windows PowerShell と上記 PS1 の絶対パスを `-NoProfile -ExecutionPolicy Bypass -File` で呼び、終了コードを表示・伝播し pause する。
- 入力欄やユーザー認証情報の値は取得・表示しない。固定 Python の対話セッションでの使用可否、実行時の画面表示、実 Credential Manager、アプリの PAT ライフサイクルは未確認。OS が管理するポリシーや環境により起動が拒否される可能性はある。

## 検証

- `launcher-synthetic-01/test_launcher.py` SHA256 `39CA97CB65549EB419E076FFC3E28E5CC912072D14D0C79801DF4025C0B67944` は別の新規領域に PS1 を複製し、実 supervisor 呼出し行のみ無害な `SAFE_TEST_REACHED` 出力に置換して試験。固定対象と偽ファイルのハッシュを隔離 fixture 用に差し替え、実 Credential API を呼ばない。２回目のハーネス実行は終了0、正常、開始拒否、ディレクトリ不存在、ファイル不存在、SHA不一致、one-shot 既存、Python不存在の7ケースが期待終了コード・到達状態と一致。ユーザーの対話画面や実 OS 動作はこの試験の範囲外。
- `launcher-synthetic-01/test_cmd.py` SHA256 `41BD03982283DF04DB7706F4C298BDC3E9806C298FDEFDDDC3B1E1F90106F4C5` は新規領域へ cmd を複製し、PS1 パスを `exit 7` の偽スクリプトに置換して実行。終了0でテスト成功し、cmd 側が `Exit code: 7` を表示、終了7を伝播した。元 cmd は実行していない。
- `Get-FileHash -Algorithm SHA256` で上記4実体を確認。既存 EXE と対象スクリプトは操作・再実行していない。テスト出力はツール結果として確認したが、別ファイルの実行ログは保存していない。

## 失敗と未確認

1. 最初の合成ハーネスで正常ケースが終了1・到達せず失敗。標準エラーを保存しておらず直接原因は未確定。ハーネスが実 cmd と異なり `-ExecutionPolicy Bypass` を付けていなかったため、ハーネスを修正し別ディレクトリで成功した。修正と失敗の因果関係を断定しない。
2. Read-Host の挙動確認用の別コマンドは引用の誤りで exit1。その後 ASCII here-string で同機能を限定確認し終了0。欠落した出力は復元していない。

親担当には両失敗を報告済み。共有失敗記録はこの担当の所有外であり未更新。対象版の独立レビューと失敗記録の反映が必要。実行・OS状態が不明の場合は再送や手動削除をしない。新規 launcher・専用合成試験と本結果以外は変更せず、削除対象なし。
