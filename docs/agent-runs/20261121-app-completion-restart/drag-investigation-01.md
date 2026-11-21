# Issue カードのドラッグ不能の独立調査

## 結論

最も有力な原因候補は Tauri の Windows 向けドラッグハンドラーと HTML5 ドラッグの競合である。`tauri.conf.json` は `dragDropEnabled` を指定しておらず、Tauri 公式設定リファレンスは既定値が true で、Windows の HTML5 ドラッグには false が必要と説明する。ユーザーは #11 のカードが動かず禁止マークが表示されたと報告した。ただし現行版のイベント到達・設定変更後の実動作は未観測であり、原因確定や解決済みとはしない。

## 対象と根拠

- 担当 `/root/drag_investigation`、依頼 `drag-investigation-request-01.md` SHA256 `90BEA37157993CF865C98CF62E2D836883C3EC9A2DABC557A78A3908213E2F29`。作業ディレクトリ `D:/Projects/GithubIssueWidget`、branch `main`、HEAD `4961b25bec53a93439acdf5230de77f959908af0`。HEAD は未追跡・未コミット差分を識別しない。
- `widget/src/main.tsx` SHA256 `B22504C9DE48D275726C912820DA93E02F10C1838CCC68555C156D35030165D7`、`widget/src/style.css` `EB49BF65AE4FCE6701FBEF277055EDCAD42011248E5EA49905B6AFA4FD288B37`、`widget/src-tauri/tauri.conf.json` `5C1F3A0E2E487DD4345A2DD335FE4AACA483FEEEA26C11AAB99B374278EC1364`、`widget/src/main.test.tsx` `10AA28AD8D343F969C8BDB8C2D2232BF6007CB926619CB49850020DD53A40FBD`、release EXE `D19A72750006DF5BB6CBCA4FB15F87F69BFF48A5785F5569C5B05E40C9B93F26`。main と EXE は依頼指定に一致する。handoff は main と EXE の同一ハッシュを記録するが、本調査では EXE のビルド入力を独立再現していない。
- Issue #6 は3列間移動、ラベル保持、失敗後再取得などを要求する（公開 GitHub REST の番号指定 GET 成功）。
- `main.tsx:223` はカードの `draggable`・`onDragStart` と列の `onDragOver`・`onDrop` を使用する。`moving`、`creating`、`createOpen` が真ならドラッグは無効。同列ドロップは `moveIssue` 内で戻る。列全体がドロップ先であり、ウィンドウ移動の `startDragging` は `header.titlebar` のマウスダウンにのみ割り当てられる（206行）。CSS のリサイズ領域はウィンドウ端のみ。通常のカード中央とタイトルバーの直接重複はコード上認められないが、実際のヒット先は未確認。
- `main.test.tsx:82-83` などは `fireEvent.dragStart` と `fireEvent.drop` を直接発火し `dataTransfer` を模擬する。イベントを WebView2 が実際に配送できるか、ネイティブのドロップハンドラーとの干渉は検証しない。
- [Tauri 2 Configuration: dragDropEnabled](https://v2.tauri.app/reference/config/#dragdropenabled) によると、この設定は既定で true であり、Windows の WebView2 で HTML5 ドラッグを使うため false が必要。現行の window config には同項目がない。ドロップの `preventDefault` が正常動作しても、イベントそのものが届かない場合は模擬テストでは検出できない。
- F-015/F-034 は自動操作のウィンドウ移動・リサイズ失敗であり、手動のカード移動とは対象・経路が異なる。共通原因は未確認。関連する教訓は入力の到達・IPC・OS 状態を区別すること。

## 推奨する切り分けと修正

1. 同版で作成画面が閉じられ移動中表示がないこと、#11 のカード中央から別列の空白へドラッグしたことを確認する。現在の Issue #11 の状態は別担当が読取り中。原因確認のために実 Issue を繰り返し変更しない。
2. 独立した実装担当が `app.windows[0].dragDropEnabled: false` を設定し、ネイティブのファイルドロップイベントに依存する機能がないか確認する。設定変更は HTML5 カード移動を可能にする狙いであり、実際の事象を解決するとまでは未証明。
3. ビルド後、識別済みの新 EXE を用い、実 Windows/WebView2 のユーザー操作で #11 だけを別列へ移動して結果を確認する。移動・サイズ変更、リンク、作成、エラー時の再取得も回帰確認し、実 GitHub の状態・ラベルを照合する。実際のイベント配送を確認する統合試験を検討し、モックのみを実 GUI 証拠にしない。

## 実行と失敗

`Get-FileHash`、`git branch --show-current`、`git rev-parse HEAD`、対象限定 `rg`、`Get-Content`、Issue #6 の REST GET は終了 0。Tauri 公式資料は検索経由で確認した。実 GUI・テスト・EXE 起動・外部書込み・製品変更は行っていない。

失敗: `rg` に Windows PowerShell 未展開の `docs/failure-log*.md` を渡し終了1、結果は部分的。その後、`rg --files docs` はアクセス拒否ディレクトリを含み終了1かつ出力上限超過で切り詰められた。検索方法は異なるが、不適切な検索範囲・対象指定の失敗を2回認識したため、同種検索を停止した。後から既知の正確な `docs/failure-log-001.md` と `docs/failure-log-002.md` を指定して必要な対象のみ確認。欠落出力は未確認であり網羅証拠として使わない。共有失敗記録は所有範囲外のため未更新、管理へ引継ぎと別担当による独立分析を要請する。教訓: 1. Windows では `rg` の `-g` または実在パスを用いる。2. 出力上限とアクセス拒否を避けるため run 配下を無制限に走査しない。対策効果は未検証。

変更はこの新規結果ファイルのみ。確定ファイル自身の SHA は自己参照を避け、管理が確定後に取得する。削除対象なし。アプリコード・設定・認証・GitHub は変更していない。
