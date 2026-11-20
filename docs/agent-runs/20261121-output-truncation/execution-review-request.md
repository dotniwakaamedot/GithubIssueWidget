# 実 GitHub 実行前レビュー依頼

一次資料は `docs/handoff-20261121.md`、`docs/agent-runs/20261120-current-review/live-plan-02.md` と本 run の `harness-review.md`。レビュー対象は `run-live.py` SHA256 `502D1A7F614D4D7A24581C1C353E3C5563EC95D6E67B3CB7FA3ABDF36BE2340B`、隔離 `creation.rs` SHA256 `15B29DE5A997FBF93EB8A2D038122C87256FC615944118A2D688D3D68D0B6E5F`、隔離 `movement.rs` SHA256 `ADDD844D3CBF35859A15499390DBD083FA482F640C8A51E2D9123B8C3D01C583`、試験 EXE SHA256 `193B97EDA5F965077229EE5F8D642A88BBB092BFC46470CA703FBFD3E1756EE1`。製品 main/creation/movement は引継ぎ記載の SHA と一致。`cargo test --no-run` と `cargo test live_identity` は終了コード0。

実行せず、対象版を確認して製品との差分、実行・子プロセス・タイムアウト・資格情報・出力・再実行防止・異常時の安全性を計画と独立照合する。全ページ照合とレビュー未了のまま書込みしない。指摘を severity とともに結果ファイルへ記録する。
