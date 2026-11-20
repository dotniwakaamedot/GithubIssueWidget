# 基盤ウィンドウ中央位置の独立照合

- 判定: **記録済み数値の算術と対象版について部分確認**。Issue #2 の中央表示・実機動作を承認するものではない。作者 `/root` と確認 `/root/window_observation_review` は別担当。今回の記録の数値整理には blocking 指摘なし。以下の DPI・証拠の限界は非 blocking の注意として引き継ぐ。
- 依頼 `window-center-review-request-20261119.md` SHA256 `68B474DCFF2CD9C77895BCA85C23C720884003211B096E67BD2A3DD6AB019011`。指定の `window-center-result-20261119.md` の SHA256 `8E8DB419BC14053356E5F43C260A89308656557C1BED5C111C8627A9142B7047` と `window-center-20261119.ps1` の SHA256 `AA5A73C8968A9F234AB140B9511EF55ACCA3E1F17263CD4B1CABD06EEED943DA` は依頼に一致。対象 release exe の現在の SHA256 `472DEED04C601BE378C8C8B471F332F98C4043C82A0174FAA5F246D3D5898DC7` は結果記載に一致する。
- 対象 worktree HEAD `59578f8a4a775b4bd66c6acdde318b2e934abc08`、通常の未追跡を含む `git status --short --untracked-files=all` は空。基準 `a4ac97286785a2a5ba0d070a909b58003fc49ff1` との差分は README 変更と widget 16ファイル追加で削除なし。ignored 生成物全体の所有・内容は対象外。

## 数値と Issue #2 の意味

- 結果に記載された DWM 拡張フレーム `(1477,796)-(2379,1318)` の中心は `(1928,1057)`、クライアント原点 `(1478,797)` と幅900・高さ520から計算した中心も `(1928,1057)`。作業領域 `(0,0)-(3840,2112)` の中心 `(1920,1056)` との差は、記載された数値上はそれぞれ `(8,1)` である。計算自体は正しい。
- スクリプトは `ClientToScreen`、`GetClientRect`、`DwmGetWindowAttribute` 属性9、`GetMonitorInfo` の返り値を確認しており、結果の項目と対応する。観測値・exit 0 は作者の会話内ツール結果を転記した記録に依拠し、独立した生ログはない。本担当は元の実行結果を別途再生・再実行していない。
- **非 blocking の注意:** スクリプトは当該時点の対象 DPI や観測スレッドの DPI awareness を記録していない。[Microsoft の GetWindowRect 文書](https://learn.microsoft.com/windows/win32/api/winuser/nf-winuser-getwindowrect) は DWM 拡張フレームが DPI 調整されないことを示し、[高 DPI の説明](https://learn.microsoft.com/en-gb/windows/win32/hidpi/high-dpi-desktop-application-development-on-windows) はスレッドの DPI context による一部 Win32 API の座標仮想化に注意を求める。このため、異なる API の値を実物理座標として比較できる前提は、同時刻の DPI/context と座標系の確認が必要。前の別実行で DPI 96 の記録があるが、今回と同一時点の証拠ではない。現時点では `(8,1)` は記録された返却値の差として扱い、その原因や他環境での物理的な差と断定しない。
- Issue #2 は「中央表示」を要求するが許容誤差は本文にない。設定 `center: true` は確認したものの、記録された差だけから仕様違反や適合を断定できないとする結果の結論は妥当。対象 exe が同じ SHA で現在存在することと、実行時のソース全体が当該 HEAD からビルドされたことは別であり、後者の独立した同時証拠は確認していない。

## 未達と確認範囲

Issue #2 の表示、移動、サイズ変更、最小制約、最前面切替、許可外 URL 拒否、開発起動・release build の対象版に結び付く証跡は今回の記録だけでは完了しない。画面の実際の可視性・非遮蔽、再起動間の変化、他モニター・DPI は未確認。実行・UI・プロセス・GitHub は操作せず、結果・スクリプト・exe の SHA、Issue #2、対象 config と worktree を読み取った。

## 今回の失敗と教訓

1. 依頼指定のスクリプト名を確認せず `window-center-observation-20261119.ps1` を推測し、絶対パスで `Get-Content` したところ PathNotFound が1回発生した。正しい `window-center-20261119.ps1` は依頼本文を読み直し、`Test-Path` で実在確認してから読取・SHA 照合した。過去に親 checkout と worktree の相対パス取違えが独立調査された履歴があり、今回も同じ PathNotFound 症状だが直接の原因は推測したファイル名である。ユーザー指定の失敗回数の扱いは親が判断する。以後、依頼本文から正確な名前を取得し存在確認してから開く。
