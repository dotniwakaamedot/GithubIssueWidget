# 監督器の部分実装結果（未完成）

担当 `/root/project_survey`。`sync-supervisor-work-request-03.md` の移管を確認し、旧6ファイルの SHA は `supervisor-backend-answer-01.md` と一致した。**一体型 Windows backend と合成障害試験は未完成。実行・Vitest には使用できない。**

## 作業内容

- `reservation.py` を追加。排他的な新規予約ディレクトリと append・flush・fsync、保守的な状態遷移を構想した非接続部品。親パスの安全性・ACL、生成後の権限、ディレクトリ自体のクラッシュ永続性、部分書込みの読戻し・復旧、複数プロセス・OS 障害は未検証。これ単独では要求された耐障害予約の証明ではない。
- `test_reservation.py` は予約再利用の拒否と fsync 失敗のテスト案。`test_policy.py` は正常時の close と COMPLETE の順序の誤った期待を訂正し、再開・終了要求・収集・ログ・close の失敗を fake で表す案を追加。未実行でありテストが実際に成立することは未確認。
- `win_launch.py` に保持ハンドルからの実行パス・作成時刻照会と有期限待機・終了コード取得部品を追加。Windows API の型・振る舞い・失敗経路は未検証で backend に未接続。パスとハッシュの照合まで実装していない。

## 固定版 SHA256

| ファイル | SHA256 |
| --- | --- |
| `reservation.py` | `8E90CBE9D01B10C64C83EA5FECAC8C31E8B6CE2B39506ED1F2A6D997FD62D552` |
| `test_reservation.py` | `1A09358E5753E2E154FEDAD3391477CF6167AF7AE84C64EC3880CE21F0949205` |
| `test_policy.py` | `FDA41F1322422C8B94ADDFE8862D4020087311450F745DBF2F16A4F491A9C10E` |
| `win_launch.py` | `51B3611360463039D11C1979FDBAF7BD13DD262BE4026D790655DBFE67ED3791` |
| `supervisor.py` | `A26966623FE698DE2D8F09258F38C23B9FE7E0B6FEA6B6ADCE201A1621256568` |
| `win_job.py` | `9A98FF19388F66AF50EB05EE7C3F4000D02DB02FA5C28901D0F4336F1D1CC025` |
| `design.md` | `5A9D8A57B51B737B41DA670AFE501EE1903AA65C1D6BEB36BECA29BD5B47C2D8` |
| `stop_scenarios.md` | `FEE6889A9C35A259B88B8E19171CB50655A81F6B252844BCB33933755A35011E` |

Python の AST 構文検査は6ファイルとも終了0。型・実行試験、合成子、Vitest、実 GUI・GitHub・資格情報操作は未実行。削除対象なし。製品・既存依存・旧 worktree は変更していない。

## 残課題

予約・Job・生成を統合する backend、明示ハンドル継承と上限付き同時ログ排出、生成直後の保持ハンドルの実体ハッシュ・所属検証、期限・直接子と Job/ログ終了の確認と異常時復旧、合成子と外側監督者による実停止・故障注入は未実装。通信・資格情報の実効隔離も未確認。実行前に構造体・API・予約の耐障害性と test 設計を独立レビューし、承認された専用合成試験の方式と安全条件を確定する。権限と時刻・断電を含む不確定条件を成功とみなさない。今回新規の実行失敗はない。
