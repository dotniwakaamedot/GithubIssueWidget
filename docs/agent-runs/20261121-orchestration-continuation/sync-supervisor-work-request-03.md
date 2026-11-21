# テスト監督器の担当移管・依頼 03

- 担当 `/root/project_survey` に変更。前担当 `/root/process_safety` の `sync-supervisor-handoff-answer-01.md` は `sync-supervisor-work-request-02.md` 受領後の書込みなし、6ファイルの SHA、未保存変更なし、書込み停止を記録。新担当はその6ファイル・未追跡・現行ハッシュを着手前に照合し、不一致なら保護し管理へ連絡する。
- 所有範囲と目的、依存、制約、未検証の扱いは `sync-supervisor-work-request-02.md` を継承し、旧担当 ID のみ訂正する。対象 `test-supervisor-01/` と新結果 `test-supervisor-result-02.md` を担当し、他者の編集を戻さない。テストや合成子を実際に生成する前に固定版の独立レビュー・開始条件の確認を受ける。
- 作業の成否、未追跡を含む全変更ファイルの SHA、操作・終了結果・未確認を新しい結果へ確定し、書込み停止後に結果パスを通知する。
