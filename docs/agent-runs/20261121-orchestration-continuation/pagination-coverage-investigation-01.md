# #4 ページネーション・分類の既存カバレッジ調査

2026-11-21、担当 `/root/issue_survey`。依頼 `pagination-coverage-investigation-request-01.md` SHA256 `E16AE27B460687BEE4EE94CBCE27E65517970D0AC33755B5DE9CC8DB938065F0`。GitHub コネクタで Issue #4 本文を取得し、現行 `widget/src-tauri/src/main.rs` SHA256 `61595B68317E15FDD8F2BED88B920A468D5516740E3D3124710384709C52FC24` と既存ログを読取り照合。開始時・終了時ハッシュは一致。テスト、通信、製品・既存出力の変更は実施していない。

## 結論

現行コードには 1,050 件・11 ページを取得する合成テストがあり、対象ソース版に対応する既存実行記録では当該テストを含む通常 Rust テストが成功している。ただし大件数テストの確認値は件数のみで、全 Issue の番号・重複・分類を同時に照合していない。実 GitHub での1,000件超、途中通信断やレート制限、実 UI は証明されない。

## 要件と実装

- Issue #4 は Open / Closed 全ページ、PR 除外、複数ラベルの優先順位、100件超・1,000件超、途中失敗・レート制限、60秒同期と接続切替中の競合を要求する。
- `fetch_pages` は `state=all&per_page=100&page=1` で開始し、`Link` の next を検証して追跡する。重複 URL または10,000ページ上限で全件未取得のエラー、同一 Issue 番号は重複排除、PR を除外、状態・URL 不正はエラー。いずれかのページがエラーなら蓄積途中の Vec を返さない。10,000 はページ数上限であり、件数を無制限と主張できない。ページ取得中の外部更新に対して完全な時点固定スナップショットを保証しない。
- `classify_issue` は closed を done、open の status ラベルは todo・in-progress・done の順に優先し、ラベルなし open は todo とする。`fetch_pages_checked` は全取得後に接続スナップショットの確認へ進む。表示と周期・切替は React 側も関係するため本調査のみでは完了としない。

## 現行版のテストと証拠

| テスト | 合成入力・確認値 | 限界 |
| --- | --- | --- |
| `fetches_more_than_one_thousand_issues` | loopback `mock_pages` で10ページ×100件、最終50件、全て open・ラベルなし、HTTP 200 と next。返却長1050を確認。サーバーは各 `page` を照合。 | 各番号の網羅・並び・重複なし、closed・PR・ラベル混在はこのテストで確認しない。 |
| `board_classification_prioritizes_status_labels_and_excludes_pull_requests` | 分類を open の複数ラベル、ラベルなし、closed のラベルなし・複数で直接検査。別途1ページ3要素（open、closed、PR）から2件を返し closed は done と確認。 | 複数ページにまたがる混在、実 API は未検証。 |
| `later_page_failure_never_returns_partial_results` | 1ページ目 open 100件の HTTP 200、2ページ目 HTTP 429。結果がレート制限エラーであることを確認。 | 実サーバーや HTTP 500・切断・不正 JSON の後続ページでの挙動はこのテストの対象外。 |
| `short_page_with_next_still_fetches_all_pages` / `follows_repository_id_cursor_without_reconstructing_it` | 1件の短いページ2枚、または repository ID の cursor を含む next の2枚を合成。返却長2、後者は次要求の cursor も確認。 | 大量混在データ・GitHub の現行レスポンス全形態は未証明。 |
| `duplicate_page_link_is_an_error` / `secondary_limit_403_starts_cooldown_before_any_retry` | 同じ page への next をエラー、初回403二次レート制限をエラー・待機中と確認。 | 循環の全形態と実際のレート制限は未検証。 |

`rust-regression-01.log` SHA256 `43EBD3CC248121C32E1D7B86C9495AFC17870767C6C3359B5B3EF3B65587C695` の各該当行は分類・429・1050件を `ok` と記録。`product-test-result-01.md` は上記 main.rs SHA に対応して `cargo test --locked --offline` 終了0、通常67件成功・10件 ignored、Opener別1件成功と記録。今回の再実行ではなく、対象版対応と終了コードは既存実行者記録に依存する。旧実 GitHub 読取りは10件で Closed・PR 0件（`docs/handoff-20261121.md`）；大量データやそれらの実分類の証拠にはならない。

## 具体的な追加検証案

1. 変更担当と対象版を別途確定し、合成サーバーで 11 ページ・1,050件以上に Open、Closed、PR、複数・なしラベルを混在させる。期待する Issue 番号集合、総件数、重複なし、状態ごとの分類を個別に照合する。PR も API のページ枠に含まれる前提で期待値を作る。ページ境界の重複番号と重複除外も明示する。
2. 100件成功後の2ページ目で HTTP 500、途中切断・タイムアウト、不正 JSON をそれぞれ試し、成功の部分一覧ではなくエラーを返すことを検証する。429 と403二次制限の既存試験を維持し、画面側では失敗と旧データの扱いをモックで照合する。
3. 繰り返し URL・不正 next と取得上限について既存の境界ケースを精査し、上限の合理的な注入手段がある場合のみ高速な境界試験を設計する。10000件の実 HTTP 要求を無条件に追加しない。
4. 合成サーバーの `mock_pages` は `TcpListener::bind("127.0.0.1:0")` とスレッドを使い、accept/read に期限を設けていない。新規試験や実行前に無期限待機とプロセス監督・専用出力を設計・独立レビューする。実 GitHub 検証は別途承諾・認証・対象・秘密保護と環境確定後とする。

追加テスト・製品変更の所有範囲は、未コミット `main.rs` と並行担当の保護・所有権確認を行った上で個別に決定する。現時点では提案のみで、アプリの欠陥や追加テストの成功を断定しない。

## 読取り・未確認

コネクタの #4 取得と `Get-FileHash`、対象範囲の `Get-Content`、限定 `rg` は終了0。最終 SHA を再計測した。調査中の失敗は認識していない。大規模な実 GitHub 応答・レート制限・途中障害、実 Windows UI、未読の他テストによる追加カバレッジは未確認。
