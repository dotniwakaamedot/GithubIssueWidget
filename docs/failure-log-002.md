# 失敗記録 002

### 2026-11-23 ルール整理時

- F-001 関連1回: 指示３文書の一括表示と `docs` 以下の広いファイル列挙を同一操作で実行し、生成された依存物を含む大量の出力が内外の上限で切り詰められた。欠落箇所と元の処理全体の終了結果は未確認。必要な対象や合計出力量の事前確認が不足した。その後、正本のファイルと必要な範囲に限定して別途取得したが、元の出力は復元していない。過去の欠番・重複を含む累計は未照合。既存の独立分析 `agent-runs/20261121-app-completion-restart/workflow-output-independent-analysis-01.md` と照合し、一覧の対象限定・生成物除外・件数と最大行長の事前集計を `instructions/work.md` に具体化した。今回の独立分析は利用できず未完了、対策の効果は未確認。
- F-057 関連1回: 同操作で PowerShell の既定の読み取りによる日本語表示が判読不能だった。変換境界と元ファイルの破損は未確認。別途 Python で UTF-8 を明示して必要な文書を読み、表示を確認した。元の出力は復元していない。過去累計と予防効果は未確認。

### 2026-11-21 対話セッション用ランチャーの準備

- F-088 発生1: `/root/logon_session_resolution` が `py -3 --version` を実行すると `No installed Python found!`、終了1。別の `python --version` と実行ファイル確認は終了0で Python 3.11.15 を観測した。Python launcher が利用可能なインストールを見つけられなかったことを示すが、原因の詳細と本人の通常 PowerShell での利用可否は未確認。実行環境・実体・版を先に確認し、本人環境の結果と混同しない。参照: `agent-runs/20261121-app-completion-restart/logon-session-resolution-01.md`。本作業では実 Credential Manager を操作していない。
- F-089 発生1: `/root/interactive_launcher` の `launcher-synthetic-01/test_launcher.py` による正常ケースは終了1、`reached=False`。子 PowerShell の標準出力・標準エラーは保存されず、直接原因は未確認。初回ハーネスは実 cmd と異なり `-ExecutionPolicy Bypass` を省いていた。実行ポリシーによる拒否は候補だが断定しない。ハーネスを実 cmd の呼出しに合わせ別の新規 fixture で7ケース成功・終了0との作者報告があるが、元の失敗の原因証明ではない。実行前に本番と試験ハーネスの引数・環境を照合し、秘密を含まない診断情報と個別終了状態を保持する。独立ログと対策効果は未確認。参照: `interactive-launcher-result-01.md`、`launcher-failure-answer-01.md`。
- F-090 発生1: 同担当の Read-Host 確認では、Python の raw string 中の `\u0024x` を含む PowerShell `-Command` で子終了1、外側終了0。子 stderr を保存・表示しておらず直接原因は未確認。`\u0024x` が `$x` に復元されない式構成は候補。別の ASCII here-string で `YES` を渡す限定確認は子終了0、stderr 空と作者が報告した。これは元の表示の復元ではない。F-035・F-060 とは引用の問題が関連するが同一原因は未確定とし、言語境界の実引数と子の終了状態を確認する。参照: `launcher-failure-answer-01.md`。予防効果は未確認。
- F-091 発生1: `/root/launcher_failure_log` が上記 F-090 を追記する際、JavaScript 文字列からパッチへ転記したエスケープ文字を１つ余分に記した。追記直後の実ファイル読戻しで作者の回答と不一致を確認し、作者の回答に合わせて修正した。元の失敗コマンドの意味や直接原因はこの転記ミスからは確定できない。異なる言語間のエスケープは適用直後に一次資料と実ファイルを照合する。F-090 自体の失敗とは別操作であり、再発防止効果は未確認。

### 2026-11-21 実 OS 合成資格情報試験と残留確認

