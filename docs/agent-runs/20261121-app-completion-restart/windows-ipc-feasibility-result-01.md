# Windows 合成資格情報・IPC 試験の実行可否

## 結論

現行 `tests::windows_ipc_guard` は実 Credential Manager と模擬 Tauri IPC を結ぶ有力な既存試験だが、**指定された安全条件を現状では立証できず、実行を保留する**。標準出力 flush は識別子の外部への出力であって、耐久記録と読戻しではない。NoEntry 確認と UUID による固有名も、書込みまでの競合排除を証明しない。既存の対象版成功証拠は今回確認した資料には見つからず、試験が成功したとは扱わない。確認範囲外に存在しないという意味ではない。

## 対象と一次要求

- 担当 `/root/windows_ipc_feasibility`、作業場所 `D:/Projects/GithubIssueWidget`、HEAD `4961b25bec53a93439acdf5230de77f959908af0`。HEAD は未追跡・未コミット変更を識別しない。作者とは別の調査担当であり、この結果の独立レビューは未実施。
- `windows-ipc-feasibility-request-01.md` SHA256 `1DD1033F0DBF4F91E0CB13D7BFC74CAB8E88179AD715FF11E784F752C9DBF89F`、承認計画 `plan-approved-03.md` `355CD3D07C9CEAD72576CACC02D31B27484F7C165AEAB967E2FBCC548DD175C1`、対象 `widget/src-tauri/src/main.rs` `AEFF1E65AD93EFA04773CEBBB9736A6BDC5B9938DB9FAF937DD88B3D9CBFE86F`。依頼記載の計画・対象 SHA は実測一致。
- GitHub コネクタで Issue #3 を番号指定取得。Windows Credential Manager の初回登録・変更・削除・再起動後取得、React への保存済み PAT 非返却、保存途中失敗の通知と整合性を要求する。模擬試験の合格だけでは実アプリ・実 PAT の完了条件を証明しない。
- 先行 `windows-credential-test-result-01.md` SHA `5C833BD55B2091034132C7A42BCEFFDDC7A12A41BEA3D60B62F59C2ACA8B177F` は別の `isolated_windows_credential_lifecycle` を識別子事前記録がないため未実施と記録。`credential-safety-design-01.md` `76057E5039F7BA536545B4F0C0247B000B32C24B7C9CD5D6CF51CBEFF789E6ED` と独立レビュー `credential-safety-review-01.md` `80B17212E51F1184B936BB3005E08F216EC05DFA2A3F0BEE90D7D8012E1F4A05` は排他・耐久記録等を後続の実行前提とし、その実現を証明していない。

## コードで確認した経路

1. `main.rs:698` の親は Windows の ignored test。temp 配下に UUID root、marker、専用 settings ディレクトリを作り、`github-task-widget-ipc-test-<UUID>` の service と UUID の account ID を生成する。keyring の NoEntry の後、`fixture_root`・`test_service`・`synthetic_id` と書込み前 stage を標準出力に出し flush してから合成文字列を書込む。`entry` は test/windows かつ `WIDGET_IPC_CONTEXT` 設定時、service prefix と両 UUID の正規表記を検証する。これらは名前や経路の検査であり、既存 entry の原子的予約ではない。
2. 親は現実行ファイルから `--exact tests::windows_ipc_guard_child --ignored --test-threads=1 --nocapture` の子を起動し、APPDATA、root、marker、service、account、settings path を環境変数で渡す。子は canonical path・marker・許可ファイル等を検査し、合成値を読んで Tauri `mock_builder` と `get_ipc_response` を使う。`connection_info` の返却に合成秘密と account ID がないこと、revision 不一致の一覧・作成・移動・保存・削除の拒否、最後の `delete_token` と NoEntry を検査する。合成値を変更して旧 revision を拒否し、元の合成値に戻す経路もある。実 WebView、実 PAT、実 GitHub 操作の検証ではない。
3. 子の待機は40秒、timeout 時 kill と最大5秒の直接子待機を試みる。親は子の終了を確認できなければ cleanup を禁止して panic する。終了を確認した後に cleanup を許可し、`RaceCredentials` は期待する元の合成値に完全一致した entry のみ削除し、その後 NoEntry を確認する。別値・読取エラー・削除エラーは削除完了としない。子が合成値を変更した後、復元前に異常停止した場合は、元値に一致しないので cleanup が拒否される可能性がある。

## 安全条件と未確認

| 条件 | 現行コードの証拠と不足 |
| --- | --- |
| 識別子の事前記録 | service・account・root は write 前に出力・flush されるが、ログ先の作成・権限・flush の耐久性・sync・読戻しを保証する処理はない。ログ保存不明時は対象を特定できず、実書込みを始めない経路が必要。 |
| 所有権・排他 | NoEntry 後に set するが、間の他者作成を排除する条件付き書込みはコード上確認できない。UUID は競合を低減しても排他の証明ではない。使用 keyring 実装の API 性質と、試験専用 Windows ユーザーなどの排他環境は未確認。 |
| 親・子・孫 | 親は直接子ハンドルで終了を監督するが、親停止後に子が生存しない保証、子孫の範囲と終了、OS 障害時の停止は確認できない。stdout/stderr の pipe は終了後に回収されるため、大量出力による待機阻害の可能性も未評価。PID だけを終了根拠にしない。 |
| 失敗時の残留 | spawn 直前に cleanup を禁止し、spawn 失敗も識別子を残して panic。書込み応答不明、子の終了不明、予期しない値や cleanup 失敗では自動的な他 entry 削除を避けるが、耐久 journal・再開手順・独立した残留照合の実装は確認できない。NoEntry とアクセス失敗は区別する必要がある。 |
| 対象の限定・秘密 | コードは合成 service と合成値を意図する。APPDATA は子のみ隔離、親の通常環境への影響、ログ ACL、fixture の所有権、同時操作、ライブラリ内部出力は未検証。実ユーザーの PAT や既存 service を照会・操作してはならない。 |

## 再開に必要な対応

後続の独立担当が、対象版に対応する既存成功証拠を必要な範囲で照合し、無ければ専用コピーの監督・復旧経路を具体化する。識別子と実行体を事前に安全な journal へ排他作成・sync・読戻しし、対象完全一致・ユーザー隔離または原子的所有権、子孫と終了、障害・ログ保存不能時の停止と残留確認を合成障害試験と独立レビューで証明する。条件付き作成や排他の成立を確認できない場合は実書込みを禁止したままとする。既存コードや実資格情報を変更する本依頼ではない。実 OS 試験後も、実アプリの登録・更新・削除や実 WebView の非露出は別の検証である。

## 実施と変更

作業開始時の `Get-FileHash`、`git rev-parse HEAD`、対象限定のソース・先行資料読取りは終了0。対象限定 `git status` は終了0だが個人 ignore の読取拒否警告あり。Issue #3 は GitHub コネクタ取得成功。`docs/handoff-20261121-current.md` と依頼で指定された先行記録に対象版の成功根拠は確認できなかった。今回、テスト・実 OS 資格情報・アプリ・GitHub 書込みは行っていない。

変更はこの新規結果ファイルのみ。既存ファイルの削除・変更は行っていない。対象版の終了時 SHA 照合と実データ UTF-8 読戻しを公開前に行う。認識した作業失敗なし。結果自身の SHA は自己参照を避け管理側で確定後に取得する。
