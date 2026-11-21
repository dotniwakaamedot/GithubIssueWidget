# GUI 段階診断の隔離案

担当 `/root/project_survey`。`gui-next-diagnostic-01.md` の案に基づき `gui-diagnostic-isolated-01/` の2ファイルに限って診断コードを追加。製品・既存 build・保護 fixture を変更せず、コンパイル・GUI 起動・GitHub・Credential Manager 操作は行っていない。これは実行可能と確認された診断版ではなく、別担当の固定版レビュー後に独立した build・実行方式を定める。

## 対象と段階

- 出典 `widget/src-tauri/src/main.rs` SHA `AEFF1E65AD93EFA04773CEBBB9736A6BDC5B9938DB9FAF937DD88B3D9CBFE86F`、`widget/src/main.tsx` SHA `6FCF2A34145450C5029B7FF80DDB3EC1DA5E27173AAB84E57F16D6D54C0E6A3E`。診断版 main.rs SHA `50692BB0FCEBB9B3B32D1FC88A43DCE1B0238EE681E534A6BC5D62E3EC87F4FD`、main.tsx SHA `603473420E487660FB9B5467806268F8507BC00DE8AE7BF9DA93953A438B834C`。出典とコピー直後・編集後のハッシュを確認。前回 release の製品入力との対応は作者に照会中で未確認。診断版と元 EXE は別物。
- Rust は `before_run`、Tauri setup の `setup_entered` を固定 ASCII で記録。フロントは effect の mount、connection_info と pending_creation の要求・成功・失敗を診断用 IPC で固定 stage に対応させる。入力文字列は列挙済み値にマップし、未知は `invalid_stage`。token・設定・Issue・例外・パスを marker に記録しない。
- 記録先は `GIW_DIAGNOSTIC_DIR/stages.log`。追記・flush を行い、書込み失敗は固定 `diagnostic_write_failed` のみを stderr に出す。実行時は親が新規の空かつ適切な ACL の専用領域と一意な run を固定し、環境変数にその絶対パスを与える必要がある。コード自体はパスの ACL・実体・再解析点を検証せず、既存ファイルを上書きせず追記するため、事前確認なしに実行しない。元の panic/依存ログは診断 marker 以外を含み得るので無保護のまま保存・表示しない。

## 解釈・未確認

`before_run` は builder より前、`setup_entered` は setup callback 到達を示す。前者のみでは窓生成中に限らず途中の複数の失敗・停止が考えられる。front marker は非同期 IPC が処理された証拠であり、呼出し順や実処理との完全な順序を保証しない。marker の欠落は未到達・IPC 未完了・ファイル障害を区別できない。診断ステージ到達と元 EXE の正常・障害を同一視しない。

書込みは専用標準 mutex で同一プロセス内を直列化するが、別プロセス間排他、flush のクラッシュ耐性、出力上限・ファイル置換競合は保証しない。React StrictMode 等では同じ marker が複数回現れ得る。起動前に専用空 APPDATA と空資格情報参照、実 API 書込みなし、監督・期限・直接子の識別と終了、子孫不明時の保持を別途固定・レビューする。保護された元領域や旧プロセスを操作しない。

PowerShell のコピー・SHA 照合は終了0、`git diff --no-index --stat` は差分あり終了1。静的な差分は main.rs 35追加1削除、main.tsx 8追加。型・Rust コンパイル、合成秘密・固定分類、診断書込み失敗、GUI 実測は未実施。今回失敗なし。成果は2診断ファイルと本結果のみ、削除対象なし。
