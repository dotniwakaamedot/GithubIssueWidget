# I02 開発起動ドライバー準備の再開判断 02

- run_id: `20261118-160129-mvp-restart`。進行管理: `/root/resume_manager`。宛先担当: `/root/build_verification`。共有 run: `D:\Projects\GithubIssueWidget\docs\agent-runs\20261118-160129-mvp-restart`。
- 対象依頼: `tasks/I02/dev-driver-preparation-request-02.md` SHA256 `0962F309C3DA04802F40A7EB5D3F0AC7B34494481CF48AF0FE7C3E824F9A2BC9`。親は本依頼に沿う driver 準備の再開を判断した。独立レビュー・親判断前の実行は許可していない。
- 旧担当 `/root/build_verification` の停止と同一担当への専用 scratch の引継ぎを親が確認した。担当者は `D:\Projects\GithubIssueWidget-dev-20261119-01` の source 17件・manifest・overlay の一致を報告し、driver は未完成。開始前に本人が実体を再照合し、既存物を保護する。
- 直前の read-only 照合: GitHub Issue #2 は OPEN、担当なし、`enhancement` と `status:in-progress`、コメント2件、全状態の PR は取得範囲0件。GitHub 既定 `main` と origin/main、元 checkout・#2 worktree の HEAD は `a4ac97286785a2a5ba0d070a909b58003fc49ff1`。#2 worktree は README 変更と widget 配下の未追跡16件。ローカル共有 run はこの run のみ確認。外部環境の run や後続の競合の不存在は保証しない。
- `copy-manifest.json` SHA256 `BD683EE7F754CC57DCFFE296960BB0DC3245BEBA0ED34A31548F09D95E47BEC7`、`dev-overlay.json` `5DD74560FEAC1C32C39BD1A5EBE8618FFC6EE7A716977A820CB6C856D416AE75` は実体と一致。依頼書に列挙する他の固定参照・17ファイルの照合を担当自身が着手前に再実施し、不一致や所有不明なら停止する。
- 旧診断 handle `70494` / `19143` は所有者の確認でいずれも `Unknown process id`。別担当は旧 PID/path と TCP 1435 listener を観測時点で検出しなかった。終了コード・原因・外部環境の不存在は未確定。正常終了と断定せず、診断を再起動しない。既存の生成物・run・process を保護する。
- 直近の日本語破損と過去の2件を踏まえ、PowerShell の here-string から Python の標準入力へ日本語を pipe する経路は禁止する。共有ファイルは同一ディレクトリの新規 `.tmp`、strict UTF-8 decode と意図した Unicode 原文との全文照合、既存先を置換しない rename、確定後の全文・SHA256 再確認を行う。該当経路や不一致を発見したら後続を止める。親は起動前に担当 ID・依頼 SHA とこの制約を照合する。
- 同種の問題が初回を含め2回生じたら当該操作を停止して別担当が客観調査し、3回目で全作業を停止する。過去に自動ポリシーが拒否した window metadata レビューの特定公開操作は再試行・回避しない。
- 本判断は既存 Issue コメント・ラベルや GitHub の状態を更新していない。実装再開・外部更新が必要となる前に、適用手順に従い最新の Issue・全コメント・PR・run・担当と状態記録を再照合する。認証や権限の照会でトークン等を出力しない。
