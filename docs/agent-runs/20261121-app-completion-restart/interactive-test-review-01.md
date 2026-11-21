# 対話セッション用合成資格情報テストの独立レビュー 01 — 要修正

- 担当 `/root/interactive_test_review` は作者 `/root/interactive_test_prepare` と別担当。作業場所 `D:/Projects/GithubIssueWidget`。
- 対象 `interactive-test-prepare-01.md` の依頼値・実測 SHA256 は `AA6256419EE9E91A71BDB1D8C7AC40B5CD50CC00898C5BE081E7E04A924B3285`。親の指定で訂正記録 `interactive-test-prepare-correction-01.md` の期待・実測 `93944337911D56C4226792A284A412AB617B867375EAABF754DB968BAB00C789` を合わせて確認した。
- **判定: 実行手順に blocking があるため、本人への実行案内は保留。** 以下は対象版の静的・記録照合であり、実 Credential Manager、Issue #3 の完了、製品完成を承認しない。

## Blocking

1. **作業ディレクトリ変更の失敗後に相対パスの別スクリプトを実行し得る。** `widget/credential-check-20261121/実行手順.md` は `Set-Location -LiteralPath 'D:\Projects\GithubIssueWidget\widget\credential-check-20261121'` の次の行で Python に `.\supervisor.py` を渡す。Windows PowerShell の `Set-Location` の非終端エラーが生じた場合、後続行が続く可能性があり、元の作業ディレクトリに同名スクリプトがあると意図しないファイルを実行し得る。影響は対象版の特定・安全境界が崩れること。**修正案:** 絶対パスでレビュー対象 `supervisor.py` を指定し、対象ディレクトリ・スクリプト・EXE の存在と対象ハッシュが不一致なら明示的に停止する手順とし、Python が利用不可の場合も本処理へ進まないことを示す。修正した手順の版を独立に再確認する。

## 条件ごとの照合

1. **対象版と訂正:** `Cargo.toml`、`Cargo.lock`、`src/main.rs`、`supervisor.py`、`recovery.py`、`diagnostic_summary.py`、EXE、`実行手順.md` の実体 SHA256 は元結果の表に一致。`RECOVERY.md` は元結果の値と不一致だが、実測 `F2F2D33AA2B3BC4CDC057384E38F2FBCF1BDAE10199D70946A5868C9A5B47E45` は訂正記録および旧承認版の実体と一致した。元結果の誤記を訂正なしで承認しない。
2. **旧版との差分と manifest 固定先:** 旧 `credential-error-capture-01` と新規領域の Cargo manifest・lock・Rust・recovery・summary・RECOVERY の SHA は各々一致。`supervisor.py` の差分は期待 EXE SHA 更新１箇所。新 EXE 内に `D:\Projects\GithubIssueWidget\widget\credential-check-20261121` の ASCII バイト列14件、ビルドログに同じ manifest ディレクトリがある。Rust はその `CARGO_MANIFEST_DIR` と journal 親を canonicalize して一致させる。移動・コピー先での実行には使えない。文字列存在・ビルド記録は実 OS 起動の証拠ではない。
3. **記録・対象・停止:** `supervisor.py` は固定 EXE SHA と専用 root を確認し、one-shot と start を新規・同期・読戻し、直接子の PID・パス・SHA・journal を記録後に grant を送る。子は journal に合成 service UUID・account・PID・EXE を記録し、grant/EOF 待ちの後で NoEntry、合成値 A/B の書込み・照合、値一致後の削除と NoEntry を行う。NoEntry→write と照合→delete は原子的でなく、同名操作の競合は完全には排除できない。grant 後の監督者停止や OS 障害の無条件保証はない。レビュー時に旧記録以外の当該新領域 one-shot・journal・outcome は検出されなかった。
4. **復旧・秘密:** `recovery.py` は marker・対象ファイル・PID・EXE SHA・journal・専用 UUID service と account を照合し、単一 target の分類のみを表示する。未知の終了・stage・値、grant 配送不明は cleanup を拒否する。別操作の read/delete 間には残余競合がある。`diagnostic_summary.py` は固定カテゴリと数値コードのみを処理し、値や例外本文を出力しない。実 OS の権限、ctypes、異常時復旧、外部干渉は未実証。終了 0 と completed stage は削除後 NoEntry を試験した意味であり、後の同時変更の不存在を保証しない。
5. **テストの範囲:** 新領域の path-test は１件成功・９件対象外、summary-test は５件成功、build log は live build 完了を示す。各保存 exit は0、ログ SHA は作者結果に一致。自分は元プロセスを監督・再実行していない。旧版の承認や模擬試験を新 EXE の実 OS 動作の証拠としない。本人の通常 PowerShell で指定 Python が使えること、Windows エラー1312が起きないことは未確認。
6. **一次要件と安全な報告:** Issue #3 は実アプリでの初回登録・変更・削除・再起動後取得と保存済み PAT 非返却を要求する。合成データでの OS 試験が成功しても、実アプリ・実 PAT・実 WebView の各条件を証明しない。`実行手順.md` は PAT 入力禁止、実行１回、未知結果の再送・推測削除禁止、秘密を報告しないことを記載する。

## 実施範囲・未確認

`Get-FileHash`、対象限定 `git status`、旧新スクリプトの no-index 差分、コードとログの読取り、EXE の manifest 文字列のバイト検索、Issue #3 の番号指定コネクタ読取りを実施した。no-index の終了1は差分ありを示し、同一ファイル比較は成功。Git は個人 ignore 設定の読取拒否を警告した。今回、実 credential API、アプリ、テスト、GitHub 書込みは行っていない。作者が削除対象なしと記載し、本レビューにも削除対象はない。全 workspace の削除有無は確認していない。今回認識した失敗はない。
