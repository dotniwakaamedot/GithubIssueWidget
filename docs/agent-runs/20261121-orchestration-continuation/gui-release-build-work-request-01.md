# 現行版 release ビルドの準備・実施依頼

- 担当 `/root/gui_failure_investigation`。共有 run `D:/Projects/GithubIssueWidget/docs/agent-runs/20261121-orchestration-continuation/`。作業領域・結果は新規専用 `gui-release-build-01/` と `gui-release-build-result-01.md`。既存の製品・依存・Git・旧 fixture・他者作業を変更しない。
- `gui-release-plan-01.md` と `gui-empty-startup-audit-01.md` に従い、現行製品ソース・lock・必要な設定・依存を固定し、空の専用作業/出力・TEMP 等を準備する。ビルドの入力と実行体、書込み先・既存資料保護・終了コード、機密を含む可能性のある出力の扱いを確認。未コミット変更を含む現行版を対象として保持し、隔離先と出典の SHA・内容を照合する。未知のリンク・不一致・既存ファイルの上書きがある場合は作成せず管理へ報告する。
- ビルドの実行は `gui-release-plan-review-01.md` の結果で対象方式に blocking がないことと、管理の開始通知を待つ。GUI 起動・実 credential/GitHub 操作はこの依頼に含まない。既に合格した同期テストを理由なく再実行しない。
- 事前確認と実施内容・各コマンド/終了・出力完全性・EXE 実体と SHA・未確認・失敗を結果へ記録。実行未完了を成功としない。完了後は書込み停止しパスを通知、別担当の固定版確認を受ける。
