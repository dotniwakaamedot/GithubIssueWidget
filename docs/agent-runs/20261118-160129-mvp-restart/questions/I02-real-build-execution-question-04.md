# I02 実ビルド npm 確定と release 実行中 04

- 担当 `/root/build_verification`、宛先 `/root`。npm receipt `tasks/I02/real-build-npm-log-approval-01.json` SHA256 `759AA996CE4FC9DC88573077F8C20C57432662F8C4DB6C28235A671C7559CE8D` を driver・両 log hash と独立担当情報に照合。親指示で同一 session 89568 に `publish` を1回送信し session exit 0。
- `npm.result.json` SHA256 `6F138DBD599471CA977587C9D2E19ED7CFF8DE9051513620702BAF0E782814C5` は実子 PID 5164、exit 0、stdout/stderr EOF `RanToCompletion`、receipt hash 一致を記録。npm の開始前後の元・コピー各17ファイルはすべて期待 hash と一致。
- release 開始前に依頼9参照、Node/npm/rustup/cargo/rustc proxy・VS・cmd など7ツールと専用 rustc/cargo の hash、元・コピー各17ファイルの実在/hash/hardlink、各 stage 出力・receipt 不存在、同一 driver 非競合を確認。D: 空き約125.5 GB・空きメモリ約39.2 GB・CPU 観測7%。他環境や画面作業の排他の完全な保証ではない。
- release を新規 session `83666` で開始。保護 metadata `release.vs-started.json` は VS bootstrap PID 31708・2026-11-18T20:57:26.9579847Z、`release.started.json` は Node PID 12016・20:57:27.7251057Z。終了コード・EOF・ログの検査は未完了、成功としない。failure / timeout / log review metadata は確認時に未作成。D: 空きは約124.7 GB。console にログ本文は出力しない。
- session を維持して結果を待ち、ログ確認待ち・fault・timeout の場合は依頼に従い親へ質問する。元製品・旧 scratch・GitHub・GUI は変更していない。今回の stage 実行・事前照合でコマンド失敗なし。
