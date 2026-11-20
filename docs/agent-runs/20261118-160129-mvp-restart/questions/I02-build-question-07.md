# I02 B03 進捗 07

- 担当 `/root/build_verification`、宛先 `/root`。`I02-build-answer-06.md` に従い、ASCII の Node スクリプトから17件の相対パス・SHA の機械可読 manifest を生成し exit 0。元一覧 SHA `520044FE6138609B99595C5FAF1D3973DD2D62EB82F27FE90392568C9599ED13`、生成 manifest SHA `BB5253F1D02C83E183C86A6981CA3768A01892E3D1B40EF3AC4AA6A014DE0033`。
- 元17件の全存在確認・SHA照合後、通常バイトコピーを scratch の `source/` に作成し、元・コピーの17件が全件一致。`copy-source-02.ps1` exit 0、copy manifest SHA `230F07C865D4A0F37C0D2ED7408F3D5A0086832E16BC0A76EAA444B14165D9A4`。元の製品ファイルは変更していない。
- npm lockfile: 134 package、取得元 host `registry.npmjs.org`、install script は esbuild と fsevents。既存 Windows 環境では esbuild のスクリプトを読取済み、fsevents は未導入。Cargo registry の `build.rs` は38ファイルを発見し、process・filesystem 等の静的調査を実施中。必要な全依存・間接コードの安全性を証明したわけではない。
- 専用 rustup と Cargo 領域を新設し toolchain を取得するスクリプトを準備したが、まだ実行していない。環境変数は子プロセス限定とし、秘密らしい名前の変数を除外、既存の資格情報をコピーしない設計。
- 前回報告したコピー前の抽出失敗1回以降、セクション抽出とコピーの新たな失敗はなし。最初の読み取り検査が対象行0件を見落とした誤観測は質問06に記録済み。依存導入・build/dev は未実行。引き続き必要な前提を検討する。
