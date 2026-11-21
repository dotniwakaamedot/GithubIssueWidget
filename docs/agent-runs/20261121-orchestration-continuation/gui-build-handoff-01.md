# GUI release ビルド領域の所有移管情報

担当 `/root/gui_failure_investigation`。依頼 `gui-build-handoff-request-01.md`。ビルド用セッション `66951` は終了し直接 cmd 0、詳細は `gui-release-build-result-01.md`。本担当は `gui-release-build-01/` の書込みを停止済み。保持中の別 GUI 観測対象は既に同一ハンドルで直接終了を確認済み（`gui-release-observation-result-01.md`）；未追跡子孫は不明。

- 旧 input manifest `gui-release-build-01/input-manifest.json` SHA256 `BE90B991F9B5B4B1460CED30ABE968E59A7E9C851B6B44B19DB7A0B27D6CE89A`。対象 `widget/src/main.tsx` は `6FCF2A34145450C5029B7FF80DDB3EC1DA5E27173AAB84E57F16D6D54C0E6A3E`、`widget/src-tauri/src/main.rs` は `61595B68317E15FDD8F2BED88B920A468D5516740E3D3124710384709C52FC24`。
- `cargo-target/release/github-issue-widget.exe` SHA256 `23C383EB2E46CB205E052A9D4A24EE73B968EB2C776F7150826350685419B306`、`build-output.log` SHA256 `82E7D088500E604F157214A789A3404B24927349A932D1E470DAD53F425FC55E`、`build-status.txt` は `exit_code=0`。いずれも今回読取りで一致、**上書き・削除せず保持する**。target や widget の既存入力は前の成果のため、移管時はバックアップ・競合保護・旧 EXE の別場所保管を先に具体化する。
- 専用ルート ACL は build ログ保護のため元担当ユーザー・SYSTEM・Administrators に限定しており、別担当はアクセス拒否になる場合がある。共有可能な製品 EXE のコピー `gui-release-observation-01/github-issue-widget.exe` は上記同一 SHA。ログや専用 cache を広く共有する前に内容とアクセス権を管理が判断する。個人情報・ログ本文をチャットへ出さない。
- `cargo-home`・`cargo-target` は専用、`CARGO_NET_OFFLINE=true` で前ビルド使用。前回の直接 cmd は終了したが、未追跡子孫の不在と後続の他者書込みは保証しない。共有 Rustup/MSVC は環境のツールであり所有対象外。現 checkout のテスト入力は実行後に1件変わっていた（同結果参照）。

本結果は読取り時点の証拠であり、再利用の認可ではない。親が正式に移管し、同時作業・保護を再確認するまで別担当は同領域に書き込まない。本結果作成以外の変更・削除・実行はない。`Get-Content`・`Get-FileHash` と manifest の限定読取りは終了0。
