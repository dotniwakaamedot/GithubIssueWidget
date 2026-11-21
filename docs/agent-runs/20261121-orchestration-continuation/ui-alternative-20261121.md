# Windows UI 代替検証経路（2026-11-21）

担当 `/root/ui_alternative`。対象 HEAD `4961b25bec53a93439acdf5230de77f959908af0`、実行ファイルは `widget/release-20261121-02/GitHubIssueWidget.exe`。アプリの起動・操作、資格情報操作、GitHub 書込みは実施していない。

## 確認と判断

- `runtime-audit-20261121.md` は同 EXE の SHA256 `D19A72750006DF5BB6CBCA4FB15F87F69BFF48A5785F5569C5B05E40C9B93F26` と、CUA の初期化エラー `0x8007001F` を記録する。別担当は再試行していない。
- `docs/handoff-20261121-current.md` は別 release と診断版の直接プロセス生存、および CUA によるウィンドウ識別不能を記録する。ユーザー提供の画面画像も記録されているが、その画像だけでは EXE の版を特定できない。
- PowerShell で `SESSIONNAME=Console`、`Environment.UserInteractive=True`、PowerShell の Session ID 2 を確認。`Get-Process -Name GitHubIssueWidget -ErrorAction SilentlyContinue` の出力はなく終了コード 1。これらは対話デスクトップの操作可能性や、不明な WebView2 子孫の不存在を証明しない。
- 起動時の `connection_info` は設定回復と資格情報の読取りを呼ぶため、APPDATA 等の分離だけでは Windows Credential Manager を隔離できない。既存の不明な子孫やアクセス拒否の記録もあり、単純な再起動や別プロセスの終了では安全かつ版に紐づく UI 証拠を得られない。実起動は行わなかった。

## 次の検証

CUA の表示オーバーレイ障害を別途解消し、またはユーザーの対話デスクトップで実行ファイルの SHA とプロセスを同定して操作できる状態を確保する。専用設定先と Credential Manager の副作用、プロセス終了の監督条件を確認した後、実版の起動・通常終了・各操作を検証する。認証は本人が直接入力し、PAT をログや会話に出さない。現時点で正常起動・表示・操作・通常終了は未検証。

## 失敗

`main.rs` の広い `rg` 検索は原出力 2814 tokens が内側上限 2500 を超え切り詰められた（終了 0、完全ログなし）。失われた部分は未確認。別担当 `/root/output_failure` に連絡し、以後は検索対象・件数を限定した。共有失敗記録と教訓の更新は親が指定した担当が行う。
