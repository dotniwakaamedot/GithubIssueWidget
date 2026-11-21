# GithubIssueWidget 進捗・引き継ぎ（2026-11-21）

## 現状と方針

- **MVP は未完成。** GitHub Issue を Windows 11 上に表示し、作成・列移動する Tauri 2 / React アプリは実装されている。現行 release のビルドは成功。ユーザー提供画像で接続設定画面と `dotniwakaamedot/GithubIssueWidget` の一覧表示を確認したが、画像は EXE の版や書込み権限を証明せず、実 GitHub での作成・移動は未検証。
- ユーザーは完成度より早期利用可能な MVP を優先し、**GitHub の最新状態を正とする**ことを指定。競合時に自動上書き・再送をせず再取得し、一部反映の可能性を知らせる。複数 API 書込み間の同時変更を完全には防げない。
- ユーザーはオーケストレーション専任と、同じ失敗が２度起きた場合の別エージェントによる調査・解決を指定。成功済みの同一版・同一目的のテストは、変更や新たな懸念なしに繰り返さない。計画・レビューの反復より使用開始を優先する。
- ユーザーは現在の作業をコミット・プッシュし、次のセッションに引き継ぐことを依頼。コミット・プッシュの結果はこの記録の作成時点では未確認であり、次回は `git status` とリモートを確認する。PR 作成やマージの許可と混同しない。

## 対象版・変更

- 作業場所 `D:\Projects\GithubIssueWidget`、`main`、記録更新時の HEAD `5e74e1642fd18fe630dce0e684afd853721276ae`。`origin/main` を追跡。コミット後の HEAD は次回確認する。HEAD だけでは未コミット変更の版を識別できない。
- `widget/src/main.tsx`: 取得開始時刻を基準とする同期、作成応答不明時の一覧再取得。`widget/src/main.test.tsx`: それらと移動失敗後の再取得失敗の回帰テスト。
- `widget/src-tauri/src/movement.rs`: 最終状態不一致時に部分反映の可能性を通知し、競合の回帰ケースを追加。`widget/src-tauri/src/main.rs` と新規 `pagination_tests.rs`: 既存変更を保護しつつ大量ページ・後続失敗のテストを追加。`README.md`: 起動・基本操作と PAT の権限・安全な入力の案内を追記。
- 記録更新時点では、**上記を含む変更は未コミット。** 他にも `docs/failure-log-001.md`、`docs/recurring-lessons.md`、`docs/agent-runs/20261121-goal-continuation/` 内のスクリプトと pycache に既存変更がある。`docs/agent-runs/20261121-issue-completion/`、`docs/agent-runs/20261121-orchestration-continuation/`、２つの `widget/release-20261121*`、`pagination_tests.rs` は未追跡。EXE、pycache、未追跡 run はコミット対象外としてローカルに保持する予定。Git はユーザー ignore ファイルのアクセス拒否警告を出したため、ignored を含む全体の不存在は確認できていない。コミット後の現状は `git status` と `git log` で確認し、既存の変更・未追跡・他の worktree を勝手に戻さない。

### 現在の主な SHA256

| 対象 | SHA256 |
| --- | --- |
| `README.md` | `3D2A457B647B8B514866A693A49E6D70AA948C68B38F9BBA17CE439D5DF8CB9E` |
| `widget/src/main.tsx` | `B22504C9DE48D275726C912820DA93E02F10C1838CCC68555C156D35030165D7` |
| `widget/src/main.test.tsx` | `10AA28AD8D343F969C8BDB8C2D2232BF6007CB926619CB49850020DD53A40FBD` |
| `widget/src-tauri/src/main.rs` | `AEFF1E65AD93EFA04773CEBBB9736A6BDC5B9938DB9FAF937DD88B3D9CBFE86F` |
| `widget/src-tauri/src/movement.rs` | `88BF21BB45C077DFDA592D906C753D168C5641E35EB12324D4ACF1652374695B` |
| `widget/src-tauri/src/pagination_tests.rs` | `BF690E818EB182FE16395E0AE3946B043D41DADD2B75235EFEACDC4873F4DE3D` |
| `widget/release-20261121-02/GitHubIssueWidget.exe` | `D19A72750006DF5BB6CBCA4FB15F87F69BFF48A5785F5569C5B05E40C9B93F26` |

