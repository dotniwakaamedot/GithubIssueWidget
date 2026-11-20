# Issue #4 ページ取得の独立調査

- 担当: `/root/pagination_diagnosis`。依頼 `tasks/issue-04-pagination-diagnosis-request-20261119.md` SHA256 `F21D7773F7C3B4A3040AA5386C77CFB46FDD9738CD462F0DDF3819C976D3DECB`。
- 結論: **2件はどちらも全ページ取得を保証できない指摘だが、具体的な原因は異なる。** 初版は応答件数で終了を判断し、修正版は `Link` を読むものの実際の GitHub の next 形式を拒否する。修正版では独立した公開 API 観測により、複数ページ取得が失敗する経路を確認した。

## 対象版と確認事項

- 初回レビュー `issue-04-review-20261119.md` SHA256 `0C31B3BD86A8DA52A2BFABF589A7AE4FB4C3D979A4A9B9EE968EF3A49A57FBE6` の B04 は `Link` 未使用と少ない件数での早期終了リスクを指摘。実 API で短いページに next が存在すること自体は未確認。
- 2回目レビュー `issue-04-review-20261119-02.md` SHA256 `49C22F29535EBFCD93FFB4477D91F2D9CBB62298990B60236DB68771A19C686B` の B01 は正規の `repositories/<id>` と cursor を拒否すると指摘。現時点の `widget/src-tauri/src/main.rs` SHA256 `60143EE5415282116641FA9FCC32281C49967586505E2D170D8818DA6825A070` は2回目依頼の固定値と一致。`has_next_page` は元 endpoint と同じ path、`state`、`per_page`、`page` の3 query のみを要求し、`fetch_pages` は取得した next を使用せず page 番号を増やす。
- 2026-11-19 00:22 UTC 頃、資格情報なしで公開 `octocat/Spoon-Knife` の `GET /repos/octocat/Spoon-Knife/issues?state=all&per_page=100&page=1` を `curl.exe` で実行し HTTP 200、next の scheme `https`、host `api.github.com`、port 443、path `/repositories/1300192/issues`、query 名 `state,per_page,page,after` を観測。本文は `NUL` に出力し、cursor 値は記録しない。別の公開 `GET /repos/octocat/Spoon-Knife` は `id=1300192`、`full_name=octocat/Spoon-Knife` を返した。両コマンド exit 0。これは当該時点の一例であり、すべての応答が同じ形式とは限らない。
- [GitHub 公式 pagination](https://docs.github.com/en/rest/using-the-rest-api/using-pagination-in-the-rest-api) は `Link` の next を使う例と `/repositories/<id>/issues` の形式を示し、endpoint によって `page`、`before`/`after`、`since` が使われると説明する。[Get a repository](https://docs.github.com/en/rest/repos/repos#get-a-repository) の応答には対象の数値 `id` と `full_name` が含まれる。next の cursor を page 番号だけで再構成できることは確認できない。

## 推奨する安全な修正案

1. 一覧開始前に、選択された接続先を固定し、同じ認証主体で `GET /repos/{owner}/{repo}` の成功応答から正の数値 repository ID と canonical `full_name` を取得・照合する。これらの呼出しも timeout・失敗・レート制限を扱う。接続先が途中で変わった結果を表示しない現在の対策を維持する。リポジトリ名変更や転送による canonical 名の不一致は推測で受け入れずエラーまたは再接続とする。
2. 初回 URL は固定 `https://api.github.com/repos/{owner}/{repo}/issues` から作り、GitHub の next URL 自体の opaque cursor を保持して使用する。送信**前**に URL を標準パーサーで検証し、scheme `https`、host 厳密に `api.github.com`、有効ポート 443、username/password なし、fragment なし、期待する完全な issues path（初回 `/repos/<検証した owner>/<repo>/issues` または `/repositories/<取得済み ID>/issues`）だけに限定する。ID や接続先が不一致の URL、host 偽装・別 endpoint・認証情報付き URL は拒否。初回と next の path の許可方式・正規化はテストし、redirect の自動追従は無効のままとする。
3. next の query は既知の設定条件 `state=all` と `per_page=100` の維持・重複や矛盾を検査し、GitHub が実際に返した `after` 等の cursor を不透明値としてそのまま受け渡す。query の許可範囲は公式と実 API に照らし、未知形式はサイレントに落とさず未完了エラーとする。パースした cursor を URL に再付与する際、二重エンコードや同名パラメータを生じさせない。完全 URL の記録・エラー表示には token、cursor、非公開データを含めない。
4. 正規化された次 URL の訪問集合で繰返しを検出し、無限ループ対策の明示的なページ上限または中断条件に達したら部分結果を成功として返さない。Link ヘッダーが壊れている、next が重複する、URL が拒否される、JSON の途中失敗、403/429、通信断も全体をエラーにする。`Link` の解析は quoted parameter と複数 header を扱える方法を採り、単純なコンマ分割・文字列連結に依存しない。
5. これは次リンクの資格情報送信先と対象 repository を制限する設計案であり、既存版に実装・検証済みではない。URL と ID の照合だけではデータが API 側で正しいこと、取得中の同時更新下でスナップショット一貫性があることまでは保証しない。Issue の変化による重複・欠落を考慮し、可能なら番号・ID による重複排除と不完全・変更中の表示方針を検討する。

## 検証案

- 合成 transport/localhost で `repositories/<固定 ID>/issues?...&after=...` を返し、**次の実要求が cursor を含むこと**と、短いページ＋next、最終ページ、ちょうど100件の最終ページ、1,000件超、Pull Request 除外、途中429/403・無効 JSON・通信断で部分成功にならないことを検証する。レスポンス本文や合成 token 自体をテストログへ表示しない。
- 別 host、userinfo、別ポート、別 repository ID、別 path、fragment、query 重複・変更、壊れた Link、同一 URL および複数 URL の循環、不正 cursor、上限到達を拒否するテストを用意。拒否されたホストに Authorization が送信されないことを mock で確認する。
- 同一の公開多ページ repository で、認証情報を使わない実 API の検証を別途実施し、対象版と結果件数・完走／エラーのみ記録する。ユーザーが指定した検証用 repository・実 PAT の利用範囲は親が確認する。実環境の高件数・変更中の一貫性・制限回復は未検証。

## 範囲・停止条件・教訓

依頼、両レビュー、対象ソース・SHA256、GitHub 公式一次資料、上記2件の公開 GET を読み取り専用で確認。製品・GitHub・資格情報・UI は変更していない。今回のコマンド・調査で新たな失敗はない。ユーザー指示どおり、独立調査後さらに同種失敗が2回発生した場合の停止判断・回数記録は親が行う。

1. 模擬サーバーのページリンクを実サービスで観測・文書化された形に照らし、リンク自体と実際の次要求を検証する。単に固定ページ順の応答を返すテストだけではカーソルや URL 正規化の不具合を検出できない。
