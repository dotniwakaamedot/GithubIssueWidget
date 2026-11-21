# 実 GitHub 検証の事前確認

## 結論

2026-11-21 14:08 JST 頃、公開 API で対象リポジトリと必要な3ラベルの存在を確認した。アプリに保存された PAT の権限、アプリの現在の接続先、実書込みの成否は未確認であり、書込み前に本人による確認が必要。以下は読取り調査であり、Issue や資格情報を変更していない。

## 確認結果と許可範囲

- 匿名 GitHub REST `GET /repos/dotniwakaamedot/GithubIssueWidget` は HTTP 200、`private=false`。
- `GET /repos/dotniwakaamedot/GithubIssueWidget/labels/status%3Atodo`、`status%3Ain-progress`、`status%3Adone` は各 HTTP 200、応答の `name` はそれぞれ `status:todo`、`status:in-progress`、`status:done` と一致。各ラベルは個別取得であり一覧のページ終端には依存しない。取得後の変更や PAT による書込み権限は証明しない。
- `docs/handoff-20261121-current.md:40` は既存承諾を公開 `dotniwakaamedot/GithubIssueWidget` の検証用 Issue 1件の作成、移動、クローズと記録。Issue #5 と #6 の開始前コメントはユーザー指定テスト用リポジトリへの限定を記録し、#6 は新規テスト Issue のみ・既存 Issue を変更しないと明記。これは引継ぎ・コメントの記録であり、元会話を本担当が直接確認したものではない。
- Issue #5、#6 本文およびそれぞれのコメントを公開 REST の番号指定 GET で取得、各 HTTP 200。コメントは各1件、各 `Link` ヘッダーなし。#6 は失敗・不明応答時に実状態再取得とエラー表示を要求する。

## 実施前・実施中の条件

1. 本人がアプリの接続先が `dotniwakaamedot/GithubIssueWidget` であると確認する。公開対象以外または接続先不明の場合は書込みを保留。既存 Issue・PR を変更しない。
2. 本人がアプリの使用する PAT に対象リポジトリへの Metadata 読取りと Issues 読書き権限があることを確認する。匿名 API の成功、コネクタ権限や一覧表示は PAT の書込み権限を証明しない。トークンや非公開情報を共有・記録しない。既存資格情報の上書きや削除は行わない。
3. 一意で秘密や個人情報を含まない題名で検証 Issue を最大1件作成する。作成応答が不明の場合、再作成せず GitHub 側で題名と時刻、一覧を照合する。判別不能なら停止する。
4. 作成した Issue のみで全6方向の移動を試し、操作ごとに GitHub の open/closed と status、および事前にある非 status ラベルを確認する。途中のエラー・応答不明・部分反映では次操作を止めて GitHub 上の実状態を再取得し、結果を記録する。復旧は実状態を確かめた後に判断し、不明な操作を自動再送しない。最後にその検証 Issue をクローズし履歴を残す。ラベルの消失、権限不足や接続先不明では進めない。

## 対象・検証・変更

- 担当 `/root/test_preflight`、依頼 `user-test-preflight-request-01.md` SHA256 `A255C9F64074E247379FCF970CE55F08F1B6AC38C96D2E8950FA9A1D4095A3F1`。作業場所 `D:/Projects/GithubIssueWidget`、branch `main`、HEAD `4961b25bec53a93439acdf5230de77f959908af0`。HEAD は未追跡・未コミット内容を識別しない。
- Python 標準ライブラリによる匿名 HTTPS 個別 GET の出力・終了コード: リポジトリと3ラベルの操作は終了0、各 HTTP 200。Issue #5/#6 本文・コメント GET は終了0、各 HTTP 200。応答本文やトークンは保存していない。端末出力は必要な項目に限定した。
- 参照 `user-test-procedure-01.md`、`docs/handoff-20261121-current.md`。対象パスの実在、結果確定名が事前に未存在であることを確認。出力は同一ディレクトリの `.tmp` を確定名に移して公開し、UTF-8 日本語を読戻す。
- 変更は未追跡の本結果 `user-test-preflight-01.md` のみ。自己参照を避け、確定後の SHA256 は管理側が取得する。削除対象なし。アプリ・既存設定・資格情報・Git・GitHub は変更していない。認識した失敗なし。
- 未確認: 本人 PAT・実際の権限、現在のアプリ接続先、操作時点のラベル、実書込み・移動・復旧。元会話の許可文は本担当未取得。後続の実施・判定は親担当が行う。
