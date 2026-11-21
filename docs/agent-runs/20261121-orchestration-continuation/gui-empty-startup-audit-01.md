# 現行版の空設定起動経路監査

担当 `/root/gui_failure_investigation`。依頼 `gui-empty-startup-audit-request-01.md` SHA256 `D9F8DF76275D8FFFF629109B68A7D42C44C3E9B71591F066641E34D26CC2018C`。作業場所 `D:\Projects\GithubIssueWidget`、branch `main`、HEAD `5e74e1642fd18fe630dce0e684afd853721276ae`。下記は固定版の静的なコード経路であり GUI 実測ではない。

## 結論

新規空の実体ディレクトリを `APPDATA` とし、起動中に設定・復旧記録を含む他者の書込みがなく、利用者が画面操作をしない条件では、製品の起動時の `connection_info`・`pending_creation` の経路は Windows Credential Manager と GitHub API に到達しない。ただし設定ディレクトリと `settings.lock` を作成する。これは Tauri/WebView2・依存全体の非通信、同一ユーザー競合防止、実際の実行結果の証明ではない。`LOCALAPPDATA` の変更は credential store の隔離ではない。

## 対象版の処理

1. `main.tsx` は初期値 `showSettings=true`、`hasToken=null`、`connection=null` で、マウント時に `connection_info` と `pending_creation` を呼ぶ。`list_issues` の自動更新は `showSettings || !hasToken || !connection` なら return。空設定から取得した `ConnectionInfo` は owner/repo 空・has_token false のため、条件を維持し `list_issues` の GitHub API 呼出しには進まない。エラー時もこの初期条件を解除しない。保存・削除・作成・移動・ブラウザー開放は画面操作等で呼ばれる別経路であり、今回の静的条件では操作しない。
2. `connection_info` はアプリ内 mutex と `process_lock()` を取得する。`settings_path()` は `APPDATA/github-task-widget/settings.json` を組み立て、`process_lock()` は親フォルダーを `create_dir_all` し、`settings.lock` を作成・排他取得する。失敗時は後続へ進まずエラーを返す。次に `recover_pending()` が settings と `credential-recovery.json` を確認。両方が不存在なら復旧は早期 return し、削除 closure は呼ばれない。その後 `read_settings()` の不存在は None、`read_token("")` は即 None を返し `entry`・keyring を呼ばない。
3. `pending_creation` も同じ mutex と `process_lock()` の後、同じディレクトリの `pending-issue.json` を読む。ファイル不存在は None、ここで credential や GitHub は呼ばない。２つの IPC 順序は固定されないが、mutex で直列化される。
4. `APPDATA` **未設定**なら `settings_path()` がエラーを返し、当該経路はディレクトリ作成・復旧・token 読取りに進まない。空文字列は未設定と同一ではなく、相対パスとなり得るので新規絶対パスとして扱わない。`LOCALAPPDATA` は上記製品コード経路での設定保存先ではないが、WebView2 等の独自使用や他の設定はここでは未検証。

## 条件が崩れた場合

- 設定ファイルが存在して有効な credential ID を持てば `read_token` は実際の Credential Manager にアクセスする。`credential-recovery.json` が現れれば `recover_pending` が設定と記録を照合し、条件により古い ID の credential 削除と復旧記録削除を行う。単に環境変数を変えただけでこれらが隔離・非競合とは証明できない。
- 事前に空を確認しても同一ユーザーの別プロセスが途中で書き込めば上記仮定は崩れる。実行前に新規・専用・非リンクのディレクトリの実体・アクセス権・空と所有権を確認し、実行中の競合制御を実際に確保する。これらが不明なら無 credential 接触を主張しない。空設定を意図的に維持し、画面で保存・復旧・認証操作をしない。
- 実際の release の生成元と現行ハッシュ、JavaScript が対象版として組み込まれたか、Tauri/plugin/WebView2 自身の副作用・通信・資格情報利用、実 OS の ACL・プロセス競合、起動と終了は未確認。別途固定ビルドと独立レビュー、通常 Windows 環境での限定観測が必要。結果を認証・実 GitHub・全 GUI 要件の証明に拡張しない。

## 読取りと対象

`Get-Content -Encoding UTF8` の固定範囲読取り、`rg -n` の対象ファイル限定検索、`Get-FileHash -Algorithm SHA256` は終了0。GUI・合成子・GitHub・資格情報・ネットワーク操作なし。新規操作失敗なし。成果は本ファイルのみ、削除なし。提出後は書込みを止める。

- `gui-f078-next-investigation-01.md`: `4B0B5D16DB7FCADF87B35AE4085DB8C307D2F62AF6EB33AF143C6F20EF5FBE0D`
- `widget/src/main.tsx`: `6FCF2A34145450C5029B7FF80DDB3EC1DA5E27173AAB84E57F16D6D54C0E6A3E`
- `widget/src-tauri/src/main.rs`: `61595B68317E15FDD8F2BED88B920A468D5516740E3D3124710384709C52FC24`
- `widget/src-tauri/src/creation.rs`: `61FB68E208B231664DB236E50A37BBB4870763AD61C0071383BACA5730982A87`
- `widget/src-tauri/tauri.conf.json`: `5C1F3A0E2E487DD4345A2DD335FE4AACA483FEEEA26C11AAB99B374278EC1364`。

本監査は独立レビュー前。ソースが変われば対象版を再照合する。
