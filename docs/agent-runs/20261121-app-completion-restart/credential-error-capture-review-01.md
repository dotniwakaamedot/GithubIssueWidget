# 合成資格情報テスト診断版の独立レビュー 01

- 担当 `/root/credential_error_capture_review` は作者 `/root/credential_error_capture` と別。作業場所 `D:/Projects/GithubIssueWidget`。親からの依頼対象 `credential-error-capture-result-01.md` の期待・実測 SHA256 は `17D36DB0BAE8F00F9C115B38255FE12C0B3B2FD316A9C9DD1298BF272E2B27BA` で一致した。会話による依頼と対象版を確認した。
- **判定: 対象版の診断機能・安全境界に blocking なし。** 承認は本レビューの範囲と当該版に限る。実 OS 試験の自動許可、実アプリ・PAT・WebView の動作、Issue #3 や製品完成の承認ではない。親は実行環境と既存の失敗・残余リスクを判断する。

## 完了条件と根拠

1. **一次要求と対象実体:** GitHub コネクタで Issue #3 を番号指定して取得し、実アプリの Windows Credential Manager 登録・変更・削除・再起動後取得、保存済み PAT の非返却、保存失敗時の保護・通知を確認した。今回の専用合成 service は実アプリでの条件を代用しない。結果に記載された `src/main.rs`、`supervisor.py`、`recovery.py`、`diagnostic_summary.py`、`test_diagnostic_summary.py`、3ログ、2終了記録および live EXE の実測 SHA256 はすべて結果と一致。EXE は `credential-error-capture-01/target/debug/giw-credential-synthetic-check.exe`、SHA256 `9AEC7CDC8FCC5AB3109153C930CE45900CA5290B284D1F03B052C33B72CF845F`。
2. **旧承認版との差分:** `credential-live-ready-02/` と比較し、`supervisor.py` の差分は期待 EXE SHA の更新のみ。`recovery.py`、`Cargo.toml`、`Cargo.lock`、`RECOVERY.md`、`test_ready.py`、`mock_parent_exit.py` はそれぞれ旧版と SHA 一致。Rust 差分は keyring エラー分類・診断出力と対象単体試験に限定された。既存の新規 one-shot 予約、子の識別記録・読戻し後の grant、direct child wait/timeout、不明状態の記録、指定 run のみの inspect・制限された cleanup のコードは維持される。旧版承認を新版の実機成功に流用しない。
3. **対象と復旧:** `reserve` は UUID service、固定 `synthetic` account、子 PID と EXE を新規 journal へ書き flush・sync・読戻し後に grant を待つ。`execute` は NoEntry を確認し合成値 A/B のみを使い、B の一致後に削除し NoEntry を確認する。`recovery.py` の marker・start・child・outcome・journal・EXE SHA の照合と、単一 target の値非表示分類・認識済み stage と完全一致を条件とした cleanup は旧版と同じ。NoEntry→write・値確認→delete は原子的ではなく、同名第三者との競合は排除できない。grant 後の親停止や OS 障害で outcome が欠ければ安全な復旧を自動保証しない。
4. **秘密非出力と診断:** `classify` は keyring の固定カテゴリと Windows 型に downcast できる場合の u32 コードのみ抽出し、provider のメッセージや値を format しない。診断は検証済み run ID の専用領域直下に create-new・flush・sync・読戻しする。`diagnostic_summary.py` は `read_identity` 後に ASCII・256 bytes 上限、run ID・固定 operation/kind と u32 を検査し、target や値を出さない。例外本文も CLI は出さない。診断の欠落・不完全、journal や outcome がない場合は判別不能であり成功や原因確定ではない。実際のプロセス経路・OS 応答で診断が保存されることは今回未検証。
5. **試験とログ:** `diagnostic-test-02.log` に対象 Rust 1件成功・9件除外、`live-build-02.log` に live build 完了、各保存 exit に `0`、`summary-test-02.log` に Python 5件成功を確認。ログ・exit SHA は作者結果に一致。Python の `read_identity` はモックされ、実 Credential API や実プロセスによる診断接続を検証しない。終了値は作者の保存記録で、本担当が元プロセスを監督したものではない。テスト・ビルドの再実行はしていない。
6. **旧 run と成果物:** `credential-live-residue-review-01.md` は旧 run `dcdc5dfdf4524ffa8d9139ee5a781ae4` の単一合成 target について `absent` を示す inspect outcome を照合・承認した。ただし観測時点に限り、元の書込み成否や終了1の原因は不明。今回の専用領域は未追跡の実体として確認し、`live-once.json`、journal、diagnostic はレビュー時点で見当たらなかった。結果に削除対象の指定はなく本レビューも削除していない。workspace 全体の変更・削除有無は確認していない。

## 未確認と実施上の条件

- 新版 EXE の実 OS 起動、Win32 エラーの実際の捕捉・summary 接続、異常終了後の復旧、実アプリ credential 登録・更新・削除、実 WebView IPC は未実施。診断ファイルの作成失敗・強制終了・対象外エラー等では分類が得られない場合がある。
- 別 run を行う場合、親が旧 run の終了と対象不在、専用版 SHA、識別・子監督、外部書込みの安全条件、非原子的競合と異常時停止・復旧方法を改めて判断する。分類や保存終了コードを元の直接原因の復元として扱わない。

## 実施と失敗

`Get-FileHash`、限定 `git status`、Python 差分照合、対象コード・ログの読取り、Issue コネクタ取得を実施した。Git は個人 ignore 設定の読取拒否を警告したが限定状態を返した。認識した失敗は２種類各１回: EXE パスを推測して存在確認した結果 missing（正しいパスは `supervisor.py` から確認し SHA 一致）、Issue 取得の repository 引数名を誤り `InvalidActionArgumentsError`（定義確認後、正しい引数で取得成功）。いずれも元操作の欠落を成功扱いしていない。共有失敗記録・教訓への反映は管理へ依頼する。同種２回は認識していない。実 credential・GitHub データ・製品・他者の成果に変更はない。
