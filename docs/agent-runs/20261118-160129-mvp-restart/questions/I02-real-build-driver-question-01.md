# I02 実ビルド driver 準備の進捗 01

- 担当 `/root/build_verification`、宛先 `/root`。依頼 `tasks/I02/real-build-driver-request-01.md` SHA256 `EECFAD527EEA925C001B9A64BA859869648CE7248DAEF30A4E241FAFCC11E99E` の固定参照9件を実体照合し一致。元 worktree の17ファイルは存在・通常ファイル・各ハードリンク1件・SHA 一致、branch と HEAD は前結果と一致。
- 新規 scratch `D:\Projects\GithubIssueWidget-build-20261119-02` は未存在・親 reparse なし・owner と free space を確認後に作成。以前の構造化 `source-manifest.json` を hash 一致でコピーし、元17ファイルを通常バイトコピー、元とコピーの17件一致。新規 `copy-manifest.json` SHA256 `BCE67077C4CB323586F52E527318B6584B3D48BBC956E879B75774ED6F5482E0`。元 worktree・旧 scratch は保護。
- 現在、専用 `Process` による stdout/stderr 同時消費、実 child の exit/EOF、timeout 時の同一 handle 保持、VsDevCmd 環境取込みと資格情報除外を具体化している。Node / npm-cli / rustup / VsDevCmd の各実体は存在し hash を取得済み。
- 今回のコマンド失敗なし。実 build・依存導入・外部取得・driver 実行・GUI は未実施。固定版の作成後、独立レビュー・親判断を待つ。拒否された window metadata 公開操作は行っていない。
