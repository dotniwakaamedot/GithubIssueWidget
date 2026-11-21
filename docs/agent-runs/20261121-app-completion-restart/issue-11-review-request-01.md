# Issue #11 の実機・公開履歴の独立レビュー依頼

- 担当予定 `/root/issue_11_review`。親 `/root` が ID を照合して起動。作者 `/root/issue_state` と別担当。共有 run `D:/Projects/GithubIssueWidget/docs/agent-runs/20261121-app-completion-restart/`。
- 対象 `issue-11-state-04.md` SHA256 `44F629CB8B2E14ECB0FB8FC82A541D17A97022F4BD549734BB82D4ABAC5D2DF3`、`user-observation-07.md` `F2B593C7B5093AA225F4DCF85395D117C902319ACDA3FEAF847ACD09391BBC67`、`user-observation-08.md` `261EC6EDAF2A30209D4646FB7FE28CEB4DBE4D2847E5A980DDFAACCF3EF5B483`。公開 API の #11 本体とイベントを読取り、ページ終端、6方向に対応するラベル・状態変化、最後 closed・done の記録を照合。イベントの非原子性、本人報告の範囲・対象 EXE SHA、非 status ラベルなしと未検証を区別する。独立読取りは未実施の実 UI や全障害を証明しない。
- 新規 `issue-11-review-01.md` のみ所有。対象 Issue やラベル・資格情報・実アプリ・既存資料は変更しない。不要な本文や秘密は出力せず、対象版・根拠・blocking・未確認を記録。UTF-8 実データ読戻し後に一時ファイルから確定しパスのみ通知。他者の変更を戻さない。出力を絞り同種失敗2回では停止・報告。
