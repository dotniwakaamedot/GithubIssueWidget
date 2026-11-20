# Issue #5 初期実装の独立レビュー

- 判定: **要修正（blocking）**。対象 `C:/Users/semis/orca/workspaces/GithubIssueWidget/issue-5-create-resume`、branch `dotniwakaamedot/issue-5-create-resume`、HEAD `3d906e5926f3106ce12b2f3bb3bb878a64368f64`。作者 `/root` と確認 `/root/creation_review` は別担当。
- 依頼 `tasks/issue-05-review-request-20261119.md` SHA256 `FAB992B4273AF77925D0DBCF8A5C4A35E03A49A9B4932671BFB6471AF184541A`。指定の5ファイルの実 SHA256 は依頼と全件一致。`git status --short --untracked-files=all` と `git diff --name-status` で変更4件と未追跡 `widget/src-tauri/src/creation.rs` 1件を確認し、削除は観測されなかった。確認前後の対象 SHA は一致。

## B01: 作成成功応答でステータスラベルが付いたことを確認していない

- 根拠: `widget/src-tauri/src/creation.rs` の `CreatedIssue` は 50〜51行で番号だけを読み、91〜98行の POST 成功後、121〜125行でラベルの確認なしに `created` を返す。86〜90行の事前 GET はラベルが存在することの確認であり、作成結果にそのラベルが適用されたことの確認ではない。[GitHub 公式の Issue 作成 API](https://docs.github.com/en/rest/issues/issues?apiVersion=2022-11-28#create-an-issue) は、権限条件によって指定ラベルが適用されない場合があると説明する。
- 影響: Issue 自体は作成されても `status:in-progress` 等が付かず、アプリが成功を表示し得る。たとえばラベルなし Open は Todo に分類されるため、選択した In Progress とボードが一致しない。Done も close の成否だけで成功とされ、`status:done` の存在は保証されない。これは公式仕様とコードからの障害経路であり、実 GitHub で再現したとは主張しない。
- 修正案: 作成成功後に応答または対象 Issue の再取得で対象ステータスラベルと期待状態を確認し、欠落・取得不能時は**作成済み番号とリンクを保持した部分失敗**として通知する。POST を再送せず、権限不足でラベルが省略される 201 応答、取得失敗、Done の close と組み合わせた模擬試験を追加する。

## B02: 作成済み番号を把握していても保存失敗時に UI で失う

- 根拠: `creation.rs` 103〜107行は POST 応答から番号を得た後、`pending-issue.json` の `created_open` 更新に失敗すると番号・URL 付き `partial` を返す。この場合、以前の `pending` 記録が残るならその番号は `None` のまま。`widget/src/main.tsx` 110〜112行は `CreateResult` を保持するが、136行のリンクと表示番号には `pending.number` のみを使い、`CreateResult.number` と `CreateResult.url` を表示しない。さらに記録そのものを読み取れず `refreshPending` が失敗すると、`pending` がない場合は result バナー自体が表示されず、一般的な再読込みエラーだけになる。
- 影響: 作成済み Issue の番号と URL がプロセス内で分かっているにもかかわらず、ユーザーには具体的な作成先が示されず、Issue #5 の「作成済み Issue を示す」部分失敗要件を満たせない場合がある。記録更新・読戻し失敗を実際に注入した GUI 実行は行っておらず、上記はコードの制御フローからの確認。
- 修正案: 結果受信時の番号・URL を確認記録の再読込成否にかかわらず表示し、同時に永続記録の不整合と再作成防止を維持する。記録の書込み失敗・読込失敗と再起動を注入した UI/バックエンド試験で、判明済みの Issue を示し、不要な POST を行わないことを確認する。

## 非 blocking の注意

1. `creation.rs` 88〜89行はラベル取得の HTTP 404 を一律「必要なラベルがありません」と表示する。接続先や権限の変化等でも 404 になり得るため、存在不足とアクセス不能を断定しない案内が望ましい。実環境の特定の権限設定で 404 を再現したものではない。
2. 未確認操作の表示 `main.tsx` 136行は記録された owner/repo にリンクするが、番号付きリンクの見出しにはリポジトリ名を表示しない。別ウィンドウ等で接続先が変わったときの誤認防止には、記録された owner/repo も表示することが望ましい。

## Issue 完了条件との照合

| 条件 | 静的に確認したこと・未確認 |
| --- | --- |
| ヘッダー・各列の初期値、必須入力 | ヘッダーは Todo、各列は対応値を渡し、タイトル空白を backend でも拒否する。実 Windows UI と各列の全操作は未検証。 |
| 対応ラベル付き作成と同期 | 対象ラベルの存在確認と POST payload、後続 board refresh はある。実際のラベル適用確認は B01、実 GitHub 試験は未実施。 |
| Done の close と部分失敗 | POST 後に PATCH を送り、応答番号と closed を確認。失敗時は partial とし記録を残す。失敗時の既知番号表示には B02 がある。実 GitHub と Windows UI は未検証。 |
| 未入力・通信・認証・書込み・ラベル不足 | 模擬テスト定義には空タイトル、404、403、応答喪失などがある。作者報告の `cargo test --locked` 38件、Vitest 8件、build 成功は会話上の報告であり、本担当は再実行・独立ログ照合をしていない。実 PAT・実 GitHub・Windows UI は未実施。 |
| 重複と秘密保護 | POST 前に操作 ID・owner/repo 等のみを記録し、結果不明時・再起動相当の確認まで次作成を拒否する設計と模擬テストは存在。記録の故障注入全経路、実 OS 異常終了、IPC・ログの実行時漏洩検査は未実施。 |

## 確認範囲・教訓

依頼、Issue #1/#5、差分・未追跡ファイル、5ファイルの SHA と静的コード・テスト定義、GitHub 公式 API 文書を確認。製品実行、テスト再実行、資格情報の読み書き、UI 操作、GitHub 更新は行っていない。今回の確認で新たな失敗はない。
