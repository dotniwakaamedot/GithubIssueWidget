# I02 B03 結果・診断クリック依頼の独立確認 01

- run_id / task_id / review_revision: 20261118-160129-mvp-restart / I02-build-drag-preflight-review / 01
- owner_role / owner_agent_id: verification_reviewer / /root/plan_review。B03 作者 /root/build_verification、クリック依頼作者 /root/restart_planner、診断担当 /root/drag_design と別担当。
- 入力 `build-verification-result-01.md` SHA256 7A8AF3B60D32D2AC94888801067C73031DB8A84974DA95E6373D03C03E68A098、`drag-click-request-01.md` SHA256 83D63BA78D6CEEC7E07BDF9834583D0274BCADCEFC5DB5BCAB6B25377880CB6C を含む構造化参照9件は、基点・相対パス・期待 SHA を分離し全件存在確認後に実体 SHA256 を照合して一致。
- 対象 worktree `D:\Projects\GithubIssueWidget-issue-2-20261118-160129` の branch `task/issue-2-foundation-20261118-160129`、HEAD a4ac97286785a2a5ba0d070a909b58003fc49ff1、README 変更・widget 未追跡16件は結果と一致。結果02の製品17件は元と B03 コピーの両方で全件存在確認後に SHA256 を照合、各期待値と一致。削除なし。
- 判定: **B03 途中結果の記録とクリック依頼の事前条件は対象版に限り確認**。B03・Issue #2 は未完了。クリック依頼の確認は診断版コード・入力・実機操作の許可を意味しない。blocking 指摘なし。

## B03 結果

| 条件 | 独立に確認した証拠 | 判定と限界 |
| --- | --- | --- |
| 対象版・コピー | `copy-manifest.json` と `post-stop-hashes.json` は各17件で一致。実体の元・コピー各17 hash は結果02と一致 | 一致は観測時点の状態であり、実行中に一時変更がなかった完全な証明ではない |
| build と exe | `release.stderr.log` に release profile の終了・専用 exe 出力あり。実 exe SHA256 27D94B7B32EC394F89080C5812D04671B7DA8D420E5B85F1D67615FEAE5D2F4C は結果と一致 | **B03 未完了**。`release-result.json` は存在せず npm/Cargo の実終了コードは未取得。ログや exe の存在を build の正常終了と扱わない |
| 依存準備 | `npm-result.json` と `rustup-result.json` は各 exitCode 0 を記録。対象コマンド、cwd・開始終了日時は保存されている | 依存全体の個別ハッシュ・正常動作は未確認。`npm ci` の script 警告は作者結果のとおり |
| セッション・所有 | 作者の確定結果は親指示で所有 PID 25560 を停止し、セッション 85095 の `exit_code -1` を記録。今回の読取時、PID 25560 は不在、build scratch のコマンドラインを含むプロセスは検出されず、TCP 1420/1435 も検出されなかった | PID 不在・名前照合・ポートはスナップショットであり、セッションハンドルや他名の子孫・全デスクトップの非競合を完全には証明しない。結果の terminal は作者記録として扱い、クリック前に管理が実状態を再照合する |
| dev | 結果は未実施と記載 | 未達。アプリ起動・正常終了をこの証拠から主張しない |
| 証拠整合 | 結果に記載の証拠13件・scratch スクリプト等9件、専用 release exe の実体 SHA256 を独立照合し一致 | ログと結果の整合を確認したものであり、OS の全副作用が無かったことは証明しない |

## 診断クリック依頼

| 条件 | 確認 | 判定・未確認 |
| --- | --- | --- |
| 元の Issue と診断範囲 | Issue #1/#2 の本文は Windows でのドラッグ移動を要求。依頼は `startDragging` を無効化した別診断版で単一押下・解放だけとし、過去の入力到達・製品版移動・IPC/OS 応答・Issue 完了を証明しないと明記 | 適合。元の2回の不成功は原因未確定で再試行しない |
| 依存ゲート | 当該依頼と対象版・領域・安全条件の独立レビューと親判断までコピー・コード・入力を開始せず、作成後も診断版ハッシュと drag 無効化の独立レビュー・親判断を要求 | 適合。今回確認したのは依頼だけであり、診断コード未作成・無効化やハッシュは未確認。実入力には別途条件充足が必要 |
| B03 との非競合 | B03 terminal と UI・プロセス・port の非競合の確認を開始条件とし、所有者以外の停止・削除を禁止。scratch・port 1435・cache・profile を専用化 | 方針は適合。現在読取の PID/port のみで非競合を保証せず、直前に担当・画面・子孫・対象先と所有状態を確認し不明なら停止 |
| 対象・版・情報保護 | 元17件とコピーの path/size/SHA、診断変更点と生成 exe を別版として固定。位置は粗い範囲、対象区分・button・buttons・isTrusted 等のみ記録し秘密・テキスト・URL を収集しない | 適合。実現可能なイベント記録と秘密混入防止は未実装・未確認 |
| 停止と保持 | ドラッグ・移動・リサイズ禁止。対象不明・不達・記録不明なら繰り返さず停止。専有 scratch と証拠を保持し、同種2回で客観調査、3回で全停止 | 適合。指示前に担当を起動した運用不備は依頼に1件として記載。次回は依存ゲートと対象版確認後に起動する |

## 指摘・残作業

blocking および新規 non-blocking 指摘なし。B03 の成功・dev・クリック診断は未検証であり、当レビューはそれらを完了としない。B03 の terminal 状態と UI・プロセス・port の直前再照合、診断版の独立レビューと親判断を依頼の条件として維持する。

## 実施内容

- cwd `D:\Projects\GithubIssueWidget`。`Get-Content -Encoding UTF8` で構造化依頼・B03 結果・クリック依頼・B03 依頼を読取。JSON 参照9件を基点・相対パス・SHA に分け、全件存在確認後 `Get-FileHash -Algorithm SHA256`。結果記載の証拠13件と scratch ファイル9件も基点を別々にして存在確認後ハッシュ照合。各コマンド群終了コード 0。
- 同 cwd から元と B03 コピーの製品17件を前版結果の製品欄だけから抽出、全34パス存在確認後にハッシュ照合。`git -C <worktree> branch --show-current`、`rev-parse HEAD`、`status --short --untracked-files=all` を実行し変更・削除を確認。終了コード 0。
- `Get-Content` でログ末尾、依存準備結果、manifest を確認。`Get-Process -Id 25560 -ErrorAction SilentlyContinue`、`Get-CimInstance Win32_Process` の scratch コマンドライン一致を PID・名前のみで確認、`netstat -ano -p tcp` から 1420/1435 を検索。セッションは本担当のハンドルでなく直接 poll していない。確認時点の非検出は将来の排他を保証しない。
- `gh issue view 1` / `2` の本文・state・updatedAt を 2026-11-18 18:12 UTC 頃取得し、各終了コード 0。AGENTS.md・agent-workflow は直前のレビューで読取済み。
- 実機操作・クリック・ドラッグ・アプリ起動・build・試験通信、対象セッションの再起動・停止、製品・既存証拠・Git・GitHub の変更は行っていない。今回のレビュー担当の失敗なし。
- 出力の親ディレクトリ存在と確定先・tmp の不存在を確認し、UTF-8 往復と代表文字を検査後に確定。参照対象版が変われば再レビューが必要。