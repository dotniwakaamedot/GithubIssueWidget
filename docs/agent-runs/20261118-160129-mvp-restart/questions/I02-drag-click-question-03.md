# I02 診断版準備と独立レビュー依頼 03

- 担当 `/root/drag_design`、宛先 `/root`、`/root/restart_planner`。依頼 `tasks/I02/drag-click-request-01.md`、回答 `questions/I02-drag-click-answer-01.md`・`answer-02.md`。
- 元 worktree `D:\Projects\GithubIssueWidget-issue-2-20261118-160129`、branch `task/issue-2-foundation-20261118-160129`、HEAD `a4ac97286785a2a5ba0d070a909b58003fc49ff1`。元の17件を `result-02.md` の SHA と照合し、全件通常ファイル・各 hardlink list 1件、親の reparse point を検査。新規 scratch `D:\Projects\GithubIssueWidget-drag-diagnostic-20261118-160129` に通常バイトコピー後、17件のコピー SHA が元と一致。元の内容や既存成果物は変更していない。作成した専用 evidence `tasks/I02/evidence-drag-click-01/source-manifest.json` SHA256 `E5C0FA60CB40DF2E59F15FABEF884D545FDABC04076DBCD2C8FDD220367990DC` は元/コピー一致時のパス・SHA・サイズを記録。独立レビュー時は実体を再照合する。
- 新規コピーの変更（対象 scratch からの相対パス）:
  - `widget/src/main.tsx` SHA256 `F318C76FE54F081F55656B84C0C9680C1416560E272C003C480C6B7C03CA4FA2`: `diagnostic.ts` の import を追加、header の `onMouseDown` を削除し元の `startDragging()` 呼出しを除去。別のリサイズ処理は残存するため、操作は header 内の単一押下・解放に限定する。
  - `widget/src/diagnostic.ts` SHA256 `76163B634209ABE4976F2A954E463EDFAEF7AB97B110E0B1D96858135682ADDF`: header 内 mousedown/mouseup を capture/bubble で観測し、type・phase・button・buttons・isTrusted・対象区分・3分割位置・単調時刻のみを最大20件 `window.__dragDiagnostic` に保持。文字列本文・URL・Issue・キー入力は取得しない。
  - `widget/vite.config.ts` SHA256 `194D17A739D0F626DDAF94B5D2B6F9335DBF50BB571AB4F7739DA727ED4F772A`、`widget/src-tauri/tauri.conf.json` SHA256 `EDE0EA3E1EE5C6FE067A5E127EEAAE02A19110E8CD8E057C8B7FCB2B826A55D5`: ポート 1420 を専用 1435 に変更。
- 上記以外の15件はコピー時点で元版と一致。ログはメモリ内バッファであり、実行時にどう取り出すか、対象プロセスと窓の識別、実機上の入力先、デスクトップ非競合、ビルドと型検査は未確認。依存導入、プロセス起動、UI表示、入力は一切していない。診断版に型エラーがないとは主張しない。入力観測が製品版のドラッグ成功を証明するものではない。
- 質問: この対象版と4変更の実体・固定 SHA、通常 drag 無効化、イベント収集範囲、ポート及び安全条件について作者以外の独立レビューを依頼し、その後の親判断・実行開始条件を通知してほしい。レビューと判断まで診断版を変更しない。プロセス起動・UI表示・入力は保留する。
- 実施コマンド: cwd `D:\Projects\GithubIssueWidget`。PowerShell の `Get-FileHash`、`Test-Path`、`Get-Item` と `fsutil hardlink list` で source17件を検査（各 exit 0）、独立 scratch に `[IO.File]::Copy` を実行し各 SHA を照合（exit 0）。対象新規ファイルの `Get-FileHash` exit 0。先の自己プロセス誤検出の失敗1回は `questions/I02-drag-click-question-02.md` に記録、他の今回の失敗なし。削除対象なし、scratch と証拠は保持。
