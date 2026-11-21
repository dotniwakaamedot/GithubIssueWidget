# release GUI 限定観測の実行判断

`gui-release-observation-review-01.md` は固定 EXE SHA `23C383EB2E46CB205E052A9D4A24EE73B968EB2C776F7150826350685419B306`、観測器 SHA `F82AD7CADEC4F94B1E7832A32A701C0CE59CEE9E713AAD1833834FC839F48CF7` の空設定観測方式を承認。`/root/gui_failure_investigation` は実行直前に5領域と親の実体・再解析点・空・ACL・競合、対象版と直接プロセス/ウィンドウの識別方法を確認し、条件を満たす場合に一度のみ `gui-release-observation-request-01.md` と preflight の限定観測を実施する。不明・不一致は実行せず記録。

設定入力・実 PAT・GitHub 操作はしない。PID のみで他窓に触れず、観測できた対象だけを通常閉鎖し、同じ保持ハンドルで直接終了を確認。未知の子孫や旧プロセスの終了は推定せず、不明時は領域を保護し再起動しない。過去の101の詳細は破棄され、今回の DEVNULL 観測でも原因は得られない。結果と対象版を確定し通知する。
