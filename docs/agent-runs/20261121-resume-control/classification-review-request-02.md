# Closed 優先の確認依頼 02

- 確認担当 verification_reviewer `/root/classification_check`。共有 run `D:\Projects\GithubIssueWidget\docs\agent-runs\20261121-resume-control`。
- ユーザーへ「Closed の Issue に `status:todo` や `status:in-progress` が付いている場合、どの列に表示しますか？ Issue の文面に両方の解釈があり、現在のアプリは Closed を優先して Done に表示します。」と質問し、選択肢「Closed を優先して Done に表示（推奨）」「ステータスラベルを優先して表示」を提示した。回答は「推奨通りにしてください。」。記録 `classification-decision-01.md` を参照。
- 対象 `widget/src-tauri/src/main.rs` SHA256 `A3BAA6E82701F6D80DADBB688C22F0C54A466B358647F3D285EF9F4FF4100B56`、`README.md` SHA256 `54D113C712BA0B4B8C062172086EDFC2A27195C1F4BE9E6DF6064F1A73647D1C`。差分は README とテストで、製品分類実装は既存の Closed 優先に戻した。親の `cargo test --locked --offline board_classification_prioritizes_status_labels_and_excludes_pull_requests` は終了コード0、1 passed。ログ `closed-priority-test.log` SHA256 `139FC8E196B2015D5F0E63EADB3ACBFB6DE6C8E546187BF0CB1C7BD6B0AEF500`。
- 前回 B01 とユーザー判断の整合、README とテストの内容、移動に関する残りを独立照合する。ファイルや外部状態を変更しない。必要な資料は分けて取得し、出力量を制限する。他者の変更を戻さない。結果を `classification-review-02.md` に確定し、パスだけ通知する。
