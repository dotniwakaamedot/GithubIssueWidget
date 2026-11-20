# Issue 一覧の読み取り確認

2026-11-21、`issue-enumeration.py` SHA256 `CBF821F87E477F255A9EF58D9DCADBEE9239A417D2D02C57F83709DF58BD27AC` と `test_issue_enumeration.py` SHA256 `71C615E6B5ED14F74E3C419DFBF050681CC80FC5D40A1FC9819E819B6D9D311F`。合成試験は４件成功、親プロセス終了コード0（`test-issue-enumeration-02.log`）。空・複数・100件での次ページ・101件・不正データを確認した。

上記の `issue-enumeration.py` を一度実行し、終了コード0、１ページ・Issue と PR 合計10件、固定マーカー一致０件を得た。実際の応答本文や認証情報は出力・保存していない。公開リポジトリの読み取り時点での結果であり、後続の変更がない保証、GitHub のページングが同時変更下でも完全なスナップショットである保証、実行前レビューや実 GitHub 書込みを認める証拠ではない。POST は行っていない。
