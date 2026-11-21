# README 利用手順の独立確認

- 確認者 `/root/plan_review`、README 編集担当とは別。親の依頼対象 `README.md` SHA256 `F4730B786CED48B18A36A5F6FE9D3E5429E9858484E97BCE706DE4830F3316F4` は開始・終了時に一致。削除対象なし。
- 判定: **対象版の利用手順を承認**。blocking なし。実 UI・GitHub の動作を実測したものではなく、アプリ全体の完成承認でもない。
- `git diff -- README.md` と現行 `widget/src/main.tsx` を照合。カードのクリックによる Issue URL の open、上部の作成ボタンと各列の＋、カードのドラッグによる状態変更はコード上の操作と一致。Done/他列の Closed/Open は既存の move 処理と一致する。
- `npm.cmd run tauri -- build --no-bundle` の通常ターゲットの EXE 名・配置説明はプロジェクトの build 構成と矛盾しない。今回の配布済み `widget/release-20261121/GitHubIssueWidget.exe` は別のコピー先であり、前記の説明がその配布コピーの場所を示すものではない。
- GUI・実 GitHub・利用者の画面確認は本レビューでは未実施。過去の独立確認 `mvp-release-result-review-01.md` は配布 EXE と固定入力の一致を確認しているが起動成功を証明しない。テスト・ビルド・外部操作を再実行せず、README も編集していない。

`D:/Projects/GithubIssueWidget` で対象の diff・UI と結果記録を読取り照合、開始・終了 SHA は一致（終了0）。対象版が変われば再評価する。
