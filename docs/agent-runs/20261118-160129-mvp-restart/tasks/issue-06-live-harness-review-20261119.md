# Issue #6 実 GitHub 移動テスト補助コードの独立レビュー

判定: **指定の運用条件付きで対象版を承認（blocking なし）**。実行やテスト成功を意味しない。作者 `/root` と確認担当 `/root/creation_review` は別担当。

対象 worktree `C:/Users/semis/orca/workspaces/GithubIssueWidget/issue-6-move-resume`、HEAD `ab6122ef26bd62d5ec8dffbda1d35cd1a09606e0`、`widget/src-tauri/src/movement.rs` の SHA256 は前後とも `D7C987DB85F69482EF005FB95D9DA4C85DA5CBAACC2C1207B6A5F26EE5496E9F` と一致。未コミット差分は同ファイルの ignored テスト追加のみ、削除・未追跡なし。

| 条件 | 確認結果 |
| --- | --- |
| 対象固定と照合 | URL は `dotniwakaamedot/orcaDiversion` の Issue #13 に固定。各操作前に番号、作成時の一意なタイトル、Issue であること、予想状態と status ラベルを確認する。#13 は作成結果記録にあるユーザー承認済みのテスト Issue。 |
| 6方向と読戻し | todo→in-progress、in-progress→todo、todo→done、done→in-progress、in-progress→done、done→todo の各前後で GET し state と status ラベルを照合。実処理は PATCH 応答のラベル集合も照合する。各状態が期待と異なれば直ちに停止する。 |
| 失敗時と再送 | timeout、リダイレクト拒否、クライアントの自動 retry 無効。操作失敗・不明・読戻し失敗では後続方向に進まない。新規の絶対パスのディレクトリ作成により同じパスでの再実行を防ぐ。 |
| 通常テストと秘密情報 | `#[ignore]`、確認文字列、トークン、状態ディレクトリの環境変数が必要。トークンは Authorization に渡され、結果・固定エラーに含める経路は確認した範囲で見当たらない。実行時の環境・ログは未確認。 |

**非 blocking の運用条件:** 別の新規ディレクトリを指定すると再実行できる。失敗・応答不明時は別ディレクトリでも自動再送せず、記録を保持して実 Issue を読戻し、状態と完了済みの方向を特定する。操作前の照合と PATCH は原子的ではなく、他の利用者による同時更新を完全には防げない。親はテスト対象と初期状態を実行直前に確認し、実行結果と各読戻しを記録し、トークン環境変数を復元する。

依頼・作成結果・対象の実体と差分を確認。作者報告の `cargo test --locked --no-run` は独立再実行・保存ログ確認をしていない。実 GitHub 書込み、テスト実行、資格情報操作は行っていない。実 UI・Fine-grained PAT・異常時の実動作は未検証。今回新たな失敗・教訓はない。
