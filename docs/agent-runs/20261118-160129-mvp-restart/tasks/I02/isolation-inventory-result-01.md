# I02 隔離環境・Windows 実機の読み取り調査 01

- run / task / owner: 20261118-160129-mvp-restart / I02-isolation-inventory / `/root/isolation_inventory`。依頼は `tasks/I02/isolation-inventory-request-01.md`。
- 結論: Windows 11 Home の現在の環境と開発ツール・既存依存の存在は検出したが、承認済み設計が求める専用 Windows VM の存在・起動・利用権限・完全な隔離は確認できない。現在の Windows がユーザー操作可能な物理実機かも未検証。代替案は以下の静的検討であり実施・承認していない。
- 対象 worktree `D:\Projects\GithubIssueWidget-issue-2-20261118-160129`、branch `task/issue-2-foundation-20261118-160129`、HEAD `a4ac97286785a2a5ba0d070a909b58003fc49ff1`。README 変更・widget 未追跡16件、削除なし。`result-02.md` の製品一覧のみから基点と相対パス・64桁 SHA を分けた17ファイルの存在を全件確認し、その後ハッシュを照合、全件一致。
- 依頼で指定された7参照を、共有 run の絶対基点で全件の存在と SHA256 を照合し一致。実体は依頼と同一。調査時刻は 2026-11-18 UTC、cwd は `D:\Projects\GithubIssueWidget`。

## 確認された事実

| 対象 | 読取結果と意味 |
| --- | --- |
| Windows | `Get-CimInstance Win32_OperatingSystem` は Windows 11 Home、10.0.26300、64-bit。`Win32_ComputerSystem` は Inversenet Inc. / FRONTIER、HypervisorPresent=True、`Win32_ComputerSystemProduct` は同 Vendor / Name。モデル名・hypervisor 存在だけでは物理実機、専用 VM、または独立性を証明できない。 |
| 対話環境 | 現在の PowerShell プロセス SessionId=2、`[Environment]::UserInteractive=True`。実際のデスクトップへの入力可否、ユーザーの操作と競合しないこと、ウィンドウ測定の可否は未確認。UI 操作はしていない。 |
| VM 管理手段 | `Get-Command` で Get-VM、VBoxManage、vmrun は未検出、wsl.exe は存在。既定の VBoxManage / vmrun の想定ファイルも存在せず。インストール済みプログラムの表示名に VirtualBox / VMware は検出されなかった。`vmcompute` サービスは Running。これは専用 Windows VM やその構成が利用可能という証拠ではない。HKCU Lxss 登録の子キーは0件だが、他ユーザー・別方式・非標準配置の VM は未調査。WSL を起動・照会していない。 |
| 開発ツール | `Get-Command` で node、npm、rustc、cargo を検出。Visual Studio Community 2022 17.14.38 と 2026 18.9.0、WebView2 Runtime 157.0.4341.60 のインストール登録を確認。Visual Studio Installer の vswhere.exe は所定パスに存在。実コンパイラ・SDK の完全性やこの調査での起動、ライセンス・権限、VM への持込み可否は未検証。 |
| 既存依存・生成物 | 既存 worktree の `widget/node_modules` と `widget/src-tauri/target`、既存ユーザーの Cargo registry cache / src、rustup の 1.90.0 toolchain、npm `_cacache` は存在。内容の完全性、オフライン再現性、出所と移送権限は未確認。存在だけで offline install の成功を意味しない。削除・利用・変更していない。 |
| 容量 | ローカルディスクの観測値は C: 空き約 1.20 TB、D: 約 138 GB、E: 約 611 GB（10進バイト、調査時点）。利用権限や必要容量の保証ではない。 |
| 対象プロセス | `Get-Process` の名前照合では対象アプリ、VirtualBoxVM、vmware-vmx、vmwp は検出されず、msedgewebview2 は19プロセス検出。既存 WebView2 の所有先は未特定のため触れていない。別名プロセスや他環境の不存在は証明しない。 |

## 専用ディレクトリ方式の静的評価

- 版固定したソースの実体コピーを別ディレクトリに置き、Vite の dist と Tauri の gen をコピー内に限定し、`CARGO_TARGET_DIR`、`TEMP` / `TMP`、npm / Cargo / Rustup の各 cache・home を新規専用領域に設定すれば、元 worktree の通常の相対出力との衝突を減らせる。ただしリンク・junction・ハードリンクの混入、設定ファイルや build script の絶対パス、子プロセスの書込み先を確認しない限り元生成物の保護を保証できない。
- ファイル出力を分離しても、同じ Windows ユーザープロファイル、APPDATA / LOCALAPPDATA、レジストリ、Credential Manager、WebView2・ブラウザのプロファイル、MSVC/SDK・npm lifecycle scripts 等の子プロセス、OS 管理領域、ネットワーク、開発ポート、同一デスクトップは自動では隔離されない。環境変数による変更範囲は各ツールの実測と権限設定が必要であり、隔離を確定できない。
- より弱い方法を提案する場合は、専用アカウントやアクセス権・別プロファイル、元ディレクトリに書込みできないこと、再解析ポイントと想定外出力、各プロセスの書込み／registry／ネットワーク監視、依存の固定とオフライン可用性、保存場所・保持・復旧を具体的に設計し、独立レビューと親判断を経る必要がある。単一ホスト上では完全な副作用遮断を前提にしない。これを承認済み VM 設計の代替として自動採用しない。
- 独立した検証環境が用意できない場合でも、対象版と生成物を保護した静的な調査・手順詳細化は可能だが、それだけでは Windows 上の build、画面操作、IPC の許可・拒否の完了条件を満たさない。

## 実施記録・未実施

- `Get-CimInstance Win32_OperatingSystem` / `Win32_ComputerSystem` / `Win32_ComputerSystemProduct` / `Win32_LogicalDisk`、`Get-Command` による候補 CLI 発見、`Get-Service` と `Get-Process` の名前照合、アンインストール登録の `Get-ChildItem` / `Get-ItemProperty` 読取、候補パスとレジストリの `Test-Path` を実行。存在しない CLI は起動せず、VM・対象アプリ・WSL・通信試験・インストール・設定変更は行っていない。各コマンド群 exit code 0。
- 指定参照7ファイルと製品17ファイルの存在確認・SHA256 照合、`git -C <worktree> branch --show-current` / `rev-parse HEAD` / `status --short --untracked-files=all` を実施。各コマンド群 exit code 0。前後の状態照合は途中の一時変更の不存在を完全には証明しない。
- 今回の調査で失敗なし。前回のパス参照失敗・文字破損・移動失敗は既存履歴のとおりで再実行していない。生成物、製品、Git、GitHub を変更せず、削除対象なし。結果記録のみ新規作成。