# 実ビルド driver 修正版の独立レビュー 02

- 担当 `/root/process_test_review`、作者 `/root/build_verification`。対象 `real-build-driver-design-02.md` SHA256 `0D6F42F22537F7DE5C943BC0E53AD98795227EE5719A20FDD7476EB505B17E21`、`build-driver-03.ps1` `BA0269227A5239F5E7B9CA0EF2802D3A5D5C7678821E67C141CC4A1958927FA9` は依頼の期待値と一致。依頼内の10参照と元・専用コピー各17ファイルは全件の存在確認後に実体 SHA を照合し一致。
- 判定: **対象版の設計・コードに限り承認**。前版の blocking B01/B02 は設計上是正された。実行・実 build 成功・Issue #2 完了の承認ではない。実行前の前提が確認できない場合は実行しない。

| 対象 | 独立照合 | 残る未確認 |
| --- | --- | --- |
| B01 一時ログと公開 | 保護ディレクトリを SID/ACL で作成し、一時ログを両 EOF・close 後に hash と秘密形式の指標を確認。独立担当による確認、版・hash・担当・時刻を結ぶ receipt と親の publish 入力の後だけ非置換確定。検出・読取不明は保持。 | ACL、監査、receipt、検出器の実動作と実ログの内容は未検証。パターン不一致は秘密不存在を保証せず、同一ユーザーや管理者に対する境界でもない。 |
| B02 fault と EOF | process 実行・timeout・EOF 待機・VS bootstrap で task の Faulted/Canceled と RanToCompletion を区別。例外時は利用可能な Process/task と一時ログを保持して失敗状態を記録・管理判断待ち、再起動しない。 | 開始直後や制御入力途絶、task 生成・file 操作の例外時に各 handle を取得・追跡できるか実行未検証。EOF の成功と process 終了は独立に評価する。 |
| stage・対象 | rustup、npm、release の直起動対象・専用キャッシュと成果、前段の success と源17ファイル照合を確認。VS bootstrap の処理と Node/npm の exit を別記録。 | npm→shell→Tauri→Cargo の個々の exit や失敗伝播は直接証明されない。公開取得・依存スクリプト・OS cache・レジストリ等の副作用を全て否定できない。 |
| timeout・資源・競合 | 同一 handle を保持し、管理判断後も process 終了と両正常 EOF を continue 条件とする。100 MiB は厳密な上限ではなく観測・相談閾値。 | 待機中も出力は増え得る。実行前に所有、容量・メモリ・CPU、画面作業と共有資源、出力先・リンク・許可・接続先を確認する必要がある。 |
| 版・証跡 | source manifest とコピーを比較し stage 前後の hash、実プロセス exit、ログ hash、release exe hash を対応付ける方針。 | 前後一致は途中の一時変更や他環境の非競合を完全に証明しない。Build/GUI/認証/URL の要件達成は未検証。 |

## 実施内容と限界

- 依頼・design-01/02・driver-02/03 と前レビューを UTF-8 で読み取り。依頼の10参照を全件存在確認後に SHA256 照合し、元とコピーの各17対象を manifest に基づき全件存在確認後に SHA256 照合（cwd `D:\Projects\GithubIssueWidget`、exit 0）。未追跡を含む実体を対象にした。旧成果の削除提案はない。
- 同じ意味の blocking 指摘の再発は確認していない。静的確認に限定し、driver、依存取得、試験、build、process 操作、画面、外部送信や製品変更はしていない。拒否された window metadata 公開操作は再試行・回避していない。
- 読取コマンド失敗なし。変更後は新しい対象版を再レビューする。
