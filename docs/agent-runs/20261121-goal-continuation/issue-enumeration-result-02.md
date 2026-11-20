# Issue 一覧の再確認

2026-11-21、`issue-enumeration.py` SHA256 `9DF91F06B83198B718F3A8CE78DAE08C24EB2E7500485B439BB573ABBBA509E4` と `test_issue_enumeration.py` SHA256 `FE8739B18C66B1D23D3E0A1A00EC0A2909AE17A87E3E7733EC25228FFD66940C`。`test-issue-enumeration-03.log` は合成８件 OK、親が終了コード0を確認した。短いページに継続がある場合、異常な継続、途中失敗と偽秘密の表示抑止を含む。

同版で GitHub の Issue・PR 一覧を読取り、親が終了コード0・１ページ・10件・固定マーカー０件を確認。プログラムは `Link` の next を検査するが実ヘッダー自体は保存していない。応答本文や認証情報は保存していない。並行更新中の完全なスナップショットは保証しない。実書込みの許可や直前 preflight の代用ではなく、対象版の独立再レビューは未完了。前版の結果は `issue-enumeration-result.md` を参照。
