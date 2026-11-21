# 移動競合3ケースの限定実行準備依頼

- 担当 `/root/existing_work_review`。共有 run `D:/Projects/GithubIssueWidget/docs/agent-runs/20261121-orchestration-continuation/`。所有は新規 `movement-rust-test-execution-01/` と結果 `movement-rust-execution-result-01.md`。製品・既存 target・他担当のファイルを変更しない。
- `movement-rust-review-02.md` の承認版 `movement-rust-isolated-02/movement.rs` SHA `88BF21BB45C077DFDA592D906C753D168C5641E35EB12324D4ACF1652374695B` と製品 `movement.rs` `ED49F4BA52C6887BCE427E4A6664D335092A1C3311E1B1ED61B4CE322791ABA5`、main `61595B68317E15FDD8F2BED88B920A468D5516740E3D3124710384709C52FC24` を開始前に照合。不一致なら中断。
- `pagination-test-execution-01/` の限定 Rust 実行環境は現在別担当が使用中のため触らない。親から実行完了・書込み停止の確定通知を受けるまでは読み取り検討のみ。終了後に再利用可能と確認できれば、既存成果を変更せず、固定依存や必要な build 出力を新しい専用領域へコピーする方法を検討・固定。競合・書込みの不明や予期しない版差があれば共有環境を変更しない。新規の専用 Cargo プロジェクト・target を使う選択も可能。
- 追加3ケース `movement::tests::post_write_competing_` のみを正確に選択し、3件・各成否・直接終了コード・専用ログと有限時間を記録する。既存全試験を無理由に再実行しない。実行前に対象・コマンド・環境・依存・出力・ループバックの終了・秘密非継承を固定し、必要な独立確認と管理通知を受ける。GitHub・credential・GUI は操作しない。
- 現段階は実行可能な準備と実行条件の確認まで。結果・質問は固有ファイルへ確定しパスを通知。未確認子孫やコンパイル未実施を成功と扱わない。
