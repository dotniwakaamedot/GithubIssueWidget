# I02 作業結果 01

- 担当: `/root/foundation`。依頼 `tasks/I02/request-02.md` SHA256 `CA64B05910EA68BB2731D38A0B108640A535EBCFFDB7353B1B536DB2B9B8AC84`。
- 作業場所: `D:\Projects\GithubIssueWidget-issue-2-20261118-160129`、branch `task/issue-2-foundation-20261118-160129`、HEAD `a4ac97286785a2a5ba0d070a909b58003fc49ff1`。対象は未コミット差分・未追跡を含む下記ハッシュ。主 checkout の製品ファイルは変更していない。削除対象なし。
- 判定: **実装途中・完了条件未達**。レビュー・追加調査が必要。ウィンドウ移動の自動操作は2回位置変化なし、原因未確定。3回目の同種失敗を避け試行を中止した。

## 変更と要件

- `widget/` に React / TypeScript / Tauri 2 / Rust の基盤、枠なしウィンドウ、最小・初期サイズ、中央、初期最前面、ドラッグ、リサイズ、最前面切替、最小化・閉じる操作を実装。接続・Issue ボードは範囲外で準備中と明示。
- capability はウィンドウ操作の個別権限と `opener:allow-open-url` の `https://github.com/` とその配下に限定。Opener の広いデフォルト権限は指定していない。他ホスト・GitHub Enterprise は現時点で対象外。制限の拒否動作は実行未検証。
- `README.md` に開発とリリースビルドの手順を追加。公開配布用のインストーラは対象外として `--no-bundle` の実行ファイルを生成した。

## 対象ファイル SHA256

```text
50BA7D33CBC285E07F54D7025C1BACFFE1CFD16A3118334A3C4BD28942BC3F43  README.md
768CBE8EEFE3381633E8698AD64F3C90DF67BDF9DABDAE238F5BE68E8BD3C640  widget/.gitignore
865DE72E838AB7726882C0F16CE24013D7809C51DB4656B686401972323B41EE  widget/index.html
E300C165CA2F81735D2073B19FB1ADC886EAB484F5D837D968C726AFB2EF3B38  widget/package-lock.json
38F2D50EFDE435E345D5D60C44E8846AC6F250276B30A347EC0A263EF55E11EF  widget/package.json
9CBDE07377ABDCE69BC9580ADB0315D52E2A09D6494A2005550A7E6733D9688B  widget/rust-toolchain.toml
B12DDABB69A903D065664324F8400FEF88E641803759D378CF6BB23B06C7BF3B  widget/src-tauri/Cargo.lock
8380153DCA2188496950E96882FFFFB0C42B70A657980343C67585F691279038  widget/src-tauri/Cargo.toml
8AF41C0062014FC594E4BB9899E60EE9930AB57D4CF0C73041FF557D6EC736E6  widget/src-tauri/build.rs
EE3B901B492DF38B46A5D7E7DEC5C7B1707472DFDE0AE8ECF08C6BEC529A0AA1  widget/src-tauri/capabilities/default.json
8873FAA6A22A407B470B9536F03D87E0A092F0ED3FDEF13FA3DFE5F1F70B44FD  widget/src-tauri/icons/icon.ico
D6CB80A41F03AF392AC6F34430FE5F27AF3ED16397828908BB4152D992197243  widget/src-tauri/src/main.rs
3F736678BC54BFD18A13669721D150F39ADD16978B5CBD5E10B26EB407457DC6  widget/src-tauri/tauri.conf.json
07318B40089C01EEB897EFDB5A933DD15982C51E51CDAEABC097AC219C867AFE  widget/src/main.tsx
335DAC73CE72A8D2B0F57E10D87903B0F5ECC09B35FB8CE75746643EA31BE024  widget/src/style.css
E31CD39572A32CC34B7E3DF7EE0E97992241CA17C9E135325359219674ABB660  widget/tsconfig.json
3CF43B5EFBCB9255F4867F2F89CDDEFB2FD901F9788227B12745D5588A5EBE81  widget/vite.config.ts
```

## 検証と証拠

- Windows 11 Home build 26300、Node v26.13.1、npm 11.20.0、Rust 1.90.0、Visual Studio 18 MSVC、WebView2 Runtime。`widget/` で `npm install --no-audit --no-fund` exit 0。警告: esbuild の install script が npm により未承認と表示。`npm run build` は型の修正後 exit 0。
- `widget/` で Visual Studio 18 の `VsDevCmd.bat -arch=x64` から環境変数を読み込み `npm run tauri -- build --no-bundle`、最終版 exit 0。`src-tauri/target/release/github-issue-widget.exe` SHA256 `6D322C27151564F706482E0EAE5870EB30B535B39E3623D5742B34206033D149`。bundle identifier の `.app` 末尾に関する macOS 向け警告あり。成果物 hash はファイル一覧とは別の生成物。
- 同様に `npm run tauri -- dev` は初回 Vite の Rust 出力監視が EBUSY で exit 1。`src-tauri` を監視対象から除外した後、debug 実行ファイル起動と Windows ウィンドウ表示を確認。閉じた後のコマンド exit 0。最終修正は Vite HMR で反映された。
- Computer Use で 902×522 の画面（外枠を含む表示寸法）を観測、修復後に日本語テキストを表示。最前面ボタン操作後にラベルが「最前面固定を解除」から「最前面に固定」に変化。OS の最前面フラグの独立測定は未実施。GitHub ボタン操作後 `GitHub - Brave` ウィンドウが現れたことを観測。スクリーンショットはその場で観察し保存していない。
- タイトル部ドラッグはオリジナル実装と `startDragging` へ変更後に各1回試し、どちらも画面上の位置が `1477,796` から変化しなかった。原因は未確定。再試行停止、移動の完了条件は未達。最小サイズ、中央位置、OS の最前面、全8方向のリサイズ、拒否 URL の動作は未確認。ブラウザ上の URL までは確認していない。
- 静的確認以外の自動テストは未追加・未実施。操作時の UI 環境と実寸はスケール・外枠の影響を含むため config 値の実証として扱わない。

## 環境の副作用・失敗・教訓

1. `rustup toolchain install 1.90.0 --profile minimal --no-self-update` exit 0 により共有 rustup に 1.90.0 を追加。既定の stable は変更していない。npm / Cargo は依存をダウンロードし、`widget/node_modules`、`widget/dist`、`widget/src-tauri/target`、`widget/src-tauri/gen` は ignored の生成物として残る。削除していない。
2. PowerShell から Python へのパイプで文字コードを明示せず、アプリソースと README の2回、日本語が `?` に変換された。`apply_patch` で双方を修復し、README を UTF-8 で読み戻し、アプリの画面・アクセシビリティ表示で日本語を確認した。この経路を再使用せず、同種3回目を防ぐ。
3. TypeScript が非公開 `ResizeDirection` の import を拒否、1回失敗後 `Parameters` から取得して解消。`rg` の Windows でのパス glob 誤用、`cmd.exe` 引用符誤用、Windows アイコン欠落による初回 release build 失敗は各1回で修正。前記 Vite EBUSY は監視除外で修正した。
4. 明示的な開始ドラッグへ変更後も位置が変わらず、UI 自動化・Tauri・OS の因果を区別できていない。再試行せず、別担当による原因分析と検証方針を管理へ依頼した。GitHub の書込み、push、PR、merge は行っていない。
