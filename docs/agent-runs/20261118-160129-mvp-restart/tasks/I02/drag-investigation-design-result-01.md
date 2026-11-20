# I02 ドラッグ失敗の追加切り分け方法 設計結果 01

- run / 担当: `20261118-160129-mvp-restart` / `/root/drag_design`。依頼 `tasks/I02/drag-investigation-design-request-01.md` SHA256 `4E1CA571E5A9E05BCFD77F32570A4D8FB7CAF693CC1D69ADD254DD313A2CFABE`。
- 結論: **既存証拠では原因を特定できず、移動の完了条件は未達。** 元のドラッグを追加実行せず、まず別の診断版で入力到達と権限の両段階を分ける。以下は計画案であり、実施結果・レビュー承認ではない。
- 対象: `D:\Projects\GithubIssueWidget-issue-2-20261118-160129`、branch `task/issue-2-foundation-20261118-160129`、HEAD `a4ac97286785a2a5ba0d070a909b58003fc49ff1`。README 変更と widget 未追跡16件。開始時に17件すべて実体 SHA256 を取得し `tasks/I02/result-02.md` の製品一覧と一致。HEAD は未コミット内容を特定しない。削除対象なし。
- 参照: 依頼指定の9ファイルは先に全件の存在を確認し、SHA256 を実体計算して全件一致。とくに `drag-result-01.md` は `B34245F943AD4B9E69F5DBC8020C762499055A2ABDE878F57446131038C5CCB6`、`combined-review-01.md` は `F6DF723A067C988F144087B6A4833EDD306AAB56071068894424D359F70004D3`。

## 確認された事実と区別

過去の2回は異なる実装版で自動ドラッグ後に座標不変と報告された。元スクリーンショット、旧版 SHA、OS の押下状態や WebView のイベント・IPC 成否・OS メッセージは未保存。対象版の `widget/src/main.tsx` SHA256 `07318B40089C01EEB897EFDB5A933DD15982C51E51CDAEABC097AC219C867AFE` は header の左ボタン押下で、button 以外に `startDragging()` を呼ぶ。capability SHA256 `EE3B901B492DF38B46A5D7E7DEC5C7B1707472DFDE0AE8ECF08C6BEC529A0AA1` に `core:window:allow-start-dragging` がある。これらは過去の動作成功の証明ではない。

