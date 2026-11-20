# I02 実ビルド用ドライバー準備依頼 01

担当 /root/build_verification。独立レビュー process-test-result-review-01.md の無害試験承認は限定的な証拠であり、元 build 成功や待機原因は未確定。親は実ビルド用の準備・版固定・独立レビューを指示。実 build・依存導入・外部取得・ドライバー実行は独立レビューと親判断まで行わない。

新規作業場所 D:\Projects\GithubIssueWidget-build-20261119-02 を候補とし、作成前に親・正規化先・再解析点・不存在・所有と資源競合を確認。元 worktree、旧 build scratch、無害試験 scratch・証拠、GUI と他者プロセスを変更・削除しない。新領域の所有と出力先、通常 OS 副作用を記録。

result-02.md に固定された未追跡を含む製品17ファイルの相対パスと SHA を機械可読 manifest へ分離し、基点内・存在・通常ファイル・hash を実体照合。コピーが必要なら新規領域に行い元とコピーを全件照合し、前後 SHA、HEAD の限界を記録。空集合を成功扱いしない。対象版変更・照合不明・所有不明なら停止し質問。

実ビルドの具体的なコマンド、環境、専用 source・target・dist・cache・temp・ログ・出力・通信先、認証情報保護を設計し、専用ドライバーを準備。特に .NET Process の同一 handle、stdout/stderr の並行非同期消費、終了後の実 exit code と EOF、開始終了時刻・PID、ログ、生成 exe SHA と17ファイル版の紐付けを設計。npm/Cargo と shell・子孫の終了コード伝播、閉じた標準入力、実出力負荷と timeout の未検証を区別する。

timeout では同じ handle と所有セッションを保持、停止や continue を勝手に実行せず親へ質問。開始成否・結果・入力不明でも再起動しない。CPU・メモリ・ディスク・port・画面作業の競合を実行直前に確認し、無競合を推測しない。外部通信や通常 OS の副作用・限界も明記。

準備結果は tasks/I02/real-build-driver-design-01.md、質問は questions/I02-real-build-driver-question-01.md から空き番号。コード、manifest、対象 exe・依存構成と全参照を SHA 固定し、事実・未確認・作成物・前提条件・失敗停止条件を記録。公開方式に従い確定しパスのみ通知。独立レビューと親判断までビルド実行しない。拒否された window metadata review の公開操作は再試行・回避禁止。

同種2回で該当操作を中断し別担当が客観調査、同種3回で全作業停止。画像不一致・browser inventory エラー・直接公開逸脱・依頼形式訂正検査停止は各2回で調査済み、各同種3回目は全停止。

## 管理データ

```json
{
    "assignee":  "/root/build_verification",
    "references":  [
                       {
                           "path":  "D:\\Projects\\GithubIssueWidget\\docs\\agent-runs\\20261118-160129-mvp-restart\\tasks\\I02\\process-test-result-review-01.md",
                           "sha256":  "3B2BA6D87D2D40CCCDE6DED595F6C29A6A5624C69CF7A614EFFA5AB5FDC3652C"
                       },
                       {
                           "path":  "D:\\Projects\\GithubIssueWidget\\docs\\agent-runs\\20261118-160129-mvp-restart\\tasks\\I02\\process-test-result-01.md",
                           "sha256":  "E0E7FABA107E515CEF1DC0B65A839DBDD832748DB2DD20D0F30249174B2DF175"
                       },
                       {
                           "path":  "D:\\Projects\\GithubIssueWidget\\docs\\agent-runs\\20261118-160129-mvp-restart\\tasks\\I02\\build-wait-diagnosis-result-01.md",
                           "sha256":  "B538DC70E8C0CAC5036E13C983A1BC2222AA9A3B7B5943C91AECF5E8451859D9"
                       },
                       {
                           "path":  "D:\\Projects\\GithubIssueWidget\\docs\\agent-runs\\20261118-160129-mvp-restart\\tasks\\I02\\build-verification-result-01.md",
                           "sha256":  "7A8AF3B60D32D2AC94888801067C73031DB8A84974DA95E6373D03C03E68A098"
                       },
                       {
                           "path":  "D:\\Projects\\GithubIssueWidget\\docs\\agent-runs\\20261118-160129-mvp-restart\\tasks\\I02\\verification-design-result-03.md",
                           "sha256":  "B2B552301D897717CC80404A64297C460BADED4C8AE4B0F130B1557F7B08DAED"
                       },
                       {
                           "path":  "D:\\Projects\\GithubIssueWidget\\docs\\agent-runs\\20261118-160129-mvp-restart\\tasks\\I02\\verification-design-review-03.md",
                           "sha256":  "37899D597AAC6DA8F94E2775818A8DDC76796C0EBAA730E9404BCE14EEE20C25"
                       },
                       {
                           "path":  "D:\\Projects\\GithubIssueWidget\\docs\\agent-runs\\20261118-160129-mvp-restart\\tasks\\I02\\result-02.md",
                           "sha256":  "520044FE6138609B99595C5FAF1D3973DD2D62EB82F27FE90392568C9599ED13"
                       },
                       {
                           "path":  "D:\\Projects\\GithubIssueWidget\\docs\\agent-runs\\20261118-160129-mvp-restart\\process\\evidence-publication-protocol-01.md",
                           "sha256":  "65E36A266503EB12E20B4045BC7D2A346C8F72B97BA3ED3EF73A3B80B36B3382"
                       },
                       {
                           "path":  "D:\\Projects\\GithubIssueWidget-process-test-20261119-01\\driver-04.ps1",
                           "sha256":  "F24B68C56FA99A67685BFB864F9B39F768EBDBA0CC1D7B2E92A12CA4DF0C7FDD"
                       }
                   ],
    "investigate_at":  2,
    "stop_all_at":  3,
    "direct_publication_violation_count":  2,
    "format_correction_check_failures":  2
}
```