- F-087 発生1: `/root/credential_live_execute` の実 OS 合成資格情報試験１回は監督終了1、直接子は wait 済み `completed`・終了1、journal 最終段階は `before_write_a`。stderr が破棄され、書込み開始前の停止・書込み失敗・成功後の記録失敗を区別できず、直接原因と書込み成否は未確定。独立調査 `agent-runs/20261121-app-completion-restart/credential-live-failure-investigation-01.md`、後の読み取り専用診断 `credential-live-residue-inspect-01.md` は当該合成 target に限り観測時点で `absent`・終了0を記録し、独立 `credential-live-residue-review-01.md` は対象版を照合した。`absent` は元の書込み成否や終了1の原因を証明せず、終了コードは作者記録。再送・削除は行わなかった。失敗時は秘密やエラー本文を出さずエラー種別・OS コード等の安全な診断情報を保存し、結果不明時は再送せず一意の対象・子の終了・残留を照合する。予防効果は未確認。実行・独立調査・診断は一連の１発生であり二重計上しない。
- F-001 関連１回: `/root/credential_live_failure_investigation` の run 全体のファイル名表示が内側上限で切り詰められた。元の全出力・最大長・終了コードと欠落箇所は未確認。後に run ID 限定へ変更し、失われた出力の復元とは扱わない。過去の欠番・重複を含む累計は未照合。独立分析 `agent-runs/20261121-app-completion-restart/live-failure-record-analysis-01.md` に従い、対象・変換後の行長・内外の上限を事前に見積もる。効果は未確認。
- F-012 関連１回: `/root/credential_residue_inspect` は `Get-FileHash` で推測した EXE を含む複数パスを指定し、当該 EXE に非終端の不存在エラー。複合操作の終了0を当該 EXE の成功と扱わず、後に supervisor 定義と `read_identity` の固定 SHA 照合で正しい対象を確認した。独立分析と本人回答は `live-failure-record-analysis-01.md`・`live-failure-analysis-answer-inspect-01.md`。元の独立ログと過去累計は未確認。実在パスと各操作の終了を事前に確認する。
- 文字照合・表示の各１回: 同検査担当の PowerShell here-string から Python 標準入力への日本語代表文字照合は `utf8_markers=False`・終了0。失敗時の Python 入力実バイトは未保存で、伝達境界の変換は原因候補。別の ASCII Unicode escape と strict UTF-8 照合は成功と本人が報告。F-057・F-086 との同一原因は未確定。別に `/root/live_failure_recorder` は分冊を Python UTF-8 読取り後に表示した日本語が判読不能となった（終了0）。後に PowerShell で別条件の見出しを可読取得したが、元の表示を復元していない。表示文字化けの F-057 と対策は関連し、元ファイル破損・変換境界・過去累計・対策効果は未確認。独立分析 `live-failure-record-analysis-01.md` に従い入出力の文字コードを明示し、復号後の値と対象版を比較する。

### 2026-11-21 合成資格情報試験の照合と失敗記録の検索

- F-086 発生1・2: `/root/credential_supervisor` の同一作業中、文字列の期待値照合が２回失敗した。１回目は Windows 生パスを JSON エスケープ済みファイルの生バイトと比較して `AssertionError`、終了1。JSON を復号して意味値を比較する修正後、作者は模擬７件成功を報告した。２回目は結果 tmp の strict UTF-8・代表文字等の複数 assert のいずれかで `AssertionError`、終了1。実引数と失敗した条件は未保存で、エスケープの二重化は原因候補にとどまる。作業を停止し、別担当 `/root/credential_supervisor_failure` が `agent-runs/20261121-app-completion-restart/credential-supervisor-failure-analysis-01.md` に独立分析を確定、別担当 `/root/credential_supervisor_finalize` が実バイト・代表文字・ログを照合し結果を確定した。旧 F-085 の代表文字誤指定との同一原因は未確定。値はデコード後に比較し、代表文字は対象本文から確認、各 assert の入力と終了を区別する。元出力、２回目の直接原因と予防効果は未確認。実 Credential Manager は操作していない。
- 失敗記録担当 `/root/supervisor_failure_record` は分冊を探す際、`rg` の glob がファイル名だけを絞るのに対し探索範囲を `docs` 全体とし、run 配下のアクセス拒否を含み終了2、部分結果となった。別に PowerShell から Python へ日本語リテラルを渡す見出し分類が終了0・空出力となり、後の ASCII Unicode escape を用いた読取りで該当見出しを確認した。前者は探索範囲・アクセスに関係し、後者は入力伝達または照合の問題が候補で、元の実引数・直接原因と両者の過去累計は未確認。同一作業で同種検索失敗の可能性として当該方法を停止し、別担当の `agent-runs/20261121-app-completion-restart/supervisor-record-search-analysis-01.md` が独立分析した。F-012・F-062 の過去件数への加算は保留し、実在する正本２分冊を直接指定し、入力・出力の符号化または ASCII escape を確認する。元の完全出力と予防効果は未確認。

