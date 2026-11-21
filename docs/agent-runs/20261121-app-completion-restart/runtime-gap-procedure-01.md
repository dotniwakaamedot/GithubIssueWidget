# 実画面の残要件と安全な確認手順

## 結論

修正版では #11 の全６方向移動とドラッグ像、作成画面の入力文字選択、再起動の本人報告がある。旧版ではウィンドウ移動・サイズ変更・最前面切替・許可リンクの成功報告があるが、修正版での同じ操作、初期・最小寸法の実測、中央表示と初期最前面、許可外 URL の実拒否、非 status ラベルの実保持は本調査で確認できていない。設定や模擬試験を実画面の成功として扱わない。

以下は親への**手順提案**であり、本担当は実画面操作・Issue 更新・資格情報操作を行っていない。すでに確認済みの６方向移動を、非 status ラベルの保持という新しい確認条件なしに再依頼する必要はない。実データ操作は #11 のみとし、適用範囲や既存承諾を親が判断する。

## 版・一次要求・証拠

- 作業者 `/root/runtime_gap_procedure`、作業場所 `D:/Projects/GithubIssueWidget`、branch `main`、HEAD `4961b25bec53a93439acdf5230de77f959908af0`。HEAD は未コミット・未追跡を表さない。依頼 `runtime-gap-procedure-request-01.md` SHA256 `D7FBCF01F7B5CC99EBDFAED39514B2B093391642D554EE450AB40B157E72D1E8`。
- 現在の受渡 EXE `widget/release-20261121-selection/github-issue-widget.exe` は SHA256 `0D8AB47088B7A19233EF0D7D08CF033B681B20CA0E729FEAF73EAB74DCC98F55`、12,885,504 bytes。`selection-build-result-01.md` は SHA256 `471185044A0D807DE1AF2D8799CE0C9095D27334ABCB7A1E2E69314CB43E371D`、独立 `selection-build-review-01.md` は `90D125A70C4497885004CA523CEDA48CD638F243871673ABFBCB0D1487378B97`。本人が起動した隔離ビルド版とコピーはハッシュ一致を記録しているが、**受渡先パス自体での起動は本担当未確認**。
- `tauri.conf.json` SHA256 `0132FCC92345F5D60DE84FD25197FCE853044AD63DAE28757AB4592CC72B991E` は width 900、height 520、minWidth 600、minHeight 300、center と alwaysOnTop が true、decorations false、resizable true、dragDropEnabled false。これは設定値であり実測結果ではない。
- `capabilities/default.json` SHA256 `FC67C4593DCFA5D79C09E08D7F1FC5CC3474FF343E74D4095719CBF83C67B564` は `https://github.com/` と配下を Opener 許可する。`main.tsx` SHA256 `B22504C9DE48D275726C912820DA93E02F10C1838CCC68555C156D35030165D7` の 206–208 行はタイトル領域ドラッグ、固定切替と閉じる操作、223 行は Issue カードのドラッグと URL 操作を実装する。`style.css` SHA256 `FFE871A15BDA8FAD0BB5E28E1717C93D2878ABB1F5885F730EC624DEC983FCFE` と `README.md` SHA256 `552827810A83303D430C143626E84ED4FAD5C94693A9CBBB3163071930778112` は修正版。`git status` では README・設定・CSS が変更済み、EXE は未追跡で、他担当の成果として保護した。
- Issue #1・#2・#6・#7 を GitHub コネクタ `fetch_issue` で番号指定、各成功。#1/#2 は Windows 11 の寸法・中央・最前面と許可／拒否 URL、#1/#6 は各列移動と非 status ラベル保持、#7 は実環境統合を要求。#1 の共有チャット本文は今回未取得で、追加要件の有無は未確認。
- `user-observation-02.md` SHA256 `619C303F1888991EACDF0F64EC4F5FC12F4BE47A1E4D1A3A971CE59D58E020D8` は旧 `release-20261121-02` の移動・リサイズ・最前面切替・GitHub リンクを本人が成功と報告した記録。新 EXE の当該回帰、初期・最小寸法、中央、拒否 URL の証拠ではない。
- `user-observation-07.md` SHA256 `F2B593C7B5093AA225F4DCF85395D117C902319ACDA3FEAF847ACD09391BBC67` は同じ SHA の修正版のドラッグ像改善・移動を本人が報告。`user-observation-08.md` `261EC6EDAF2A30209D4646FB7FE28CEB4DBE4D2847E5A980DDFAACCF3EF5B483` と `issue-11-state-04.md` `44F629CB8B2E14ECB0FB8FC82A541D17A97022F4BD549734BB82D4ABAC5D2DF3` は本人報告と公開イベントの全６方向・最終 closed/done の整合を示す。#11 に非 status ラベルはなく保持を実証しない。
- `user-observation-09.md` `83D7461C97B278354C3480361738D8AD6841DAC873B7E709E1F11CFD3037A896` と `user-observation-10.md` `AF186CDF5DF6533FD1A74E56A3C5BB1C0CBA7FA5CB38F90C21B7535772741548` はフォームの閉じる・仮文字選択と、トークン再入力なしで開いた本人報告。前者の言葉だけでは再起動後の一覧取得・初回登録・削除成功を確定できない。
- `sync-user-observation-02.md` `91B385F9C7F995AB0955E63B5470FA4AF8636AC98D644911184A859C8BAC16B8` は本人の60秒同期と復旧報告を記録するが、計時・他操作の有無は未記録。`issue-11-sync-state-02.md` `A32F09527BA15A74363362937BF63D7D9F7CD84B24C4582005E2FF79E19C894F` の REST イベントは403で未取得。本人画面と API 状態は別証拠。

