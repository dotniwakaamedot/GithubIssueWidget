# Issue #2 詳細化・開始記録の準備 01

- 対象 `dotniwakaamedot/GithubIssueWidget` #2、親 #1。親判断: G01 結果 SHA256 BFDB22179081951FD8F7365F9391ED82BD7304F281C42CB3C17B768E6C8376E5 は独立 review-01 で承認、未決事項を #2 実装前に整理する条件は満たす。ただし選択肢は仕様確定せず、実環境の動作は未確認。
- 親は Issue の本文詳細化・状態ラベル・必要な進行コメント・レビュー提案登録をユーザーの進行依頼の範囲内と判断。push / PR / merge は別途判断。認証 gh keyring、viewerPermission ADMIN。
- 2026-11-18 16:20 UTC 頃、#2 OPEN、updatedAt 2026-11-18T15:52:35Z、担当なし、ラベル enhancement のみ。全ページの issue comments は空。全状態の PR は0件（全ページの REST API）。worktree は main checkout 1個、HEAD と origin/main は a4ac97286785a2a5ba0d070a909b58003fc49ff1。共有 run は本 run のみローカル確認、外部 run なしとは断定しない。
- 全ラベル10件を列挙し `status:in-progress` は未存在。認可範囲内で同名がないことを直前に再確認してから作成する必要あり。状態ラベル以外の `enhancement` を保持。追加するラベル案: name `status:in-progress`、description `Issue の計画・実装に着手中`、color `fbca04`。
- 本文変更案 `body-01.md` は既存の完了条件を維持し、具体的な検証と未検証を追加。適用直前に既存本文と updatedAt・担当・全コメント・PR・run を再照合し、変化・所有不明なら中止。読戻しで本文の完全一致を確認。
- 進行コメント案 `progress-comment-01.md` は識別キー `<!-- issue-progress:20261118-160129-mvp-restart:start-01 -->`。UTC 時刻は実際の送信時刻に合わせた新版を発行し、予定ブランチは衝突確認。投稿不明時は全ページを識別キーで照合するまで再送しない。
- 本文、状態ラベル、進行コメントの実際の送信前には親へ本記録と案を提示。送信後に状態ラベル・コメント両方を読戻し、その直後に担当、ラベル、全コメント、関連 PR、run を再照合する。いずれか不明なら実装しない。
- 進行レビュー提案 P01: process/issues/P01.md の識別キーを全 open/closed Issue 7件で照合し一致なし。全7件の本文と題名を閲覧し、依頼・結果ファイルの順序に関する同一提案は見当たらない。照合後に Issue が変化しうるため、登録直前に識別キーと内容を再確認。作成応答不明なら重複検索前に再作成しない。作成後は URL・本文を読戻す。