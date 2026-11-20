# Issue #2 環境調査結果

担当: `/root/recheck_environment`。読み取り専用の調査。作業ディレクトリ: `D:\Projects\GithubIssueWidget`。

## 一次資料

`gh issue view 2 --json number,title,body,state,labels,comments,url`（コマンド終了コード 0）で [Issue #2](https://github.com/dotniwakaamedot/GithubIssueWidget/issues/2) を取得。OPEN、コメントなし、ラベル enhancement、依存なし、親 #1。Tauri 2・React・TypeScript・Rust の実行可能な基盤、Windows での開発起動・リリースビルド、装飾なしのダークテーマ・900×520 初期表示・600×300 最小サイズ・中央表示・初期最前面・移動／サイズ変更／最前面切替、Opener の外部リンク許可先制限・最小権限、および起動・操作・ビルド結果と未確認環境の記録が完了条件。

## 環境の確認

以下はそれぞれの PowerShell コマンド群の終了コードが 0 であったことを確認。個々のコマンドの終了コードは別途記録していない。

- `Get-CimInstance Win32_OperatingSystem | Select-Object Caption,Version,BuildNumber,OSArchitecture | ConvertTo-Json -Compress`: Microsoft Windows 11 Home、10.0.26300、build 26300、64-bit。
- `node --version` / `npm --version` / `rustc --version` / `cargo --version` / `rustup show active-toolchain`: Node v26.13.1、npm 11.20.0、rustc 1.84.1 (e71f9a9a9 2025-01-27)、cargo 1.84.1 (66221abde 2024-11-19)、stable-x86_64-pc-windows-msvc (default)。
- `vswhere.exe -products '*' -requires Microsoft.VisualStudio.Component.VC.Tools.x86.x64 -format json -utf8 -property installationPath`: Visual Studio Community 2022 と 18 のインストールパスを検出。
- `vswhere.exe -products '*' -requires Microsoft.VisualStudio.Component.VC.Tools.x86.x64 -find 'VC\Tools\MSVC\**\bin\Hostx64\x64\cl.exe' -utf8`: 2022 の MSVC 14.38.33130 と 14.44.35207、18 の 14.51.36231 の `cl.exe` を検出。
- Windows Kits Installed Roots の `KitsRoot10`: `C:\Program Files (x86)\Windows Kits\10\`。登録先の存在から SDK の完全性や利用可能性は確定できない。
- `rustup target list --installed`: `x86_64-pc-windows-msvc`。
- `HKLM:\SOFTWARE\WOW6432Node\Microsoft\EdgeUpdate\Clients` および `HKCU:\SOFTWARE\Microsoft\EdgeUpdate\Clients` の登録情報から Microsoft Edge WebView2 Runtime `157.0.4341.60` を検出。実アプリでの使用は未確認。

## 未確認・制約

MSVC と Windows SDK を使った実コンパイル、依存関係のインストール、Tauri アプリの起動、操作、リリースビルド、WebView2 の実使用は未実施。Rust・Node のバージョンと今後選択する依存パッケージとの互換性も未確認。環境・GitHub・アプリは変更していない。

## 失敗と教訓

今回の実行で確認した失敗はなし。前回の日本語文字化けの再発を防ぐため PowerShell 出力を UTF-8 とし、vswhere は `-utf8` と必要なパスのみに限定して取得した。