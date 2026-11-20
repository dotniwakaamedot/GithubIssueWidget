# I02 実ビルド用 driver 修正設計 02

- 担当 `/root/build_verification`。修正依頼 `tasks/I02/real-build-driver-revision-request-01.md` SHA256 `B95B676EF8051623B4A41F72B4639E7FD51C0BD73C69ABC660BA352F93863B64` の固定参照8件は実在・SHA 一致。前版 `real-build-driver-design-01.md` SHA256 `F4D173AAEF4964FB8C1B46B2A9BD4D71E16B4DDCD7EDAFB45651B79D6BA9E50D` と独立レビュー `real-build-driver-review-01.md` `1738E469B6F0D3D974F27CD4BA1E5784896AD911BAA171A8BB8F636AEBF8E2DF` を一次資料とする。
- **本版は未実行の修正案。** 旧版の B01/B02 に対応するコードを新規版に用意した。独立再レビューと親判断まで依存導入・実 build・driver 実行・GUI・外部取得は行わない。拒否された window metadata review の公開操作は再試行・回避していない。
- 修正版 `D:\Projects\GithubIssueWidget-build-20261119-02\build-driver-03.ps1` SHA256 `BA0269227A5239F5E7B9CA0EF2802D3A5D5C7678821E67C141CC4A1958927FA9`。旧 `build-driver-02.ps1` `6B0E8CEB7C86C52E06982A8FB83A024E34EEE18C226B285319777F68608EC078` を上書きせず新規 `.tmp` として修正、PowerShell parser tokens 4,749・errors 0、UTF-8 読戻しと hash 照合後に確定した。構文検査は runtime の正しさを証明しない。
- 前版の3 stage、17ファイル固定、専用 source / target / dist / cache / temp、ネットワーク・資源条件、対象版・OS の限界を維持する。実行案の driver 名だけ `build-driver-02.ps1` から `build-driver-03.ps1` に変更する。dev・ウィンドウ・Opener は範囲外。元のビルドの exit code と待機原因は依然不明。

## B01: 一時ログの保護と公開ゲート

1. 各 stage の実子開始前、新規 `<stage>.protected-logs/` を専用 scratch に作成。`DirectorySecurity` を用いて継承を無効にし、当該 Windows user SID と SYSTEM のみ FullControl とする ACL を作成・読み戻し、owner・rule・reparse 属性を確認する。確認不能なら子を起動しない。これは同一ユーザーの別プロセスや管理権限・バックアップ権限を持つ主体を遮断する保証ではない。
2. 実行中の stdout/stderr は同ディレクトリの `stdout.log.tmp` / `stderr.log.tmp` のみへ同時書込み。内容を標準出力・質問・共有 run・外部へ表示・送信せず、EOF と flush/close が確認されるまで確定名へ移動しない。VS bootstrap の stdout/stderr は必要な環境値をメモリ内で処理し、内容をファイルや console に出さない。VS エラー内容の詳細は保存しない制約が残る。
3. 両 stream の正常 EOF と close 後にそれぞれ SHA256 を取得し、UTF-8 strict のストリーミング走査で GitHub token 形式、Bearer、token/password/secret 等の表示パターンを確認。結果は `clear-patterns-only`・`detected`・`unknown` のみで、文字列・一致箇所を出力しない。`detected`、デコード・読取不能等の `unknown` は `Hold-Failure` へ移り、ログ非公開・同一 handle/証拠を保持して親へ知らせる。
4. `clear-patterns-only` は**秘密がない保証ではない**。コードは `<stage>.log-review-needed.json` に stage、PID、driver SHA、両一時ログのパス・hash・走査状態と receipt の予定パスだけを保存し停止。管理は別担当を割当て、非公開ログを必要最小限に確認させる。パターン外・誤検出・不明な文字列は検出できない可能性があるため、不明時は公開しない。reviewer は値・断片を質問や receipt へ転記しない。
5. 別担当の判断を管理が確認後、固定パス `docs/agent-runs/20261118-160129-mvp-restart/tasks/I02/real-build-<stage>-log-approval-01.json` に新規の hash 結び付け receipt を適用される公開手順で確定する。driver は次のフィールドを要求する: `stage`、`decision`（`approved-no-detected-sensitive`）、`reviewer`（作者以外の `/root/...`）、`reviewedUtc`（UTC ISO 形式）、`driverSha256`、`stdoutSha256`、`stderrSha256`。確認者 ID の実在・独立性と承認権限は管理が別途確認する。単なる文字列は認証ではない。
6. 親判断後の同一 session への `publish` 入力時に receipt の存在・reparse 属性・前後 hash、項目、対象 driver・両ログ hash の一致を再確認。両パターン走査が clear で receipt が一致した場合だけ、**保護ディレクトリ内**で tmp を確定名へ非置換移動し hash を再照合する。値不一致、未読取、別操作による変更、input 不明は `Hold-Failure`。同一ユーザーによる競合を完全に排除するものではない。

