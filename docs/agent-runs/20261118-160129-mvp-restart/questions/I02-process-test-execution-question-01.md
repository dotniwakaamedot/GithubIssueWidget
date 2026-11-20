# I02 無害プロセス試験 timeout 待機 01

- 担当 `/root/build_verification`、宛先 `/root`・`/root/restart_planner`。依頼 `tasks/I02/process-test-execution-request-01.md` SHA256 `F2AF7E3D6027C692288C0E40660FC98EBF100E1AC5A3805A4B04199E11359966`。開始前の固定参照8件は全件 SHA 一致、scratch の親・所有・出力32候補の不存在と同一試験のプロセス非競合を確認。
- `normal` は実子 PID 30424、exit 0、stdout/stderr EOF true、各8 bytesで期待全文一致、外側 driver exit 0。`nonzero` は PID 40820、意図した実子 exit 7、EOF true、各8 bytesで期待全文一致、外側 driver exit 0。`flood` は PID 57816、実子 exit 0、EOF true、各524,296 bytesで期待全文一致、外側 driver exit 0。各 JSON とログを新規所有領域に保存し、独立した読戻しで確認。別ストリーム間の発生順序は未証明。
- `timeout` は同一 Process handle を保持する driver セッション `61171` で開始。子 PID 52720、開始 2026-11-18T20:10:42.9599350Z、500 ms timeout 観測 20:10:43.4844428Z。`timeout.timeout.json` は exitCode null、streamsComplete false、SHA256 `3702CFA137B0027D23FAD2742DDF9D4AEBD03DDA4D2EB83E68E1F87987F9539D`。観測時に Node PID 52720 は生存中。driver は管理判断待ちの画面を出力し、同一 session 61171 が生存。入力が読み取れるかは実際には未確認。
- 120秒自然終了後の子の状態を同じ PID と handle で照合し、自然終了が確認できれば改めて質問する。それまでは `continue`・停止・再起動を行わない。timeout は現時点で未完了。元 build の成功は証明しない。
- 今回のケース実行・確認中の失敗なし。元製品・旧 scratch・GitHub・外部ネットワーク・GUI・拒否された window metadata 公開操作は使用・変更していない。