Tauri 公式 [Window Customization](https://v2.tauri.app/learn/window-customization/) は `data-tauri-drag-region` が子に継承されないことと mousedown から `startDragging` を使う例を示す。旧版の対象 DOM は不明であり、非継承は仮説にとどまる。ローカルの `tao` 0.37.2 が `WM_NCLBUTTONDOWN` を非同期に投稿するという既存診断は実行時到達の証拠ではない。[GetAsyncKeyState](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getasynckeystate) は取得時点の物理ボタン状態を示し、アクセス失敗時も 0 を返し得る。0 を必ずボタン解除と判断しない。

## 実行可能な段階的な検証案（未実施）

1. **固定・競合確認**: B03 セッションの所有者・生存状態・画面/ポートを管理が確認。動作中の B03 には干渉しない。担当ごとの専用コピーを既存17ファイルと一対一の SHA256 で対応付け、元ソース・設定・lock・生成 exe・修正差分・ツール版を記録。実行中に元またはコピーが変化すれば止め、版を再固定する。診断用ログとコピーの所有・保持を明記。実行可否を決めるのは管理と独立レビュー。
2. **入力だけを診断する版**: コピー上の診断版では titlebar の通常 drag 呼出しを明示的に無効にし、対象 header の capture/bubble で event type、`button`、`buttons`、`isTrusted`、`target` が header/非 button 子/button のどれか、相対位置を粗い範囲、単調時刻のみ専用バッファへ記録する。テキスト・URL・キー入力・Issue 内容は取得しない。対象に単一の押下・解放のみを行い、移動や drag はしない。button と title の境界・対象 HWND/PID・座標系を操作前に目視確認。未受信なら target/送達/診断版設定を調べ、受信しても過去の入力到達は証明できない。診断版は SHA が変化し製品版の動作証明にはならない。ログ記録はイベント処理の時間に影響する。
3. **権限と invoke の分離**: 対象 main WebView と capability の対応を静的確認。実 IPC の確認には専用の診断版と安全に停止できる検証先を別途設計し、許可・拒否と promise の解決/拒否、サニタイズ済みエラー種別を記録。`startDragging` の直接 invoke は OS の移動処理を起動し得るため、この段階では呼ばない。権限設定の静的存在や、別の無害なコマンドの成功だけで drag の許可を証明しない。ログ・添付への秘密混入は停止する。
4. **Windows 側の観測設計**: 専用診断版と対象 HWND/PID を固定し、DPI、モニター、外枠/クライアント矩形と対象区画を記録。`GetAsyncKeyState` は物理ボタンの瞬間的状態に限り、アクセス/desktop の不明を別途記録。`WM_NCLBUTTONDOWN/HTCAPTION` の送達を調べる場合は所有する診断プロセス内のウィンドウメッセージ計測方法を先に実装・レビューし、注入や global hook を行わない。時刻、受信の有無、座標変化、OS 終了イベントを対応づける設計とし、未受信だけで根本原因を確定しない。該当メッセージ記録に自前 subclass が必要なら挙動と競合の可能性を評価し、不可なら未確認。
5. **判断と実機証明**: 上記から具体的な原因が確定し、修正・観測方法を独立レビューした後にのみ、管理が新しい作業依頼で実機ドラッグの方法を決める。今回2回位置不変があるため、原因が不明なまま同じ試行を再実行しない。修正後も OS での実際の移動・UI 回帰と元事象の因果を別々に証明するまで Issue #2 を完了扱いにしない。実行環境・入力対象を誤る可能性が残る場合は未検証とする。

## 対策・停止・証跡

各段階は対象 HWND/PID・操作範囲・ログ保存先と排他担当を決め、開始前に元とコピーの全17件 hash を照合。結果は時刻・cwd・正確なコマンド・終了コード・実体 SHA と対象版を紐付け、画像は対象窓のみで個人情報の混入を確認。元 worktree、既存証跡、進行中 build と共有ブラウザを変更しない。専用プロセスのみ所有者が終了し、画面位置・設定変更があれば事前状態を記録して復元、所有物は削除せず保持。環境変数分離を OS 権限の隔離と誤認しない。

対象や版の不一致、入力先不明、許可外操作、ログへの機密混入、予期せぬプロセスへの干渉は直ちに停止し保護。ユーザー追加指示に従い同種問題が初回を含め2回で該当操作を中断し別担当が客観調査、3回で全作業中止。診断版でもドラッグ移動の再試行を無断で行わず、計画・対象版ごとに独立レビューを受ける。

## 実施記録と未確認

- 2026-11-18 18:03 UTC 頃。cwd `D:\Projects\GithubIssueWidget` で `Get-Content -Encoding UTF8` により AGENTS.md、agent-workflow、依頼・旧診断・レビュー・計画を読取、依頼記載9参照と依頼自体を `Test-Path` → `Get-FileHash -Algorithm SHA256` で確認（exit 0）。
- 同 cwd の `gh issue view 1` / `2`（body・state・updatedAt）は各 exit 0。#2 はドラッグ移動を要求。対象 worktree で `git status --short --untracked-files=all`、`git rev-parse HEAD`、`git branch --show-current` は各 exit 0。`git ls-files --others --exclude-standard` と README の計17件を存在確認し hash を実測、結果02に一致（exit 0）。前後照合は同時変更の完全な不存在を証明しない。
- 対象の main.tsx・capability・設定を読み、上記 Tauri・Microsoft 一次文書を参照。Microsoft の `WM_NCLBUTTONDOWN` 個別ページの open は内部エラー1回で内容未取得。同一要求は再実行していない。同種2回で調査、3回で停止の対象として報告。ほかにコマンド失敗なし。
- 本担当の変更は本結果のみ。アプリ起動、入力、ドラッグ、OS 操作、build、実行中セッションや生成物への操作、外部書込みは未実施。IPC/DOM/OS の観測装置の実現性・本当の原因・実機移動は未確認。独立レビューと管理の次の判断が必要。