## 次の確認手順案

### 1. 修正版ウィンドウと許可リンク

1. 本人が現在操作中の画面・未保存内容を確認し、閉じて問題がない場合に現在のウィジェットを閉じる。上記 EXE のパス・SHA と Windows 版を確認して本人が１回起動し、初期表示直後、移動・リサイズ・固定切替前に対象プロセスの実体とウィンドウを識別する。不明・同名複数・起動失敗なら再起動を繰り返さず停止する。画面・ログに PAT や非公開 Issue を出さない。
2. 別の担当が読取り専用の Win32 ウィンドウ測定を用意する場合、事前に対象 PID・EXE パス・表示中 HWND の完全一致、プロセス権限、DPI と座標系を確認する。`GetWindowRect` の物理座標、`GetDpiForWindow`、対象モニターの `rcMonitor` と `rcWork` を別々に記録し、96 DPI 基準へ換算して 900×520 と比較する。Windows の DPI 仮想化・ウィンドウ枠の扱いを調整できない場合は未確認とし、見た目だけで数値を断定しない。モニター全面と作業領域の中心を区別し、中央との誤差・解像度・倍率を記録する。API の実装・利用可否、観測方法は未確認であり、担当による事前検証と本人画面の識別が必要。
3. 初期固定状態は固定ボタンの表示だけでなく、別の無害な通常ウィンドウとの前後関係、可能なら対象 HWND の `WS_EX_TOPMOST` を操作前に読み取る。本人がタイトル部をドラッグして移動、外周を縮めて最小までリサイズし、同じ座標系で寸法を読み 600×300 と比較。固定を解除し通常ウィンドウを前面にした後、再び固定して前後関係を確認する。予期しない挙動・識別不一致時は停止し、記録に元状態と時刻を残す。画面が他アプリに隠れる操作は本人が行い、勝手にフォーカスを奪わない。
4. 既に使用した安全な GitHub リンクを修正版で１回開き、既定ブラウザでホストと結果を本人が確認する。リンク先は公開 GitHub のみとし、個人情報やトークンを記録しない。旧版で成功済みの確認を新 EXE の回帰として実施する理由は、Tauri のドラッグ設定変更後の対象版を確かめるため。

### 2. 許可しない URL の拒否

