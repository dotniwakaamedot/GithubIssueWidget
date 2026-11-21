# 統合後の型検査・ビルド計画レビュー 01

- 担当 `/root/plan_review`、作者 `/root/planning` と別担当。依頼 `sync-post-integration-check-review-request-01.md`、対象 `sync-post-integration-check-plan-01.md`。
- 期待 SHA256 と開始・終了時実測 `99E859B8E4D27A706ACA58E1D29A02D93FB5E77935169BE4EF2BE58623C14C76` は一致。削除対象なし。
- 判定: **固定計画を承認**。blocking なし。事前照合と実行条件が満たされた場合の隔離型検査・フロントエンドビルドの方式に限る。実行結果、Tauri の Windows 配布物や実 UI・GitHub を承認するものではない。

| 確認項目 | 結果・条件 |
| --- | --- |
| 入力と対象 | 現 `widget/src` と fixture `project/src` は main、test、style の3ファイル。`tsconfig.json` の対象は `src` と `vite.config.ts`、`index.html` は `src/main.tsx` を参照。fixture に HTML を追加する前後の一致確認は計画にある。実行時には他の入力・依存・現行製品版も確認する。 |
| コマンド | `package.json` の build は `tsc --noEmit && vite build`。計画の固定 Node 経由の2コマンドは同等の型検査と Vite build を意図し、前者成功時だけ後者を行う。実際の CLI 実体・解決・終了は実行まで未確認。 |
| 状態の保護 | 過去 manifest・既知の生成物と新規 HTML・ログ・出力を区別し、未知の変更は上書きしない。以前の Vitest token は内容を読まず保持。固定環境・秘密を渡さない先行方式を再利用し、実行前に変更と隔離を確認。 |
| 実行と証拠 | 各コマンドの stdout/stderr・終了コード・生成物を別々に記録し、時間切れと終了を区別。子孫の未確認・共有 OS のリスクを明記し、未知のプロセスを推測して止めない。 |
| 限界 | Vite 生成物は Web フロントエンドであり、Tauri の実行ファイルや Windows 実機・実 API の動作を証明しない。現製品と対象ファイルの一致を実行後にも確認する必要がある。 |

非 blocking: `index.html`、設定・style・実行体の実測 SHA、専用環境変数名、ログと dist の位置、各コマンドの実際の終了コードと実行後の入力一致を結果に残す。失敗や未完了は成功と混同しない。前回の直接プロセス終了は全子孫終了の証拠ではない。

`D:/Projects/GithubIssueWidget` で依頼・計画・先行結果、`widget/package.json`、`tsconfig.json`、`vite.config.ts`、`index.html` と両 src ディレクトリを読取り照合。開始・終了時の対象 SHA を確認（終了0）。型検査・ビルド・テスト・GUI・外部操作・製品変更は未実施。版が変われば再レビューする。
