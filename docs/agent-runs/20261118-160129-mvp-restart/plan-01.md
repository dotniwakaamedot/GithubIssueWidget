# 計画

- run_id: 20261118-160129-mvp-restart
- revision: 01（計画レビュー待ち）
- owner_role / owner_agent_id: progress_manager / /root/restart_planner
- shared_run_absolute_path: D:\Projects\GithubIssueWidget\docs\agent-runs\20261118-160129-mvp-restart
- ユーザー原文: 「Issueを確認し、プロジェクトを進行してください。あなたはオーケストレーションに徹してください。同じ間違いや失敗を２度繰り返したら中止してください。」
- 目的: Issue #1 の Windows 11 / Tauri 2 / React / TypeScript / Rust による GitHub Issue ウィジェット MVP を、子 Issue #2〜#7 の依存関係に従って進める。仕様一次資料は https://github.com/dotniwakaamedot/GithubIssueWidget/issues/1 と各子 Issue の本文。上記の原文と Issue 本文の条件が優先。
- 完了条件: #2〜#7 と親 #1 の各完了条件を対象版と検証証拠に照合し、未検証を未完了と扱う。認可済みの統合、独立レビュー、Windows 実機での最終確認と Issue/PR の整合性を確認する。
- 対象外 / 権限: 既存の未関連変更を保護。push、PR、merge、GitHub 外部更新、削除の認可は親が判定する。設定例や ADMIN 権限はユーザー認可の代替ではない。未確認の対象操作を保留する。
- 作業場所 / 基準候補: D:\Projects\GithubIssueWidget / main / a4ac97286785a2a5ba0d070a909b58003fc49ff1。GitHub 既定 main、2026-11-18 UTC に ls-remote で同 SHA。作業実施前に再確認し、承認後は作業別の分離 worktree・branch を作成する。
- 既存変更・他担当: 起点は git status clean、worktree は起点のみ。旧担当 /root/issue_inventory と /root/environment_check は interrupted、/root/workflow_audit は終了。再開環境確認担当 /root/recheck_environment は実行中。共有 run は以前存在しなかった。外部環境の作業不存在は未確認。
- GitHub 認証: gh の keyring 認証、viewerPermission ADMIN を確認。資格情報は記録しない。権限と操作認可は別。
- 計画レビュー依頼・対象 SHA256・判定: tasks/plan/review-request-01.md に別記。判定待ち。作者は /root/restart_planner、レビュー担当は作者以外を親が指定。

| task_id | 状態（未着手/着手/終了） | 担当ID | 所有ファイル・作業場所 | 依存 | 完了条件・検証 | 最新依頼/結果/承認 |
| --- | --- | --- | --- | --- | --- | --- |
| P01 | 着手 | /root/restart_planner | 本共有 run の計画・管理記録のみ | なし | Issue・現在状態を照合し対象版を固定、独立レビューへ引継ぐ | 計画 revision 01 |
| E01 | 着手 | /root/recheck_environment | 環境調査担当の結果ファイルのみ、アプリ変更なし | なし | Windows / Node / Rust / MSVC / WebView2 の必要条件と未確認を証拠付き確認 | 親から依頼済み、結果待ち |
| I02 | 未着手 | 未割当 | 独立 worktree のアプリ基盤と対象証拠。作業依頼で詳細化 | 計画承認、Issue 詳細化と着手記録の読戻し、環境確認 | #2 の全条件: 起動・リリースビルド・ウィンドウ操作、Opener 権限制限と検証 | なし |
| I03 | 未着手 | 未割当 | 独立 worktree、接続設定・認証 | I02 の統合 | #3 の全条件、Windows Credential Manager と秘密保護を含む | なし |
| I04 | 未着手 | 未割当 | 独立 worktree、一覧・同期・ボード | I02、I03 の統合 | #4 の全条件、ページネーションと遅延応答を含む | なし |
| I05 | 未着手 | 未割当 | 独立 worktree、Issue 作成 | I03、I04 の統合 | #5 の全条件、部分失敗と重複防止を含む | なし |
| I06 | 未着手 | 未割当 | 独立 worktree、Issue 状態変更 | I03、I04 の統合。I05 と共有部が競合する場合は直列化 | #6 の全条件、競合・他ラベル保持を含む | なし |
| I07 | 未着手 | 未割当 | 統合対象版・検証証跡 | I02〜I06 の統合 | #7 の全条件、実 GitHub 書込み先と許可を確認し Windows 11 で実証 | なし |
| F01 | 未着手 | 未割当 | 最終レビュー・報告 | I07 と親 #1 全条件 | 対象版・証跡・PR 状態を独立照合、完了後整理確認を親が一度依頼 | なし |

## ブロッカー・レビュー待ち・再開地点

- 計画は未承認。アプリ実装、基準からのブランチ作成、Issue 状態変更・コメントは開始していない。
- Issue #2 本文は調査・詳細化中。詳細化、関連 PR・全コメント・担当・run の再照合、必要な状態と進行コメントの書込み権限確認、双方の読戻し後の再照合を経るまで実装しない。
- 親子 Issue #1/#2 の統合ブランチ・PR base および外部更新の認可は未確定。既定 main と実際の PR base を混同しない。
- ローカル環境と Windows 実機検証の可否は E01 待ち。実機要件を省略しない。
- 他担当の独立計画レビューを最大3回。blocking が残る場合は開始せず、同じ意味の指摘が2回生じたら停止。

## 判断・改善案の採否（管理だけが更新）

| 提案ID・根拠 | 採用/保留/見送り | 理由 | 次担当・作業依頼 | 実施状況 | 効果の確認方法・結果 | Issue URL・登録状態 |
| --- | --- | --- | --- | --- | --- | --- |
| なし | 保留 | 独立進行レビュー未実施 | 親が依頼 | 未着手 | レビュー結果待ち | 未登録 |

## 履歴・最終確認

- 前回、日本語出力の文字化けが2回と判断され中止。今回出力元ごとに UTF-8 を設定し確認する。
- 2026-11-18 UTC: PowerShell 5.1 で `Get-Date -AsUTC` が未対応で日時取得に1回失敗。`[DateTime]::UtcNow` で成功。同種の失敗は未反復。
- 本計画は今後の責任者・依頼・レビュー・対象版を別途記録し、変更すれば再レビューする。