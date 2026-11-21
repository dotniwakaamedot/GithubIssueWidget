# 大量ページ回帰試験の隔離案

2026-11-21、担当 `/root/issue_survey`。承認計画 `pagination-regression-plan-02.md` SHA256 `336EF903F20C0719D529751C675A88522AB6721F817C08DD124381D0738CD463` と `pagination-regression-plan-review-02.md` を確認。作業場所 `D:/Projects/GithubIssueWidget`。新規 `pagination-isolated-01/` 内に試験案を作成し、製品・Git・既存依存を変更していない。テスト・ビルド・サーバー・実 GitHub・資格情報・GUI は実行していない。

## 入力と変更

- 製品 `widget/src-tauri/src/main.rs` は開始・終了時 SHA256 `61595B68317E15FDD8F2BED88B920A468D5516740E3D3124710384709C52FC24` で一致。作業開始前に隔離先と本結果ファイルが存在しないことを確認し、新規ディレクトリへコピー。コピー直後の SHA も一致。
- 隔離 `pagination-isolated-01/main.rs` SHA256 `AEFF1E65AD93EFA04773CEBBB9736A6BDC5B9938DB9FAF937DD88B3D9CBFE86F`。製品との差分は末尾の `#[cfg(test)]`・`#[path = "pagination_tests.rs"]`・モジュール宣言のみ（`git diff --no-index` 終了1は差分あり、取得出力を確認）。
- 新規 `pagination-isolated-01/pagination_tests.rs` SHA256 `BF690E818EB182FE16395E0AE3946B043D41DADD2B75235EFEACDC4873F4DE3D`。隔離コピーは単独でコンパイルできる完全な Cargo プロジェクトではなく、他のモジュールや依存・出力先を後続実行方式で別途固定する必要がある。どちらも未追跡の新規ファイル。

## 試験案

- `mixed_pages_preserve_exact_issue_set_and_classification`: 11ページ各100要素と最終6要素。番号10と2000を PR、番号1をページ間重複とし、PR と重複を除いて固有 Issue 1,103件を期待する。Open/Closed、複数ラベル、無ラベルと各ステータスを含む。期待値は 1〜1104 から10を除いた番号集合を基に、明示例の state・status を設定し、返却数、重複なし、全番号と各分類、および PR 番号2000の不在を比較する。各ページ最大100要素、次ページリンクと `page` 値の照合を設けた。番号10の除外も期待集合に含む。
- `later_failures_never_return_the_first_page_as_success`: 初回100件 HTTP 200 の後、HTTP 500、応答前の接続切断、HTTP 200 で不正 JSON の3パターンを別々に与え、いずれも部分一覧ではなく `Err` を要求する。意図的な期限超過の再現は未実装・未検証であり、切断を採用した。
- `serve` は loopback のみ、非 blocking accept の15秒期限、接続ごとの読取り・書込み各2秒期限、8KiBの要求行上限、クライアント全体3秒期限を設定。各ケースで `fetch_pages` 後にサーバースレッドの join と結果を確認する。合成サーバーはページ数が有限であり、期限切れ・EOF・入出力失敗を `Err` にする。テストの上位プロセス全体の期限、パニックや OS 障害時の監督・子孫の確認、実際の TCP 終了は独立した実行計画で扱う。既存 `mock_pages` 自体は変更していない。

## 検査と未確認

`Test-Path` で専用領域の非存在確認、`Get-FileHash`、コピー、差分の読取りは終了0（`git diff --no-index` は差分ありの終了1）。`git diff --no-index --check` は出力なし・終了1であり、これだけで空白検査成功と断定しない。新規コードは静的に読んだのみで、Rust 構文・コンパイル・テスト成功は未確認。実行環境・既存の依存と target の隔離、合成接続・プロセスの期限、対象 SHA を固定し、独立レビューと開始通知を得るまで実行しない。実 API・同時更新・10000ページ上限・OS 画面は本案の範囲外。操作失敗は認識していない。
