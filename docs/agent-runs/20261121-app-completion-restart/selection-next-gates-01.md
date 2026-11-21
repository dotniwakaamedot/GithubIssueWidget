# 文字選択抑制の後続ゲート

親の依頼に基づく順序と境界。各ゲートの個別確定依頼は前段の成果 SHA が確定してから別名で発行し、未確定の値を推測しない。

1. `/root/selection_fix` の `selection-fix-result-01.md` と `widget/src/style.css` の実体と SHA を取得し、独立担当 `/root/selection_review` へ版指定のレビューを依頼。カードと列見出しに限定され、入力・設定の選択が維持され、既存 `dragDropEnabled=false` が保持されていることを確認。blocking のある版を承認しない。
2. レビュー承認後、別担当 `/root/selection_build` に、対象 CSS と既存 `tauri.conf.json` を含む入力の SHA を固定して、既存 release や既存 build を上書きしない新規隔離領域で release build を依頼。実資格情報、既存アプリ、GitHub は変更しない。入力、終了コード、ログと EXE の SHA を記録し、別担当のレビューを経る。
3. 親が版を確認してユーザーに旧版を終了し新 EXE を起動してもらい、#11 に限るドラッグ、テキスト入力・選択、ウィンドウ操作を確認する。UI の成功と合成・静的確認を区別し、途中エラーでは停止。#11 の外部状態を読戻す。

各担当は共有 run `D:/Projects/GithubIssueWidget/docs/agent-runs/20261121-app-completion-restart/` を使い、自分の所有範囲以外を変更しない。依頼は担当 ID・対象 SHA・依存・検証・結果パスを確定してから起動し、版変更後に古い承認を使わない。作業が安全に成立しなければ未実施と理由を記録する。
