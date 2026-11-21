# 製品の Link 解析修正の検証

- 対象ソース `widget/src-tauri/src/main.rs` SHA256 `61595B68317E15FDD8F2BED88B920A468D5516740E3D3124710384709C52FC24`。
- Visual Studio 2022 Community の `VsDevCmd.bat -arch=x64` を適用し、`widget/src-tauri` で `cargo test --locked --offline` を実行。終了コード0、通常67件成功・10件 ignored、opener の別テスト1件成功。ログ `rust-regression-01.log`。新規テストは relation 属性名と登録 relation 値の大文字小文字と複数値を検査する。既存テストは引用符内テキストの誤認を検査する。
- 同じ環境で `widget` から `npm.cmd run tauri -- build --no-bundle` を実行し終了コード0。ログ `release-build-01.log`。リリース EXE の SHA256 は `9157AB9B23BA0DD5406E5C85044BAF278AAD8A5066BCB15E40AD88761348073B`。
- 対象版での Windows UI、実 GitHub、Credential Manager、ignored の試験は未実施。GitHub の実際のレスポンスに大文字の relation が存在するかも未確認。
