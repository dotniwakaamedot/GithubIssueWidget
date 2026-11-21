# 監督器の出力回収部品追加（未完成）

担当 `/root/gui_failure_investigation`。依頼 `sync-supervisor-work-request-04.md` SHA256 `CB2165E252A26B2A4F9C42E4889D7B0D0B407B655424E9928CB41A00778DEE1F`、作業場所 `D:\Projects\GithubIssueWidget\docs\agent-runs\20261121-orchestration-continuation\test-supervisor-01`、branch `main`、HEAD `5e74e1642fd18fe630dce0e684afd853721276ae`。旧8ファイルの開始時 SHA は `test-supervisor-result-02.md` と一致し、前担当は終了済み。旧ファイルは変更していない。

## 結果

**依頼された一体型 backend と実停止試験は未完成**。出力回収の未接続部品とプロセスを生成しない試験案のみ追加した。従って本成果は実行可能な監督器でも、終了や秘密保護が成立する証拠でもない。引き続き実行に使用しない。

- `output_capture.py` はストリームの読取りを専用スレッドで継続し、保持量を制限、上限後も EOF まで破棄する。EOF、上限超過、読取り失敗、観測期限を区別し、任意の生出力を返さない。未知の出力や秘密を保存しないための部品だが、認証や OS 標準ハンドルを隔離しない。スレッドが期限までに終わらない場合、終了とみなさない。
- `test_output_capture.py` はメモリの模擬ストリームで EOF、上限超過、例外文の非表示、待機期限を検査する案。依頼の実行前ゲートによりテスト自体は未実行。同期・デッドロック・プロセスからの排出を証明しない。

## 残作業と理由

予約・Job・生成・パイプを繋ぐ backend、制限されたハンドル継承・stdin 設定、保持ハンドルの実体ハッシュ・所属確認、停止時の復旧、実行証拠の永続化、外側監督者と OS 障害注入は未実装。とりわけ `win_launch.py` は標準ハンドルなし・継承無効で、上記出力回収に接続できない。パイプの継承範囲、作成に成功した直後の例外、所有 Job の close と子孫・捕捉スレッドの終了を安全に組み合わせる実装・独立検証の根拠がまだ揃っていないため、完成や安全性を推測して接続していない。B2 の資格情報と通信隔離も別途未解決。

次担当は新たな依頼で `PROC_THREAD_ATTRIBUTE_HANDLE_LIST` と Job 属性の同時設定・所有ハンドルの全失敗経路、予約を含む状態遷移の実装を完了し、対象版を独立レビューする。専用合成子による通常・時間切れ・生成/所属/ログ/終了失敗・監督者停止を実行するには、具体的な監督・復旧計画と開始条件の承認が別途必要。旧不明プロセスや既存 fixture を推測で操作しない。ユーザー指定の同種失敗を踏まえ、過去の未完成と今回の未完成の経緯は管理が別担当による分析・再割当てを判断する。

## 変更と検証

- `test-supervisor-01/output_capture.py`: SHA256 `07A3DD16A4E6FFB8E98AF3E76D50CF97808657D7985E347862E8418B98278384`。
- `test-supervisor-01/test_output_capture.py`: SHA256 `BCF95796A4CB61D5527FE81AD9633D4AC85C09A2F735D4469B718776E3A9C7D1`。
- 確定依頼前の質問 `supervisor-implementation-question-01.md`: SHA256 `D6FE4DA3D7C27CC3D6E74C67A70B7BF071A2AA4E1E31CADB123A321C85D27065`。
- 削除対象なし。既存製品・旧 fixture・認証・外部サービスを変更していない。
- 作業ディレクトリ `D:\Projects\GithubIssueWidget` で `python -c "import ast,pathlib; files=[pathlib.Path('docs/agent-runs/20261121-orchestration-continuation/test-supervisor-01/output_capture.py'),pathlib.Path('docs/agent-runs/20261121-orchestration-continuation/test-supervisor-01/test_output_capture.py')]; [ast.parse(p.read_text(encoding='utf-8-sig'), filename=str(p)) for p in files]; print('syntax_ok',len(files))"` は終了0、`syntax_ok 2`。構文のみで正しさや API の適合性は未確認。
- テスト、合成子、Vitest、GUI、ネットワーク、資格情報操作は未実行。新たなコマンド失敗は認識していない。成果提出後は対象の書込みを停止する。
