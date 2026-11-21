# GUI 診断版の製品入力変更

担当 `/root/gui_failure_investigation`。承認された診断 `main.rs` は旧製品 SHA `61595B68317E15FDD8F2BED88B920A468D5516740E3D3124710384709C52FC24` に基づくが、現在の製品 `widget/src-tauri/src/main.rs` SHA は `AEFF1E65AD93EFA04773CEBBB9736A6BDC5B9938DB9FAF937DD88B3D9CBFE86F` に変化した。`main.tsx` は引続き旧入力 `6FCF2A34145450C5029B7FF80DDB3EC1DA5E27173AAB84E57F16D6D54C0E6A3E` と一致。別担当の作業を保護し、現在の変更を診断版へ反映した新しい版の用意・レビューか、旧観測版の原因切分けとして旧版に限定して実施するか管理が判断するまで、既存入力・target を変更しない。

旧ビルド領域内の入力を `git diff --no-index` で参照したところプロセスからアクセスできず終了1。PowerShell の SHA 読取りは成功した。固定版の差分内容と後続の診断版に与える影響は未確認。同じ失敗を繰り返さず、管理に記録を依頼する。
