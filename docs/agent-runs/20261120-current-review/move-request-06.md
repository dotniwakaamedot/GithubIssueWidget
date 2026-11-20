# ラベル追加応答の独立確認依頼 06

原文: 「現在の進行状況を確認しアプリの完成まで進行してください。一度の処理で2度同じ失敗をした時は停止してください」
対象: widget/src-tauri/src/movement.rs SHA256 571C67DD98F148CF7B97F4056F1C8A3CAFB6ADE454DBE29C34B075588EED777D。変更前の review-04.md を参照。
変更目的: ラベル追加後の HTTP 200 応答に目標ラベルが含まれない、または応答を解析できない場合、旧ラベル削除と state 更新を行わず、部分反映の可能性を通知する。
参考: https://docs.github.com/en/rest/issues/labels?apiVersion=2022-11-28 の Add labels to an issue 応答。
テスト: 一時フォルダーの giw-move-confirm-7f3dc760556c4600bc8378af6bbbb53c.log。Rust 本体 64 passed・9 ignored、URL 範囲 1 passed。
依頼: 応答形式、後続書込み停止、異常時の通知、既存の６方向・競合・部分失敗試験と残る制約を独立に確認し、review-06.md に対象版と範囲を記録。コード・設定・資格情報・GitHub は変更しない。実 PAT・実 GitHub 未検証を明記。同種失敗２回で停止。
