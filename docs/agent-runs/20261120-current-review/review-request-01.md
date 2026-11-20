# 独立レビュー依頼

- 担当 ID: `/root/credential_review`。共有 run: `D:\Projects\GithubIssueWidget\docs\agent-runs\20261120-current-review`。作業場所: `D:\Projects\GithubIssueWidget`。
- ユーザー原文: 「現在の進行状況を確認しアプリの完成まで進行してください。一度の処理で2度同じ失敗をした時は停止してください」。AGENTS.md、`docs/agent-workflow.md` と対象の Issue #1 を一次資料とする。Issue の読取りができない場合は既存の `docs/verification-20261120.md` との照合に限定し、未確認を明記する。
- 対象範囲: 接続先・資格情報の世代照合、一覧／作成／移動の安全性、作成途中失敗とフォームの送信先保護。元要件を勝手に拡張せず、誤更新、秘密の露出、異常時の誤成功、実際に再現可能な競合を優先する。テストの欠落と製品の欠陥は区別する。
- 対象 SHA256:
  - `widget/src-tauri/src/main.rs`: `C938EEB22F1C2E97EB03E597D21ED1C87B903868C3673474B05043E36338641E`
  - `widget/src-tauri/src/creation.rs`: `61FB68E208B231664DB236E50A37BBB4870763AD61C0071383BACA5730982A87`
  - `widget/src-tauri/src/movement.rs`: `B240B50FDCDB34AFB3156B2EDC2AAFD11B03F808B033939716AB1B0FAFCA8791`
  - `widget/src/main.tsx`: `1AB362408673E420BED96D0D77127E5FB9DA70B3C6C6B4305386589F9B58DE75`
  - `widget/src/main.test.tsx`: `98504EC4DB2B733A6C726C094EF242617C8BA6B7915D2CEC776A980A2849C65A`
  - `widget/src-tauri/target/release/github-issue-widget.exe`: `24B86C6FB434BDEAF53B663414E46A796B7674D1407063FF046BC6C4A8C564E3`。`movement.rs` の最後の差分はテスト専用で、製品版はその変更前に構築された点を区別する。
- 成果物と所有範囲: `review-01.md` および必要時の質問ファイルのみ。アプリ、Git/GitHub 状態、資格情報、他人の作業を変更しない。共有文書は一時ファイルから確定し、完了通知は結果ファイルの絶対パスのみとする。
- 実 GitHub 書込み、Credential Manager 操作、実 UI の認証操作、外部送信を行わない。テスト結果は既存証拠を読んで確認し、実行環境や対象版が一致しない場合は未検証とする。
- 出力を分割し、長い行と内外の上限を考慮する。失敗は記録し、今回同種２回なら停止する。指摘は確認済み事実・推測・未確認を分け、優先度、根拠、影響、具体的対処を示す。