## 実施済み検証と限界

- 直前版の UI モック Vitest 39件成功、型検査と Vite build 成功。追加で移動不明後の再取得失敗 UI 1件、作成応答不明後の再取得 UI 1件、移動競合 Rust 3件、大量ページ Rust 2件がそれぞれ限定実行で成功し、独立確認された。新規テストでは他の既存ケースを無目的に再実行していない。UI の作成試験はモック IPC の回数であり、実 HTTP POST の回数を証明しない。
- 大量ページは合成 12ページ・1,103 Issue の番号、分類、PR／重複除外と、後続ページの HTTP 500・切断・不正 JSON を検証。実 GitHub の大規模データ・同時更新の証明ではない。
- 上記最新版からの `npm.cmd run tauri -- build --no-bundle` は専用環境で終了0。配布 EXE は上記 SHA、前版 `widget/release-20261121/GitHubIssueWidget.exe` は上書きせず保持。ビルドとファイルアクセス権の確認は画面・認証の成功を意味しない。
- 旧 release は空設定起動で終了101が２回、原因不明。後の別 release と診断版は直接プロセスの生存を確認したが CUA が対象ウィンドウを識別できず、表示や通常終了は未確認。各自己起動の直接子は実体と作成時刻を照合して終了した。診断版は `before_run` のみ記録、setup 未到達と記録失敗は区別できず、原因は未特定。未知の子孫や旧合成子の全終了は未確認で、不明プロセスを推測して終了しない。

## 次に行うこと

1. ユーザーから「表示されました」と報告され、接続設定画面の画像を受領した。前後の会話では `widget/release-20261121-02/GitHubIssueWidget.exe` の起動を依頼していたが、報告は版の指定がなく、画像自体も EXE の版を証明しない。再起動を無目的に繰り返さず、必要な場合に版を確認する。PAT を会話やログに求めない。
2. ユーザー提供画像で対象リポジトリの Issue 一覧表示を確認した。画像は EXE の版、Issue の網羅性や書込み権限・作成／移動の成功を証明しない。必要な PAT は本人がアプリに直接入力する。README の案内は fine-grained PAT の Metadata 読取と Issues 読書。実 credential の保存・更新・削除・再起動後復元、設定やログの秘密非露出は未検証。
3. 既存承諾は公開 `dotniwakaamedot/GithubIssueWidget` に検証用 Issue を１件作成し、移動後に閉じる範囲。実行前に対象・現在の権限を確認し、結果不明の POST を再送しない。実 GitHub と実 UI での作成・移動・ラベル保持・競合を確認する。コネクタで repo 読取りと `status:in-progress` は確認、`status:todo` と `status:done` の現在の存在は取得不能で未確認。取得失敗を不在扱いしない。
   直近の実操作担当は、作成ボタンの座標取得エラーでフォームを表示できず、Tab 操作は中断され結果不明と報告した。Issue 作成の送信や外部書込みは行っていない。アプリの現在状態は未確認であり、再開時に確認する。
4. Windows の開発起動・正常終了、画面移動・リサイズ・最前面・リンク、実認証・権限・通信障害等、Issue #1〜#7 の未確認要件を現行版に紐づける。実サービスの障害や大量データを無理に作らず、合成結果と実環境を区別。#6 の単一 PATCH 希望に対し現在は非ステータスラベル保護のため複数の個別操作を使う設計差が残る。

Issue #1〜#7 の本文は個別確認済みだが、Issue 一覧・関連 PR の全件、#1 が参照する外部共有チャット、旧 worktree 担当の現在の停止と旧変更の完全な同値性は未確認。GitHub コネクタの権限表示はアプリに入力する PAT の権限を証明しない。既存作業を保護し、取得不能を不存在・成功扱いしない。

今回の整理では、解決済みの中間質問17件を削除し、検証結果・ログ・実行ファイル・他の worktree は保持した。保持した後続文書には、削除された質問への参照が一部残っている。`docs/failure-log-001.md` と `docs/recurring-lessons.md` の参照先には未追跡のローカル資料が含まれ、別 checkout には存在しない場合がある。
