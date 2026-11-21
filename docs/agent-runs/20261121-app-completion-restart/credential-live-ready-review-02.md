# 合成資格情報の実 OS 試験候補レビュー 02 — 対象版を承認

- 担当 `/root/credential_live_ready_review` は作者 `/root/credential_live_execution_prep` と別。作業場所 `D:/Projects/GithubIssueWidget`。親が会話で指定した `credential-live-ready-result-03.md` の期待・実測 SHA256 は `9DA26D79CE8B5E5E111F97EA001C39B050BD1188DC33D266B7FA13EBB45FF964` で一致。前回 `credential-live-ready-review-01.md` の対象とは別版であり、新版の実体・コード・証拠を照合した。
- **blocking なし。** 承認は記載された専用候補版・確認範囲に限る。実行を自動的に許可するものではなく、親が実行環境と残余リスクを判断する。実 Credential Manager、実アプリ、実 PAT、WebView、Issue #3 やアプリ完成の承認ではない。

## 前回 blocking と今回の確認

1. **子の登録前に処理が進む問題:** 新版 `supervisor.py` は EXE SHA 照合後に one-shot marker と start 記録を新規・同期・読戻し、直接子を stdin pipe 付きで起動する。`child.json` に PID・パス・SHA・journal を新規・同期・読戻しした後で `GO:<run_id>` と EOF を送る。`main.rs` の子は識別情報と `awaiting_grant` を journal に同期し、grant 全文と EOF を15秒以内に受けない限り entry の構築・Credential API 前で停止する。登録失敗・送信例外は kill/wait を試し、送信済みの可能性を `grant_delivery_unknown_*` と区別する。終了不明を終了済みとして扱わない。
2. **対象限定の診断・復旧:** 新規 `recovery.py` と `RECOVERY.md` は marker・start・child・outcome・journal・バイナリ SHA・PID・専用 service UUID4・account を照合し、直接子の wait 済み結果がない場合は停止する。inspect は単一 target を値を出さず `absent`／合成 A・B／`other`／`error` に分類する。cleanup は別操作で、同じ journal SHA、書込みが正常復帰した次の stage、前回診断・直前再読取での合成値一致を要求し、削除後 NoEntry を確認する。未知状態や外部値は削除せず、grant 配送不明は inspect のみで cleanup しない。認識できない stage や不確実な削除の残留は停止・保留され、万能な復旧ではない。
3. **不明後の再実行:** `live-once.json` は `create_new`・flush・fsync・読戻しで最初の子生成より前に予約され、同一領域では成功・失敗・不明を問わず削除せず後続起動を拒否する。実際の実行後に別コピーで再開する権限や安全性を本レビューは与えない。marker が途中まで書かれた場合もファイルが残れば再実行を拒否し、状態は調査対象となる。

## 対象・証拠・制約

- 結果表の `Cargo.toml`、`Cargo.lock`、`src/main.rs`、`supervisor.py`、`recovery.py`、`test_ready.py`、`mock_parent_exit.py`、`RECOVERY.md`、live と mock の EXE、Rust ２ログ、`ready-tests-oneshot-02.log` と exit を実体から SHA256 再計算し記載値と一致。live EXE は `211CAD118123794B3C15C27E14F88BEC60B4AD0906ED9145AEF71939A2706910`。限定 `git status` は結果と専用領域を未追跡と示し、作者結果・本レビューに削除対象指定はない。全 workspace の削除有無を網羅した確認ではない。
- 保存ログは live feature の fake Store/Journal ９件成功、live build 完了、Python の最終版20件成功を示し、保存 exit は0。対象コードは別 process の mock 子を使った EOF・timeout・親終了前の未 grant・登録失敗・正常 grant、fake backend の値不一致・不明状態・対象限定 cleanup、one-shot marker 等を検査する。終了値は作者が保存したもので、当担当は元プロセスを監督・テストを再実行していない。mock 子で marker を patch する統合試験と marker 単体試験は区別する。
- `keyring 3.6.3/src/windows.rs` の現物で `set_password` が UTF-16LE の null 終端なし blob、既定 target が `user.service` であることを確認し、`recovery.py` の `synthetic.<service>` と合成 A/B の分類方式に静的に一致する。ctypes と Windows API の実行・環境依存の動作は未検証。
- NoEntry→write と値比較→delete は原子的ではない。UUID4 付き合成 service は偶発的な名前衝突を実用上大きく減らすが、同名第三者による意図的・同時操作は排除できない。記録は同じ workspace を改変できる主体に対するセキュリティ境界でもない。親は環境・対象・未承認の実行・競合リスクを判断し、不明時は操作しない。これを理由に一律に別 Windows ユーザーや衝突ゼロを要求するものではない。
- 親が grant 後に停止すると子が進行し得る。outcome または停止証拠が欠落した場合、復旧ツールは対象を触らず調査待ちになる。タイムアウトで kill/wait が失敗した場合も状態不明で再送・cleanup しない。OS や監督者の突然停止からの自動復旧は保証しない。診断・cleanup の前にも対象版・停止の根拠と実施範囲を別途確認する。

## 実施・失敗

適用指示、前回レビュー、Issue #3 の本文、対象コード・模擬試験・ログ・終了記録を読み、限定 `git status` と `Get-FileHash` を実行した。Git は個人 ignore 設定の読取拒否警告を出したが限定状態を返した。実 credential・アプリ・GitHub・対象成果は変更しておらず、新たに認識した失敗はない。Issue #3 の実アプリによる初回・変更・削除・復旧と実 WebView IPC は別途未検証。
