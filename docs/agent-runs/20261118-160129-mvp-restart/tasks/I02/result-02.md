# I02 追加検証結果 02

- 担当 `/root/foundation`。依頼 `tasks/I02/request-05.md` SHA256 `F01FFAE95DFD63405313F7C3BE3A0D3165A01FAEEA2E800994AB3F6036932EC9`。再開後、依頼の入力4件と対象17ファイルのハッシュを照合し、全件一致した。過去の `evidence-02/` は保護し変更せず、追加記録を `evidence-03/` に作成した。
- 作業場所 `D:\Projects\GithubIssueWidget-issue-2-20261118-160129`、branch `task/issue-2-foundation-20261118-160129`、HEAD `a4ac97286785a2a5ba0d070a909b58003fc49ff1`。製品に変更なし。`README.md` 変更、`widget/` 以下16ファイルが未追跡のまま。削除対象なし。
- **全 blocking 条件は未解決**。今回の結果は実装・動作の承認ではない。元の移動動作は完了条件未達とし、前2回の不成功から追加の移動操作を行っていない。

## B01〜B04

| 項目 | 実施と結果 | 失敗・未確認・残作業 |
| --- | --- | --- |
| B01 移動 | 過去2回の `sky.drag` の実行コード、window ID、前後の観測、版の限界を `evidence-03/drag-history-01.md` に記録し診断担当へ通知。 | 新規の移動操作なし。前回2回は座標変化なし。原因と実際の移動可否は未確定。独立診断後に管理が次の手順を判断。 |
| B02 窓寸法・最前面・リサイズ | 前回中断前の `evidence-02/ui-observation.json` を読み戻し、debug 実行時の画像領域 902×522、origin 1477,796、固定ボタン押下後のアクセシビリティ文字列変化を確認した。 | 外枠や DPI、画面全体を測定しておらず、900×520・中央表示の成功を示さない。OS 最前面フラグ、最小サイズ、リサイズは未実測。移動操作と紛らわしいドラッグを避け、未実施。 |
| B03 起動・リリース | 前回中断前の `dev.log`、`release.log`、`method.md` を読み戻し、Windows 11、Visual Studio 18 MSVC を用いた実行記録を確認。担当会話履歴の process 結果は dev 0、release 0。最終版のソース17ファイルと生成実行ファイルの hash を照合。 | ログは npm stderr を PowerShell が `NativeCommandError` の形式で装飾しているが、末尾に起動・build 完了がある。終了コード 0 はプロセス結果と method の記録に基づき、ログ単体には記録されていない。前の release exe hash は異なり、再ビルド後の今回の hash を対象とする。 |
| B04 外部 URL | 前回中断前に debug 版からリンクを押し、Brave のアドレスバー `github.com`、document URL `https://github.com/` を観測。Opener 2.7.3 と同じ glob 実装を使う限定的なハーネスは、許可2件・別ホスト等7件の文字列マッチで期待どおり、実行 exit 0。 | 実行時の Tauri IPC とブラウザの URL 正規化を含む不許可 URL の拒否は未実施。glob 単体の結果は実際の拒否成功を証明しない。外部状態を変更せず IPC ACL を試験する方法を別途設計する。 |

## 実行環境・方法・証拠

Windows 11 Home 10.0.26300 x64、Node 26.13.1、npm 11.20.0、Rust/Cargo 1.90.0、Visual Studio 18 Community MSVC。WebView2 157.0.4341.60 は環境調査による検出。新たな依存導入なし。前回中断前、`widget/` で `VsDevCmd.bat -arch=x64` の環境を呼び出しプロセス内に読み込み、`npm.cmd run tauri -- build --no-bundle` と `npm.cmd run tauri -- dev` を実行した。正確な PowerShell コマンド、ログ保存、glob ハーネスのコンパイル・実行コマンドと限界は `evidence-02/method.md` にある。生成・ignored の `node_modules`、`dist`、`target`、`gen`、Rust toolchain 1.90.0 は保護し削除していない。

証拠 SHA256（相対パスは `tasks/I02/` 基準）:

```text
49233DFDBC3A610E1FE99670607463C26E917F496022D9756EAC3A07E74B7B02  evidence-03/drag-history-01.md
255E5E54E9C0605756E84EFB08ED26853A9A0D74A857D4528CBFDAEE48AA134C  evidence-02/method.md
DB2DA247E47F0EC824231FC5493CBB498CE6B1C312940867F4105577B2F77AB3  evidence-02/dev.log
5F1C495A60F03CF4AF1590D01F03236FD092A89CF0DA61A7E1D71D6CF6AF39AD  evidence-02/release.log
0D6DE49C786A6ECEBF982B185CFCFEE211027438A0A4FFA6E92F1D2C3C84D9E8  evidence-02/scope-check.log
635DD2D3D5249B18C19423FA3BD13251BA79DDA8E16C03F494D45F0B08D313FF  evidence-02/scope_check.exe
5AEDDC6C00048AF19F4AE0D7A89D84311E93B45DEFECDBAD4DCF9A394B515F45  evidence-02/scope_check.pdb
2EED2808E8AE2CB3844DCB0F21FE817DF761BC3CE9C903EC21369729EC7CF443  evidence-02/scope_check.rs
10698CA329149C8068F716804D0A0BD23F7F435780766D98047A27F80CBE0E19  evidence-02/ui-observation.json
5E6877A9B2C89078BBDEA65033F8588D867E17A83689734170719880D08AE144  widget/src-tauri/target/release/github-issue-widget.exe（作業場所相対）
064907CEB4C92A78901E0094B47EF329B4330E771DA85160B3557637DA6D5845  widget/src-tauri/target/debug/github-issue-widget.exe（作業場所相対）
```

## 現在の製品ファイル SHA256

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

## 失敗と教訓

1. 前回中断前、ブラウザ前面時に Computer Use からウィンドウを閉じようとして `coordinate input geometry is unavailable` が1回発生。対象を前面化し再観測後、閉じる操作は成功した。
2. 文字破損2回・移動操作で位置変化なし2回は前回結果の履歴として維持。文字コード破損の経路と追加のドラッグ試行は行っていない。
3. 今回の再開後、担当による新たなコマンド失敗や仕様・参照不整合は確認していない。過去の未解決事項を解消済みとは扱わない。