## B02: fault / cancel / 起動不明の保持

- `Get-TaskState` は task 未作成・pending・`RanToCompletion`・`Faulted`・`Canceled` を区別。通常の process 待機中は1秒ごと、timeout / EOF 待機中は250 ms ごとに各 task を監視する。`WaitForExit` の成功と stdout/stderr の `RanToCompletion` は別条件で、fault/cancel は EOF 成功と扱わない。VS bootstrap の `ReadToEndAsync` も同様に監視し、例外の詳細・環境値は出力しない。
- 実子開始、stdin の close、非同期 task の作成・wait・GetResult、ログの書込み・flush・close・hash、secret 走査、receipt・出力確定の例外は `Hold-Failure` へ移す。Start が false / 例外の場合は PID と状態を取得できれば記録し、取得できなければ `start-or-state-unknown`。同じ stage を自動再起動しない。
- `Hold-Failure` は stage・PID（取得可能時）・running/exited/unknown・分類・両 task 状態・exitCode null だけを `<stage>.failure.json` に新規記録するよう試み、記録自体が失敗した場合は内容を出さず console に不明と通知する。Process と存在する FileStream/task の参照を保ったまま待ち、管理へ所有・子孫・保存状況を質問する。fault 後に task が失われたり handle を取得できなかった場合は追跡不能を明示し、勝手に停止・再起動しない。異常時の詳細な OS 状態・所有範囲は管理側の独立確認が必要。
- timeout / EOF 待機中は `ReadLineAsync` により control input を待ちながら各 task の状態を監視し、input fault/cancel や stream fault/cancel は `Hold-Failure`。入力が閉じた場合も process handle を保持。親が対象の終了・所有を確認した後に `continue` を指示し、`HasExited` と両 task `RanToCompletion` がそろう場合のみ先へ進む。子の終了や EOF 不明時に code を推定しない。
- 保護ログの空き容量閾値・timeout の判定後も対象 process は動き続け得るため、管理が稼働状態とディスクを監視し、止める場合は所有・子孫・停止範囲を別途判断する。閾値は自動 kill や厳密な容量上限ではない。最終的な UI 操作や dev も本件に含めない。

## 静的検査・未確認

- `Get-FileHash -Algorithm SHA256` で修正依頼の8参照と元・コピーの17件を照合、全件一致。`git -C <元worktree> branch --show-current`・`rev-parse HEAD`・`status --short --untracked-files=all` exit 0。HEAD `a4ac97286785a2a5ba0d070a909b58003fc49ff1`、README 変更と widget 未追跡16件を保護。manifest `BB5253F1D02C83E183C86A6981CA3768A01892E3D1B40EF3AC4AA6A014DE0033`、copy manifest `BCE67077C4CB323586F52E527318B6584B3D48BBC956E879B75774ED6F5482E0`。
- PowerShell/.NET の読み取り reflection は ACL 付き `Directory.CreateDirectory` と `TextReader.ReadLineAsync` の overload 存在を確認。ACL 適用・検出器・receipt 検証・fault/cancel・stream の実動作は未実行。旧無害試験の `ReadToEndAsync` 成功はこの `CopyToAsync`・例外・保護ゲートを証明しない。
- 結果作成場所 `D:\Projects\GithubIssueWidget`、削除対象なし。今回の作成物は scratch の `build-driver-03.ps1` と本設計・質問。元製品、旧 scratch、既存証跡、GitHub を変更していない。同種2回で該当操作を止め別担当に客観調査、3回目は全停止。本修正作業のコマンド失敗はなし。次のゲートは独立レビューと親判断。
