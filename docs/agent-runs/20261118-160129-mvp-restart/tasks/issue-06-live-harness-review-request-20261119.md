# Issue #6 実 GitHub 移動テスト補助コードの独立レビュー依頼

ユーザーは `dotniwakaamedot/orcaDiversion` でテスト Issue 作成・状態変更を許可。実作成済み #13 と一致識別子・状態を `tasks/issue-05-live-result-20261119.md` で確認し、既存 Issue は変更しない。トークンの値は表示・保存しない。

対象: `C:/Users/semis/orca/workspaces/GithubIssueWidget/issue-6-move-resume` の承認済み HEAD `ab6122e` に対する `widget/src-tauri/src/movement.rs` の ignored テスト追加のみ。同ファイル SHA256 `D7C987DB85F69482EF005FB95D9DA4C85DA5CBAACC2C1207B6A5F26EE5496E9F`。`cargo test --locked --no-run` 成功、移動の実書込みは未実施。

対象固定、テスト Issue とマーカーの照合、各6方向の前後状態、失敗時の停止と再送防止、秘密情報保護、通常テスト実行時の書込み防止、実行後の読み戻し・環境復元についてレビューする。実 GitHub に書き込まず、コードも編集しない。対象ファイルは存在確認してから参照。判定を `tasks/issue-06-live-harness-review-20261119.md` に記録して親へ通知する。
