# 合成資格情報 IPC テスト計画の独立レビュー

- 担当: `/root/snapshot_review`。対象 `ipc-plan.md` SHA256 `37F7AE6147D414D0B1B81DB51E77E943443E139D0E1EA413D68B743DA422E0FB`。
- 判定: **要修正**。モック IPC と既存の隔離 fixture を利用する方向は妥当だが、実資格情報に影響しない削除条件と、実装・検証する対象版の固定が不足する。実行や製品完成の承認ではない。

## Blocking 指摘

### B-01: 合成 ID の事前確認と実際の削除が一体ではない

- 根拠: 計画は `delete_token` の前に設定 ID と合成値を確認する。しかし現行の製品コマンドは引数として期待 ID を受けず、自分でロックを取得した後に `recover_pending` と `read_settings` を実行し、その時点の ID を削除する。事前確認から IPC コマンドのロック取得までに別の書き手が fixture の設定・復旧記録を変えた場合、事前確認した ID と削除対象が一致することは計画上保証されない。
- 影響: 通常の新規一意 fixture ではそのような書き手を想定していないが、想定外の並行変更があれば意図しない credential を対象にする可能性がある。実削除は未観測であり、同一ユーザーの攻撃が成立するとの主張ではない。
- 対応案: IPC 呼出しを子プロセスに限定し、親の設定変更担当が以後 fixture を変更しないことを明確な同期で証明したうえ、fixture と復旧記録を呼出し直前に再検証する。さらに、対象 ID の確認と削除の間に想定外の設定変更があり得る限界を明記する。より強い隔離が必要なら、テスト専用の一意な credential service を使う等、他の service の credential を削除できない仕組みを検討し、その場合に製品と異なる範囲を記録する。いずれも実行前にコードと対象 SHA を独立レビューする。

### B-02: 実装基準・変更範囲と検証対象が固定されていない

- 根拠: 計画は Tauri の dev-dependency を追加して既存試験を拡張するとするが、着手前の `main.rs` / `Cargo.toml` / `Cargo.lock` の SHA、変更可能なファイル、実行コマンド・ログの確定先を列挙していない。`Cargo.lock` が変わる場合の扱いも明示されない。
- 影響: 未追跡を含む既存変更の保護、計画と実装版の対応、feature の製品ビルドへの影響を独立に照合しにくい。
- 対応案: 現行実体 SHA と所有・変更範囲を計画に記録し、単独 ignored テスト・通常テストの作業場所、コマンド、終了コード、ログ・credential cleanup の確認方法を具体化する。依存や lock が変われば変更版を改めて照合する。

## 確認結果と実装時の注意

- ローカルにある `tauri 2.12.7` の crate manifest は `test = []` feature を提供し、test module に `mock_builder`、`mock_context`、`get_ipc_response` を定義する。これはモック API が存在する事実であり、今回の環境でのコンパイル・IPC 成功を証明しない。
- 同モックの `get_ipc_response` は内部で期限なしの `recv()` を使う。IPC 呼出しは既存の期限付き子プロセス内に限定し、期限超過後も子とスレッドの終了を確認できない場合は合成 ID・fixture を保持し cleanup しないことを実装・実行時に確認する。
- 返却値の検証は token・credential ID を含まない固定メッセージの条件式とし、失敗時に JSON 応答や実資格情報を `assert_eq!` / debug 出力しない。`delete_token` 前の fixture・復旧記録確認と削除後の合成 ID 不存在確認を行う。
- dev-dependency の feature が通常リリースに有効にならないこと、ローカル mock runtime が製品 IPC ハンドラーを実際に呼んでいることは実装後に証拠で確認する。実 WebView、IPC capability、実 GitHub、`save_connection` の通信、最終照合後の変更を証明しないという計画上の限界は妥当。
- 本レビューは対象の存在・SHA、計画、現行コマンド、ローカル Tauri の manifest と mock API の読み取りのみ。テスト、GUI、Credential Manager・外部サービスへの操作、製品ファイル変更は行っていない。認識した失敗はない。
