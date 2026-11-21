# 製品変更・ローカル EXE 統合レビュー依頼

- 担当予定 `/root/product_integration_review`。親 `/root` が ID 照合のうえ起動。各作者と独立。共有 run `D:/Projects/GithubIssueWidget/docs/agent-runs/20261121-app-completion-restart/`、作業場所 `D:/Projects/GithubIssueWidget`、基準 HEAD `4961b25bec53a93439acdf5230de77f959908af0`。
- 対象: `README.md` SHA256 `552827810A83303D430C143626E84ED4FAD5C94693A9CBBB3163071930778112`、`widget/src/style.css` `FFE871A15BDA8FAD0BB5E28E1717C93D2878ABB1F5885F730EC624DEC983FCFE`、`widget/src-tauri/tauri.conf.json` `0132FCC92345F5D60DE84FD25197FCE853044AD63DAE28757AB4592CC72B991E`、`widget/release-20261121-selection/github-issue-widget.exe` `0D8AB47088B7A19233EF0D7D08CF033B681B20CA0E729FEAF73EAB74DCC98F55`。開始時照合し相違時は停止・質問。
- HEAD との製品全差分・Issue #1–#7 の一次要求・README と修正の整合・対象 EXE のビルド入力／実機報告／個別レビューを照合。認証・非公開情報・URL 許可・同期・作成・移動の影響と既存未コミット変更の保護を検討し、実機、模擬、静的確認と未確認を分ける。アプリ完成全体の監査と取り違えず、blocking や必要な追加確認を示す。
- 読取りのみ。製品・Git・資格情報・Issue・他担当の成果に書き込まない。対象・出力量を絞り、同種失敗２回は停止して報告。所有は新規 `product-integration-review-01.md` のみ。共有環境で他者の変更を戻さず、UTF-8 実データ読戻し後に一時ファイルから確定、パスのみ通知する。
