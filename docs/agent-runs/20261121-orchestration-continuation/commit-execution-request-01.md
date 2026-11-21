# ユーザー承認済みのコミット・プッシュ依頼

- 担当 `/root/commit_prepare`。作業場所 `D:/Projects/GithubIssueWidget`。ユーザーは「一旦ここで作業をコミット＆プッシュして次のセッションに引き継ぎたいです」と明示した。親は下記11件をコミット対象に確定。作業前に `commit-prepare-result-01.md` と最新の独立レビューを確認する。
- 対象は `README.md`、`widget/src/main.tsx`、`widget/src/main.test.tsx`、`widget/src-tauri/src/main.rs`、`widget/src-tauri/src/movement.rs`、`widget/src-tauri/src/pagination_tests.rs`、`docs/handoff-20261121-current.md`、`docs/agent-runs/20261121-goal-continuation/issue-enumeration.py`、`docs/agent-runs/20261121-goal-continuation/test_issue_enumeration.py`、`docs/failure-log-001.md`、`docs/recurring-lessons.md`。main の module 宣言と pagination test を一緒に含める。
- `.pyc`、`widget/release-20261121/` と `widget/release-20261121-02/` の EXE、未追跡 agent run 全体・fixture・ログ・他のファイルは stage せず、削除・復元・上書きしない。本人が起動したアプリ、credential、実 Issue は操作しない。実 Issue 作成は未実施の担当報告を超えて主張しない。
- 着手前に現行 `main` と HEAD・`origin/main`・remote の既定/現在状態を安全に再確認し、先行の stage が空であること、各対象が確定版・所有停止であること、特に引継ぎに記録/教訓の未追跡資料への参照が共有先で欠ける制約が明示されていることを確認。不一致、他者の stage・未知の変更、秘密や権限不明があればコミットせず親に連絡。トークン・非公開情報・ログ本文は出力しない。
- この sandbox は `.git` を read-only としており、index・commit 更新には利用可能な `require_escalated` 経路を使う。正確な個別11パスだけ `git add --` し、`git diff --cached --name-only`・差分・機密の限定確認・`git diff --cached --check` と対象の SHA を照合。対象外が混じる、権限拒否、出力不明は続行せず、他者の stage を勝手に取り消さない。`git add .`・`git commit -a` は使わない。
- 検査が成功した場合のみ適切なメッセージでコミットし、同じ `origin/main` が最新 commit の親に対し競合なく進められることを確認して通常の push を行う。force push・remote 履歴書換え・他ブランチ変更はしない。push 失敗/応答不明は remote の実状態を照合し、成功と誤認・盲目的再送しない。操作の実行結果・commit SHA・remote 反映・残る未コミット/未追跡を区別し、秘密を含まない `commit-execution-result-01.md` に記録する。
- 利用可能な承認経路が拒否・不明であれば無関係な作業を保護し、具体的操作と理由を親へ報告する。同じ失敗が2回あれば別担当に調査を依頼する。不要なテストの再実行やユーザー利用中アプリの停止はしない。
