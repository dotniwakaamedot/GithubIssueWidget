# 合成資格情報試験の安全な識別方法の調査依頼

- 担当予定 `/root/credential_safety_design`。親 `/root` が ID を照合し起動。共有 run `D:/Projects/GithubIssueWidget/docs/agent-runs/20261121-app-completion-restart/`、作業場所 `D:/Projects/GithubIssueWidget`。
- `windows-credential-test-result-01.md` は既存 ignored test の合成 service 名が書込み前に外部記録されず、異常終了後の対象識別を保証できないため実行を保留した。この判断を尊重し、既存実装・ユーザーの認可範囲・`docs/instructions/work.md` の外部書込み監督の指示を読み、具体的な安全な検証方法を独立に設計する。
- 一意な合成 service の事前予約・識別の耐久記録、既存 service と重複しない確認、実 OS 書込み、更新、削除・後読取り、終了、異常時の識別と再開、監督者・子孫停止時、登録不能時の禁止事項、秘密の非出力を順序と失敗表で整理。生成用コードとテストを提案する場合は対象・所有・隔離・事前検証・実 OS と合成の違い・制限を明記し、無条件の終了保証を主張しない。ユーザーの実 PAT と既存 service の操作を認めない。
- 読取り・設計のみ。実 Credential Manager、実アプリ、GitHub、既存設定・製品は操作しない。他者成果を変更せず、新規 `credential-safety-design-01.md` のみ所有。UTF-8 実データ読戻し後に一時ファイルから確定してパスのみ通知。出力量を限定し、同種失敗2回では停止し親へ報告。後続の独立レビューを要する。
