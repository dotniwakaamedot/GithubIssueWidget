# 現在の状態記録（2026-11-20）

## 結論

アプリは未完成であり、ゴールは active である。GitHub API の読取り・Issue 作成・全6方向の状態変更とリリースビルドは確認済みだが、Windows のウィンドウ移動とサイズ変更の実機検証が未完了である。直近の2回のドラッグ操作では、位置・サイズのいずれも変化を観測できなかったため、同種失敗として停止を報告した。

## 実装版

- 作業場所: `C:/Users/semis/orca/workspaces/GithubIssueWidget/issue-6-move-resume`
- 基準コミット: `ab6122ef26bd62d5ec8dffbda1d35cd1a09606e0` (`Add provisional issue status moves`)
- 未コミット変更: `widget/src-tauri/src/main.rs` と `widget/src-tauri/src/movement.rs`。いずれも実 GitHub 検証用の ignored テスト追加であり、通常テストでは実行されない。
- リリース実行ファイル: `widget/src-tauri/target/release/github-issue-widget.exe`、SHA256 `F704EFA24350D7295F5C3FABBF92818A7421A598865551A21C9DA53B877EFB65`。

## 確認済み

- Rust 自動テストは、実 GitHub 用 ignored テストを除く時点で `46 passed / 4 ignored`。画面側 Vitest は `18 passed`。`npm run build` と `npm run tauri -- build --no-bundle` は成功。
- テスト用リポジトリ `dotniwakaamedot/orcaDiversion` では、テスト Issue #13 を1件作成し、アプリと同じ実装で Todo / In Progress / Done の全6方向の状態変更を確認した。現在 #13 は `OPEN`、`status:todo`。
- アプリの接続確認と全ページ取得を実 GitHub に対して実行し、#13 を Todo として取得・分類できることを確認した。
- Windows Credential Manager には、UUID付きの合成テスト資格情報を登録・取得・更新・削除する ignored テストを実行し成功した。実トークンは使用していない。
- Windows でリリース版を起動し、接続設定画面の表示と最前面固定ボタンの表示切替を確認した。セッションはその後ロック解除済み。

## 未完了・未検証

- タイトルバーのドラッグ後にウィンドウ位置は変化せず、右下端のドラッグ後にもサイズは変化しなかった。原因は、アプリのドラッグ処理、入力送達、DPI / 座標変換、または Computer Use 側のいずれかと考えられるが、未特定。
- 移動・リサイズ・最小サイズ・中央表示・許可 URL / 拒否 URL の実機確認、実 UI 経由の PAT 保存・再起動・接続先切替、通信障害・レート制限・競合の実機統合確認は未完了。
- Issue #1〜#10 は open。#2〜#6 は `status:in-progress` が付与されている。実装が main に統合・PR 承認・Issue close された状態ではない。

## 失敗の扱い

直近のウィンドウ操作では、移動とサイズ変更でそれぞれ期待した変化が確認できなかった。ユーザー指定の「同じ失敗を2回繰り返したら別エージェントに調査、その後さらに2回で停止」に従い、同種操作の追加試行は行わない。再開する場合は、入力到達・Tauri 呼出し・OS のウィンドウ状態を別々に記録できる計測方法を先に設計・独立レビューする。
