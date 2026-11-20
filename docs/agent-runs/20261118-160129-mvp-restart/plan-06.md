# 計画

- run_id: 20261118-160129-mvp-restart
- revision: 06（承認済み revision 02 に対する状態更新）
- owner_role / owner_agent_id: progress_manager / /root/restart_planner
- shared_run_absolute_path: D:\Projects\GithubIssueWidget\docs\agent-runs\20261118-160129-mvp-restart
- ユーザー原文: 「Issueを確認し、プロジェクトを進行してください。あなたはオーケストレーションに徹してください。同じ間違いや失敗を２度繰り返したら中止してください。」
- 目的: Issue #1 の Windows 11 / Tauri 2 / React / TypeScript / Rust による GitHub Issue ウィジェット MVP を、子 Issue #2〜#7 の依存関係に従って進める。仕様一次資料は https://github.com/dotniwakaamedot/GithubIssueWidget/issues/1 と各子 Issue の本文。上記の原文と Issue 本文の条件が優先。
- 完了条件: #2〜#7 と親 #1 の各完了条件を対象版と検証証拠に照合し、未検証を未完了と扱う。認可済みの統合、独立レビュー、Windows 実機での最終確認と Issue/PR の整合性を確認する。
- 対象外 / 権限: 既存の未関連変更を保護。push、PR、merge、GitHub 外部更新、削除の認可は親が判定する。設定例や ADMIN 権限はユーザー認可の代替ではない。未確認の対象操作を保留する。
- 作業場所 / 基準候補: D:\Projects\GithubIssueWidget / main / a4ac97286785a2a5ba0d070a909b58003fc49ff1。GitHub 既定 main、2026-11-18 UTC に ls-remote で同 SHA。作業実施前に再確認し、承認後は作業別の分離 worktree・branch を作成する。
- 既存変更・他担当: 起点は git status clean、worktree は起点のみ。旧担当 /root/issue_inventory と /root/environment_check は interrupted、/root/workflow_audit は終了。環境確認担当 /root/recheck_environment は終了。共有 run は以前存在しなかった。外部環境の作業不存在は未確認。
- GitHub 認証: gh の keyring 認証、viewerPermission ADMIN を確認。資格情報は記録しない。権限と操作認可は別。
- 計画レビュー依頼・対象 SHA256・判定: tasks/plan/review-02.md により plan-02.md SHA256 A6CB37DC369D69B334564235CD00C3AF284250109D4A56BE63C6CC7B104FE990 を承認。revision 03〜06 は状態と判断の更新であり、承認範囲の変更はない。作者は /root/restart_planner、レビュー担当は作者以外を親が指定。

| task_id | 状態（未着手/着手/終了） | 担当ID | 所有ファイル・作業場所 | 依存 | 完了条件・検証 | 最新依頼/結果/承認 |
| --- | --- | --- | --- | --- | --- | --- |
| P01 | 終了 | /root/restart_planner | 本共有 run の計画・管理記録のみ | なし | Issue・現在状態を照合し対象版を固定、独立レビューへ引継ぐ | plan-02.md 承認、tasks/plan/review-02.md |
| E01 | 着手 | /root/recheck_environment | 環境調査担当の結果ファイルのみ、アプリ変更なし | なし | Windows / Node / Rust / MSVC / WebView2 の必要条件と未確認を証拠付き確認 | tasks/environment/result-01.md 確定・ハッシュ照合済み、独立環境確認待ち |
| G01 | 終了 | /root/parent_issue_analysis | 親 #1 未決事項と環境の調査結果、共有 run の割当領域 | 計画承認、E01 確定結果 | 親 Issue 原文に列挙された各未決事項の事実・根拠・対応案・未確認と検証環境を整理し、独立確認を経て適用範囲・実装前条件を親が判断。外部更新の認可と未確認を区別 | tasks/G01/result-01.md、review-01.md、親判断済み |
| I02 | 着手 | /root/foundation | 独立 worktree のアプリ基盤と対象証拠。作業依頼で詳細化 | 計画承認、G01 の独立確認・親の実装前条件判断、Issue 詳細化と着手記録の読戻し、環境確認 | #2 の全条件: 起動・リリースビルド・ウィンドウ操作、Opener 権限制限と検証 | tasks/I02/combined-review-01.md にて B01〜B04 未解決。build-verification-result-01.md と drag-click-request-01.md は独立レビュー待ち |
| I03 | 未着手 | 未割当 | 独立 worktree、接続設定・認証 | I02 の統合 | #3 の全条件、Windows Credential Manager と秘密保護を含む | なし |
| I04 | 未着手 | 未割当 | 独立 worktree、一覧・同期・ボード | I02、I03 の統合 | #4 の全条件、ページネーションと遅延応答を含む | なし |
| I05 | 未着手 | 未割当 | 独立 worktree、Issue 作成 | I03、I04 の統合 | #5 の全条件、部分失敗と重複防止を含む | なし |
| I06 | 未着手 | 未割当 | 独立 worktree、Issue 状態変更 | I03、I04 の統合。I05 と共有部が競合する場合は直列化 | #6 の全条件、競合・他ラベル保持を含む | なし |
| I07 | 未着手 | 未割当 | 統合対象版・検証証跡 | I02〜I06 の統合 | #7 の全条件、実 GitHub 書込み先と許可を確認し Windows 11 で実証 | なし |
| F01 | 未着手 | 未割当 | 最終レビュー・報告 | I07 と親 #1 全条件 | 対象版・証跡・PR 状態を独立照合、完了後整理確認を親が一度依頼 | なし |

