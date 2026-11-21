# 対話セッションでの合成資格情報テスト引き継ぎ

この文書は `docs/handoff-20261121-latest.md` の補足。アプリ完成と Issue #3 の完了は未証明である。既存の未コミット・未追跡ファイル、旧 run、資格情報を保護し、削除・上書きしない。

## 背景と準備

合成資格情報の実 OS テストは２回とも終了コード 1 だった。１回目は標準エラーを保存しておらず直接原因は未確定。２回目は `operation=write`、`kind=no_storage_access`、Windows コード 1312 を記録した。セッションがその状態になった理由や、両回が同一原因かは未確定。２つの記録済み合成ターゲットは、それぞれの終了後の読取り専用 inspect で `absent` と確認された。旧 run の one-shot、journal と outcome は保護し、再送・推測削除しない。根拠は `credential-live-residue-review-01.md` と `credential-os-failure-analysis-01.md`。

旧版はコンパイル時に元ディレクトリを固定参照するため、`widget/credential-check-20261121/` を manifest の場所として再ビルドした。`interactive-test-prepare-01.md` とハッシュ転記訂正 `interactive-test-prepare-correction-01.md`、実行手順修正 `interactive-test-guide-fix-02.md` を参照。修正版の独立 `interactive-test-review-02.md` は対象版を承認したが、実 OS テストやアプリ完成は承認していない。

本人に提示した手順は `widget/credential-check-20261121/実行手順.md`、SHA256 `A6F45828F142FDDF7B29F2BEEE7AC9FDCC9428E3A7281BC4CF1A29236DF0EB97`。通常ログオンした Windows の PowerShell から、手順にある Python の確認と合成テストを１回実行するよう依頼済みで、本文書の確定時点では本人の回答を受け取っていない。手順の Python が本人環境で使えるか、対話セッションで 1312 が解消するか、テストが成功するかは未確認。手順にある実行対象と SHA の確認に失敗したら実行しない。

## 次の作業

本人から表示された状態と run ID を受け取り、対象版、監督結果、journal と削除後の不存在を照合する。異常・結果不明なら再送・手動削除・記録移動をせず、終了状態と一意の対象を確認し独立調査する。値やトークンを受け取らず表示・記録もしない。この試験は PAT を使用しない合成 Credential Manager テストであり、実アプリによる PAT の初回登録・更新・削除、実 WebView IPC、別環境での動作を証明しない。実アプリの認証検証に使える専用テスト用トークンについても本人の回答待ちである。

製品の修正版、実機での移動・同期・ウィンドウ操作、利用案内、未検証事項は前掲の最新引き継ぎと各確定結果を参照する。観測後の外部状態を推測せず確認する。
