# 実 OS 試験と記録時の失敗に関する独立分析依頼

- 担当予定 `/root/live_failure_recorder/independent_analysis`。管理 `/root/live_failure_recorder`。共有 run `D:/Projects/GithubIssueWidget/docs/agent-runs/20261121-app-completion-restart/`、作業場所 `D:/Projects/GithubIssueWidget`。
- 対象: `credential-live-execution-result-01.md` の実 OS 合成試験終了1、`credential-live-failure-investigation-01.md` の出力切り詰め1回、`credential-live-residue-inspect-01.md` の推測パス不存在1回と検査担当が報告した文字照合失敗1回、管理の分冊見出し取得時の Python 日本語表示文字化け1回。`credential-live-residue-review-01.md` を参照。秘密と実資格情報を出力・取得しない。
- 実 OS 試験は同一 run の再表示・分析を二重計上せず、終了1の直接原因・書込み成否は未確定、当該合成 target は読取り時点で absent と区別。対象失敗の事実・元出力・終了・原因の確度、既存 F-001・F-012・F-057 と対策の対応、同一性や累計が不明な範囲を分析。元の実 OS 操作、資格情報読取り・書込み・削除、失敗コマンドを再実行しない。必要なら担当者へ質問ファイルを作成する。
- 所有は新規 `live-failure-record-analysis-01.md` と自身の新規質問のみ。既存失敗記録・製品・資格情報・他者成果を変更しない。共有作業中であり他者変更を戻さない。出力範囲・長さを事前に絞り、同種失敗2回は停止して管理へ通知。UTF-8 実データ読戻し後に一時ファイルから確定、通知はパスのみ。
