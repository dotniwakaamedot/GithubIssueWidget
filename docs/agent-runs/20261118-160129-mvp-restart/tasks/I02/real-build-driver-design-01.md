# I02 実ビルド用 driver・専用コピー設計 01

- 担当 `/root/build_verification`。依頼 `tasks/I02/real-build-driver-request-01.md` SHA256 `EECFAD527EEA925C001B9A64BA859869648CE7248DAEF30A4E241FAFCC11E99E` の固定参照9件は実在・実体 SHA 一致。無害試験 `process-test-result-01.md` は別担当が対象版を承認したが、以前の実 build の待機原因や exit code は未特定。
- 結論: **実行前の設計と新規 driver を準備した。実 build・依存導入・外部取得・driver の実行は今回一切行っていない。** 対象版の独立レビュー・親判断まで実行しない。GUI・port・旧 build scratch・過去に拒否された window metadata 公開操作は使用していない。
- 元対象 `D:\Projects\GithubIssueWidget-issue-2-20261118-160129`、branch `task/issue-2-foundation-20261118-160129`、HEAD `a4ac97286785a2a5ba0d070a909b58003fc49ff1`、README 変更と widget 未追跡16件。HEAD だけでは版を識別しない。今回、結果 `result-02.md` の製品17件に対応する機械可読 manifest の実体 hash を確認し、全パス存在・通常ファイル・hardlink 各1件・hash 一致を確認した。主要親ディレクトリの reparse なしは前回検査済みであり実行直前の再確認が必要。

## 固定入力と作成物

新規所有 scratch `D:\Projects\GithubIssueWidget-build-20261119-02` は作成前に未存在、親 `D:\Projects` は通常ディレクトリ・owner `HOME_PC01\semis`、空き約126 GB、scratch を指す既知 process なしを確認。未存在の `source/` に17ファイルを通常バイトコピーし、元とコピーの各 hash・サイズを再照合した。コピー済み source は実行直前と各 stage 前後にも再照合する。前後照合は途中の一時的な変更の不存在や他環境の排他を証明しない。旧 build scratch、元の node_modules / dist / target / gen は変更・削除していない。

| 対象（scratch から相対） | SHA256 | 役割 |
| --- | --- | --- |
| `source-manifest.json` | `BB5253F1D02C83E183C86A6981CA3768A01892E3D1B40EF3AC4AA6A014DE0033` | 元結果 `result-02.md` の17件の相対パス・hash を分離した既存 manifest の通常コピー |
| `copy-manifest.json` | `BCE67077C4CB323586F52E527318B6584B3D48BBC956E879B75774ED6F5482E0` | 元とコピーの17件・hash・サイズ・作成時刻 |
| `build-driver-02.ps1` | `6B0E8CEB7C86C52E06982A8FB83A024E34EEE18C226B285319777F68608EC078` | 今回の独立レビュー対象。PowerShell Parser の構文エラー0、**実行未検証** |

driver の未提出 provisional `build-driver-01.ps1` は同一内容として保持する。対象は必ず `build-driver-02.ps1` とし、provisional を実行しない。コピー先・manifest・driver の再解析点や hardlink の再確認、および未追跡含む全対象 SHA と他担当の停止・競合確認を親判断と実行直前の前提にする。

利用する既存 executable / script の観測 hash: Node 26.13.1 `C:\Program Files\nodejs\node.exe` `F5109DC868EC38703AD8F305E0ACEECE717CB60D4889873F434017AF4E96CCCB`、npm-cli.js `3CE7CBA6F5128DD5F54C98B6A5036B0F850496878CC2E21044B675FE3C594E3E`、rustup / cargo / rustc proxy 各 `193D6C727E18734EDBF7303180657E96E9D5A08432002B4E6C5BBE77C60CB3E8`、VS18 `VsDevCmd.bat` `A7815E85D636828E5CF70FE4235B11FCB41750896A4D2D60756B43BACAF4F0B2`、Windows `cmd.exe` `CCD79A7568EA9240CCD6FE124807B9D3E4DBC3A4CF92954299AC3029F4A91D02`。更新があれば実行せず版を再固定する。`Process.WaitForExit(int)` / `Stream.CopyToAsync(Stream)` / `FileStream.Flush(bool)` の API 存在は PowerShell 5.1 / .NET Framework 4.0.30319.42000 で reflection 確認、実動作は未実施。

## 実行案（未承認・未実行）

専用コピーを cwd とし、承認後に各 stage を別の所有セッションで順に開始する。各 stage の成功、保存済みログと EOF、実プロセス exit、入力17 hash と出力の一致を確認してから次の stage を検討する。標準入力を対象子へ閉じ、UI・port を使う dev は対象外。

```powershell
& 'C:\Windows\System32\WindowsPowerShell\v1.0\powershell.exe' -NoProfile -File 'D:\Projects\GithubIssueWidget-build-20261119-02\build-driver-02.ps1' rustup
& 'C:\Windows\System32\WindowsPowerShell\v1.0\powershell.exe' -NoProfile -File 'D:\Projects\GithubIssueWidget-build-20261119-02\build-driver-02.ps1' npm
& 'C:\Windows\System32\WindowsPowerShell\v1.0\powershell.exe' -NoProfile -File 'D:\Projects\GithubIssueWidget-build-20261119-02\build-driver-02.ps1' release
```

