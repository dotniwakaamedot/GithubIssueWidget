# 実 GitHub 読み取り専用テストの確認依頼 09

原文: 「現在の進行状況を確認しアプリの完成まで進行してください。一度の処理で2度同じ失敗をした時は停止してください」および「現在のリポジトリはダメですか」。
対象: widget/src-tauri/src/main.rs SHA256 021488C4C1615EBEFA3185EBDF024A43547657E357715957128F403541A67A97。追加した ignored テスト authenticated_github_readonly_repository_and_issues と、使用する get_api_response・fetch_pages を確認する。
予定: 現在リポジトリ dotniwakaamedot/GithubIssueWidget に限定し、gh auth token の標準出力を親プロセス変数へ非表示で受け取り、子プロセス環境に一時設定して Rust テストを単独実行。設定前の環境変数値を finally で復元し、トークンを出力・記録しない。cargo 出力を一時ログへ保存し、終了コードと要約のみ表示。実 GitHub の API は GET のみ。資格情報、本文、タイトルは出力しない。
通常テストのログ: 一時フォルダー giw-readonly-local-35c38a60f92b4b8b92da658fe9e5bf41.log、65 passed・10 ignored、URL 1 passed。
依頼: 意図せぬ書込み、トークン・Issue 情報の露出、URL 逸脱、認証失敗時の出力の危険と、結果の証明範囲を独立レビューして review-09.md に記録。実 GitHub・資格情報に触れない。コード・設定変更は禁止。同種失敗２回で停止。
