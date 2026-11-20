# 親 Issue の実装前未決事項の調査依頼 03

- run_id / task_id / request_revision: 20261118-160129-mvp-restart / G01 / 03
- 担当 ID: /root/parent_issue_analysis、役割 task_worker。管理: /root/restart_planner。共有 run: D:\Projects\GithubIssueWidget\docs\agent-runs\20261118-160129-mvp-restart
- ユーザー原文: 「Issueを確認し、プロジェクトを進行してください。あなたはオーケストレーションに徹してください。同じ間違いや失敗を２度繰り返したら中止してください。」
- 再開: request-01 の担当 ID 不一致を request-02 で訂正後、旧担当 /root/parent_issue_analysis は interrupted。旧担当の書込み停止と結果未作成を確認。質問と回答は questions/G01-question-01.md と G01-answer-01.md。再開は同一担当に対する本新版を使用。作業目的・所有範囲は維持。
- 対象計画: plan.md revision 02 SHA256 A6CB37DC369D69B334564235CD00C3AF284250109D4A56BE63C6CC7B104FE990。tasks/plan/review-02.md は当該 SHA の計画を承認、実装や外部更新は承認していない。
- 入力: GitHub https://github.com/dotniwakaamedot/GithubIssueWidget/issues/1 の本文を必ず取得し原文を一次資料として扱う。#2〜#7、適用先 AGENTS.md、docs/agent-workflow.md と docs/issue-workflow.md、環境調査 tasks/environment/result-01.md SHA256 7232E15B14A883FD63BED6A12054263D64662883AE0B620D887377E66A8E5885、tasks/E01/management-01.md。HEAD 基準候補 a4ac97286785a2a5ba0d070a909b58003fc49ff1 を確認する。
- 目的: 親 #1 が「未決事項・実装前の確認」に列挙するすべての事項と検証環境について、既に Issue で決定済みの内容、根拠、不明、設計上の選択肢、検証可能な条件、必要な権限を整理する。既存 Issue と矛盾しないか照合し、不明を推測で確定しない。特にラベル欠落・複数ラベル・同時更新・部分失敗、資格情報と設定の途中失敗、大量 Issue・レート制限・ページ取得失敗、開発・Windows 11 実機・既存コード例へのアクセスを漏れなく扱う。
- 各項目が #2 着手を妨げるか、理由・根拠と代替案を示し、適用範囲が不明なものは親の判断待ちとする。Issue 本文の詳細化案を成果へ含めるが GitHub は変更しない。#1 全体の実装は対象外。
- 結果の検証: コマンド・作業場所・終了コード・取得時刻・Issue の版や根拠、未確認を記録。事実、仮説、提案を区別し、全未決事項と成果の対応表を含める。人の操作、実ビルド、API 書込み等を実施していない場合は明記。
- 所有範囲: 共有 run の tasks/G01/result-01.md と自担当の questions ファイルのみ。書きかけ .tmp から確定し、結果を対象 SHA256 付きで通知する。アプリ、計画、他担当成果、GitHub、Git の変更は禁止。別担当が同じ作業場所にいるため他者の作業を保護する。
- ユーザー追加指示原文「失敗を3回繰り返すまでに緩和します」。初回を含め同種の失敗が今回3回発生した時点で直ちに中止し親へ報告。依頼原文の2回条件よりこの追加指示を優先。書込み先の親ディレクトリと宛先不在を事前確認し、書込み失敗時は後続の依存操作を止める。出力文字コードは各プログラムで確認し、資格情報を表示しない。