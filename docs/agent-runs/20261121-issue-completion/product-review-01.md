# 製品 Link 解析の独立レビュー 01

- 判定: **指定された修正範囲を承認**。作者 `/root`、確認者 `/root/parser_review` は別担当。`main.rs` の SHA256 は開始時と終了時に依頼値と一致した。本承認は対象版の指定差分に限り、アプリ全体や実環境の完成判定ではない。

## 完了条件との照合

- `link_has_next` は引用符とエスケープの状態を追跡して属性を区切り、属性名の `rel` と登録 relation 値の `next` を ASCII の大小文字を区別せず比較する。新規 `next_relation_is_case_insensitive` は `REL="NEXT"` と `ReL="last NeXt"` を、以前からの `quoted_parameters_do_not_forge_a_next_relation` は引用値内の `rel=next` の誤認防止と正規 next を検査する。差分を確認した。
- `next_page` は next URL の scheme・host・port・repository path・userinfo・fragment、許可するクエリ名・重複・空値・必須値・長さを検査する。`fetch_pages` は既出 URL と取得上限を検査する。`next_page_cannot_change_host_or_repository` は別ホスト・別 repository を拒否する。これは全入力を検証したという意味ではない。
- `rust-regression-01.log` は通常 67 passed・10 ignored、opener 1 passed を示す。終了コード0は `product-test-result-01.md` の実行者記録に基づく。今回、テストやビルドの独立再実行はしていない。実 GitHub 応答、Windows UI、Credential Manager と ignored テストは未確認。
- 対象ファイルは Git 上で変更済み、削除対象の指定なし。レビュー中に製品ファイル・実データを変更していない。blocking 指摘および操作失敗なし。

## 対象と証拠

- `widget/src-tauri/src/main.rs` SHA256 `61595B68317E15FDD8F2BED88B920A468D5516740E3D3124710384709C52FC24`。
- `rust-regression-01.log` SHA256 `43EBD3CC248121C32E1D7B86C9495AFC17870767C6C3359B5B3EF3B65587C695`。対象版との対応と終了コードは管理者の記録に依存する。
