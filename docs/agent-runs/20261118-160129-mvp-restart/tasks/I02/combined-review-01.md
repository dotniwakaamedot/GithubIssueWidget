# I02 追加検証・ドラッグ診断の独立レビュー 01

- run_id / task_id / review_revision / 回数: 20261118-160129-mvp-restart / I02-combined-review / 01 / 1
- owner_role / owner_agent_id: verification_reviewer / /root/plan_review。検証結果作者 /root/foundation、診断作者 /root/drag_diagnosis と別担当。
- 参照と SHA256: `request-05.md` F01FFAE95DFD63405313F7C3BE3A0D3165A01FAEEA2E800994AB3F6036932EC9、`result-02.md` 520044FE6138609B99595C5FAF1D3973DD2D62EB82F27FE90392568C9599ED13、`drag-request-03.md` 46C1451583A57D0D71EC2A607F409C92CE0F7FA7D1E81790CA2FBA83CE5E7BEE、`drag-result-01.md` B34245F943AD4B9E69F5DBC8020C762499055A2ABDE878F57446131038C5CCB6。
- その他の依頼指定参照 `result-01.md` F0AEAD322062EED3054B1A51B0A44DC05EB85E257A3A2EE59026F99C9112BE03、`review-01.md` 7DB95B62CF8D23894AAEB4FCFAB51DD4A4634CE42C1C330421E4689E4FE6DEF9、`evidence-03/drag-history-01.md` 49233DFDBC3A610E1FE99670607463C26E917F496022D9756EAC3A07E74B7B02、`management-03.md` 3A59B412A078E8D22D0EDC269A5257F12ABBCE397E7BF9DFA4F603EC9E0ABF77、`plan-02.md` A6CB37DC369D69B334564235CD00C3AF284250109D4A56BE63C6CC7B104FE990。全て依頼と実体が一致。
- 対象 worktree `D:\Projects\GithubIssueWidget-issue-2-20261118-160129` の branch `task/issue-2-foundation-20261118-160129`、HEAD a4ac97286785a2a5ba0d070a909b58003fc49ff1 は一致。結果列挙の製品17ファイル、証拠9ファイルと release/debug 実行ファイルの SHA256 は全て実体と一致。status は README 変更と widget 未追跡16件のみ、削除なし。
- 判定: **成果の事実・限界の整理は対象結果版に限り確認。I02 製品は引き続き未承認・未完了。** B01〜B04 の未達を保持する。診断は原因特定・移動の成功・後続操作の承認を意味しない。

## Issue #2 と B01〜B04

| 指摘 | 対象版・証拠から独立に確認できたこと | 判定と未確認 |
| --- | --- | --- |
| B01 移動 | `drag-history-01.md` に window ID・相対座標・2回の呼出し・座標不変の事後記録あり。元スクリーンショット・保持時系列・旧版 hash はない | **未解決**。事後記録は過去の実行結果の独立保存ではなく、イベント到達や実際の移動可否は未確認。追加ドラッグは禁止 |
| B02 初期・最小寸法、中央、最前面、リサイズ | `ui-observation.json` は画像領域902×522・originとボタン文言の変化を示す事後保存。config に要求値がある | **未解決**。DPI・枠・作業領域・OS topmost・各方向・最小制限は未実測。画像も保存なし |
| B03 開発起動・release build・証跡 | `method.md` はコマンド・環境・終了コード報告を記載。`dev.log` に debug 実行開始、`release.log` に build 完了があり、両 exe hash は照合成功。作者は exit 0 をプロセス結果で確認したと記録 | **部分的前進、未解決**。ログに独立した終了コード・その場の全17ソース hash の証跡はなく、ログ単体で当該対象版との完全な対応や開発起動後の正常終了を証明しない。UI 観測も事後保存であり実行結果を過大評価しない |
| B04 Opener の許可・拒否 | UI 観測記録は Brave と `https://github.com/` を示す。`scope_check.rs` と `scope-check.log` は指定2パターン対して2許可・7拒否の glob マッチ結果を示す。capability と一致 | **未解決**。事後保存から既定ブラウザであることの独立証明はできず、glob 単体は Tauri IPC ACL と URL 正規化を含む拒否を実証しない |

