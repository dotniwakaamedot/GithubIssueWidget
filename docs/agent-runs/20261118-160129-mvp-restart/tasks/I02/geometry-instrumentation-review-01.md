# I02 診断版 DOM 境界観測の独立レビュー 01

- run_id / task_id / review_revision: 20261118-160129-mvp-restart / I02-geometry-instrumentation-review / 01
- owner_role / owner_agent_id: verification_reviewer / /root/plan_review。作者 /root/drag_design とは別担当。
- 対象 `geometry-instrumentation-result-01.md` SHA256 31A5ECC4D7699F27B00E6B6717FF69D40CAB857BDF463BF671D1A7EA0DF41B52。構造化依頼の参照15件は基点・相対パス・期待 SHA を分離し、全件存在確認後に実体 SHA256 を照合して一致。
- 旧 scratch `D:\Projects\GithubIssueWidget-drag-diagnostic-20261118-160129`、新 scratch `D:\Projects\GithubIssueWidget-geometry-diagnostic-20261118-160129`。`copy-manifest.json` SHA256 F75B6663167FADC32277BE610DAA67C1C3B96B7FC74BA1AEB6890C9EF3C56DC1 の18件について旧新36パスを全件存在確認後に hash 照合。旧18件は manifest と一致、新版は17件不変・`widget/src/diagnostic.ts` 1件変更。
- 対象 `widget/src/diagnostic.ts` の旧 SHA256 56A62632E8D63AEE5DD0AB636C954E30FBB76562C90C88CD9C0D53AB497B6D37、新 SHA256 840D7F69349AEED52C0A5D6FA6319317CCE432390A74463F8E824069F26E412D。新版 `main.tsx` 820F164EF8947DA6B87F6ACE8B74D787E09ED9AF7A6610E2C07678DFA2C4AE5C、`vite.config.ts` 194D17A739D0F626DDAF94B5D2B6F9335DBF50BB571AB4F7739DA727ED4F772A、`tauri.conf.json` EDE0EA3E1EE5C6FE067A5E127EEAAE02A19110E8CD8E057C8B7FCB2B826A55D5 は依頼と一致。
- 元 worktree `D:\Projects\GithubIssueWidget-issue-2-20261118-160129` の branch `task/issue-2-foundation-20261118-160129`、HEAD a4ac97286785a2a5ba0d070a909b58003fc49ff1、README 変更と未追跡16件は維持、削除なし。
- 判定: **対象 SHA256 の診断コードと明記された実行前ゲートに限り承認**。blocking 指摘なし。実際の build・起動・capture・入力や製品完成を承認するものではない。

## 要求ごとの確認

| 要件 | コード・設計の確認 | 判定・未確認 |
| --- | --- | --- |
| 前レビュー B01 の具体的矩形取得 | `getBoundingClientRect()` で header・marker 各1、button 3、resize 8の CSS viewport 矩形を同期呼出し中に読取り、固定 output に JSON を表示。不足要素は `geometry-unavailable` | **設計上是正**。output が実際に main WebView の accessibility / screenshot で完全に読めるか未実証。別版であり旧 AX と画像の不一致の解消を意味しない |
| 非重複・12 CSS px | 各矩形間に `max(b.left-a.right, a.left-b.right, b.top-a.bottom, a.top-b.bottom)` を算出し全 button/resize との最小を表示。すべて正の有限矩形であれば最小 >=12 は各ペアで少なくとも一軸に12以上の分離を意味する。marker 内・center hit と合わせて判定する設計 | 計算方法は適合。コード自身は有限値・正の幅高さを網羅検証せず、実施時ゲートで全矩形の有限・正寸法と完全な JSON を確認すること。未確認なら入力しない |
| 同期・変更・transform | viewport・header・marker の開始/終了比較、単調開始・終了、UTC を出力。計画は全矩形を含む複数サンプルと時刻の対応を要求 | `stableDuringSample=true` は button/resize の途中変化、同一瞬間、transform や遮蔽を保証しない。最新複数サンプルと実画面・OS 窓・DPI・操作 API の対応を確認し、不明なら入力しない |
| 非機密・target | 幾何 output と event output は固定分類・数値・時刻のみ。DOM text・URL・キー・Issue の読み取りコードなし。`elementFromPoint` は marker の DOM 最上位判定のみ | 適合。別 OS 窓の遮蔽や対象 process の同一性は示さない。出力が長く切り詰められる場合は未確認とする |
| drag と UI 安全 | 旧診断の header `startDragging` 呼出し除去を維持。実装の nav button、8 resize 要素、drag capability は残存し、diagnostic output は pointer-events:none | 適合。OS/表示上の hit target と安全余白の確認前にクリックしない。診断版の成功は元の過去ドラッグや製品の移動を証明しない |
| 所有・競合・停止 | 新旧コピーと B03 を分離。port 1435 は旧版と同一のため同時起動禁止。旧 handle/PID は作者の当時の観測であり現在と仮定せず、所有者・対象・port・profile を直前に再確認 | 適合。画像不一致既往1回、browser inventory error既往2回。指定された同種2回で中断・客観調査、3回で全停止を維持する |

## 指摘・残条件

blocking および新規 non-blocking 指摘なし。旧レビュー B01 の「矩形を得る具体的入口」の欠落は設計上解消された。ただし診断コードは未起動・未型検査であり、その実現性を仮定しない。画像と AX、実 window・DPI・座標・全矩形の有限正寸法と非遮蔽、イベント記録と空状態、対象 PID/版・時刻、所有と競合を実行前に確認できなければ入力しない。

## 実施内容・未実施

- cwd `D:\Projects\GithubIssueWidget`。構造化依頼の JSON 参照15件について全件存在確認後 `Get-FileHash -Algorithm SHA256` で照合。結果・新 diagnostic.ts・copy manifest を UTF-8 で読取。終了コード 0。
- manifest の旧新各18パスは全件存在確認後に hash 照合。`git -C <元worktree> branch --show-current`、`rev-parse HEAD`、`status --short --untracked-files=all` で元の状態を確認。終了コード 0。
- 実装・型検査・build・起動・画面取得・UI 操作・クリック・ドラッグ・通信試験・元/旧新 scratch・既存証拠・Git/GitHub の変更は行っていない。getState/browser inventory・失敗した画面取得は再実行していない。レビュー担当の失敗なし。
- 出力の親ディレクトリ存在・確定先と tmp の不存在を確認し、UTF-8 往復と代表文字を検査後に確定。対象版が変われば再レビューが必要。