# 反復失敗への判断

ユーザーの明示指示に基づき、独立担当 `/root/failure_a` と `/root/failure_b` の `repeated-failure-review-a.md`、`repeated-failure-review-b.md` を照合した。F-001、F-073、F-016 の各対策を採用し、`AGENTS.md` の作業基本方針と Issue 運用、および `docs/recurring-lessons.md` に具体的な運用を追記した。効果は今後確認する。元の失敗の未確認原因は確定しない。

進行レビューからの改善提案 P01〜P03 は採用。提案書は `repeated-failure-proposal-a-p01.md`、`repeated-failure-proposal-a-p02.md`、`repeated-failure-proposal-a-p03.md`。既存の承諾はテスト用 Issue 一件の作成であり、これらの追加 Issue 登録への認可は確認できていないため外部に登録していない。対象の重複照合・登録・読戻しも未実施。新規 Skill 提案なし。

F-073 は現行の `test_job_process.py` SHA256 `795132F85FC9E7E2BF00ABE03E0EDFD9794669C6931B6D4E49F7A1C05DCBCF5A`、`job_process.py` SHA256 `EAEE9B53AA64D63FCAEAB824CE6E4BA512EE080B6D90DDEEEC644109BE53A71E` を対象に、仮想環境の起動子ではなくベース Python で制約付き合成試験を一度実行し、親が終了コード0・1件 OK を確認した。ログ SHA256 `F68E0131DC838145E2048C4A0A8948D95FB03C48614FDA603E47E536B59A5E8D`。実際の gh / Rust と OS 異常時を検証したものではなく、実 GitHub 書込みの無条件停止は解除しない。

追加確認: `probe-fixture-list.py` は固定 SHA256 `193B97EDA5F965077229EE5F8D642A88BBB092BFC46470CA703FBFD3E1756EE1` の Rust 試験 EXE に単一プロセス Job で `--list --exact` を指定した。終了コード0、対象テスト名が列挙された。テスト本体は呼び出さない列挙を指定したが、子へ親環境が継承され、認証操作や通信の有無は独立観測していない。テスト本体や gh の適合性・終了保証の証拠にはならない。詳細は `live-next-review.md`。

`probe-gh-version.py` は SHA256 `DF05089117A23889B3F6EA08D7644849AFE7B6025647A09F33B45C71F1418F75` の `gh.exe` に同じ制約下で `--version` を指定し、終了コード0と期待する出力形式を確認した。子へ親環境が継承され、認証や通信の有無は独立観測していない。`gh auth token` の実行経路は未検証。
