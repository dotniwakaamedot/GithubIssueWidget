# I02 再開前の版確認 01

- `request-01.md` 確定・担当起動後、管理が `plan.md` を状態更新したため、同依頼内で指定した `plan.md` revision 02 のハッシュと実体が一致しなくなった。承認済みの原本は同じ SHA256 の `plan-02.md` に保存済み。状態更新以外に仕様の変更はない。
- この参照整合の失敗を確認後、/root/foundation を interrupted とし、作業停止を確認。対象 worktree の `git status --short --untracked-files=all` は空、HEAD は a4ac97286785a2a5ba0d070a909b58003fc49ff1。担当による変更は現在見られない。
- 最新の版を参照する `request-02.md` を確定。新たな実施は本依頼に基づいて同一担当を再開する。今後は固定承認版を immutable snapshot として先に保存し、そのパスを依頼に指定してから状態更新と担当起動を行う。