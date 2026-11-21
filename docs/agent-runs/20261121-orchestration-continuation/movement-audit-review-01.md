# Issue #6 移動監査の独立レビュー

確認者 `/root/existing_work_review`、監査作者 `/root/issue_survey` とは別。Issue #6 の直接取得、監査、現在の実装・テスト・既存ログを読み取りで照合。レビュー対象は静的監査の結論と下記の実ファイル版に限定し、Issue の完成承認ではない。

## 判定

**静的監査の主要な結論を承認する。** blocking な監査誤りや今回確認した範囲の新たな確定欠陥はない。現 UI は元監査時と別 SHA であり、同じ承認を流用せず現行コードとテストを改めて照合した。実 GitHub、UI、通信失敗の実環境と要件全体は未確認。下記は非 blocking の証拠不足と次の検証であり、完了扱いにしない。

## 要件との照合

- **ラベルと6方向**: `move_in_repo` は対象ラベルの存在、直前 Issue の ID・PR 除外・`updated_at` を確認し、必要な移動先ラベルの追加、取得時点にある不要ステータスの個別削除、状態変更、最終 GET を行う。非ステータスラベルを全配列で上書きしない。`all_six_transitions_preserve_labels_added_after_the_initial_get` はローカル模擬サーバーの6方向で `bug` と GET 後に追加された `external`、対象状態・ラベル、旧ラベル削除を検査する。`multiple_status_labels_remove_only_obsolete_statuses` は複数ラベルの固定応答を検証し、実 GitHub の同時更新を再現するものではない。
- **単一 PATCH と競合**: Issue の「可能な限り一度の PATCH」に対して現行方式は複数回のラベル操作と必要に応じた state PATCH を選ぶ。取得直後の他者の非ステータス追加を、古い全ラベル配列で消す危険を減らす判断には根拠がある一方、原子的ではなく中間状態・部分反映と API 呼出しが増える。要件との適合はこのトレードオフを明示した管理判断を要し、単一 PATCH の実装と表現しない。`updated_at` 事前確認と最終 state/status 検査が検知できる競合は通知するが、最終状態が偶然一致する変更や最後の GET 後の変更は検出できない。
- **失敗と再取得**: 成功済みの前段書込み・結果不明は `write_step` が部分反映可能性を通知し、追加・削除の応答不正や最終読取り失敗を成功にしない。現 UI は `moveIssue` の finally で `boardEpoch` を更新し一覧を再取得、失敗時は `moveError` を保持する。現 UI の一覧取得失敗経路は `setIssues([])` とエラー表示を行うため古いカードを残さない設計だが、「移動失敗→再取得も失敗」を一連で確認する専用試験は今回見つけておらず、実行もしていない。
- **欠落ラベル・競合試験**: `changed_issue_and_missing_label_never_send_patch` は事前の `updated_at` 不一致・ラベル404で後続書込みを行わないこと、`final_status_conflict_is_not_reported_as_success` は最終応答に追加ステータスが残る場合にエラーとなることを検査する。実際の他者による初回 GET と書込み間のステータス追加・削除や state 変更を実行した証拠ではない。404 は不在とアクセス不能を区別せず案内し、ラベルの自動作成は行わない。
- **現画面試験**: `refreshes after an uncertain move and keeps the error visible` は `move_issue` が拒否された後、一覧が成功して実態を反映しエラーが残るモック試験で、再取得失敗は扱わない。移動先と同じ分類への drop は無視されるため、Done 列に Open の部分作成 Issue が残る場合の回復しやすさは実 UI で別途確認する。列間移動要件違反と断定しない。

## 検証証拠と次の作業

既存 `rust-regression-01.log` は上記6方向・複数ラベル・欠落ラベル・最終競合の試験名が `ok`、全体67 passed / 10 ignored と記録する。これは今回の再実行ではない。同ログと現 `movement.rs` のバイト単位の対応は本レビューでは独立に確定していない。`product-test-result-01.md` は main.rs の版・終了コードを記すが、movement.rs の SHA を結びつけていない。現 UI は元監査後に変更されており、過去の UI 試験証拠は新版への証明に使わない。

優先して、合成サーバーで GET 後の他者ステータス追加・削除・state変更と最終不一致、UI で移動不明後の一覧取得失敗・古いカード非表示・両エラー表示を対象版に対して検証する。競合が同じ最終状態に収束するケースの検出限界と、実 GitHub での6方向・非ステータス保持・権限や通信障害は別途明記する。テスト実行には適用される対象・安全・独立レビューを経る。

## 対象と操作

開始・終了時の SHA256 は一致: `movement.rs` `ED49F4BA52C6887BCE427E4A6664D335092A1C3311E1B1ED61B4CE322791ABA5`、`main.tsx` `6FCF2A34145450C5029B7FF80DDB3EC1DA5E27173AAB84E57F16D6D54C0E6A3E`、`main.test.tsx` `66E118A5A7062854C23BE405D2E9578C974FA283BEA332F2A7C51DCA1BFFD13C`、監査 `5B8447DD4D17D8929B2F4198F311FA15E0E48BDD409C8D772C06FA3D03E3F41D`。削除対象なし。

GitHub コネクタの Issue #6 取得、固定 SHA、限定 `rg`、必要範囲の `Get-Content` は成功。別の試験結果文書から `Test Files` 等を探した検索は終了1・一致なしで、試験の失敗や不存在とは扱わない。テスト・GUI・外部書込み・製品変更はしていない。新たな操作失敗は認識していない。
