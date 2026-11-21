# Windows テスト監督 backend の独立調査

担当 `/root/gui_failure_investigation`。依頼 `sync-supervisor-backend-investigation-request-01.md` SHA256 `4FAF1ABE9D75D0CFAAC6D5F8AE8321E9921E4C019B8A2F3990AFAA5AF9AFB27C`。作業場所 `D:\Projects\GithubIssueWidget`、branch `main`、HEAD `5e74e1642fd18fe630dce0e684afd853721276ae`。作者 `/root/process_safety` の `supervisor-backend-answer-01.md` で書込み停止を確認し、対象6ファイルの SHA256 を照合した。

## 結論

固定版は Windows API の未接続プリミティブとポリシーであり、実行可能な監督器ではない。Job を生成時に指定し停止状態で識別してから再開する既存案を基礎に、予約の永続化、標準入出力、ハンドルの寿命、期限付き終了確認と外側の停止試験を具体的に実装するのが現実的。下記は設計提案で、API 動作・隔離・プロセス終了を実証するものではない。B1/B2 の blocking は残る。

## 確認された不足

- `win_job.py` は匿名 Job の作成、kill-on-close 設定、active count、終了と close の部品のみ。`win_launch.py` は生成時 Job 属性と `CREATE_SUSPENDED` の部品を持つが、予約、実行ファイルの実体・作成時刻・所属を合わせた照合、待機、ログ誘導、子とスレッドのハンドル寿命の管理を持たない。構造体・API・対象 OS の挙動も未検証。
- `supervisor.py` は `Backend` 契約であり実装はない。`UNKNOWN` での後続禁止は永続ストア側の実装と運用に依存する。`finally` の `close` 失敗を抑制するため、それ自体は確実な Job 終了の証拠ではない。close 成功も外部に複製・継承されたハンドルがないことを別途示さねば kill-on-close の発火証明ではない。
- 生成 primitive は `bInheritHandles=False`、標準ハンドル未設定。単にパイプを追加しても出力捕捉できるとは限らず、限定されたハンドル継承方式を別途設計する必要がある。実機失敗ではなく静的な不足の指摘。
- `stop_scenarios.md` は停止試験の案であり外側監督者の実装はない。環境だけで Windows 資格情報ストアや通信を隔離できず、依存の安全性は別担当の B2 の対象。

## 実装境界と順序

1. 独立した専用領域を排他作成し、リンク・実体・ACL・共有資源・所有権を確認。固定 Node 実体・スクリプト・依存・設定・作業ディレクトリのハッシュ、引数・環境・期限・ログ先を予約に記録して読戻す。OS の Credential Manager と通信を使わない/遮断する根拠は環境変数の除去とは別に確認し、実効性不明なら Vitest を実行しない。旧対象とはパス・ハンドル・識別子を共有しないが、未知の対象による無条件非干渉は主張しない。
2. 永続予約の `GENERATING` を生成 API より前に確定し、失敗なら呼ばない。Job を非継承ハンドルで作成、属性リストの寿命を保持し `PROC_THREAD_ATTRIBUTE_JOB_LIST` を `STARTUPINFOEXW` に設定して対象 EXE を明示した `CreateProcessW` を停止状態で呼ぶ。ABI、戻り値、対応 OS、既存親 Job の制約、環境ブロックと不適切な引数・パスの拒否を確認。サポートされない場合は失敗扱いにして代替の未所属起動をしない。API 中断や戻り不明は予約を保持する。
3. 戻った直接子の保持ハンドルで作成時刻・実行パス・期待実体と Job 所属を照合、成功後に永続記録してからスレッドを再開。直接子のハンドル、Job、stdout/stderr 等の所有者・継承・close の順序を明示する。各 API の失敗と戻り値の不明を区別し、他人の PID や同名プロセスを終了対象にしない。
4. 標準出力と標準エラーは同時に排出し、上限到達後もパイプを閉塞させない動作（固定分類・破棄または明示停止）を決める。STARTUPINFO の標準ハンドル、継承可能なハンドルの明示リストと Job 非継承を整合させ、子孫がパイプを保持するケースを検証する。ログは専用領域・容量・ACL・保持期間・欠落を固定し、例外本文・秘密を出さない。上限超過は完全ログ・テスト成功と扱わない。
5. 単調時計の期限、直接子の終了コード、保持する Job の active count、排出器 EOF/終了、識別を個別に観測。期限超過や失敗時は所有 Job のみ終了要求し、戻り値・期限付き待機・直接子と Job の状態を別々に確認。終了成功でも timeout はテスト成功ではなく、Job count 0 は breakaway や未追跡子孫の不在を証明しない。不明時は予約と証跡を保持し、同じ run の再利用を禁止する。
6. 監督者の途中停止は予約から復旧し、Job が最後のハンドル close で終了したという期待を実測と区別する。生成中の停止やハンドル消失は unknown のままとし、識別・終了の根拠がなければ別のプロセスを停止しない。外側の試験監督者自体の停止・観測不能も失敗境界に含める。電源断・OS 障害・breakaway の無条件終了を主張しない。

