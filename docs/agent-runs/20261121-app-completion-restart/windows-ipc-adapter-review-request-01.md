# Windows IPC 隔離経路の独立レビュー依頼

- 担当予定 `/root/windows_ipc_adapter_review`。親 `/root` が ID 確認後に起動。作者 `/root/windows_ipc_adapter` と別。共有 run `D:/Projects/GithubIssueWidget/docs/agent-runs/20261121-app-completion-restart/`、作業場所 `D:/Projects/GithubIssueWidget`。
- 対象結果 `windows-ipc-adapter-result-01.md` SHA256 `2891DEAF7F9CE658CDFF23A66C306552A7F37D145CAEF2C838DB524A278E9976`、`windows-ipc-adapter-01/main.rs` `F10F86CCC567801D197B92222CE478F0D98F8CC0C4575A9DA0E9CFA8D4DED61E`、`adapter.py` `0A350F40863D3E80FA621195EAB8143E404026C594BF802CB382971EA6F421B0`、`test_adapter.py` `A3682FD85B4A59DFE6FEF24F00712D63C0A758B46B37D2F8277CF00E9FB8DCA5`。実体と SHA を照合し、異なる場合は停止・質問。
- 元の依頼・本番 `main.rs`・コピー差分・10模擬試験・ログ・終了状態を照合し、合成識別子 journal と所有権、監督、終了不明・cleanup の安全境界、および実 OS 禁止ゲートの有無を実コードに基づき確認。Python 模型が Rust guard を呼ばず、Rust 未コンパイルである制限を明示し、改善が必要な具体的阻害事項と、実行経路接続の安全な次作業を区別する。実 OS 安全性や製品完成を推測して承認しない。
- 読取りのみ。実 Credential Manager、実 PAT、GitHub、アプリ、製品、他者素材を変更しない。所有は新規 `windows-ipc-adapter-review-01.md` のみ。対象・出力を限定し同種失敗2回は停止・報告。共有環境の他者変更を戻さず UTF-8 読戻し後に一時ファイルから確定しパスのみ通知。
