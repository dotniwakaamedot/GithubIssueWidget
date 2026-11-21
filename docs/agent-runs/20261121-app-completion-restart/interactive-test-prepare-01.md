# 対話セッションでの合成資格情報テスト準備

- 担当 `/root/interactive_test_prepare`。作業場所 `D:/Projects/GithubIssueWidget`。親の依頼に従い、新規 `widget/credential-check-20261121/` に候補を準備した。実 Credential Manager、実アプリ、PAT、GitHub は操作していない。実 OS 実行は未承認であり、別担当による対象版レビューと親の判断を要する。
- 参照: `logon-session-resolution-01.md` SHA256 `C6A0A3B218353AB115D7C340A59891B821022EF8058DB6B1A3AEE50190EDCDC3`、`credential-error-capture-review-01.md`。旧 one-shot・journal・outcome はコピーも変更もしていない。
- 単純な EXE コピーでは `src/main.rs` の `env!("CARGO_MANIFEST_DIR")` が旧ビルド場所に固定され、コピー先の journal を `journal_outside_workspace` として拒否する。そのため新規ディレクトリにソース・依存をコピーして同じコードをオフライン再ビルドした。新旧 EXE の SHA は異なり、旧版の実行承認を新版へ流用しない。

## 対象 SHA256

| 新規ディレクトリ内の対象 | SHA256 |
| --- | --- |
| `Cargo.toml` | `8DBF35BE0D2AB67489DAD4AB2168E69856D67253B260D0207D775F4D43747513` |
| `Cargo.lock` | `79B1644981C6B122726DDD3AC55E54616C815F9E56AF52ADE760C3B402D8EAA5` |
| `src/main.rs` | `C8403557CCA04064AD5951567F82975065C577F8112706670469829981D32960` |
| `supervisor.py` | `43C7874E56F97E4CBBDC85789501D2D70CBD6FF9D7B5B1951ED72E0F33C34EA7` |
| `recovery.py` | `04A3911C622E09B1416962C388972443041916D6E06A14C0FB7F01646B86A302` |
| `diagnostic_summary.py` | `25C683CB1057CC3E0C77EDC4FF1984EC13502F89160C80A3D4F64A576F499BB0` |
| `RECOVERY.md` | `F2F2D33AA2B3BC4CDC057384E38F2FBCF1BDAE10199D70946A5868C9A3C3EED` |
| `target/debug/giw-credential-synthetic-check.exe` | `4E459DAEEF54F55BC59497FE10DD4FC2AE4A90C18378A3B6C58DE514CB45A082` |
| `実行手順.md` | `1FA68692BF49C3C7CBE99E0683BC2242E0B76312395D483FC68511ED321EB2D2` |

`supervisor.py` の変更は期待 EXE SHA の１箇所のみ。Python の構文解析、EXE SHA と設定の一致、新規領域に `live-once.json` がないことを確認した。依存 registry は旧隔離領域から新規領域へコピーし、Cargo は専用 `CARGO_HOME`・専用 `CARGO_TARGET_DIR`・`--locked --offline` を使用した。旧 run の入力10ファイルは元とコピーの SHA が一致した。registry 全ファイルを個別に照合したとは主張しない。

## 検証結果

- 新規ディレクトリで `cargo +1.90.0 test --locked --offline --features live -- --exact tests::journal_path_is_limited_to_dedicated_workspace` は終了0、1件成功・9件対象外。新 manifest 場所に対する正規経路・外部経路の拒否を合成テストした。`path-test.log` SHA256 `531A9FAC8461A1675ED7FC8A569E5EB45E1405BFE1F8FF82D34256669DADD906`。
- 同所で `cargo +1.90.0 build --locked --offline --features live` は終了0。`live-build.log` SHA256 `9F8449AA7F865EF096D8DE8C957AE9D98A229EF2BD88FEA1805C39419009D764`。実行可能 EXE は起動していない。
- `python -B -m unittest -v test_diagnostic_summary` は終了0、5件成功。`summary-test.log` SHA256 `4DF26767652932B2E8FF62AB9C9D345388AC6C3D07BDF64F66BE2FA42712264D`。模擬テストであり実 Credential API を検証しない。各 `.exit` は `0`、SHA256 `9A271F2A916B0B6EE6CECB2426F0B3206EF074578BE55D9BC94F6F3FE3AB86AA`。
- 新規の `実行手順.md` は UTF-8 strict 復号・代表文字・UTF-8 往復を確認して確定し、公開後のバイト一致を確認した。本人の通常 PowerShell で指定 Python を利用できるか未確認。対象ディレクトリを移動すると manifest に固定された経路と一致しない。

## 残る条件

新しい supervisor・EXE・復旧手順・利用案内は、親以外の独立レビューにより対象版と安全境界を確認すること。再ビルドした EXE の実 OS 起動、対話セッションでの挙動、環境・ACL、予期しない中断後の復旧と実アプリの PAT 登録・更新・削除は未検証。GUID によって同名衝突の可能性は小さくなるが、NoEntry→write と確認→delete は原子的でない。実行・終了不明時は再送や推測削除を行わず、one-shot・journal・outcome を保護する。本人がレビュー前に実行しないよう案内に明記した。削除対象なし。

今回の作業で認識した失敗はない。既存成果や本番コードの編集・削除はない。対象ディレクトリ内のテスト出力・コピー・依存は新規成果として保持する。