現行 UI には任意 URL を入力して Opener に渡す操作が見当たらず、本人に危険な URL を入力させる手順は提案しない。静的 capability と模擬 IPC の拒否は既存証拠だが、実 Windows WebView の拒否は未確認。最小の次作業は、別担当が **既存 EXE を変更しない隔離テスト版**に限り、同一版の capability と Opener を使って `https://example.invalid/` 等の予約済み無効ホストへの呼出しを１回行い、IPC の拒否と外部起動の有無を記録する実現可能性を確認すること。接続設定・実 PAT・実 Issue から隔離し、テスト用変更差分、EXE と capability のハッシュ、開発ツール・WebView と処理の終了を確認する。これはテスト版の実挙動であり、現行配布 EXE の直接操作と区別する。安全な隔離・観測が成立しなければ実行せず未検証とする。存在しないホストを開こうとしても、失敗時にはブラウザが起動する可能性があるため本人の画面利用と調整する。

### 3. #11 の非 status ラベル保持

コネクタの番号指定読取りで、今回確認時点の #11 は `closed`、`status:done` のみ、`updated_at=2026-11-21T06:48:20Z`。#1 の応答には非 status の `enhancement` が含まれたが、現在の利用可能性・本人の書込み権限・同時変更は保証しない。既存 label の定義は変更・作成しない。

1. 親が既存の #11 検証承諾に**非 status ラベルの一時付与と復元**が含まれるか確認する。不明なら本人へ具体的な対象・副作用を示して確認し、了解まで変更しない。#11 以外は触らない。本人が GitHub で #11 の現在の全ラベルと closed を読み取り、他の非 status ラベルがある・状態不明・競合中なら停止する。
2. 本人が、既存の `enhancement` が選べる場合に限り #11 に関連付け、`status:done`・`enhancement`・closed を確認する。ラベル定義や他 Issue を作成・変更しない。付与失敗・不明時は再送せず現在値を確認する。トークン・非公開内容は共有しない。
3. 本人が新 EXE で #11 だけを `Done → Todo → In Progress → Done → In Progress → Todo → Done` の順に動かす。６方向を各１回通る。各操作後、画面と GitHub で移動先 status、open／closed、`enhancement` の保持を確認し、Issue 番号・前後の列・結果だけ記録。エラー・不明応答・ラベル欠落・想定外の他者変更があれば次の移動を止め、GitHub の実状態を読取って親に伝える。自動再送・推測復旧はしない。複数 API 操作の途中状態や同時変更の可能性は残る。
4. 成功時は #11 が closed・`status:done` と確認し、本人が**自分で追加したと確実に識別できる** `enhancement` の関連付けだけを外す。第三者の同時追加や元のラベルが不明なら外さず停止して親に相談する。最終状態を GitHub で読戻し、元の `closed`・`status:done` のみと一致することを確認する。ラベルの関連付け・削除・移動イベントは履歴に残る。別担当の読取り照合は、結果が不明なら完了扱いにしない。

## 操作・失敗・残作業

- `Get-Location`・対象 `Test-Path`、`Get-Content -Encoding UTF8` の依頼・指示・指定記録の読取り、対象明示 `Get-FileHash`、限定 `git status --short`、`git branch --show-current`・`git rev-parse HEAD` は終了0。Git は個人 ignore 読取りの permission denied を警告した。GitHub コネクタ #1・#2・#6・#7・#11 の番号指定 fetch は成功。秘密・本文の不要な記録や外部書込みは行っていない。
- `rg --max-columns` の出力が２回省略された。１回目は既知の該当行を別方式で確認、２回目の省略部分は未確認。関連取得を停止し、別担当への調査依頼 `questions/runtime-gap-output-question-01.md` SHA256 `05FFE221BA9B87A73B440A227DF56B86FA35FD53C3077DA89826E88EC90C9DD1`、回答 `questions/runtime-gap-output-answer-01.md` `0595923ED6C9B3E492B0AA26F183E97F3A716E798C04A8F2BC9D7F2F942C3DDA` を確定して管理・独立担当へ通知した。共有失敗記録は所有外で未更新。教訓: 対象件数だけでなく最大行長と内外上限を見積もり、長い行は既知の個別範囲で取得する。独立分析・記録・効果は未確認。
- 変更は本新規結果と上記質問・回答のみ。自己ハッシュは同一ファイルに埋め込まず、確定後に管理が算出する。削除対象なし。GUI、アプリ、GitHub、設定、認証情報、既存文書は変更していない。ユーザー操作・外部更新・実 Windows API 測定・隔離版の実行は未実施。適切な別担当のレビューと親の判断が必要。
