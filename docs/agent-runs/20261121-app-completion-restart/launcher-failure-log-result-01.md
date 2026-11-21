# ランチャー検証の失敗記録結果

- 担当 `/root/launcher_failure_log`。作業場所 `D:/Projects/GithubIssueWidget`、共有 run `D:/Projects/GithubIssueWidget/docs/agent-runs/20261121-app-completion-restart/`。
- 一次資料 `interactive-launcher-result-01.md`、`logon-session-resolution-01.md` と、作者の確定回答 `launcher-failure-answer-01.md` SHA256 `2ECB55CE77132D4F83EFFECCD8B5698034C288FD6DC79AEDF5B7F674A384EC74` を参照した。元の子 stderr は両者とも保存されず、直接原因は未確定。期待実行結果と実観測・代替成功を区別した。
- 既存索引、分冊001・002、教訓を ID・語句で限定照合。直接原因を同一と確定できる既存事象が見つからず、`docs/failure-log-002.md` に F-088（Python launcher 未発見）、F-089（合成 preflight 正常ケース終了1）、F-090（Read-Host 確認子終了1）を各１件記録。既存 F-009・F-035・F-060 との関連は示しつつ同一原因と断定しない。各IDは現時点で本記録の１発生のみ。旧ログ全体の過去累計はこの判断で確定しない。
- 本担当が F-090 の文字列を転記した際にエスケープを余分に追加した１件を、読戻しで確認・修正し F-091 に記録した。F-090 は別担当の別操作であり二重加算しない。F-088〜F-091 は各１件で、累計３件以上の新規IDはない。既存の教訓 `docs/recurring-lessons.md` は変更せず、同内容の一般予防策を各記録に明記した。効果は未確認。
- 変更後 SHA256: `docs/failure-log-002.md` `145D5146BEE72863ABB1C3ACE0A83A4FB21FD1C0BB3D314DCC0F311D0681FB44`、索引 `docs/failure-log.md` `031B18757E095648C5AB92DBEBDCE9CBDDEA1C8999AF3144810A03B5DA3E0014`。教訓の非変更を SHA256 `96CF6237C43D051F03B3D6D6F2BFEEBB8B233C55F003D8CE764B730683E0A79E` で確認。
- 検証: 作業ルートから索引・当該分冊・一次資料の限定読取りと SHA256 照合、追記直後の UTF-8 実ファイル読戻し、`git diff --check -- docs/failure-log.md docs/failure-log-002.md` 終了0。Git は索引について LF/CRLF 警告を表示。分冊002は未追跡のため diff のみで実体を証明しない。対象資料の表示は出力切詰めなし。削除対象なし。
- 未確認: 元の子 stderr、F-089/F-090 の直接原因、本人環境の Python launcher、対策の再発防止効果、過去ID全体の累計。作者以外による独立レビューは未実施であり対象版の承認を求める。本結果自身の SHA は確定後に管理側が取得する。
