# IPC 試験の service 拒否の実行時検証

- 対象 `widget/src-tauri/src/main.rs` SHA256 `804A285387730D80A27EA6416A42F4B2A15710E02FEA2F2F72010F61D6A60C65`。
- 事実: `entry` の test-only 分岐は試験用 prefix と正規 UUID を検査し、無効な service を固定エラーで拒否する構造。一方、`windows_ipc_guard_child` は親からの正常な試験用 service でのみ `read_token` と IPC を実行し、製品 service 名・不正形式の service が拒否されること自体の assertion は見当たらない。
- 影響: 承認計画の「実アプリ用 service を指定できないことを静的・実行時に検証」のうち、現行試験が成功しても拒否分岐の実行時証拠は得られない。現行分岐の安全上の不具合は再現していない。
- 推奨: service の検証を Windows credential に触れない純粋関数などとして切り出し、製品 service 名や不正 prefix・UUID を固定メッセージで拒否し、正規の試験用値を受け入れる否定・肯定テストを追加する。拒否試験が失敗した際に実 credential を検索・削除するコードへ進まない構成とする。
- 本担当はテスト・GUI・Credential Manager・外部サービス操作および製品変更を行っていない。