| stage | .NET Process が直接起動する対象・引数 | 実行 cwd / 出力・通信 |
| --- | --- | --- |
| rustup | 固定 rustup.exe `toolchain install 1.90.0 --profile minimal --no-self-update` | scratch。専用 RUSTUP_HOME / CARGO_HOME、公開 `static.rust-lang.org` 等への取得。終了後 rustc/cargo 実体 hash を記録 |
| npm | 固定 Node から固定 npm-cli.js `ci --registry=https://registry.npmjs.org/ --no-audit --no-fund` | `source\widget`。コピー側 node_modules・専用 npm cache・空 npm 設定。esbuild の install script は npm 設定次第で動作が変わるため実際の警告を記録 |
| release | 固定 Node / npm-cli.js `run tauri -- build --no-bundle -- --locked` | `source\widget`。copy の dist・gen、専用 CARGO_TARGET_DIR・Cargo registry。公開 crates.io / index とその配布先への通信が生じ得る |

release では .NET Process の `cmd.exe /d /s /c` で固定 VS18 `VsDevCmd.bat -arch=x64` を呼び、`set` の結果をメモリ内だけで読み、成功した環境を本 stage に引継ぐ。bootstrap の PID・開始/終了・exit・stdout/stderr EOF は結果へ記録するが環境値そのものは記録しない。`VSCMD_SKIP_SENDTELEMETRY=1` は意図的に設定するが外部通信・OS 由来の副作用が完全にない保証ではない。

各子には環境を Clear し、SystemRoot、ComSpec、既知 PATH、ProgramFiles 系、専用 TEMP/TMP・HOME/USERPROFILE・APPDATA/LOCALAPPDATA・CARGO_HOME・RUSTUP_HOME・CARGO_TARGET_DIR・npm cache と空の user/global config、固定 toolchain と CI 等だけを渡す。環境変数は OS 権限の境界ではなく、ツール・build script が他所を読む可能性までは否定できない。既存 credential / token は意図的に渡さず、専用の公開レジストリ取得と lockfile を使う。親フォルダの `.npmrc`・`.cargo/config*`、専用 Cargo config が存在すれば停止する。npm の内蔵設定や各依存の間接処理の全副作用は未証明。

## 終了・証跡・timeout

- stage ごとに .NET Process handle の PID、開始 UTC、実プロセス終了後の ExitCode と ExitTime を取得。stdout/stderr の BaseStream を別々の新規 `.tmp` FileStream に `CopyToAsync` で並行消費し、2 task の EOF・エラーを別に確認して flush/close 後に非置換 rename。終了コードと各 log SHA、元・コピー17件の stage 前後 hash、exit 0 なら release exe SHA を `<stage>.result.json` 等に対応付ける。script は標準入力を子へ閉じる。出力に機密が混入した場合は公開せず保護して親へ報告。
- 前回の無害試験は `ReadToEndAsync` の別方式であり、今回の `BaseStream.CopyToAsync(FileStream)`、実 npm/Cargo の出力量・子孫・環境 bootstrap を証明しない。直接起動される Node/npm-cli.js 自身の終了コードのみを確定し、npm→shell→Tauri→Cargo の個別 exit code は独立には取れない。npm が子の失敗を伝播するかはログ・別途検証が必要で、個別コードを推測しない。`VsDevCmd` 内の子孫も別途未観測。
- rustup / npm 10分、release 30分の待機上限、または各ログが100 MiBを超えた場合、同一 handle を保持し `<stage>.timeout.json` に理由と exit null・EOF 状況を記録して管理判断を待つ。process は実行継続し得るため、容量上限は強制停止ではなく**観測・相談の閾値**であり、待機中も増える可能性がある。終了後も EOF が10秒以内に得られない場合は別の `eof-wait.json` と同一 handle で待つ。stdout/stderr task が fault した場合はログの完全性を成功扱いしない。
- timeout 後の `continue` は親判断後かつ process 終了・両 task 完了を同じ handle で確認した場合に限り受理する。標準入力が閉じた場合は待機を維持し、所有者・範囲を確認するまで停止・再起動しない。予期しない書込み、環境/対象の不一致、子孫・所有・入力・起動成否不明も該当作業を停止して親へ質問。
- 実行直前に保護対象との path/リンク非重複、空き容量・メモリ、ネットワークと必要な依存、CPU・画面担当や他プロセスとの競合、出力名・`.tmp` の不存在、対象 exe の hash と構成を担当者が確認する。driver は D: 20 GiB・物理空き2 GiB 未満なら起動前に拒否するが、この値は性能・安定性の保証ではない。通常 Windows のログ・システム cache、ネットワーク、CPU・メモリ・ディスクを共有する残余リスクがある。

## 未確認と次のゲート

コードの静的 parse は成功したが、実際の PowerShell 実行、VsDevCmd の固定引数、FileStream の非同期 EOF、依存取得と出口伝播、ログの機密性、実ビルドと exe は未検証。4件の無害試験が実際のビルド経路の成功を保証するものではない。独立 reviewer は対象版・副作用・権限・完全な stage 条件・timeout 停止の妥当性を確認し、問題があれば実行しない。

dev 起動・正常終了、最前面・移動・サイズ・Opener は本 driver の範囲外で Issue #2 の完了条件として残る。今回の作業時のコマンド失敗なし。以前の lockfile 調査2回、書込み公開逸脱2回など既知履歴は保持し、ユーザー指示どおり同種2回で該当操作中断と別担当の客観調査、3回で全停止。
