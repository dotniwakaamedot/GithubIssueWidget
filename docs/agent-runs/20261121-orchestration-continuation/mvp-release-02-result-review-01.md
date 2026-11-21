# 最新 MVP 配布成果の独立確認

- 確認 `/root/plan_review`、作業者 `/root/gui_failure_investigation` と別担当。依頼 `mvp-release-02-result-review-request-01.md`。
- 期待・開始/終了 SHA256: 結果 `EBCBF9458EE5601F77D3861F77AB2E8594E675591BCB77A6ADE33591D7D52E65`、新 EXE `D19A72750006DF5BB6CBCA4FB15F87F69BFF48A5785F5569C5B05E40C9B93F26`、manifest `2EB56D635C9CCC132D46BA80B641B8A81444DFE79CA9D75E37D58349302B586C`。削除対象なし。
- 判定: **固定入力の最新 release ビルド成果と配布コピーを承認**。blocking なし。現行の実 GUI・実 GitHub・credential や Issue 全体の完了承認ではない。

現製品・保存入力・ビルド入力の manifest 20件は現在の SHA が全件一致。新しい配布 EXE は生成元と同一 SHA、12,883,456 bytes、再解析点でない通常ファイル。旧ユーザー用 EXE の SHA `91EC5D237B71EFC4DDCD12244E0022019FD61A0D1610931CA0B9FE2D4A2D55B2` が保持されている。

保護 build ログ SHA `4B7950FE49DF79597490C0618E3FEB2F1A71E307D8F2BBB4C4BBFB30DB3D9364` と固定分類で release 完了・`error:` 行なし、status 内 `exit_code=0` を確認した。実際の起動経路とセッション終了は実行者記録にも依存。EXE ACL に組込み Users の ReadAndExecute 許可があり、ファイル ACL の deny は観測されないが、実ユーザー・親ディレクトリ・OS 保護を含む実効実行は保証しない。

実際の GUI、保存/復元、GitHub 操作、実障害、未追跡子孫終了は未確認。前回配布 EXE は新ファイルと別版であり、利用者の確認結果を新版へ自動的に流用しない。同一版の再ビルド・試験は行っていない。

`D:/Projects/GithubIssueWidget` で依頼・結果、manifest 20件、生成元/配布 EXE、旧 EXE、保護ログの固定分類と ACL を読み取り確認。開始・終了 SHA は一致（終了0）。起動・ビルド・外部操作・製品編集は未実施。対象版が変われば再評価する。
