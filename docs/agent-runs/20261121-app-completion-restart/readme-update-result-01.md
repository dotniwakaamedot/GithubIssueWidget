# README 利用手順の更新結果

担当 `/root/readme_update`。依頼 `readme-update-request-01.md` に基づき、作業場所 `D:/Projects/GithubIssueWidget` の `README.md` を更新した。対象の独立レビューは未実施。

## 対応内容と根拠

- 変更前 SHA256 `3D2A457B647B8B514866A693A49E6D70AA948C68B38F9BBA17CE439D5DF8CB9E`、変更後 `552827810A83303D430C143626E84ED4FAD5C94693A9CBBB3163071930778112`。
- 既存の開発・ビルド手順と、ローカルに生成済みの検証版 EXE の利用を分けた。受渡先 `widget/release-20261121-selection/github-issue-widget.exe` は `/root/release_copy` の `release-copy-result-01.md` 確定後に実体を読み、SHA256 `0D8AB47088B7A19233EF0D7D08CF033B681B20CA0E729FEAF73EAB74DCC98F55` を確認した。同結果の SHA256 は担当報告による `022A896DFA256661400C882B639411ACB6BF9C9DDBD23FDA492F7590DDCF4BD0`。コピー先での実起動は未検証。
- ローカルの生成物が別 checkout に自動で存在しないこと、インストーラーや署名済み配布物ではないことを明記した。実行時の WebView2 Runtime と開発時に必要なツールを区別した。
- 「＋」が新規作成であること、カード・列見出しの文字選択不可、通信・認証・権限・レート制限時の確認、結果不明や部分失敗時に GitHub の状態を確認して重複送信を防ぐ手順を追記した。既存の秘密を共有しない案内、他ラベル・競合の説明を保持した。
- 根拠として `release-readme-audit-01.md` と実装の対象箇所を参照した。認証、実 IPC、実障害など未確認の内容を検証済みと記載していない。

## 検証・残事項

- `git diff --check -- README.md` は終了コード 0。`git diff -- README.md` で局所的な変更と既存説明の維持を確認し、見出しの位置を調整した。
- PowerShell の厳格な UTF-8 デコードと代表日本語文字列の照合は終了コード 0、`UTF8 markers OK`。最終版 SHA は上記のとおり。静的な文書変更のためアプリ起動やテストは行っていない。
- ローカル EXE 受渡しの独立レビュー、README 差分の独立レビュー、実 IPC・実 Credential Manager の更新削除や障害時の検証は未完了。署名・一般配布・インストールの検証を行っていない。
- 本担当が変更した製品ファイルは `README.md` のみ、他の成果は本結果。既存の変更を戻さず、Git 操作、GitHub や資格情報への書込み、EXE の変更は行っていない。削除対象なし。認識した失敗なし。