## ブロッカー・レビュー待ち・再開地点

- 計画 revision 02 は独立承認済み。G01 調査と独立レビュー後、親は #2 の実装前整理条件を満たすと判断。#2 の詳細化・状態ラベル・進行コメントは読戻し済み。ローカル統合 branch と task worktree で実装途中。独立レビュー B01〜B04 は未解決、I02 は着手中。
- 親 #1 は未決事項と検証環境を整理してから実装着手と明記。G01 では、ラベル欠落・複数ラベル・同時更新・部分失敗、トークン更新削除と設定／資格情報の保存途中失敗、大量 Issue・レート制限・ページ取得失敗、開発・Windows 11 環境と既存コード例へのアクセスを一次資料に基づき整理する。各項目の確定事項、未確認、対応案と必要な外部権限を整理し、独立レビューと親判断を経て #2 の実装前整理条件は満たすと判断。設計案は確定仕様ではなく、実環境の未検証は残る。
- E01 の実際の結果と指定先の相違は tasks/E01/management-01.md に記録。正本 tasks/environment/result-01.md の SHA256 7232E15B14A883FD63BED6A12054263D64662883AE0B620D887377E66A8E5885 を照合。個別コマンドの終了コード等は未確認、対象版の独立環境レビューは未実施。
- Issue #2 本文は既存部分を保持して詳細化し、status:in-progress と進行コメントを読戻し・再照合済み。実装途中成果はレビュー B01〜B04 が残り、#2 完了とは扱わない。
- 親子 Issue #1/#2 の統合ブランチ・PR base および外部更新の認可は未確定。既定 main と実際の PR base を混同しない。
- E01 は検出結果を受領し対象 SHA を照合済み。SDK の完全性と実機検証等は別途必要。I02 では Windows 起動・build の作者報告があるが再現可能な証拠が不足し、操作や拒否 URL も未確認。
- 他担当の独立計画レビューを最大3回。blocking が残る場合は開始せず、同じ意味の指摘が2回生じたら停止。

## 判断・改善案の採否（管理だけが更新）

| 提案ID・根拠 | 採用/保留/見送り | 理由 | 次担当・作業依頼 | 実施状況 | 効果の確認方法・結果 | Issue URL・登録状態 |
| --- | --- | --- | --- | --- | --- | --- |
| P01 / process/review-01.md | 採用 | 依頼・結果の追跡性を改善 | 進行管理・担当 | I02 request-02.md を担当再開前に確定・固定参照を照合 | 固定版を再照合し実施順を確認、以後も継続 | https://github.com/dotniwakaamedot/GithubIssueWidget/issues/8 登録・読戻し済 |
| P02 / process/review-02.md | 採用 | 状態表とブロッカーの矛盾を解消 | 進行管理 | revision 05 で E01/G01/I02 の確定記録に整合 | revision 06 で再照合、継続監視 | https://github.com/dotniwakaamedot/GithubIssueWidget/issues/9 登録・読戻し済 |
| P03 / process/review-02.md | 採用 | 日本語破損の再発防止 | 進行管理・各担当 | 危険なパイプ経路は禁止、UTF-8 を指定 | 共有記録で UTF-8 往復確認、製品全体の保証ではない | https://github.com/dotniwakaamedot/GithubIssueWidget/issues/10 登録・読戻し済 |

## 履歴・最終確認

- 前回、日本語出力の文字化けが2回と判断され中止。今回出力元ごとに UTF-8 を設定し確認する。
- 2026-11-18 UTC: PowerShell 5.1 で `Get-Date -AsUTC` が未対応で日時取得に1回失敗。`[DateTime]::UtcNow` で成功。同種の失敗は未反復。
- revision 01 は plan-01.md として保存（SHA256 E1C053A0A545C8F005E6C8EEB602F265868128202AA930C859769D310FBFFD69）。review-01.md の B01 を G01 ゲートで是正、N01 は E01 正式記録待ち。revision 02 の独立再レビューを依頼する。
- revision 02 の承認済み原本は plan-02.md に保存。revision 03 は作業状態・判断の記録のみ。対象・完了条件など実質変更時は新版をレビューする。
- 追加指示「失敗を3回繰り返すまでに緩和します」に基づき、今回の停止条件は初回を含め同種の失敗が3回発生した時点。旧版の2回記述より優先する。
- 本計画は今後の責任者・依頼・レビュー・対象版を別途記録し、変更すれば再レビューする。
- I02 は request-02.md の固定参照・担当 ID を確認後、/root/foundation を再開。依頼・対象版に不一致があった2件は同種として数え、3件目で停止（tasks/I02/management-02.md）。

- revision 05 で P02/P03 を採用し、E01・G01・I02 の現在の記録と状態を照合。対象版の詳細は各確定結果・レビューを参照。

- revision 06: B03 専用コピーの release はログに完了表示があるが、npm/Cargo exit code 未取得で未達。所有セッションは中断後 terminal、dev は未実施。単一クリック診断の依頼は独立レビュー・B03 非競合確認待ち。対象と証拠は tasks/I02/build-verification-result-01.md、build-drag-preflight-review-request-01.md に記録。
- ユーザー追加指示は同種2回で該当操作を中断し別担当が客観調査、3回で全中止。今回の各失敗履歴と最新の依存ゲート不備は各確定記録を参照する。
- P02/P03 は Issue #9/#10 登録・読戻し済み。登録記録の元レビュー SHA 誤記は process/issue-registration-04.md が訂正し旧版を保持。
