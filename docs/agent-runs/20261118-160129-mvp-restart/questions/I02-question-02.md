# I02 進捗・確認 02

- owner: /root/foundation、宛先: /root、/root/restart_planner
- 対象: tasks/I02/request-02.md
- リリースビルド `npm run tauri -- build --no-bundle` は Visual Studio 18 の開発環境を読み込んだ後に exit 0。開発起動 `npm run tauri -- dev` はセッション 92623 でビルド中。UI の実動作はまだ確認していない。
- 失敗: TypeScript で非公開 ResizeDirection 型を import してビルドが一度失敗し、引数型から抽出して修正した。`rg` に Windows 非対応の引数中 glob を指定して一度検索失敗し、-g に修正。初回 cmd.exe の Visual Studio 環境設定で引用符が合わず一度失敗し、PowerShell で環境を取込む方法に修正。リリースビルドがアイコン不足で一度失敗し ICO を用意して成功。いずれも同種3回は未発生。
- 次の手順: 開発起動の完了を確認し、可能な範囲で Windows ウィンドウ操作を実測、未確認項目と結果を確定する。