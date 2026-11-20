# 進行確認依頼

- 担当 ID: `/root/process_review`。共有 run: `D:\Projects\GithubIssueWidget\docs\agent-runs\20261120-current-review`。作業場所: `D:\Projects\GithubIssueWidget`。
- ユーザー原文: 「現在の進行状況を確認しアプリの完成まで進行してください。一度の処理で2度同じ失敗をした時は停止してください」。`AGENTS.md` と `docs/agent-workflow.md` を守る。
- 現在の完成条件・検証・未検証を `docs/verification-20261120.md` と参照先の範囲で確認し、作業の重複、誤った完了判定、停止条件や証跡の問題があれば根拠付きで指摘する。製品コードの合否は別の確認担当が扱う。
- 読取り専用で調査し、所有する `process/review-01.md` と必要な質問・提案ファイル以外は書き換えない。GitHub 変更、認証情報の照会、実 UI、外部送信、テスト実行、削除を行わない。
- 提案を出す場合は事実と推測を分け、既存 Issue や権限が未確認なら登録済みと扱わない。出力は分割し、同種失敗が今回２回なら停止する。結果の通知は確定した絶対パスのみとする。
