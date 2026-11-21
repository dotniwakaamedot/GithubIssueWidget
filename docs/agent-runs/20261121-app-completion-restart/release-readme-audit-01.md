# 利用開始に向けた配布物・説明の確認

## 結論

現行修正を含む Windows 用 EXE は隔離ビルド済みで、本人がカード移動とドラッグ像の改善を報告している。ただし現物は作業用 run 内にあり、README に示された通常のビルド出力先とは異なる。**この EXE を一般配布済み・全要件達成済みと扱わない。** 継続利用のための安定したローカル受渡先と説明を用意し、残る実機・障害系の検証を完了する必要がある。

## 対象と一次要求

- 担当 `/root/release_audit`、依頼 `release-readme-audit-request-01.md` SHA256 `46632D9D67BBDAFB38B4DC570DD41283B3892FC5A7BE5BDB00EC264EA45E8BCC`。作業場所 `D:/Projects/GithubIssueWidget`、branch `main`、HEAD `4961b25bec53a93439acdf5230de77f959908af0`。HEAD のみでは未コミットを識別しない。
- `README.md` SHA256 `3D2A457B647B8B514866A693A49E6D70AA948C68B38F9BBA17CE439D5DF8CB9E`。`widget/src/style.css` `FFE871A15BDA8FAD0BB5E28E1717C93D2878ABB1F5885F730EC624DEC983FCFE`、`widget/src-tauri/tauri.conf.json` `0132FCC92345F5D60DE84FD25197FCE853044AD63DAE28757AB4592CC72B991E` は依頼と一致し、後者2件は未コミット変更。
- 公開 GitHub REST で #1、#2、#3、#6、#7 を番号指定で直接取得し全て HTTP 200。要求は Windows 11/Tauri、実行・ビルド手順、接続と PAT 保護、ボード同期・作成・移動、障害と競合の検証、実環境・対象版の記録。#1 の外部共有チャット本文は本調査で取得していない。
- 対象 EXE `docs/agent-runs/20261121-app-completion-restart/selection-build-sandbox-01/cargo-target/release/github-issue-widget.exe` は実在し 12,885,504 bytes、SHA256 `0D8AB47088B7A19233EF0D7D08CF033B681B20CA0E729FEAF73EAB74DCC98F55`。`selection-build-result-01.md` の SHA256 は `471185044A0D807DE1AF2D8799CE0C9095D27334ABCB7A1E2E69314CB43E371D`、`selection-build-review-01.md` は `90D125A70C4497885004CA523CEDA48CD638F243871673ABFBCB0D1487378B97`。ビルド記録は `npm.cmd run tauri -- build --no-bundle -- --locked --offline` 終了0、独立レビューは入力一覧と EXE の照合を記録。ログから直接実機動作を証明するものではない。

## 説明の対応と不足

1. README は現状「開発版」「一部の Windows 統合検証は未完了」と明記し、Windows 11、Node/npm、Rust、MSVC、Windows SDK、WebView2、開発起動・no-bundle ビルドと通常の出力先を説明する。ビルドツールが不要な**既存 EXE の利用**と開発者向けビルドの前提を分け、実際に受渡す版の場所と SHA256 を追記する必要がある。現在の隔離 EXE のパスは run 内であり、README に書かれた `widget/src-tauri/target/release/github-issue-widget.exe` と異なる。通常パスに最新版があると推測しない。
2. README は対象限定 Fine-grained PAT、Metadata 読取・Issues 読書、接続 GET が書込み権限を保証しないこと、保存先と秘密を共有しないことを案内する。入力手順と空欄時の保存済み値、Credential Manager の更新・削除の説明は既存。実 Credential Manager の登録・更新・削除、エラー時の実復旧、ログ・実 IPC の秘密非露出はなお未確認であるため、現在の実機証拠を越えて安全性を保証しない。
3. README はラベル事前作成、60秒同期、開閉と分類、作成画面、カードのクリックとドラッグ、部分反映・競合時の GitHub 状態確認を説明する。競合保護を優先する複数 API 操作と部分反映は既に明記されているが、#1/#6 の可能な限り単一 PATCH という要求との差は製品判断と合わせて追跡する。通信失敗・認証/権限不足・レート制限時の具体的な再確認方法、作成応答不明時に再送前に対象 Issue を確認する手順と、復旧できない場合の案内を利用者が追えるよう必要な範囲で具体化する。
4. `.board-column h2, .issue-card { user-select: none; }` によりカードと列見出しの文字は選択・コピーできない。フォーム全体の選択禁止ではない。利用上の制約として簡潔に記載する候補。カード自体を別の列にドラッグし、列の `+` は新規作成であることを明確にすると誤操作を防げる。ドラッグ中の見た目の改善は本人報告であり、すべての Windows 環境の保証ではない。
5. `user-observation-07.md` は修正版のドラッグ像・移動の本人報告、`user-observation-08.md` と `issue-11-review-01.md` は #11 の移動・close と GitHub の6方向に対応する履歴を記録。非 status ラベルがないため実 GitHub での他ラベル保持は未証明。`user-observation-09.md` は再起動後にトークン再入力なしで開いたことを記録するが、当該報告だけから一覧取得成功を断定しない。`user-observation-10.md` は作成フォーム内の文字選択の本人報告。旧版で確認したウィンドウ操作やリンクの結果を修正版の網羅的な回帰と混同しない。

## ローカル受渡し案

- 提案先は既存の `widget/release-20261121` と `widget/release-20261121-02` を上書きしない `widget/release-20261121-selection/`。確認時 `Test-Path` は False。実行直前に再度存在・所有・書込み範囲を確認し、既存なら別名を決める。run は調査・ビルド証拠として残す。
- no-bundle のためインストーラーを作ったとは言わず、検証済み EXE を専用の新規ディレクトリへコピーし、元・コピーの SHA256 が上記値で一致することを確認する案。適用する場合は名前・バージョン・取得元・ハッシュ・必要な Windows 11/WebView2 Runtime と既知の制約を一緒に示す。OS 別 PC への展開・署名・配布・インストール動作は本調査では確認していない。コピー先での起動も未検証。外部公開、push、PR、merge は別の権限と手続きに従う。
- README は変更のため別担当に所有範囲を指定し、更新前後の対象版・正確な起動先・残る未検証を照合して独立レビューする。成果物は受渡し先の実体と SHA を対象版として記録し、設定・Credential Manager 内の既存データを上書き・削除しない。実行可能な方法や依存が異なる環境では未確認として明記する。

## 実行・未確認・変更

- 作業場所は上記 root。`Get-Location`、対象の `Get-Item`・`Get-Content -Encoding UTF8`、`Get-FileHash -Algorithm SHA256`、`git branch --show-current`・`git rev-parse HEAD`・限定 `git status --short`、`Test-Path` は終了0。Git は個人 ignore 設定の読取拒否を警告したが指定対象の状態を返した。`rg` は対象 `main.rs` に限定し設定保存パスを確認、終了0。公開 REST 番号指定取得は Python プロセス終了0、各 HTTP 200。秘密・本文の保存や GitHub 書込みなし。
- 本調査ではアプリを起動せず、ビルド・テスト再実行も行っていない。外部共有チャット、実 Credential Manager の全操作、実 WebView の非露出、60秒同期・1000件超の実環境、通信/権限/レート制限/競合障害、Windows 11 の未実施項目や別 PC の配布は未確認。Issue #7 の全条件の完了を主張しない。
- 新規変更は本結果のみ。自己ハッシュは確定後に管理が取得する。削除対象なし。既存 EXE、README、製品、資格情報、Git と GitHub を変更していない。今回認識した作業失敗なし。
