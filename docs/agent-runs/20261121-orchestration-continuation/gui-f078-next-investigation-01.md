# F-078 の次の GUI 検証方法

担当 `/root/gui_failure_investigation`。依頼 `gui-f078-next-investigation-request-01.md` SHA256 `9EE5B16ABF1D0F089834016E5943D8F4246DB596667DACDFA63AF23063A3208B`。作業場所 `D:\Projects\GithubIssueWidget`、branch `main`、HEAD `5e74e1642fd18fe630dce0e684afd853721276ae`。読取りのみ、GUI・合成子・資格情報・GitHub は操作していない。

## 結論

直近に記録された２回の release GUI は終了コード101で、２回目の stderr に panic と WebView2 の固定文字列が存在した。元本文と panic 位置は保存されておらず、WebView2 本体・権限・製品コードのいずれが原因か特定できない。次は通常の Windows 対話環境で、対象版と独立した新規設定先を固定し、release と開発モードを別の実行として、ウィンドウと直接プロセスの状態を観測する計画を独立レビューする。自己製監督器の完成を前提にしないが、未確認の終了・秘密保護を成功扱いしない。

## 根拠と対象版

- `handoff-gui-stop-02.md` は別々の空 APPDATA/LOCALAPPDATA を使う２回の release 実行に101を記録。２回目は `panic` と `WebView2` が一致し、`failed to run widget`、`Access is denied`、`os error` の固定分類が一致しない。これは文字列分類のみで内部原因を確定しない。旧同一出力の再取得はできない。
- `handoff-process-stop-06.md` は先行合成子の生成・終了が不明と記録。`gui-investigation-result-01.md` の分析はその不明を維持し、無関係なプロセスを推測して停止しない。未知の旧対象の不在は証明されていないが、それ自体を別領域の全作業の無期限禁止にしない。旧 fixture と設定を再利用せず、干渉を確認できない点を明記する。
- 起動時 `main.tsx` は `connection_info` と `pending_creation` を呼ぶ。旧分析で設定と復旧記録がない場合に資格情報非アクセスが見込まれる条件が確認されているが、現 `main.tsx` は旧分析時と異なる SHA なので、その結論を無条件で流用せず対象版の起動経路を個別に確認する。`APPDATA` 変更は Windows Credential Manager の隔離ではない。
- 旧 WebView2 Runtime 登録は存在した記録があるが、現行対話セッションでの動作の証拠ではない。現在の通常環境の WebView2・表示・対象バイナリ・実行ユーザー・対話セッションは本調査では新たに検証していない。

## 安全な次の一手

1. 実行担当は現行の変更内容・ソース、ビルド方式・lockfile・生成物をハッシュで固定し、実行ファイルの作成者と元版の関係を確認する。専用の新規ディレクトリに APPDATA と LOCALAPPDATA を分離し、パス実体・リンク・アクセス権・空・起動中の競合を確認する。起動経路が空設定時に復旧・credential・ネットワークへ到達しないことを現行版で照合し、不明なら起動しない。旧 fixture と実 credential を使わない。
2. release は現行版をビルドし、その EXE のハッシュを控えた後に一度だけ通常の対話セッションで起動する。直接起動を担当が追跡し、プロセスハンドル・作成時刻・実体パスを用いて表示・生存・終了コードを別々に記録する。保持ハンドルに対応するウィンドウを通常の閉じる操作で閉じ、期限内に直接プロセスの終了を確認する。識別できないプロセスに終了操作はしない。ウィンドウなし・終了101・タイムアウトを別結果とし、再び同じ診断情報しか得られない場合は直ちに別方式を検討する。
3. `npm.cmd run tauri -- dev` は npm、Vite、Rust/Tauri と WebView2 など複数のプロセス・ローカル通信を伴い得る。release の直接子と同一視せず、使用される実行ファイル、子生成、専用作業領域・ポート、開発サーバーの停止方法と観測可能な範囲を前もって固定する。実際の子孫の構成は未確認。コンソール操作での終了と、直接のプロセス・把握した子孫の終了を別々に確認し、把握できない子孫は不明と報告する。構成・出力保護・終了確認が不明なら開発起動の実施を保留する。
4. 設定なしの観測でもログ・クラッシュ文字列に個人情報が入る可能性があるため、生の stdout/stderr を無保護の共有ログへ保存しない。原因切り分けが必要な再診断は、個別に確認した固定パターンのみに絞った上限付き捕捉方式を合成入力で検査し独立レビューする。旧 `gui_diagnostic.py` は漏出・停止のレビュー指摘を解消していない。stdout/stderr を破棄する単純観測は表示と直接終了の確認には使えるが、panic の原因を特定できない。
5. 実行前に実体と権限、資格情報・外部通信・出力の扱い、専用領域、タイムアウト後の操作、残存時の保護を具体的に記した版を独立レビューする。実行ごとに対象 SHA・取得できた状態・未知の子孫・エラー・終了確認の範囲を記録し、未追跡子孫の不存在や秘密の完全保護を保証しない。実 UI 操作・認証・実 GitHub は別条件で検証する。

## 本調査の失敗・検証

PowerShell で `rg` に `docs/agent-runs/20261121-orchestration-continuation/*gui*.md` を直接渡した検索は OS error 123、終了1。glob が展開されなかったため、対象ディレクトリと `-g '*gui*.md'` を指定した限定検索は終了0。失敗結果を不存在の根拠にしていない。F-012 の既存記録と重複せず１件として管理に分類・記録を依頼する。同一の新規失敗は本調査で1回。教訓は PowerShell で未展開パスを直接渡さず、実在ディレクトリと `rg -g` を指定すること。

`Get-Content -Raw -Encoding UTF8`、限定 `rg`、`Get-FileHash -Algorithm SHA256` は上記失敗を除き終了0。GUI・開発起動・テスト・再診断は未実施。成果は本ファイルのみ、削除対象なし。旧報告・製品・fixture への変更はない。実行許可や実行済みの証拠ではない。

対象 SHA256:

- `remaining-product-plan-02.md`: `4E6DCAB5916F68B9702EA57B3D572B9CA8BA844D0B6E26D5282E9C670A7F529F`
- `../20261121-issue-completion/handoff-gui-stop-02.md`: `172C4528F7E80053ABBA5DE25B36CAF3C9A4A0BC0B7A4C8502276EA401B259B2`
- `../20261121-issue-completion/handoff-process-stop-06.md`: `3E8A48514398DC0994E4EE1B9F6EE25DA68169088CBC938341D27A8159245399`
- `../20261121-issue-completion/gui-alternative-analysis-01.md`: `E2CA02CA26B3980E21E7EBB38E0E596E01C8AF1C64F94F1EF0E84656EB861871`
- `widget/src/main.tsx`: `6FCF2A34145450C5029B7FF80DDB3EC1DA5E27173AAB84E57F16D6D54C0E6A3E`
- `widget/src-tauri/src/main.rs`: `61595B68317E15FDD8F2BED88B920A468D5516740E3D3124710384709C52FC24`
- `widget/src-tauri/src/creation.rs`: `61FB68E208B231664DB236E50A37BBB4870763AD61C0071383BACA5730982A87`
- `widget/src-tauri/tauri.conf.json`: `5C1F3A0E2E487DD4345A2DD335FE4AACA483FEEEA26C11AAB99B374278EC1364`
- `widget/package.json`: `102235B5686088C87B3221DEC7AE3CBEE20F0C9B1EAE070FD730C8B631DFCEDF`。

SHA は調査時点。対象版が変われば再照合する。報告自身は独立レビュー前。
