# 再検証後の合成資格情報照会スクリプトのレビュー依頼

- `inspect-ipc-retry-credential.ps1` SHA256 `3D083D0A0BE0EF82D058DC5EF79B79FF7F916BA6B8C823A21B406BFB5EA4F48B` は、既承認 `inspect-ipc-credential.ps1` からログ名とログ SHA だけを変更した案。新しいテストログ SHA256 は `CE3D2D242F7FD2410A9992863793B898F19A44C547ADC7D5BA30A204343E88D3`。
- 対象と差分、照会対象の限定、値・ID の非出力、固定ハッシュと同一のバイト列からの抽出、読み取り専用であることを静的レビューする。スクリプトの実行、資格情報操作、秘密・値・ID の出力は禁止。結果を `inspect-ipc-retry-review.md` に保存し、通知は絶対パスのみ。
