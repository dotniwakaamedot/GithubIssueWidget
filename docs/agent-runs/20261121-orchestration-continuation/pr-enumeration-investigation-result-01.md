# PR 一覧取得失敗の独立調査

担当 `/root/gui_failure_investigation`。作業場所 `D:\Projects\GithubIssueWidget`、branch `main`、HEAD `5e74e1642fd18fe630dce0e684afd853721276ae`。依頼 `pr-enumeration-investigation-request-01.md` SHA256 `BEABB7A32E9ED87C42F2FD54CC2B0A6A45AC7F3EFDC204A3B3B695C2F8A4E1EF`。

## 結論

２件は PR 一覧を取得できなかった同種の目的上の失敗として扱えるが、実行環境・入力が異なり、同一の内部原因は確認できない。２件目はクエリ区切りを `%26` とした入力の誤りが確認されるが、それがアクセス不能応答の唯一の原因だったかは未確認。１件目の WebException の原因は不明。今回の依頼は読取り調査に限定され、再照会・合成試験は実施しておらず、PR 全件やページ終端は未証明である。

## 確認事実

1. 当事者回答 `pr-enumeration-answer-existing-01.md` によれば、2026-11-21、PowerShell・既定 sandbox の `Invoke-WebRequest -UseBasicParsing` で `https://api.github.com/repos/dotniwakaamedot/GithubIssueWidget/pulls?state=open&per_page=100&page=1` を照会。明示ヘッダーは User-Agent と Accept、認証情報は明示せず、catch の例外型 `WebException`、終了1。HTTP 状態、通信段階、応答本文は未取得。同一コマンドの再試行なし。認証不足・ネットワーク・権限・レート制限等は候補にすぎない。
2. 別の `web__run.open` に `state=open%26per_page=100%26page=1` を含む URL を指定。区切りの `&` がエンコードされ、3つのクエリパラメーターではない。ツールは内部変換後の URL と `Internal Error`・アクセス不能を返した。HTTP 状態と内部原因は未確認で、正しい URL を使った再試行もない。２件の元出力は会話上で、別の完全ログはない。
3. コネクタ専用 `search_prs` の空配列は別の成功応答。総数・継続情報・対象条件の保証が確認されておらず、PR 不存在・網羅性の証明にならない（`sync-ownership-result-02.md`）。今回上記２件に加算しない。

## 解決に向けた方式と必要条件

- GitHub コネクタでページ終端と対象の完全性が確認できる専用一覧操作があれば優先する。上記検索の空結果だけでは条件を満たさない。利用可能な通常環境の保存済み認証方式・対象リポジトリへの閲覧権限を確認し、トークン・非公開本文を出力や保存に含めない。失敗や不明を空一覧と混同しない。
- REST を使う場合、初期 URL は固定の HTTPS `api.github.com` と `/repos/dotniwakaamedot/GithubIssueWidget/pulls`、`state=open`・`per_page=100`・`page=1` を独立したクエリとして組み立てる。通信結果は安全な HTTP 状態・JSON 配列の型・件数・必要な PR 番号と base/head 識別情報・継続情報だけに絞る。外部入力の例外やレスポンス本文は無加工で出さない。
- 実一覧前に合成データで JSON 配列の型、複数ページ結合、短いページに next がある例、正常終端、途中失敗、不正・循環・対象外 host/path とページ順序、上限、空ページと next の組合せ、秘密風入力と出力抑止を確認する。各 next の scheme・host・path・クエリ条件と正の順序を検証し、循環や逸脱・途中失敗は未完了とする。`Link` の正常な next 不在を終端とし、単なる短いページや HTTP 失敗を終端としない。
- 応答中の同時更新でページ間に重複・欠落が生じ得るため、取得時刻と条件を記録し、必要時には直前に再照合する。単一回の一覧がトランザクション上の完全な瞬間像であるとは主張しない。正しい接続・取得方式の設計と対象版の独立レビュー後に実行する。
- 上記は実施案であり、通信できること・秘密抑止・全件取得の実証ではない。外部認証を変更せず権限が得られない場合は未確認を維持し、所有権や未完了 PR の不在を推測しない。

## 検証と対象版

依頼に従い資料と当事者の確定回答を読取り、`Get-Content -Raw -Encoding UTF8` と `Get-FileHash -Algorithm SHA256` は終了0。新たな GitHub 照会・通信・認証変更・外部書込み・プロセス終了・製品変更はしていない。新規の操作失敗は認識していない。質問 `pr-investigation-question-01.md` SHA256 `F44ED81AD39D5F17ACB63DB61EE3DD1B640E29A05D91396D2DA3EC407DD35640`、`pr-enumeration-question-existing-01.md` SHA256 `214EA8F0813280CAB4C481802DDDA4853B7DD3E60DD7428E92E3DD2F4BD4F0D6`、本結果を新規作成。削除対象なし。結果提出後の独立レビュー、実一覧・合成試験は未実施。

開始時と終了時に一致した SHA256:

- `sync-ownership-result-01.md`: `A8C2F73D84CFEF4B8374EDA15512EACF33B227F1350A07C452E7572A36DBA9C7`
- `sync-pr-history-result-01.md`: `FF711EBA70B06A334E3B18B06820151D0080F6429E19612FE6F48B0150353027`
- `docs/instructions/automation-auth.md`: `D5D9605C6C0ED0898EA1C51E81DF19EEB945A61368299D79D04041B00BDE15EA`
- 受領時回答 `pr-enumeration-answer-existing-01.md`: `60EA2B326DB418775D0B5E00A179AC8DACD3D36A0695577C0F448C3A0CDD95EB`。

上記うち指示ファイル以外は `docs/agent-runs/20261121-orchestration-continuation/` 配下。共有失敗記録への追加と教訓反映は管理側が同定・独立レビューを踏まえて判断する。
