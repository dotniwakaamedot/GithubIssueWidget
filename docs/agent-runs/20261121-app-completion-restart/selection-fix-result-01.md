# カードと列見出しの文字選択抑制

担当 `/root/selection_fix`。依頼 `selection-fix-request-01.md` に従い、`widget/src/style.css` に `.board-column h2, .issue-card { user-select: none; }` を追加した。ドラッグ時に複数の文字が半透明で表示される現象への候補対策であり、原因の確定や実機での解消は未確認。

## 対象と判断

- 変更前 CSS SHA256: `EB49BF65AE4FCE6701FBEF277055EDCAD42011248E5EA49905B6AFA4FD288B37`。変更後: `FFE871A15BDA8FAD0BB5E28E1717C93D2878ABB1F5885F730EC624DEC983FCFE`。
- `widget/src/main.tsx` SHA256 `B22504C9DE48D275726C912820DA93E02F10C1838CCC68555C156D35030165D7` の JSX を確認し、列見出しは `.board-column` 内の `h2`、ドラッグ対象は `.issue-card` の button と照合した。設定フォームと作成フォームはこれらの子孫ではない。
- 入力・設定画面は対象外とした。カードと見出しの文字のコピーはできなくなる一方、ボタンのクリック、フォーカス、ドラッグのコードは変更していない。これらの実 UI における動作は別途確認が必要。
- 既存の `widget/src-tauri/tauri.conf.json` は変更せず、SHA256 `0132FCC92345F5D60DE84FD25197FCE853044AD63DAE28757AB4592CC72B991E` を開始前後に確認した。

## 検証

- Python で変更直前の SHA と既存文字列の不在を確認し、元バイトへの追加のみと書戻し後の完全一致を確認、終了コード 0。
- 既存の PostCSS で CSS 全体を解析し、選択抑制のセレクタが既存 `.titlebar` と今回の `.board-column h2, .issue-card` であることを確認、終了コード 0。
- `git diff --check -- widget/src/style.css` は終了 0、`git diff --numstat -- widget/src/style.css` は追加 3 行・削除 0 行。作業ディレクトリは `D:/Projects/GithubIssueWidget`。
- 既存のビルド出力を変更しないため、ビルドは実施していない。独立した隔離ビルド、別担当の差分レビューと、Windows 実画面でのドラッグ像・１件のみの移動・列見出しの＋・クリック・キーボード操作・フォーム入力と文字選択の回帰確認が必要。

製品変更は CSS のみ、その他の成果は本結果のみ。アプリ起動、資格情報、GitHub への書込みは行っていない。削除対象なし。最初の日本語読戻し照合は PowerShell から Python への標準入力で比較文字列が文字化けし AssertionError（終了 1）。結果ファイルそのものの破損とは区別する。Unicode エスケープを用いた照合は終了 0。最終変更後にも同じ方法で読戻しを行う。