### 2026-11-21 合成資格情報試験のオフライン依存解決

- F-039 発生2・3: `/root/credential_live_test_design` が `credential-live-test-01/` の `cargo test --offline` を実行したところ、`no matching package named keyring found`、終了101。既定 `CARGO_HOME` の初回と、専用領域へ既存 registry をコピーした2回目のいずれも依存解決で停止し、試験未実行。ユーザー指定の同種2回に達したため当該作業を停止した。元ログと終了記録は `credential-live-test-01/mock-test.log`、`mock-test-02.log` に保持。分冊001の発生1と合わせ確認できた累計は3回。
- 独立調査 `/root/cargo_dependency_investigation` の `agent-runs/20261121-app-completion-restart/credential-cargo-investigation-01.md` は、対象 Rust/Cargo が1.84.1で manifest の最低1.90を満たさず、2回目のコピーには keyring の index・crate・ソースが存在したことを確認。旧失敗の直接原因は未確定。別担当 `/root/credential_cargo_fix` が `cargo +1.90.0` と専用絶対 `CARGO_HOME` でオフライン依存解決・模擬9件を成功させ、live feature のビルドと禁止ゲート終了2を確認した（`credential-cargo-fix-01.md`）。旧原因の特定や実 Credential Manager・実 PAT の検証を意味しない。独立レビュー `credential-cargo-fix-review-01.md` は限定的模擬結果を確認し、実 OS 試験は所有権・異常終了の監督・禁止解除版の未検証で未承認とした。
- 記録担当 `/root/credential_failure_log` は分冊の検索を `rg -g` で docs 全体に広げたところ、アクセス拒否ディレクトリを含み終了1、結果は不完全。正確な分冊2件を指定して確認し直した。これに加え同担当の全見出し出力が上限で1回切り詰められた。独立分析 `agent-runs/20261121-app-completion-restart/credential-failure-output-analysis-01.md` は両事象を区別し、出力の必要 ID への限定不足と Unicode escape による行長増加を原因候補とした。欠落・見積もり値・拒否理由は未確認。F-001 の過去の欠番・重複と累計は未照合。今回各失敗は1回ずつで同種2回ではない。今後は実在する対象分冊を明示し、必要な ID のみ取得する。

### 2026-11-21 利用案内と失敗記録の確認

- F-084 発生1: `/root/release_notes` が利用案内の作成用 Python ソースへ JSON 文字列を埋め込んだ際、引用符が連続し `SyntaxError: unterminated string literal`、終了1。作成処理には到達しなかった。作者の観測 `agent-runs/20261121-app-completion-restart/failure-recorder-answer-01.md` と独立分析 `agent-runs/20261121-app-completion-restart/failure-recorder-analysis-01.md` は、PowerShell が引用符を消した証拠はなく F-035・F-060 との同一性も未確定とする。代替では JSON を Python 文字列リテラルとしてエスケープし `compile` と保存・読戻しを終了0で行ったと作者が報告。データをコードに直結せず言語ごとの直列化と書込み前の構文確認を行う。元の完全出力と予防効果は未確認。
- F-001 追加2操作: 同上の構文エラー出力は約2652 tokens に対して内側上限500で切り詰め、終了1。別に `/root/failure_recorder` が分冊001の冒頭36行を取得した際、約2255 tokens に対して内側上限1800で切り詰め、終了0。独立分析は両者を別発生とし、対象・長行・例外を含む内外の上限見積りを適用すると判断した。両方とも完全ログなし、欠落は未確認。過去の欠番・重複と全累計は未照合であり確定しない。今回の各担当内では各1回。
- F-085 発生1: `/root/release_notes` が回答ファイル公開前の代表文字照合で、本文に存在しない文字列を指定して終了1。実在する文字列を使った厳格 UTF-8 読戻しは成功したと本人が報告。独立分析 `agent-runs/20261121-app-completion-restart/failure-recorder-analysis-02.md` は F-014・F-016 や既存の API 例外との同一性を確認できず、誤指定理由・元例外・完全ログを未確認とした。対象版の本文から目的に合う代表文字を確認し、照合ごとの終了を区別する。ファイル破損・符号化異常は確認されていない。

