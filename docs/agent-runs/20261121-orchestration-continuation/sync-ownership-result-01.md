# Issue #4 同期修正の所有権照合

調査担当 `/root/existing_work_review`。読取りのみで製品・既存ファイルの変更、テスト、外部書込みはしていない。依頼 `sync-ownership-request-01.md` に対する結果。

## 判断

**実装への着手可否は未確定。** 指定3ファイルは現在の checkout では HEAD に対する差分がなく、旧 Issue #4 worktree は状態一覧上クリーンだが、旧担当の書込み停止と関連する未完了 PR の網羅的照合を証明できなかった。依頼条件に従いこの3ファイルへの書込みを保留し、管理が旧担当の停止・移管と PR を確認してから作業場所・基準・対象 SHA を確定する。静的調査・計画・安全な検証準備は続行可能。

## 現在の対象版

- `widget/src/main.tsx`: SHA256 `01A7E8A6961BE2DC4F00D96F9369098321EF5137F6959650931E36ABFB357658`。
- `widget/src/main.test.tsx`: `6D1F45AFDBFC3BD66071C81FD7BA0025A186BD6A1CC0CE89022D06FB19B7D1E1`。
- `README.md`: `54D113C712BA0B4B8C062172086EDFC2A27195C1F4BE9E6DF6064F1A73647D1C`。`widget/README.md` は存在しない。
- `git diff --quiet HEAD --` の3ファイル照合は終了コード0、限定した `git status --short --untracked-files=all` は変更表示なし。ただしユーザー ignore 読取りのアクセス拒否警告あり。現 checkout は `main` HEAD `5e74e1642fd18fe630dce0e684afd853721276ae`。範囲外の未コミット・未追跡変更は保護する。
- 先行の `frontend-resume-result-01.md` は前2ファイルの同じ SHA で React モック試験33件成功と記録するが、今回再実行はしていない。

## 旧担当・Issue・worktree

- GitHub コネクターの Issue #4 直接取得は成功。open、assignees は null、`status:in-progress` ラベルあり。本文は調査・詳細化中と記載。コメント1件は 2026-11-19 時点の `/root` と `dotniwakaamedot/issue-4-board-resume` の開始予定を記録し、終了や移管を示さない。ラベル・担当欄は排他ロックではない。
- `git worktree list --porcelain` は旧 worktree `C:/Users/semis/orca/workspaces/GithubIssueWidget/issue-4-board-resume`、HEAD `3d906e5926f3106ce12b2f3bb3bb878a64368f64` を示す。旧 commit は現 HEAD の祖先ではない（`git merge-base --is-ancestor` 終了コード1）。独立 worktree は他にも存在し、いずれも削除・変更していない。
- 旧 worktree の `git status` は初回 dubious ownership で失敗。設定を永続変更せず当該呼出しだけ `-c safe.directory=...` を指定した読取りは終了コード0、ブランチ以外の変更表示なし、ignore 読取り警告あり。これは現在稼働する書込み担当の不存在・移管の証拠ではない。現チーム一覧に旧 run の担当は現れないが、別セッションを排除できない。

## PR 照合と未確認

コネクターの `is:pr is:open` 検索は通常 Issue #1〜#10 を返し、期待した PR 検索結果と判断できない。公開 pulls API の読取りは `WebException`（終了コード1）で HTTP 状態・原因不明。再試行しておらず、未完了 PR の有無は不明。失敗した照会を不在の証拠にせず、管理は別の認証済み手段または安全な診断後に照合する。GitHub・ローカル双方は同時更新され得るため着手前に再確認する。

## 操作失敗

旧 worktree の dubious ownership、公開 API の WebException、および PR 検索結果の条件不整合を管理へ引き渡す。原因が異なるため同一障害と断定しない。各結果のログへの記録・既存項目との照合を管理に依頼する。
