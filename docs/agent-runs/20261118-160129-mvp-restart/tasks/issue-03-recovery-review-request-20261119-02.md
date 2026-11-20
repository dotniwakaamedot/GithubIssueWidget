# Issue #3 復旧処理の部分レビュー依頼

担当 `/root/connection_review`。前回レビュー `tasks/issue-03-review-20261119-02.md` の R01 とその修正、および新規テストに対象を限定した部分レビュー。#3 全体の完了判定ではない。実 GitHub 接続・実 UI・実 PAT・復旧処理全体の実ストア検証は未実施と明記する。独立調査 `tasks/issue-03-test-diagnosis-20261119.md` も参照する。製品・資格情報・UI・GitHub は変更しない。

対象作業場所は `C:/Users/semis/orca/workspaces/GithubIssueWidget/issue-3-connection-resume`、HEAD `59578f8a4a775b4bd66c6acdde318b2e934abc08`。ファイル SHA256:

- `widget/src-tauri/src/main.rs`: `B61E1A7093AA9BF054C35732126F068511A68E086E833286963F976E4504D7CA`
- `widget/src-tauri/Cargo.toml`: `82A440BA35FA24E315B3A458721021A43CC7CB2EEE39D1968B0C9B52BADA0540`
- `widget/src-tauri/Cargo.lock`: `739BD4B3836227C96E62CB557D8B3EDA4ECEEA38B9065A48E832F6550A7DA8C2`
- `widget/src/main.tsx`: `3C2671FF98A1EFA5B66D2F872886153E02A64EA455F0590118C5ED3EB8C5CC5D`
- `widget/src/main.test.tsx`: `BA5FC6E3A37DF2C792459085CEFF70B201F8F50207852A393E0313260757260E`
- `widget/src/style.css`: `E2CBCCCEA0DFAF8B334F80F7F4A5CA4FA0B9E0EE9FCB4751C66517AC60D09078`
- `widget/package.json`: `102235B5686088C87B3221DEC7AE3CBEE20F0C9B1EAE070FD730C8B631DFCEDF`
- `widget/package-lock.json`: `A5D65370135F83B08F02D5F1EE329706F08551743444264BCB10DFC7600B0C73`

作者報告: `cargo test --locked` 11件成功、2件 ignored のうち合成 Windows credential lifecycle は個別実行で成功、子プロセス lock helper は親テストから実行。`npx vitest run` 1件成功、`npm run build` 成功。これらは出力を保存していない作者報告。復旧記録・構造の安全性と新たな未確認を静的に評価し、`tasks/issue-03-recovery-review-20261119.md` に UTF-8 一時ファイル経由で確定する。未実施を成功としない。
