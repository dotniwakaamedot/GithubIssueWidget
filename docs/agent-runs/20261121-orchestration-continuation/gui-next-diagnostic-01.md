# GUI 観測不能の追加証拠と診断案

担当 `/root/gui_failure_investigation`。依頼 `gui-next-diagnostic-request-01.md` SHA256 `B96B845CDE1A45E7F272648BCAEFA12F6AC9F6520A4E6FB7049C1D78B37CD00B`。作業場所 `D:\Projects\GithubIssueWidget`、branch `main`、HEAD `5e74e1642fd18fe630dce0e684afd853721276ae`。既存 evidence の読取りのみで再起動・GUI 操作・資格情報・GitHub 操作はしていない。

## 新たな証拠

前回の観測領域は起動前に空と確認していた。強制終了後の読取り時、専用 `webview2` 配下には131ファイル・59ディレクトリがあり、`EBWebView`、`Default`、`Local State`、`Crashpad` の名前が存在した。一方、専用 `appdata`・`localappdata`・`temp`・`profile` はそれぞれファイル・ディレクトリ0。いずれも再解析点0。内容・ログ・個人情報は読んでいない。これは対象の実行中またはその後に専用領域にファイルが生成された観測であり、同一ユーザー競合を排除していないため生成者・WebView2 初期化成功・全子孫終了は証明しない。`appdata` が空のままであることは `connection_info`/`pending_creation` が通常どおり `settings.lock` を作った証拠がないことを示すが、両 IPC の未実行と断定できない。

固定ビルドの Cargo.lock は Tauri `2.12.7`、wry `0.57.2`、webview2-com `0.39.1`。実際の専用 Cargo 展開ソースとローカル registry ソースの対応ファイル SHA がそれぞれ一致した。Tauri の `app.rs` は設定済み `create` ウィンドウの `WebviewWindowBuilder::build()` の後にユーザー `setup` callback を呼ぶ。wry は WebView2 の environment を非同期に作り、`wait_with_pump` で結果を待つ。これはコード上の順序であり、今回どこで停止したか・WebView2 に欠陥があるかの証明ではない。

前回は直接子が生存、MainWindowHandle と空タイトルが取得できた一方、CUA の窓一覧では対象を識別できなかった。最終 exit 1 は自分たちが指定した強制終了コードであり自発的な101ではない。旧101の例外本文は破棄済み。CUA guidance は返された窓一覧から一意に対象を選択するよう指示し、今回見つからない HWND の推測選択を許す根拠はない。Win32 等を使った別 UI 照会の許可も未確認。

## 実行可能な次の診断

1. 現 release の固定入力を別の新規診断用コピーに取り、元製品・旧領域を変更せず、`main()` の builder/run 前と `.setup` の入り口に固定 ASCII の段階だけを専用保護ファイルへ書いて flush する診断版を作る。`.setup` は設定窓の `build()` が戻った後なので、`before_run` のみなら配置箇所以前/窓 build 内のどこかで止まった可能性を示すが、原因確定にはならない。`setup_entered` があれば窓 build が戻った根拠となる。書込み失敗時も固定分類だけを出し、例外、パス、credential、ユーザー入力や Issue 内容を出さない。Tauri/依存コードに侵襲する追加 instrumentation が必要なら別担当が対象箇所・副作用を評価する。
2. 同じ診断版に限定した別の固定段階を、フロントエンドの最初の mount および `connection_info`/`pending_creation` 呼出し到達/完了に追加する案を検討する。文字列は固定 enum のみ、APPDATA は空かつ専用、実資格情報・実 GitHub の操作なし。stage の有無とファイル障害・非同期未完了を混同しない。純粋な診断版は元 EXE と異なるため、両者の差を明記し、診断版の結果だけで元版の正常・故障を確定しない。
3. 診断変更・出力先 ACL・唯一の run ID・空 fixture・入力/出力 SHA・期限・監督者停止・直接子の識別と通常終了/不明時の扱いを具体的に固定し、作者と別担当が合成入力で固定分類・秘密非出力・終了境界を確認する。前回の DEVNULL のままの無情報起動はしない。CUA の対象が列挙・同定できない場合は他窓に触らず、期限後の対応は保持対象のみに限る。プロセスの直接終了や子孫の未確認は区別し、旧不明プロセスを推測して終了しない。
4. 実行時は staging の最後に確認された段階、プロセスの生存・exit、CUA が返したウィンドウ有無を別々に記録。エラーが出ないハングでも stage 差は新たな情報となる。出力・識別・終了を安全に得られなければ未確認として保護し、同一失敗を反復しない。実測には個別依頼とレビューが必要で、今回実行はしていない。

## 失敗と検証

- 既存 wry ソースを広い語で検索した結果が 6180 tokens で上限を超え切断された。全文・欠落部分は未確認。以後は対象単一ファイルの必要範囲を分割取得した。F-001 として1件の記録を管理に依頼する。
- `tauri-2.12.5/src/app.rs` を存在確認せず指定し OS error 3、終了1。正しい `tauri-2.12.7` を一覧で確認した。F-012 として別の1件の記録を管理に依頼する。Cargo.lock の正規表現検索は引用の解釈により空結果・終了1となり、`Select-String -SimpleMatch` で版を確認した。この操作も検索条件不適合として記録が必要。いずれも実行・秘密・製品変更はない。原因と分類の最終判断は管理が行う。
- 各限定読取り・ハッシュは終了0。新規の作業成果は本結果のみ、削除対象なし。領域のファイル内容は読まず、起動・プロセス停止・ネットワーク・権限変更は行っていない。結果は独立レビュー前。

対象 SHA256:

- `gui-release-observation-result-01.md`: `FE58E2BCF8377CDE31F38DB7FCC6F97A9153614620D6BFEBF092AF31C70F22A1`
- `gui-window-identification-01.md`: `92E50567EF54668B6D3EF9C4EB8BAFF890323C7D5DF60AFCCC56AB7AF0E714BF`
- `widget/src-tauri/Cargo.lock`: `2486C9E87F8511A2CBB3C09177E278139409C68A308FD48642327EDDF1D88E7F`
- Tauri `2.12.7/src/app.rs`: `06CA361D142DF1F24CFC6B47F9E0E4F5B50978D76E3AFF18A297F1610C024721`
- wry `0.57.2/src/webview2/mod.rs`: `349C93924DD696B1204B468377814315E63336450DFA6B94F83A675CBBB68AA3`
- webview2-com `0.39.1/src/lib.rs`: `DE9EC5585A990725576DE47557F93529CDDBDF267AB3847835689E2A01C854FE`。

3件の dependency ファイルは専用 build の Cargo 展開コピーとも SHA 一致。件数・ファイル名は調査時点であり同時更新し得る。
