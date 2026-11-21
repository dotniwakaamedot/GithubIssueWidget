# 合成資格情報テストの診断版

## 結果

実 OS 試験 `dcdc5dfdf4524ffa8d9139ee5a781ae4` の終了コード 1 は、子の標準エラーが破棄され、原因が判明していない。新規 `credential-error-capture-01/` に診断版を準備し、実際の Credential Manager に触れずに対象の単体試験とビルドを実施した。**実 OS 試験の成功・原因の特定・アプリ完成は未確認であり、この版は実行前の独立レビューが必要。**

## 設計

- 承認済み `credential-live-ready-02/` の Rust・監督・復旧コード等を新規領域へコピー。元の journal・実行記録・one-shot marker はコピーせず変更しない。`supervisor.py` は新しい実行ファイルの SHA のみに更新し、識別記録・子の登録後 grant・one-shot・停止・既存の cleanup 条件を維持する。
- Rust の `keyring::Error` はエラー本文や provider が保持する値をフォーマットせず、操作名、固定された列挙カテゴリ、および `keyring::windows::Error` に downcast できるときの数値コードのみ抽出。失敗時は検証済み run ID に対応する `.diagnostic` を専用領域直下に create-new、flush・sync・読戻しする。診断ファイルの作成・書込み失敗は成功としない。静的な内部エラーには固定カテゴリの代替診断を試みる。
- 既存の journal の `stage=` 形式は変更しない。読取り専用 `diagnostic_summary.py` は `recovery.read_identity` で子の終了・バイナリ・run ID・journal の対応を確認したうえで、最大256バイトの診断ファイルを ASCII で厳格解析し、許可された操作・カテゴリと u32 のみ、終了状態・journal の最終 stage とともに表示する。値、target、例外本文、認証情報は表示せず、欠落診断は `null`、不正は固定エラー扱い。Credential Manager API は呼ばない。
- Windows の keyring 3.6.3 `windows.rs` の `Error(pub u32)` と `decode_error` をローカル依存で確認した。downcast できない provider エラーの OS コードは unavailable。journal 書込み前・強制終了・診断ファイル作成失敗などでは診断自体が残らない可能性があり、欠落は成功の証拠ではない。元失敗の stderr は失われたままで、この版から過去の OS コードを復元することはできない。

## 検証

- 作業場所は `D:/Projects/GithubIssueWidget/docs/agent-runs/20261121-app-completion-restart/credential-error-capture-01/`。分離した `cargo-home` と target、`cargo +1.90.0`、`--locked --offline --features live` を使用。`python -B verify-02.py` は診断の Rust 単体試験1件成功・9件対象外、live feature build 成功、各終了 0。試験はダミーの provider 文字列・番号と専用一時ファイルのみで、実 credential API は呼ばない。
- `python -B -m unittest -v test_diagnostic_summary` は5件成功・終了 0。許可カテゴリ・数値コード、不正入力・過大ファイル、模擬された終了と stage の対応、欠落診断、CLI の固定エラー表示を確認。`recovery.read_identity` はモックしたため実行記録との接続・実 OS を証明しない。最終ログ `summary-test-02.log` SHA256 `B35333E3833ACE340DED44C38C341F184BC44F230E475E0ECC163712F332535C`。
- 対象 SHA256: `src/main.rs` `C8403557CCA04064AD5951567F82975065C577F8112706670469829981D32960`、`supervisor.py` `D603D89B148A652AC0E54014E3AC63F78F75362D8554B07C09069E1D2FFA34F7`、`recovery.py` `04A3911C622E09B1416962C388972443041916D6E06A14C0FB7F01646B86A302`、`diagnostic_summary.py` `25C683CB1057CC3E0C77EDC4FF1984EC13502F89160C80A3D4F64A576F499BB0`、`test_diagnostic_summary.py` `FBCE6000DFD7132E101BCA2C38A3796ECC3846633AE0013D3782F071253E12B2`、live EXE `9AEC7CDC8FCC5AB3109153C930CE45900CA5290B284D1F03B052C33B72CF845F`。
- `diagnostic-test-02.log` SHA256 `72B6751C412996BEFB8DC9EAAD3F0F86C170645FFC4DBD4F4B4D109225FF0417`、`live-build-02.log` `9F5E542BA5A826EADAEBEAD6D3369B1B5697AD88BDEE8E580337F6B944CCFEF6`。両 exit ファイルは `0` で SHA256 `13BF7B3039C63BF5A50491FA3CFD8EB4E699D1BA1436315AEF9CBE5711530354`。Python テスト終了 0 は実行結果として確認、別の exit ファイルは作成していない。

## 失敗と未確認

- 最初に keyring のソースを推測した `credential-live-ready-02/cargo-home/registry/src` は存在せず `rg` が終了 2。以降は確認済みの `credential-live-test-01/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/keyring-3.6.3/` を用いた。同種の失敗は１回で、共有失敗記録への反映を親へ引き継ぐ。
- この変更は試験経路に限り、実アプリ・PAT・Credential Manager・GitHub を操作していない。生成 EXE は**未起動・未承認**。静的検査と上記単体試験だけでは親子プロセスを通じた診断・異常終了・実 OS コードの捕捉、同名競合、API や OS が返す値を証明できない。前回レビュー済みの安全境界が新しい版でも維持されるか、対象コード・ログ・EXE と read-only summary の独立レビューを要する。
- 本担当の変更は専用領域と本結果のみ。削除対象なし。実行後に可能な診断は target の値を出さない分類であり、終了不明、予期しない値や曖昧な stage では推測による再送・削除をしない。
