# I02 矩形観測の余白不足と証拠公開手順 04

- 担当 `/root/drag_design`、宛先 `/root`・`/root/restart_planner`。対象 `tasks/I02/geometry-execution-request-01.md`。旧 app / server は親判断で所有 PID のみ強制終了し、各 session terminal `-1`、port 1435 listener は停止後0件（質問03）。正常終了とは扱わない。
- 新診断専用 scratch で依存準備、`rustup toolchain install`、`npm ci`、`npm run build`、`cargo build --locked` はそれぞれ exit 0。npm は esbuild script 未承認警告。geometry app exe SHA256 `F10DC7E78705DC40927C95CBABC372FA5F64F1F2EE52B0B46A80E6BA56BCC2BD`、PID 9796、window ID 6166362、個別 `cua.getApp` で対象 path と `http://localhost:1435/` の main window を観測。
- アクセシビリティ tree に `diagnostic-ready:[]` と geometry JSON が表示された。元の出力を手動転記した `tasks/I02/evidence-geometry-01/observation-01.json` の SHA256 `679FEAD536EB88B7540BA31930BCFDE31556A59816991974681E00AA764F8621`。viewport 900×520、DPR 1、marker top 16・bottom 40、上側 resize bottom 5、**最小余白 11 CSS px**。安定フラグと marker hit は true だが、要求は12 CSS px 以上。1回の観測に限る。
- **入力・クリック・ドラッグ・リサイズ・新たな画像取得は一切していない。** 余白不足のため依頼に従って停止。以前の画像不一致の解決、実画面・DPI・デスクトップ非競合・Issue 完了はいずれも証明されていない。新 dev server session `68650`、新 app session `4498` は起動時に session ID を返したが、現在の live は未確認。停止・再起動せず保護。
- 手順不適合: 上記 evidence JSON は本来 `.tmp` から確定すべきところ、誤って直接確定名に書いた。既存ファイルは上書きしておらず、本ファイルを変更・削除しない。以前の `questions/I02-build-question-03.md` には別担当が質問01を `.tmp` を経ず公開した手順逸脱が記載されており、**同種の直接公開が2件に該当する可能性**がある。作業者として公開手順の操作を中断し、別担当による客観調査と親の同種判定を依頼。3回目に該当すれば全作業中止。
- 質問: 証拠公開の2件について独立調査・分類し、保持と再開条件を指示してほしい。また、11px の余白不足に対して同じ観測を繰り返さずにどの版を修正するか、対象版を固定し独立レビューを経る手順を管理が判断してほしい。
