# 合成資格情報の実 OS 試験候補レビュー 01 — 要修正

- 担当 `/root/credential_live_ready_review` は作者 `/root/credential_live_execution_prep` と別。作業場所 `D:/Projects/GithubIssueWidget`。対象 `credential-live-ready-result-01.md` の依頼値・実測 SHA256 は `6BC470299A0E707F045A8800067DDD13473E1FA9BAF07FDFC086C3EFEA1DE9D7` で一致した。依頼は親からの会話で受領した。
- **判定: 模擬試験の報告範囲は確認できたが、実 Credential Manager 書込みへの移行は要修正。** 実 OS の実行、Issue #3 全体またはアプリ完成を承認しない。以下の blocking は、既存の実資格情報に触れずに試験を開始・停止・復旧するための具体的な不足であり、無条件の OS 障害保証を要求するものではない。

## 条件と独立に確認した証拠

1. **一次要求:** GitHub コネクタで Issue #3 を番号指定取得した。実アプリによる Credential Manager への初回保存・変更・削除・再起動後取得、保存済み PAT のフロントエンドへの非返却、保存途中の障害時保護を要求する。合成サービスを直接呼ぶ本候補の成功は実アプリ・PAT・WebView の証明ではない。
2. **対象版・実体:** 結果表の `src/main.rs` `06EDCEB1D72A3EAE5A09FB7462B39A37E9BFD6B457AF9D5923438F268C0A847B`、`Cargo.toml` `38A06A4CCBAE1312682033DE8741DBFBB65C9BFAE40129F85B6C80A546583DA1`、`Cargo.lock` `79B1644981C6B122726DDD3AC55E54616C815F9E56AF52ADE760C3B402D8EAA5`、`supervisor.py` `8063631DEDC6CFA445CA22072A583B46F861EAD66A8C49EEA125A5E1797CFB76`、`test_supervisor.py` `125F2A4BD05DF581FAAA0878B89DEE8E6978A0BF8524C405C152F9B42D2F10DF`、`RECOVERY.md` `F85717FE9CCDA29A41E38B3D931163B74D4F588260B4E8FF1573EC4B9A3C3EED`、EXE `EFA493B07535142A3D9B98469795B4DA88E6979278BD2EBC1087E1E3AA8E649E`、３ログの各 SHA は実体の再計算と一致。３ exit ファイルも `0` と記録された SHA に一致。限定 `git status` は結果と専用領域を未追跡として示す。作者結果に削除対象はなく、本レビューにも削除対象はない。workspace 全体の削除有無は検査していない。
3. **識別・対象:** Rust は UUID 付きの専用 service と固定 account、PID・実行ファイルを新規 journal に書き、flush・sync・全文読戻し後に keyring の entry を生成する。ジャーナルは専用領域直下に限定し、合成 A/B のみを書き、削除前に B と照合する。実アプリ service・実 PAT を意図的に指定する経路は確認範囲ではない。UUID は現実的な衝突確率低減策だが、NoEntry 後の write や値照合後の delete は原子的ではなく、同一 target を他者が意図的に同時操作した場合の保証ではない。この理論的競合だけを理由に別ユーザーやゼロリスクを要求するものではなく、実行時の許容リスクの判断は管理に残す。
4. **模擬試験とビルド:** `rust-test.log` は fake Store/Journal の9件成功、`supervisor-test.log` は fake process と一時ファイルを用いた6件成功、`rust-build.log` は live feature ビルド完了を示す。保存された exit 値は全て0で作者記録と一致するが、本担当は元プロセスを監督していない。PowerShell のコンパイル進捗に対する `NativeCommandError` 表示は結果に記載され、ログ末尾に Finished がある。実 OS・実 Credential Manager・実アプリは本レビューでも操作していない。

## 実 OS 書込み前の blocking

1. **子の登録完了より先に credential 操作が始まり得る。** `supervisor.py` は `Popen` 後に `durable_new(<run_id>.child.json, ...)` を呼ぶ。Rust 子は登録完了の合図を待たず journal を作成後、直ちに `execute` で実 keyring へ進む。`child.json` の作成・同期が失敗した場合、監督側は kill/wait を試みるが失敗を抑制して例外終了し、終了不明を outcome に記録しない。影響: 操作が始まった後に子の登録や停止が不明となり、監督された実行と残留対象を安全に関連付けられない場合がある。`docs/instructions/work.md` は外部書込みを扱う監督プロセスの子生成・識別・登録・再開・送信を順序付け、不明な場合は実書込みを無効にする。**修正案:** 子に開始待ちの明示的ハンドシェイクを設け、監督側が子の識別を耐久記録・読戻し後にのみ続行を許可する。登録失敗・監督者停止・期限切れ・子の終了不明は実書込み前に停止、または状態不明として安全に記録できる構造にする。実子を使った模擬障害試験と独立再レビューで順序を確認する。
2. **失敗後に安全な対象限定確認を実行できる手段が未準備。** `execute` は書込み途中や削除前で停止した場合に A/B を残し得る。`RECOVERY.md` は終了確認後に単一 target を読み、NoEntry／合成 A/B／他値／エラーを区別してから必要なら対象限定で cleanup するよう求めるが、その診断・復旧 utility は未実装・未承認と明記している。影響: 実試験が途中で止まると、保存した識別子は分かっても値と所有状態を安全に確認・回復する実行可能な手順を現在の成果だけでは示せない。**修正案:** 実 credential 操作を無条件に有効化せず、journal の完全一致 target のみを対象に値を出力せず分類する最小診断と、プロセス停止確認・対象一致・直前の合成値一致を必須とする別の明示的復旧手順を用意し、模擬障害と独立レビューを先に行う。NoEntry／他値／終了不明では書込み・削除を止め、read/delete 間の残余競合を明記する。

## 未確認・実施記録

- Python 6試験は `wait_once`、`durable_new`、不一致バイナリ拒否等を対象とし、実 Popen の起動成功後に child 記録が失敗するケースや Rust 子の実 Credential Manager 接続を実行していない。異常停止後の対象限定診断、実 UI・実 PAT・実 WebView、別 PC は未検証。
- `Get-FileHash` で対象実体・終了記録、`Get-Content` でコード・ログを確認し終了0。Git の個人 ignore 設定は読取拒否の警告が出たが限定 status を返した。Issue #3 番号指定のコネクタ取得は成功。実 OS 資格情報・製品・他者の成果を変更せず、新たに認識した失敗はない。
