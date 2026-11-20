# Issue 分類修正の結果

- 目的: Issue #1 のステータスラベル優先と、ラベルなしの場合の Open/Closed による振り分けに合わせる。以前は Closed を先に Done とし、`status:todo` 付き Closed を Todo に分類しなかった。仕様解釈とカード移動への影響は独立確認中。
- 対象 `widget/src-tauri/src/main.rs` SHA256 `6FCE6E266D8FA7B3E0703879E7437B0D0A2C4D5B9FCE7D7E0AFF0F3F8CB1A1B2`。ラベル優先順を先に判定し、該当なしのときのみ state を使う。Closed＋Todo、Closed＋複数ラベル、ラベルなし両状態と PR 除外を検証した。
- Windows、`widget/src-tauri`、`cargo test --locked --offline board_classification_prioritizes_status_labels_and_excludes_pull_requests`: 終了コード0、1 passed。`cargo test --locked --offline`: 終了コード0、65 passed・10 ignored、別の URL テスト1 passed。後者のログ `classification-regression.log` SHA256 `65FBF8049DD9A0B3C2DE127EEA0E888E5D74CF56F93AC539ACE05601F786F37F`。
- Windows、`widget`、`npm.cmd run tauri -- build --no-bundle`: 終了コード0。ログ `classification-release.log` SHA256 `810999A094B6876054177789237352D139E31A8F2FF0BB6620CB36C3B5CD21B2`。生成 EXE SHA256 `145617E3D1AFA98E9E2FE8BDF825B2A218BDA1E1FD5C9CFBDD0191F3315CF3C2`。
- 実 GitHub での対象 Issue・実画面表示、同列へのドロップによる state 調整は未検証。生成版の起動も未確認。依存するハーネスの実 GitHub 書込みは引き続き無効。
