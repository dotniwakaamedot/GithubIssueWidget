# CIM アクセス拒否の独立調査

担当 `/root/gui_failure_investigation`。依頼 `cim-investigation-request-01.md` の SHA256 `EB5BFA34AEE0330545F436B9376BA5087560121763E1304827716802401898EF`。作業場所 `D:\Projects\GithubIssueWidget`、branch `main`、HEAD `5e74e1642fd18fe630dce0e684afd853721276ae`。

## 結論

２件は別担当の別操作だが、どちらも既定 sandbox の PowerShell で `Get-CimInstance Win32_Process` が `PermissionDenied`・HRESULT `0x80041003`、終了1となったため、同種のプロセス照会失敗として記録するのが妥当。直接原因の同一性は未確認。照会できなかったことは対象の不存在・旧合成子の終了を示さない。今回の指示に従い別担当が調査したが、旧プロセスの終了や GUI の問題は解決していない。

## 根拠と未確認

- `/root/existing_work_review` の `cim-existing-review-answer-01.md` によると、2026-11-21 に Win32_Process を取得し、名前で後段フィルタして PID・親 PID・名前・実行パスを得ようとしたが拒否され、プロセス情報は得られなかった。再試行なし。別の `Get-Process` で名前・PID の一部だけ確認した。
- `/root/process_safety` は質問へのチャット回答で、2026-11-21 日本時間09:54頃以前、同じ作業場所・標準 sandbox で `Get-CimInstance Win32_Process -Filter "Name = 'python.exe' OR Name = 'pythonw.exe'"` により PID・親 PID・作成日時・パス・コマンドラインを得ようとしたが、HRESULT `0x80041003`・終了1で一覧を得ず、再試行しなかったと報告した。後続 `Get-Process -Name python,pythonw -ErrorAction SilentlyContinue` は出力なし・終了1。チャット回答を本結果に転記したもので、当事者の専用回答ファイル・元出力の完全ログはない。
- 前者は PowerShell の後段、後者は CIM フィルタで対象を指定したが、どちらも CIM 照会は失敗した。対象の絞り込みの違いが拒否を引き起こしたかは未確認。ユーザー権限、WMI 名前空間 ACL、サービス、sandbox 等は可能性であり、いずれも原因と断定する証拠はない。照会結果を得た場合でも親子関係・PID・時刻の整合と識別根拠を別に判断する必要がある。
- 旧 F-081 は Job 作成方式の失敗、F-082 は旧合成子の未識別と停止判断の遅れで、今回の２回と同じ操作ではない。先行 run の CIM 照会不能との予防策は共有するが、独立の発生として日時・担当・分類を記録し、過去件数への編入や累計は管理が照合する。

## 安全な対応

1. 同じ拒否を再実行して原因を推測せず、管理がこの２件の担当・コマンド・環境・終了コード・情報未取得・原因未確定を失敗記録へ重複照合の上で反映し、F-081/F-082 と別の事象として整理する。`work.md` の「失敗と空結果を区別」「不明時は実書込みを無効」「個体の識別と終了確認」は適用済み方針で、効果は未検証。
2. 別経路は先に既存の保有ハンドル、子作成記録、監督ログ、OS 起動履歴など、過去の対象と時点を結び付ける証拠を確認する。独立した証拠がなければ不明を維持する。`Get-Process` の名前と PID、空出力だけで終了・不在を判断しない。終了操作・アクセス権変更は行わない。
3. 新規の読み取り方式を検討する場合は、実行の権限・取得項目・個人情報の抑制・時点・同一性・失敗時の結果を明記し、安全性を独立レビューする。上昇権限が正しい解決策とは現段階で言えない。旧対象の終了または不存在を適切に確認し、安全な監督方式が成立するまでは GUI・追加合成子の起動を保留する。

## 作業と対象版

初回調査依頼前の質問 `cim-investigation-question-01.md` SHA256 `E1A120E03A2E9C16F70847CD5619C93BBC60631720068D19EFB6331C4BCFF4B9` と、当事者向けの `cim-existing-review-question-01.md`・`cim-process-safety-question-01.md` を新規作成。成果は本ファイルのみ。削除・製品変更なし。対象版・関連資料の `Get-Content -Raw -Encoding UTF8` と `Get-FileHash -Algorithm SHA256` は終了0。CIM 再試行、権限変更、プロセス停止、GUI 起動、外部書込みは行っていない。新しい操作失敗は認識していない。成果提出後の独立レビューは未実施。

開始・終了時に一致した SHA256:

- `failure-record-draft-01.md`: `D5A7EB2A071331E8DD6A47354B98ED804E96E549A1DA6B4EC6DE3CB597983B7F`
- `output-failure-independent-01.md`: `CA672BC419D530D1C419B3D4721B18CA8297FE9DD5354E1AFA54C1D52781D9D5`
- `../20261121-issue-completion/existing-work-survey-20261121-01.md`: `7622177B6019817719B47756207B884C7A879F49C6363AA67697B5EF73658EED`
- 当事者回答 `cim-existing-review-answer-01.md` の受領時 SHA256: `01ED4E39B282A12097DDD83878B718DAE05F98A6F81198CF2369B7E63272B1DC`。