## 合成試験の実行前ゲート

まずテスト仕様と実体・ハッシュ・識別子・記録先・期待イベントを固定し、静的レビューと実行者の所有権確認を行う。専用の秘密のない子で通常・timeout・生成 API 失敗・Job 未対応・所属不明・再開失敗・ログ超過/子孫保持・照会/停止/close 失敗を個別注入する。生成前・生成中・所属確認後・再開後・停止中に内側監督を停止し、外側が境界と直接子・追跡 Job・ログ・予約の結果を別々に確認する。生成中の停止に到達した証拠がなければそのケースを合格としない。外側自身の停止、旧対象との資源関係、実行環境の API 制限も明示する。実際の OS 障害全般や未追跡子孫を網羅した証明ではない。合成試験を別担当がレビューした後、B2 と対象版の確認が揃って初めて限定 Vitest の実行可否を判断する。

## 検証と対象版

`Get-Content -Raw -Encoding UTF8` による依頼・先行案・コード・回答の読取りと `Get-FileHash -Algorithm SHA256` は終了0。ハッシュは作者回答の固定版と一致した。実行・構文試験・ネットワーク・権限変更・GUI・合成子・製品変更は未実施。新規操作失敗なし。削除対象なし。新規質問 `supervisor-investigation-question-01.md` SHA256 `9193C75B05DAD117CEDA026D9C200249A76CF2C70B60C847C9848B156AB877D0`、`supervisor-backend-question-01.md` SHA256 `7EEB3E984AC307DCA703A2F26D130D80B8AA61C2133BAFCACB66A3C9702DD947`、本結果のみ。結果提出後の独立レビューは未実施。

- `supervisor-backend-answer-01.md`: `B3C176F507C2EE0458F1E5D6D896FACA4C36B4C7CD8FC5FDDFB7B0D85CA1DCFB`
- `test-supervisor-01/design.md`: `5A9D8A57B51B737B41DA670AFE501EE1903AA65C1D6BEB36BECA29BD5B47C2D8`
- `test-supervisor-01/stop_scenarios.md`: `FEE6889A9C35A259B88B8E19171CB50655A81F6B252844BCB33933755A35011E`
- `test-supervisor-01/supervisor.py`: `A26966623FE698DE2D8F09258F38C23B9FE7E0B6FEA6B6ADCE201A1621256568`
- `test-supervisor-01/test_policy.py`: `3EAC719809E2E5277A74468904BE8994CB825FB7C3B63BB2DF864B27DF4B2D48`
- `test-supervisor-01/win_job.py`: `9A98FF19388F66AF50EB05EE7C3F4000D02DB02FA5C28901D0F4336F1D1CC025`
- `test-supervisor-01/win_launch.py`: `F1D8EF3E246CE6D778AC69BFFC256AC7F44C5FA5122FE95334347138962E040B`
- 先行 `../20261121-issue-completion/gui-supervisor-plan-02.md`: `12D9385A46F173886B8F3EC3B50E8619E9FEECF57BF03F122E4F46B2EE504097`。

相対パスは共有 run 基準。HEAD は成果を識別しない。
