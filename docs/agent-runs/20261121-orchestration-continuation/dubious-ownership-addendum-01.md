# dubious ownership 調査への追加事実

2026-11-21、担当 `/root/issue_survey`。`dubious-ownership-addendum-request-01.md` に基づき、確定済み `dubious-ownership-investigation-01.md` を変更せず、後から届いた `/root/project_survey` の直接観測報告を追記する。個人名、SID、不要な絶対パスを記載しない。

## 追加観測

- 同担当は Windows の旧 #4 worktree を対象に通常の `git -C <対象> status --short --branch` を実行し終了1。Git の診断には、表示された worktree 所有アカウントと実行アカウントが異なる旨が含まれたと報告した。報告上の実行アカウントはサンドボックス用である。
- 同担当は同じ対象を当該一回のみ `git -c safe.directory=<対象> -C <対象> status --short --branch` で読み取り、終了0・branch 表示のみ、ユーザー ignore へのアクセス警告ありと報告した。グローバル設定は変更していないとの報告。時点は 2026-11-21 の限定統合作業時であり、時刻は未取得。
- 別の当事者 `/root/existing_work_review` の `ownership-error-answer-01.md` は、既定 sandbox で同 worktree の `status --short --branch --untracked-files=all` が終了1、Git が `.git` 所有者と実行ユーザーの SID の差を報告したと記録。同じ untracked 指定を含む単回 `safe.directory` の読取りは終了0・branch のみ・ignore アクセス警告あり。実際の SID は記録せず、同資料に従いこれは Git の表示として扱う。元の出力は別ログがない。
- 上記は当事者からのメッセージに記載された Git 出力の引継ぎであり、本担当は元のコマンド出力を直接閲覧・再実行していない。表示された所有アカウントと実行アカウントが異なるとの観測は、先行結果の「所有者と実行者は未確認」を当該実行に限って補足する。

## 判断への影響と限界

これらの報告は Git の所有権安全判定に関係する具体的差を示す。ただし SID の実値・ACL、Git が内部で使用した比較・解決パス、アカウント構成の背景は直接確認していない。差が発生した原因や全警告の内部判定を断定せず、先行結果の対象限定・一時設定、旧作業保護、移管不明時の保留の判断は維持する。失敗記録と対策の共有ファイルへの反映は管理担当に委ねる。

本追記では Git コマンド、所有権・設定・既存ログの変更を行っていない。依頼書は存在確認後に UTF-8 で読み、結果ファイルを新規作成した。調査中の失敗は認識していない。
