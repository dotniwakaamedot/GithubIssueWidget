# GUI 診断版の1回の観測開始

`gui-diagnostic-observation-review-01.md` は固定 EXE SHA `57CFD705772F4AB2CBDD8BFC40EC555280B905872D347E046E4E57B07678055E` と観測器 `5A89F1E3832ADA05E4892C5C50C72873490E78B4EEF2FFA0C2E06C204677A662` の限定観測を条件付きで承認。`/root/gui_failure_investigation` は実行直前に6専用領域の実体・空・ACL・同時利用、status 不存在と対象版を確認できた場合、実 PAT・GitHub を操作せず1回起動する。不明・不一致なら開始しない。

固定 marker のみを安全に分類し、直接子・対象ウィンドウ・未知子孫の状態と marker 欠落の限界を区別して記録。通常閉鎖不能時は同一対象をパス・作成時刻と保持ハンドルで確かめ先行の限定終了条件に従う。結果を固有ファイルに確定し通知し、同じ無情報の起動を繰り返さない。
