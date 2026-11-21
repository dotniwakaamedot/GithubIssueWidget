# MVP 実行ファイルの固定結果レビュー

- 確認 `/root/plan_review`、作業者 `/root/gui_failure_investigation` と別。依頼 `mvp-release-result-review-request-01.md`。
- 期待・開始/終了 SHA256: 結果 `33B3D971D3EC8EE4BACFA48E1F778A51200F935AC05EB8FA270954DB607BEBAA`、ユーザー用 EXE `91EC5D237B71EFC4DDCD12244E0022019FD61A0D1610931CA0B9FE2D4A2D55B2`、入力 manifest `673FDABA5A91336866FB4A8F073F773E071FF67146F2F04516CD9DD337B89024`。保護ログも `2513579A881F05EB433B4C1A9E476467FCCE711FAAD0779C99DCB6B70227DB4A` と一致。削除対象なし。
- 判定: **固定入力の release ビルド成果とユーザー用コピーを承認**。blocking なし。現時点で GUI が利用可能・実 GitHub と認証が動作する証拠や Issue 全体の完了承認ではない。

| 観点 | 独立確認・限界 |
| --- | --- |
| ビルド | 保護ログ 1,654 bytes に `Finished release profile` の固定分類があり `error:` 行なし。status ファイルには `exit_code=0` を確認。runner の実行方法と終了結果の因果、ビルド前後の操作は実行者の記録にも依存する。ログ本文は公開しない。 |
| EXE・入力 | ユーザー用 `widget/release-20261121/GitHubIssueWidget.exe` は 12,883,456 bytes、再解析点でない。生成元 EXE と SHA 一致。manifest の20ファイルは現在の製品・保存入力・build 入力で全件 SHA 一致。診断ステージ付き隔離版と区別した通常製品入力。 |
| アクセス | EXE の ACL には組込み Users の ReadAndExecute 許可があり、ファイル ACL の明示的 deny は観測されない。これは実ユーザー・親ディレクトリ・OS 保護を含む実効実行や WebView2 の成功を保証しない。 |
| 未確認 | 現在の EXE の実 Windows 11 GUI 表示・通常終了、Credential Manager、GitHub 認証・作成・移動、実障害や未追跡子孫終了は未確認。ユーザーによる画面確認を実施結果と混同しない。 |

作業場所 `D:/Projects/GithubIssueWidget`。指定ファイルは存在を確認した上で SHA を開始・終了に照合し、manifest の20入力と生成 EXE・コピーを比較。ログは固定分類のみ、ACL は必要な属性だけを確認。ハッシュ・検査は終了0。起動・再ビルド・テスト・資格情報・外部操作・製品編集は行っていない。対象版変更後は再評価する。