### 2026-11-21 移動処理試験時

- 容量確認で `Get-CimInstance Win32_LogicalDisk` がアクセス拒否。複合コマンドのため個別終了コードは不明。`DriveInfo.AvailableFreeSpace` は容量を取得した。環境のアクセス拒否理由と同種累計は未照合。
- PowerShell の `$ErrorActionPreference='Stop'` が Cargo の通常の stderr 進捗を `NativeCommandError` と扱い、ラッパーが終了1、試験は結果なし。Cargo／rustc の残存不在を確認し、stderr をファイルへリダイレクトする方法に変更して対象16試験が成功した。元の合成試験対象・ログ・未確認は `agent-runs/20261121-app-completion-restart/tasks/03/test-result-01.md`。同種累計と根本原因の詳細は未照合。別種各1回であり、再発防止効果は未確認。

- F-012 関連: `/root/planning` は `widget/README.md` の存在を確認せず読み、存在しないパスとして拒否された。正しい資料 `docs/handoff-20261121-current.md` は別途参照済み。今回1件、累計の照合と独立分析は未完了。
- 2026-11-21、`tasks/03/test-review-01.md` を UTF-8 で読み戻すと、実体に `?` が812文字存在し日本語部分を判読できなかった。SHA256 `8FBDC1986DCB1033F5B9AFAB99918FAF03AE427DA9BE2A90D3FD813D38326718`。単なる表示の文字化けか、書込み時の不可逆な置換か、元の文面は未確認。F-057 の表示文字化けとの関連は未確定。レビュー承認には使わず独立分析を依頼する。
- F-057 関連: `/root/movement_review` の API 本文取得で Python と PowerShell の文字コード不一致により表示文字化け1件、UTF-8 指定後に本文を確認した。`tasks/03/test-review-02.md` の報告。過去累計、独立分析、対策の効果は未確認。同結果では上記事象について、PowerShell パイプ経由の日本語置換でファイル実体の復元不能と報告し、残った原文から新版を作成した。独立確認は未完了。
- 独立分析 `agent-runs/20261121-app-completion-restart/encoding-analysis-02.md`: 旧レビュー実体は ASCII `?` 812個で、表示設定では復元不能。作者はパイプ経由の変換を報告したが正確な境界は未確認。Issue と movement 各担当の表示文字化けは別々の発生、変換境界・同一原因は未確認。対象版の新版は原文と実試験証拠を照合し別担当がレビューする。UTF-8 指定だけでなく保存後の代表文と原文の照合を行う。効果は未確認。

### 2026-11-21 認証合成試験時

- 過去証拠の `rg` でアクセス拒否ディレクトリを含み部分結果、PowerShell 最終終了1。完全な検索ではない。
- Windows PowerShell の `Set-Content -NoClobber` が未対応でパラメーターエラー、続くハッシュ照会も対象不存在で失敗。複合コマンド全体終了0は前段成功を意味しない。別の CreateNew 方式で作成した。これらは一連の未対応引数と後続影響として記録。
- `main.test.tsx` の検索が出力上限1600に対し約1958 tokens で切り詰め。欠落範囲は未確認。同種累計は F-001 の暫定61回に1回追加、暫定62回。独立分析と教訓更新は未了。
- 根拠 `agent-runs/20261121-app-completion-restart/tasks/03/security-test-result-01.md`。各事象の原因・他の累計と再発防止効果は未照合。機密・製品の外部書込みなし。

### 実 Issue カードのドラッグ不能

