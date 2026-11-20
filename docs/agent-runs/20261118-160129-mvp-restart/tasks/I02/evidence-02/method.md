# I02 追加検証の手順と範囲

作業場所: `D:\Projects\GithubIssueWidget-issue-2-20261118-160129\widget`。Windows 11 Home 10.0.26300 64-bit、Node 26.13.1、npm 11.20.0、Rust/Cargo 1.90.0、Visual Studio 18 Community の MSVC。WebView2 は環境調査で 157.0.4341.60 と検出。ツール追加や既定環境の変更なし。

## 実行コマンド

PowerShell で次を各コマンド実行前に実行し、`VsDevCmd.bat` が出す環境変数をその PowerShell プロセスにのみ取り込んだ。読み込みに成功した場合のみ後続コマンドを実行する。

```powershell
$vs='C:\Program Files\Microsoft Visual Studio\18\Community\Common7\Tools\VsDevCmd.bat'
$lines=& cmd.exe /c "call `"$vs`" -arch=x64 >nul && set"
if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }
foreach($line in $lines){if($line -match '^([^=]+)=(.*)$'){[Environment]::SetEnvironmentVariable($Matches[1],$Matches[2],'Process')}}
```

PowerShell の実行環境では npm の stderr が `NativeCommandError` と装飾されるが、最終終了コードを別途確認している。

```powershell
& npm.cmd run tauri -- build --no-bundle 2>&1 | Out-File -LiteralPath '<evidence-02>/release.log' -Encoding utf8
$LASTEXITCODE # 0
& npm.cmd run tauri -- dev 2>&1 | Out-File -LiteralPath '<evidence-02>/dev.log' -Encoding utf8
$LASTEXITCODE # 0、アプリの閉じるボタンで終了後
```

`<evidence-02>` はこのファイルの置かれた共有 run の `tasks/I02/evidence-02` の絶対パス。記録した release 実行ファイル SHA256 は `5E6877A9B2C89078BBDEA65033F8588D867E17A83689734170719880D08AE144`、debug 実行ファイルは `064907CEB4C92A78901E0094B47EF329B4330E771DA85160B3557637DA6D5845`。依頼作業中にビルドしたため実行ファイル hash は前回ビルドの値と異なる。製品ソース17ファイルは前回と一致。

## 観測

- `ui-observation.json` は Computer Use のウィンドウ選択結果、スクリーンショットの寸法・座標メタデータ、アクセシビリティテキストとブラウザ選択後に読み取った URL の抜粋を保存したもの。画像そのものは保存していない。選択した実行ファイルは debug 版。初期画面を観察後、固定ボタンを押して表示変更を確認し、リンクを押して Brave のアドレスバー `github.com`、ドキュメント URL `https://github.com/` を確認した。
- 初期寸法の画像範囲 902×522、座標 1477,796 は枠・DPI 等を区別した OS のクライアント寸法ではない。スクリーン範囲・DPI を独立測定できておらず、900×520 または中央の検証成功とはしない。最小サイズ、リサイズ、OS の最前面フラグは実測していない。移動操作は禁止のため再実行していない。
- Computer Use で閉じる操作時、ブラウザが前面にある状態で一度 `coordinate input geometry is unavailable`。対象ウィンドウを再選択・前面化してから新しい観測で再実行し正常終了した。同種の失敗は1回。

## Scope の限定的な検証

Opener 2.7.3 の `src/scope.rs` では URL scope のマッチングに `glob::Pattern::matches` を使用し、deny を先に評価した後 allow を照合する。`scope_check.rs` は対象の capability に書かれた2つの許可パターンを再現し、ビルド済みデバッグ依存物 `libglob-d74a01eea75b995e.rlib` をリンクして単純マッチをテストする。製品アプリ・ブラウザは起動しない。

```powershell
$glob=(Get-ChildItem -LiteralPath 'src-tauri/target/debug/deps' -Filter 'libglob-*.rlib' | Select-Object -First 1).FullName
& rustc '<evidence-02>/scope_check.rs' --edition=2021 --extern "glob=$glob" -L dependency=src-tauri/target/debug/deps -o '<evidence-02>/scope_check.exe'
$LASTEXITCODE # 0
& '<evidence-02>/scope_check.exe' | Out-File -LiteralPath '<evidence-02>/scope-check.log' -Encoding utf8
$LASTEXITCODE # 0
```

正規 GitHub URL 2件が許可、別ドメイン、紛らわしいホスト名と userinfo、http・javascript・file とサブドメイン計7件が拒否になるパターン判定を確認。これは glob の単体照合であり、Tauri の実際の IPC ACL・ブラウザへ渡す全経路や、URL 正規化を含む拒否動作の検証ではない。IPC を介した不許可 URL の実際の拒否は未確認。
