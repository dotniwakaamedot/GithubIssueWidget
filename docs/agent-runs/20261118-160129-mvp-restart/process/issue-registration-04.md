# 進行レビュー提案登録記録の訂正 04

- `process/issue-registration-03.md` の提案元 `process/review-02.md` の SHA256 記載は誤り。過去の確定記録は保持し、本ファイルで訂正する。
- 正しい提案元 SHA256: E98619BA0CF370BAEFB881B7FED947D943AD348F38460E0EFE1C72F66CF63BF9
- 誤記を含む旧記録の実体 SHA256: BBCE36DD5CEE27C2167A68EEAFFE9BC2EC2E224B1011A1D766426D7AEE1FAB66
- 各値は該当ファイルの実体から Get-FileHash -Algorithm SHA256 で取得し、64桁の英数字と再計算を確定前後に照合した。
- Issue #9 / #10 は OPEN。両 Issue の本文内の提案元 SHA256 は正しい提案元値と一致し、誤った値は共有 run の登録記録だけに含まれる。Issue の追加・修正はしていない。
- 前回の担当 ID の取り違え、可変 plan 参照と本記録の SHA 誤記を参照整合の同種3件と扱って中止。ユーザーの再開後は担当 ID・固定版・ハッシュ64桁を実体照合する。各誤記の発生経路や根本原因は一律には確定できていない。