# 検証版 EXE のローカル受渡し準備

担当 `/root/release_copy`。依頼 `release-copy-request-01.md` に従い、検証版 EXE を既存物とは別の新規ディレクトリにコピーした。これはローカル受渡し準備であり、外部配布、インストール、完成の承認ではない。

## 対象と結果

- 作業場所: `D:/Projects/GithubIssueWidget`。コピー元: `docs/agent-runs/20261121-app-completion-restart/selection-build-sandbox-01/cargo-target/release/github-issue-widget.exe`。
- コピー先: `D:/Projects/GithubIssueWidget/widget/release-20261121-selection/github-issue-widget.exe`。新規ディレクトリの存在しないこと、解決済み絶対パスが workspace 内であることを操作直前に確認した。既存ディレクトリへの上書きは行っていない。
- 元とコピーの SHA256 はともに `0D8AB47088B7A19233EF0D7D08CF033B681B20CA0E729FEAF73EAB74DCC98F55`、サイズはいずれも 12,885,504 bytes。コピー直前の元のハッシュは依頼値に一致。
- `selection-build-result-01.md` は release ビルド終了コード 0、`selection-build-review-01.md` は当該 EXE ハッシュの対象版を承認と記録。`release-readme-audit-01.md` は安定したローカル受渡先が必要と記録する。

## 操作と検証

PowerShell で `Resolve-Path`、`[IO.Path]::GetFullPath`、`Test-Path`、`Get-FileHash -Algorithm SHA256`、`New-Item -ItemType Directory -ErrorAction Stop`、上書きを拒否する `[IO.File]::Copy(source, destination, $false)`、元・コピーのハッシュと `Get-Item` によるサイズ比較を実行した。一連の操作は終了コード 0、値は一致した。コピー後の実行や再ビルドはしていない。

変更は新規ディレクトリの EXE と本記録のみ。他者の既存成果、資格情報、Git、GitHub は変更せず、削除対象もない。作業中に認識した失敗はない。独立レビュー、実動作の範囲外の検証やアプリ完成の確認は別途必要。