- 2026-11-21、ユーザーが指定 release で新規検証 Issue #11 をドラッグ＆ドロップできないと報告。操作位置・入力到達・Issue の実状態と内部原因は未確認。F-015/F-034 はウィンドウ移動・リサイズに関する過去事象であり、同種と断定しない。独立調査と Issue 状態の読取りを依頼中。今回1回。
- F-012 関連: `/root/movement_design` が `rg` に `widget/src/*.css` を渡し Windows のパス解釈で失敗、終了1。main.tsx のみ部分取得となった。`rg -g '*.css' ... widget/src` の再取得は終了0。`agent-runs/20261121-app-completion-restart/movement-ui-result-01.md`。同種累計と再発防止効果は未照合。
- `/root/drag_investigation` は `rg` に `docs/failure-log*.md` を渡し Windows で未展開のため終了1・部分取得。続く `rg --files docs` はアクセス拒否を含み終了1かつ出力が切り詰められた。担当は不適切な検索範囲・対象指定の同種2回として停止。後に実在する2ファイルを限定して確認した。`agent-runs/20261121-app-completion-restart/drag-investigation-01.md`。F-012・F-001 との重複や累計、独立分析、欠落範囲は未確認。
- 独立分析 `agent-runs/20261121-app-completion-restart/drag-search-analysis-01.md` は最初の未展開 wildcard を F-012 関連1件、後の列挙切り詰めを F-001 1件と区別。アクセス拒否の原因・累計と欠落は未確認。対象の存在・解釈、検索範囲と内外上限を事前確認する。F-001 は暫定63回（欠番・重複未照合）、効果未確認。
- F-057 関連: `/root/security_review` のテスト assertion 抽出で日本語が表示上文字化けした。該当文字列は未確認、元ファイルの破損は示さない。`agent-runs/20261121-app-completion-restart/tasks/03/security-test-review-01.md`。変換境界・過去累計・独立分析は未確認。
- F-057 関連: `/root/drag_fix_review` が Issue API を表示した際に日本語が文字化けし、ASCII エスケープ JSON として別途取得して可読化した。`agent-runs/20261121-app-completion-restart/drag-fix-review-01.md`。元データ破損は確認されておらず、変換境界・累計・独立分析は未確認。

### ホスト設定作業の調査・編集

- 2026-11-25、F-001: 最初の指示文書読取りと `rg --files` の出力が切り詰められた。生成された依存物を除外せず出力量を見積もらなかった。別の限定読取りで必要な箇所を確認したが、元の出力の欠落と累計は未照合。二つの独立分析は対象限定・生成物除外・分割を推奨した。共有 run への分析記録と教訓反映の確認は未完了。
- 2026-11-25、F-057 関連: 上記の文書表示が文字化けし、UTF-8 指定の読取りは可読だった。変換境界と元データの破損は未確認。
- 2026-11-25、F-012 関連: PowerShell で未展開の `docs/failure-log*.md` を `rg` に渡して失敗した。実在するファイル名を指定し直した。累計と原因の同一性は未照合。
- 2026-11-25、F-084 関連: Python の `-c` に渡した置換式が引用符の不整合で構文エラーとなり、書込み前に停止した。PowerShell の文字列置換で対象を確認して更新した。引用符の変換境界と累計は未確認。今後は言語をまたぐ複雑な引用を避ける。

### EXE の GitHub 反映

- 2026-11-25、F-001: GitHub のコミット情報を全表示し、巨大な差分を含む出力が切り詰められた。必要な SHA のみに出力を絞る。元の欠落と累計、独立分析と教訓反映は未確認。
- 2026-11-25、EXE の一括・2分割読取りはツール出力上限により完全なデータを得られず、いずれも登録を行わなかった。5分割で読取長を照合して登録し、返却された Git blob SHA はローカルの Git blob SHA と一致した。直接原因と過去累計は未確認。
- 2026-11-25、`gh auth status` は保存済み認証の失敗を報告した。GitHub 連携ツールの認証と push 権限を別途確認し、EXE の反映とリモートのコミットを確認した。認証失敗の内部原因は未確認。
- 2026-11-25、対応ソースの読取りは自動承認審査に拒否され、ソースは送信していない。依頼された EXE のみを GitHub 連携ツールで反映した。
