# 最新 MVP 再ビルド固定版レビュー

- 確認 `/root/plan_review`、作者 `/root/gui_failure_investigation` と別担当。依頼 `mvp-release-02-review-request-01.md`。
- 期待・開始/終了 SHA256: preflight `0A2F9AF6DD0F0DAAA310DCE4B9BEDD972582D25824AEF0CA7CDDB87FF39644A2`、runner `804316F47E161BD79B2C21D5A95B96006129BB292AFE3FA78C7B621F68A62267`、manifest `2EB56D635C9CCC132D46BA80B641B8A81444DFE79CA9D75E37D58349302B586C`。削除対象なし。
- 判定: **対象版の再ビルド方式を承認**。blocking なし。直前の入力・出力・保護確認後、変更済み UI を反映するために1回実施する。実ビルド・GUI・認証・GitHub の成功を承認するものではない。

前回 manifest と比較し、ファイル追加・削除なし、変更は `src/main.tsx` と `src/main.test.tsx` のみ。現製品・保存版・専用 build 版の20ファイルは各 manifest SHA に一致。前回承認 runner との差分はログ名と status 名のみであり、従来の隔離・offline 条件を維持する。

旧ユーザー用 EXE の SHA `91EC5D237B71EFC4DDCD12244E0022019FD61A0D1610931CA0B9FE2D4A2D55B2` は保持。旧入力2件・EXE backup の存在を限定一覧で確認し、新ログ・status・配布ディレクトリが現時点で存在しないことを確認した。実行前に先行 build の終了・対象入力の一致・backup の SHA・新規出力先の再解析点と不存在を再確認し、旧ファイルを上書きしない。成功後は EXE SHA・配布コピー・読み取り/実行 ACL を確認する。

直接ビルドの終了とログ、EXE の生成を区別し、実 GUI・実 GitHub・credential や子孫全体の終了は別途確認する。変更された UI 版のビルドであり、前回成功版の無理由な再実行ではない。

`D:/Projects/GithubIssueWidget` で依頼・preflight・manifest・runner 差分と入力/旧成果を読取り照合。差分表示は差分あり終了1、ハッシュ確認は終了0。ビルド・GUI・外部操作・製品変更はしていない。対象版変更後は再評価する。
