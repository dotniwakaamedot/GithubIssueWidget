# GUI 起動設定の限定確認

担当 `/root/existing_work_review`。読み取りのみで、製品・ユーザーデータ・プロセスに変更や操作はしていない。

## 確認された事実

- `tauri.conf.json` は main ウィンドウを定義し、`visible` を明示的に false とする指定はない。900×520、最小600×300、中央、最前面、装飾なし、リサイズ可。これは Tauri の未指定時の実際の動作や画面表示成功を証明しない。
- `main.rs` の `main()` は Tauri builder に状態・IPC・opener を登録し `.run(tauri::generate_context!())` を呼び、エラーなら `failed to run widget` で panic する。明示的なアプリ起動時の hide 呼出しや独自 setup callback は、この短い範囲に見当たらない。ウィンドウ生成や WebView2 の内部動作全体は未確認。
- `vite.config.ts` は開発サーバー 1420、`tauri.conf.json` の devUrl は localhost:1420、release の frontendDist は `../dist`。HTML は `#root` と `/src/main.tsx` を指定する。設定値だけではビルド済みバンドルの内容やブラウザー実行の成功は証明できない。
- `gui-diagnostic-observation-result-01.md` は異なる診断 EXE で `before_run` だけを記録。setup 未到達と書込み失敗を区別できず、通常ユーザーでの起動失敗の原因は未特定。**この限定範囲で起動不能を説明する確定的な設定ミスは特定できない。** 他の設定・依存・実環境は未確認。

観測時 SHA256: `tauri.conf.json` `5C1F3A0E2E487DD4345A2DD335FE4AACA483FEEEA26C11AAB99B374278EC1364`、`main.rs` `AEFF1E65AD93EFA04773CEBBB9736A6BDC5B9938DB9FAF937DD88B3D9CBFE86F`、`main.tsx` `B22504C9DE48D275726C912820DA93E02F10C1838CCC68555C156D35030165D7`、`vite.config.ts` `1DAFF9E73019BD4E74489811D13553B3D227DB8457910D7F295E60AD0EB4CC7A`、`index.html` `14DE8CE58EF498F1BBE934D42DBADD9120B0052724F03FFFE532A31DC403D97E`。main.tsx は前回報告時と異なり、作業中の他担当変更を保護し現版をそのまま記録する。

## 操作失敗と引継ぎ

`Cargo.lock` 内の package 名と版を対象にした `rg` 検索は、PowerShell の引数引用が意図と異なって伝わった可能性があり、結果として作業ディレクトリの広域検索になり ACL 保護フォルダーのアクセス拒否を返し終了1。Cargo.lock の内容は取得できず、検索の完全な引数解釈と原因は未確定。保護フォルダーの内容は出力・変更していない。過去にも同種の保護先への広域検索が報告されているため、ユーザーの2回条件に従いこの担当は同じ検索や追加調査を止め、別担当へ原因分析・不足確認を依頼する。失敗記録と既存教訓への反映を管理に依頼する。上記の他の個別読取りとハッシュは終了0。
