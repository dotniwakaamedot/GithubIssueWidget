# release ビルド方式の固定と実行依頼

- 担当 `/root/gui_failure_investigation`。`gui-release-build-01/preflight-01.md` に記載した専用コピー・manifest・環境と既存 `gui-release-build-work-request-01.md` を継承。元の製品・依存・既存 target・旧 fixture に書き込まない。
- 実行に直結する単一のスクリプトまたは正確なコマンドを専用領域に固定し、`VsDevCmd.bat -arch=x64` の成否を確認後、専用 widget で `npm.cmd run tauri -- build --no-bundle` を1回実行できるようにする。専用 CARGO_HOME/CARGO_TARGET_DIR/TEMP/APPDATA、オフライン、最小限の非機密環境、ログの保護、終了コードの直後捕捉と出力完全性を明示。直接プロセスと子孫の終了・未確認を分ける。入力・manifest・出力先の開始直前チェックを含める。
- まず作成した実行器の絶対パス・SHA、入力 manifest SHA、既存 preflight と実行条件を新しい結果に記録し、別担当が固定版を確認するまで実行しない。確認後の管理通知で build を開始する。テスト39件の無理由な再実行、GUI 起動、credential/実 GitHub の操作は含めない。失敗時は出力を保護し、同じ失敗2回時は別担当に調査を依頼する。
