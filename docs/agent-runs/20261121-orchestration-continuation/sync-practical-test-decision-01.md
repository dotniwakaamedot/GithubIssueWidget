# 限定テスト実行の判断

`sync-practical-test-plan-01.md` SHA256 `C8DA60D729EFA188FF2D34291DDDB94D1BA93CB533948B075DF0C76CBE3C0BEF` は `sync-practical-test-review-01.md` により対象の限定合成テスト方式として blocking なし。`/root/project_survey` は `sync-practical-test-work-request-01.md` と担当訂正 `sync-practical-test-work-request-02.md` に従い、事前条件を全て確認できた場合に一度実行する。不一致や危険な状況があれば実行しない。

実 GUI・実 GitHub・PAT は対象外。資格情報ストアと依存全体の非アクセス・全子孫終了は無条件に保証しない。直接プロセスの終了とケース結果・未確認を区別し、同じ失敗2回時は別担当に調査させる。成果は固定版レビュー後に判断する。
