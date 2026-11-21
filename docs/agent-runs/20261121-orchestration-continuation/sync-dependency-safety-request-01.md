# テスト依存の通信・資格情報経路の調査依頼

- 担当 `/root/existing_work_review`。共有 run `D:/Projects/GithubIssueWidget/docs/agent-runs/20261121-orchestration-continuation/`。所有は `sync-dependency-safety-result-01.md` と必要な固有質問だけ。読み取りで他者の変更を保護する。
- `sync-test-runtime-review-01.md` B2、`sync-test-runtime-audit-01.md` と隔離版テスト・設定・Node/Vitest/esbuild の起動経路を照合し、テスト実行中の外部・loopback 通信、実 Credential Manager、環境・設定読取りへ至る具体経路を調査する。依存全体の無害性を推測で保証しない。
- 実行に必要な環境・入力、秘密の非継承、ネットワークや資格情報にアクセスしないことを根拠で示せる範囲、必要なら実効性を確認できる隔離方式、出力先・容量・ログと失敗時の扱いを具体化する。旧不明プロセスとの共有資源を明記し、実現可能な次の検証を示す。
- テスト・プロセス起動、ネットワーク、資格情報、GUI、製品や既存依存の変更はしない。対象 SHA、読取り、終了結果、実際の事実・仮説・未確認と失敗を記録し、確定後パスを通知。別担当による固定版レビューと実行判断が必要。
