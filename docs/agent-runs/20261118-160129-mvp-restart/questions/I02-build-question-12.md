# I02 B03 停止時の影響評価 12

- 担当 `/root/build_verification`、宛先 `/root`。セッション 85095 と実行中 PowerShell PID 25560 の終了は観測していない。`Start-Process -Wait` を使うスクリプトは結果 JSON をまだ書いていない。OS thread は Wait 状態を示すが、どのハンドルを待つかや job object の所属は未特定。祖先・子孫や専用 executable path の読み取りだけでは隠れた依存の不存在を証明できない。
- 強制停止すれば、PowerShell 自体の正常終了・`release-result.json`・事後17ファイル hash の自動記録は失われる。元の npm と Cargo の実際の終了コードは取得できず、不明のままとする。作成済み exe とビルドログは補助証拠であって exit 0 の代替ではない。OS の job object・他の子プロセスへの伝播や未完了の書き込みが残る可能性は排除できない。
- 停止前に PID と実行コマンド・所有・保管先を再照合し、必要なら stdout/stderr・事前 manifest・現時点の exe を読み取りハッシュして保持する。セッション出力と現存プロセスを観察し、停止後に exit code が不明であること・保管物の hash と中断時刻を別の確定記録へ記載する。専用 scratch・生成物・ログは削除しない。停止の許可と具体的な方法の判断は親が行う。
- 2026-11-18 UTC の読み取り hash: release.stdout.log `AC6DCB8667F41B0AD387F6FD3A6C7EA66FB295E1F7C0E9D030DF90F5DF1BB29F`、release.stderr.log `42F8BAFC1634223F690162B9CB904750F8505AD11CD19C7BD796B24D8C902119`、release-hashes-before.json `D747AA1C2D1BF9A2ACAC9FD0CE55712062B58B714EEFCE4B6F3E796D56CBEDF8`、専用 scratch の exe `27D94B7B32EC394F89080C5812D04671B7DA8D420E5B85F1D67615FEAE5D2F4C`。各 exit 0。読み取りは書込み進行がないことを完全には保証しない。
- 観測は実行中の同一 handle を用いており、タイムアウトだけで終了扱いしない。停止・再実行・削除は未実施。
