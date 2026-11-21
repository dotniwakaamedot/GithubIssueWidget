# ローカル利用案内の作成と読戻し

- 対象: widget/release-20261121-selection/利用案内.txt
- SHA256: F7E81D236CF3DDC465BDBF8000E850947C92996E53D60E24C6660539880EBF70
- UTC: 2026-11-21T10:51:35.809321+00:00
- 入力: JavaScript 文字列を JSON ASCII Unicode escape にし、PowerShell で UTF-8 の Python コードを Base64 化し、Python json.loads で復元。旧の PowerShell から Python への日本語パイプは不使用。
- 出力: UTF-8 strict で新規一時ファイルへ書込み flush・fsync。公開前に実バイトと全文を strict 復号・照合し、代表文字と置換文字非存在を確認。新規確定名へ rename 後もバイト一致を確認。
- 根拠: README.md、Issue #1/#3/#5/#6/#10、本人実機報告、EXE SHA256。全画面・全環境は未確認。
- 実行中には、先行する Python 起動コードの構文エラー1回とその出力切り詰めがあった。失われた出力は未確認。第2経路は構文確認後実行。

- 2026-11-21T10:52:30.320591+00:00 に本人報告を超えないよ再起動時の表現を訂正。元版 SHA256: 2856DD5967A30C7F4A5A46A7BAF26699752A356478DB9A91CDC4394739C31BBF。新版を UTF-8 strict で一時保存・公開前全文読戻し・訂正文照合してから所有先を原子的に置換、置換後バイトを照合した。
