# I02 専用コピー検証方式の独立レビュー 03

- run_id / task_id / review_revision: 20261118-160129-mvp-restart / I02-copy-design-review / 03
- owner_role / owner_agent_id: verification_reviewer / /root/plan_review。作者 /root/verification_design とは別担当。
- 対象 `tasks/I02/verification-design-result-03.md` は依頼値と同じ SHA256 B2B552301D897717CC80404A64297C460BADED4C8AE4B0F130B1557F7B08DAED。確認開始後と終盤に実体を照合して一致。
- 依頼の構造化データにある参照9件は全て基点・相対パス・期待 SHA を分離し、先に全件の存在を確認してから SHA256 を照合、全件一致。対象17ファイルは `result-02.md` の製品欄に限定し同様に全件存在確認後に照合、全件一致。対象 worktree の branch `task/issue-2-foundation-20261118-160129`、HEAD a4ac97286785a2a5ba0d070a909b58003fc49ff1、README 変更と未追跡16件は結果記載と一致、削除なし。
- 判定: **対象 SHA256 の専用コピー方式の設計に限り承認**。blocking 指摘なし。実環境の安全・利用可能性、依存やツールの動作、検証の実施、Issue 完了の承認ではない。

## 要求ごとの独立確認

| 要求 | 確認と根拠 | 判定・残る不明 |
| --- | --- | --- |
| 元 worktree と ignored の保護 | 相対出力をコピー内に置き target・npm/Cargo/Rustup cache・TEMP/TMP・evidence を別指定。通常バイトコピーを用い、同一 inode・リンク・既存領域との衝突を回避し、元とコピーを前後照合 | 設計上適合。実環境のリンク、子プロセス・追加出力、所有と同時利用は作業前に確認が必要。単一ユーザーの OS アクセス自体を遮断する保証はない |
| 元版・コピー・生成物・証跡の対応 | 元17件とコピー17件の SHA・branch/HEAD/status、lock・依存・環境差・コマンド・exit code・exe hash を manifest に結び、元の既存 exe と同一と決めつけない | 設計上適合。前後 hash だけで一時変更の不存在は証明できず、作業独占とログの整合を別途確認する |
| 認証・設定・通常副作用 | npm/Cargo の設定・script と取得先を事前確認し、既存の Cargo credentials や token をコピーしない。専用 APPDATA / LOCALAPPDATA は万能と仮定せず、WebView profile 不明なら UI 保留。親設定を永続変更しない | 設計上適合。環境変数はセキュリティ境界ではなく、子プロセスがアクセス可能な認証情報・システム設定、lifecycle scripts と実際の profile の挙動は未確認。重要な前提が判別できなければ実行しない |
| B03 開発起動・build | 同じ Windows 上の専用コピーで、正常な実プロセス終了コード・対象版・生成 exe とログの対応を計画。依存・初期環境が異なる場合は記録 | 検証案として適合。依存準備・toolchain・ポート・実行可能性は未実証。実行時の予期しない書込み・ユーザー操作との競合は停止条件 |
| B02 初期・最小・中央・最前面・リサイズ | 対象 HWND/PID と exe、物理/論理座標・DPI・枠・モニター・OS topmost を照合し、各方向と最小制約を別途安全確認する方針 | 検証案として適合。実機性と測定・操作は未検証。原因不明のウィンドウ移動は再試行せず未達 |
| B04 実 IPC、許可・拒否、既定ブラウザ | 実 IPC 非 mock、既定関連付けと実 URL を照合し、誤許可時の外部影響を防げないケースは実行しない。別 profile の動作を既定ブラウザと混同しない | 検証案として適合。WebDriver/IPC・既定 browser/profile の実行可能性、拒否原因と未起動の測定は未確認 |
| 所有・並行・停止 | scratch・証跡・プロセスの所有を確定、同一窓・ブラウザ・port を直列化。結果・生成物を保持し、同種2回で該当作業を止め独立調査、3回で全停止 | 適合。担当や具体パス、実行時の干渉と権限は今後の依頼で確定する |

## 指摘・限界

blocking および新規 non-blocking 指摘なし。隔離 VM を必須とせず、同一 Windows の通常の副作用と保護対象を分けた実行案として評価した。設計は実行を自動許可せず、実際の path・依存・profile・管理判断の不明点を中止条件としている。実機・ビルド・IPC の条件はこのレビューでは未完了。

## 実施内容

- cwd `D:\Projects\GithubIssueWidget`。`Get-Content -Encoding UTF8` で構造化依頼、対象結果、環境調査、作者依頼を読み、`ConvertFrom-Json` で担当と参照9件を抽出。全パス `Test-Path -PathType Leaf` 完了後に `Get-FileHash -Algorithm SHA256` で全件一致を確認。各コマンド群終了コード 0。
- `result-02.md` の製品欄のみを解析して対象 worktree の17ファイルを全件存在確認後に hash 照合、`git -C <worktree> branch --show-current`、`rev-parse HEAD`、`status --short --untracked-files=all` で対象状態を確認。終了コード 0。
- `gh issue view 1` / `2` の本文、state、updatedAt を 2026-11-18 17:39 UTC 頃取得し、Windows 実行と画面・Opener の完了条件を独立照合。各終了コード 0。AGENTS.md・agent-workflow および前版設計・レビューは直前の独立レビューで読取済み。
- 実機操作、ドラッグ、アプリ起動、build、試験通信、コピー作成、製品・既存証拠・Git・GitHub の変更は行っていない。今回のレビュー担当の失敗なし。
- 出力の親ディレクトリ存在と確定先・tmp の不存在を確認し、UTF-8 の往復と代表文字を検査して確定。対象版変更後は再レビューが必要。