## 診断の独立照合

- 対象 `src/main.tsx` は header の左ボタン押下で `startDragging()` を呼び、capability は `core:window:allow-start-dragging` を指定。ローカル JS API は `plugin:window|start_dragging` を呼び、`tao-0.37.2` Windows 実装は `ReleaseCapture` と `WM_NCLBUTTONDOWN` を使う箇所を確認。過去の実行時にこれらが到達・成功した証拠ではない。
- [Tauri Window Customization](https://v2.tauri.app/learn/window-customization/) は旧 `data-tauri-drag-region` が直接適用した要素だけに作用し、カスタム `mousedown` からの `startDragging` 例を記載。旧版の DOM・入力対象が未保存のため、それが過去失敗の原因と断定できない。
- Computer Use の `docs/api.md` の DragInput は始点・終点をウィンドウ相対と定義。保持時間、WebView・OS 到達の実測保証は参照部にない。事後記録は押下から解放の時系列を保持しない。UI 自動化・DOM 対象・IPC 権限・Windows から選択した原因を確定する根拠は不足。
- 提案された DOM/IPC/OS 観測は将来の設計案であり、対象 SHA 変化・副作用・秘密・撤去を独立確認し新版を固定する必要がある。原因確定前に移動を再試行しない制約を保持し、実施や入力注入を許可したものとはしない。

## 必要な是正

| ID | 種別 | 根拠・影響 | 完了に必要な条件 |
| --- | --- | --- | --- |
| B01 | blocking | 座標不変2回、入力・OS 到達と原因が未確定 | 同じ移動試行は禁止のまま、親が安全な診断計画と停止条件を判断し、Issue の要求を証明できる証拠を得るまで未完了 |
| B02 | blocking | 測定可能な OS 状態と最小寸法・リサイズの実証なし | 実測と独立追跡可能な証跡を用意。安全性未確認なら実施せず未確認を維持 |
| B03 | blocking | build ログ・exe はあるが終了コードと対象ソース版の同時証拠は不足 | 対象の全ファイル hash、正確な環境・コマンド、終了コード、起動と build の保存証跡を同一検証 run に結び付ける |
| B04 | blocking | 正規 URL の観測と glob 検証のみで IPC 拒否・既定ブラウザは未証明 | 外部状態を変更しない方法を管理が確認し、許可と不許可 URL の実際の経路と結果を記録する |

## 実施内容・未実施

- cwd `D:\Projects\GithubIssueWidget`。依頼9件の参照ハッシュを実体と照合し、結果2件、両依頼、drag history、`evidence-02/method.md`・`ui-observation.json`・`scope_check.rs`・`scope-check.log`・各ログ末尾を UTF-8 で読取。結果列挙の証拠と製品実体を `Get-FileHash -Algorithm SHA256` で照合。各コマンド群終了コード 0。
- `git -C <対象worktree> branch --show-current`、`rev-parse HEAD`、`status --short --untracked-files=all` で branch、HEAD、全差分と削除なしを確認。終了コード 0。
- GitHub `gh issue view 1` / `2` の body・state・updatedAt を 2026-11-18 17:03 UTC 頃読み、追加された #2 完了条件も確認。終了コード各0。Tauri 公式資料を web で参照、ローカル Computer Use API・Tauri JS・tao 実装を `rg` で確認。
- Computer Use API 文書のパスを最初に推測して `rg` を実行し、存在しないパスで OS error 3 が1回発生。再試行せず `rg --files` で正規の場所を特定し読み取った。同種の失敗の反復はない。参照ハッシュ・担当・対象の不整合は発生していない。
- 実機入力・ドラッグ・移動・build・テスト再実行・GitHub 更新・製品や証拠の変更はしていない。ログ・UI の現場再観察、当時のプロセス結果、既定ブラウザ設定、IPC 拒否挙動は未確認。