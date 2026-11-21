# 実行環境監査（2026-11-21）

対象: `main` HEAD `4961b25bec53a93439acdf5230de77f959908af0`。担当範囲は読取りと実行検証で、アプリコードは変更しない。実 GitHub への書込みと資格情報の表示は行わない。

## 確認結果

- `Get-FileHash -Algorithm SHA256` で `widget/release-20261121-02/GitHubIssueWidget.exe` は `D19A72750006DF5BB6CBCA4FB15F87F69BFF48A5785F5569C5B05E40C9B93F26`。`widget/src/main.tsx`、`widget/src-tauri/src/main.rs`、`widget/src-tauri/src/movement.rs` のハッシュは引継ぎの記録と一致した。これだけではバイナリと全ソースの対応を独立に証明しない。
- `Get-Process -Name GitHubIssueWidget -ErrorAction SilentlyContinue` は出力なし、終了コード 1。同名プロセスを取得できなかった。これだけで他名の関連プロセスの不存在は証明しない。
- Computer Use の `cua.getState()` は `SetLayeredWindowAttributes display overlay failed: ... (0x8007001F)` と返り、ウィンドウ状態を取得できなかった。操作は行っていない。親の指示により再試行しない。
- 引継ぎは同じハッシュの release のビルド成功を記録している。ビルドを今回再実行していない。実 UI 起動・通常終了、Windows 操作、Credential Manager の実保存・更新・削除・再起動後復元、実 GitHub 認証・作成・移動は今回未検証。

## 失敗

1. ３文書を一括表示した結果、外側出力が 10,188 tokens で切詰め。個別の読取りコマンドは終了 0、欠落部分の完全ログなし。
2. `rg` の検索出力が 9,554 tokens で内側上限 3,500 を超えて切詰め。Windows で指定した `*.rs` が解釈されずエラーを伴い、終了 1。完全ログなし。

同じ処理中に出力切詰めが２回発生したため対象検索を停止し、親と独立分析担当へ報告した。欠落内容は未確認。CUA エラーと合わせ、原因や修正効果は未確認。
