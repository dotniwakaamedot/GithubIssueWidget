# 利用手順 README の更新依頼

- 担当予定 `/root/readme_update`。親が ID を照合して起動。共有 run `D:/Projects/GithubIssueWidget/docs/agent-runs/20261121-app-completion-restart/`、作業場所 `D:/Projects/GithubIssueWidget`。
- 根拠 `release-readme-audit-01.md`、Issue #1〜#7、`selection-build-result-01.md` と独立レビュー、本人報告・Issue #11 公開イベントの範囲。対象 `README.md` SHA256 `3D2A457B647B8B514866A693A49E6D70AA948C68B38F9BBA17CE439D5DF8CB9E`。開始直前に実体とハッシュ・文脈を照合し異なる場合は停止して質問。
- `README.md` のみ更新。既存の開発・ビルド手順とローカルで既にビルドされた EXE の利用方法を分ける。受渡先 `widget/release-20261121-selection/github-issue-widget.exe` は並行する `/root/release_copy` の結果と SHA が確定するまで存在・完成を断定せず、必要であれば担当へ確認してから正確に記載。ローカル限定の生成物であること、別 checkout に自動で存在するとは限らないことを明示。一般配布・署名・インストーラーの成功を主張しない。
- ＋は新規作成、カードはドラッグで移動、カードと列見出しの文字は選択できない点、通信・認証・権限・レート制限、部分失敗と作成結果不明時の GitHub 状態確認・重複送信防止を実コードに即して簡潔に説明。トークンは本人が入力し秘密を共有しない案内を保持。未検証の実 credential・実 IPC・実障害・複数ページや他環境を検証済みとしない。
- 製品コード、EXE、他者の変更は変更しない。既存記述を無関係に削除せず、差分と UTF-8 実データの読戻しを検証し `readme-update-result-01.md` に対象前後 SHA、根拠、検証、未確認を記録。一時ファイルから確定しパスのみ通知。独立レビューが必要。外部公開・push・PR・merge はしない。
