# GithubIssueWidget 引き継ぎ（2026-11-21）

## 現状

GitHub Issue を Windows 11 のデスクトップに表示するアプリを完成させる依頼を継続中。**完成は未証明であり、Issue は閉じていない。** ユーザーはオーケストレーションを要求し、同じ失敗が２回発生した場合は別のエージェントによる調査と解決を指示している。ユーザーの最新依頼は「ここまでの経過を保存」。

作業場所は `D:/Projects/GithubIssueWidget`、branch は `main`、観測 HEAD は `4961b25bec53a93439acdf5230de77f959908af0`。HEAD は未コミット・未追跡を表さず、`origin/main` の表示もリモート最新状態を保証しない。`git status --short --branch` は `AGENTS.md`、`README.md`、失敗記録、`widget/src/style.css`、`widget/src-tauri/tauri.conf.json` 等の変更と、複数の run・release・指示文書等の未追跡を示す。他者の変更、既存の worktree、ログ、生成物を保護し、所有・必要性・削除許可を確認せず変更や削除をしない。個人 Git ignore 設定の読取拒否警告がある。

2026-11-21 に公開 REST で Issue #1〜#10 がすべて open と観測された。#1〜#7 は製品と統合検証、#8〜#10 は委譲・計画・文字コードの運用改善。全件の観測結果は `docs/agent-runs/20261121-app-completion-restart/tasks/01/result-04.md` と独立 `review-01.md` を参照。取得後の変化や匿名で見えない対象は別途確認する。#1 の共有チャット本文は未取得。

## 製品と利用可能な版

- 実行ファイル: `widget/release-20261121-selection/github-issue-widget.exe`。SHA256 `0D8AB47088B7A19233EF0D7D08CF033B681B20CA0E729FEAF73EAB74DCC98F55`。検証用隔離版と同じハッシュであることを確認済み。配布・署名・インストーラーを意味しない。
- 利用案内: `widget/release-20261121-selection/利用案内.txt`。詳細は `README.md`。秘密や PAT の値をチャットやログへ送らない。
- `tauri.conf.json` に `dragDropEnabled: false` を追加して Windows の HTML ドラッグを可能にした。`style.css` ではカードと列見出しの選択を抑制し、入力欄には適用しない。カード・見出しの文字はコピーできない。README にローカル版・操作・障害時の確認を追記。統合差分と EXE の独立レビュー `product-integration-review-01.md` は対象版を承認したが、製品全体の完成を承認していない。

## 検証済みと限界

- ユーザー本人は修正版で検証用 Issue #11 の作成、全６方向のドラッグ移動、最後のクローズ、半透明の他カード文字の非表示、フォームの文字選択、トークン再入力なしの再起動を報告。60秒同期も本人が確認した。`user-observation-07.md`〜`11.md` と `sync-user-observation-02.md` を参照。これは本人報告であり、独立した画面の観測ではない。
- 修正版のウィンドウ移動・サイズ変更・最前面切替・Issue クリックによる GitHub 表示はユーザーが「すべて動いた」と報告した（`user-observation-11.md`）。初期・最小サイズの実測、初期中央・最前面、許可外 URL の実拒否や他の表示環境は未確認。
- #11 の公開イベントは全６方向に対応するラベル・状態変化を示し、最後は closed・`status:done` のみ。同期確認後もコネクタで同状態を観測（`issue-11-sync-state-02.md`、2026-11-21 06:48:55 UTC）。取得後の変更は未確認。非 status ラベルがなかったため、その実環境での保持は未検証。
- 作成の Rust 合成試験18件（うち模擬サーバー自身の１件）、移動の Rust 合成試験16件、同期 UI の模擬 IPC・仮想時計６件は成功し、それぞれ独立レビューあり。実 GitHub の通信障害・レート制限・部分失敗や実 WebView の代替証拠ではない。実 Credential Manager のアプリによる初回保存・更新・削除・障害復旧、保存済み PAT の実 IPC 非露出、Windows 開発起動は未検証。詳細は `completion-audit-01.md` と後続成果を参照。

## 合成資格情報の実 OS 試験：停止中

既存 PAT やアプリの保存済み資格情報を使わず、別名と合成値で OS API を試みたが、２回とも監督・子は終了コード1。**再送・書込み・削除は行わない。** one-shot marker、journal、outcome、ログ、対象専用領域を保護する。保存済みの実資格情報や PAT に触れない。

1. run `dcdc5dfdf4524ffa8d9139ee5a781ae4`: journal 最終 `before_write_a`、子 completed exit 1。標準エラーが保存されず直接原因と当時の書込み成否は不明。後の対象限定 read-only inspect は観測時点で `absent`、独立レビュー済み（`credential-live-execution-result-01.md`、`credential-live-residue-review-01.md`）。
2. run `67c06448b9734163a1949e63d740c6e7`: 子 completed exit 1。独立調査 `credential-os-failure-analysis-01.md` は `operation=write`、`kind=no_storage_access`、Windows 1312、最終 `before_write_a` と記録。該当合成 target の read-only inspect は観測時点で `absent`。ログオンセッション状態に関するエラーだが、その状態になった原因は未特定。１回目も同じ原因とは断定できない。`credential-diagnostic-execution-01.md` と独立調査の最終レビュー・失敗記録状況は次担当が確認する。

`absent` は検査時点の当該 target の残留に限り、過去の書込み成否や既存 credential 全体を証明しない。Windows エラーを繰り返し再現するための盲目的再実行は禁止。ユーザーが専用テスト用 PAT を使えるかの質問は未回答。値の共有を求めない。現在のアプリや既存接続を保護し、専用環境・本人操作・復旧の条件を確認する。

## 次に行うこと

1. ２回目の独立調査結果 `credential-os-failure-analysis-01.md` の版・証拠、対象の absent 検査、失敗記録とレビューを確認し、停止した run を再送せず、1312 の原因と安全な次の環境・再開条件を別担当の調査に基づき判断する。環境の変更や既存資格情報への操作を推測で行わない。
2. ユーザー回答を待ち、専用 PAT・安全に分離された環境が利用可能な場合にのみ、本人が値を直接入力する実アプリの初回保存・再起動・更新・削除を、事前の復旧条件とともに確認する。合成 OS API 試験の成否を実アプリの証拠として扱わない。
3. 修正版の初期／最小サイズ、許可外 URL の実拒否、非 status ラベル保持、開発起動・必要な実障害の残証拠を Issue 原文に照合し、安全に確認する。検証用 GitHub 書込みは承諾済みの #11 のみで、再送前に実状態を確認する。
4. Issue #8〜#10 の過去に失われた日時・生証拠を捏造せず、実際の次の委譲・状態遷移と UTF-8 読戻しを確認する。最新の全要件とコード・レビュー・ビルド・実機証拠を照合し、残件をすべて満たすまでは完成・Issue 終了を宣言しない。push、PR、merge、公開は別途の権限・指示を確認する。

この文書は既存ログとユーザー報告に基づく引き継ぎであり、過去の保存内容を上書きしない。実データ・秘密の採取は行っていない。
