Backlog ドキュメント API 拡張のコーディングプラン
=================================================

作成日: 2026-09-20。検証完了日: 2026-09-21。対象は現在のチェックアウト。状態: 実装・検証完了。既存の完全 readonly ビルドの問題は末尾に記録。
以下はレビュー済みの実装計画。実装時の結果と計画との差分、および実 API の検証結果は末尾に記録する。

目的と範囲
----------

次の 4 API を共通ライブラリ、CLI (`blg`)、MCP サーバーで利用できるようにする。
一覧取得は既存実装を拡張し、残り 3 API は追加する。

| API | HTTP と入出力 | 現在との差分 |
| --- | --- | --- |
| [一覧取得](https://developer.nulab.com/ja/docs/backlog/api/2/get-document-list/) | `GET /api/v2/documents`。任意の `projectId[]`, `keyword`, `sort`, `order`, `count` と必須の `offset`。応答は配列 | 共通 API 層と CLI に存在。CLI の対象指定・応答モデルを拡張し、MCP に公開 |
| [タグ追加](https://developer.nulab.com/ja/docs/backlog/api/2/add-document-tag/) | `POST /api/v2/documents/:documentId/tags`。フォームの `tagNames[]`。応答はタグ配列 | 3 層すべてに追加 |
| [タグ削除](https://developer.nulab.com/ja/docs/backlog/api/2/remove-document-tag/) | `DELETE /api/v2/documents/:documentId/tags`。フォームの `tagNames[]`。成功は `204 No Content` | 3 層すべてに追加 |
| [件数取得](https://developer.nulab.com/ja/docs/backlog/api/2/count-document/) | `GET /api/v2/documents/count`。必須の `projectIdOrKey`。応答は `{"count":11}` | 3 層すべてに追加 |

一覧は参加プロジェクトだけが結果に含まれ、指定した未参加プロジェクトは除外される。
タグ操作と件数取得は対象プロジェクトへの参加が必要で、未参加の場合は API エラーになる。
件数 API は単一プロジェクトの件数を返し、キーワードやタグ、複数プロジェクトの検索条件を受け付けない。
タグ削除は指定ドキュメントからのタグの除去として扱う。

一覧のプロジェクトキー解決、全ページの自動収集、タグ検索、タグの一括置換、
ドキュメント本文の更新、コメント API は今回の範囲に含めない。
一覧は数値 ID、件数は ID またはキーという API の違いをヘルプにも明記する。

既存実装から再利用するもの
--------------------------

| ファイル | 再利用・修正の根拠 |
| --- | --- |
| `crates/backlog-document/src/api/list_documents.rs` | 複数 ID、keyword、sort/order、offset/count の型と `projectId[]` シリアライズが存在 |
| `crates/backlog-document/src/models/tag.rs` | `DocumentTag { id, name }` をタグ追加レスポンスに再利用 |
| `crates/backlog-api-core/src/request.rs` | DELETE を含め `to_form()` の結果をフォームボディとして送信できる |
| `crates/client/src/client.rs` | `execute_no_content()` が空の 204 応答を処理する。JSON 用 `execute()` と使い分ける |
| `backlog-mcp-server/src/document/bridge.rs` | 詳細取得で所属プロジェクトを特定し、許可確認してから変更する既存パターン |
| `backlog-mcp-server/src/access_control.rs`, `project_cache.rs` | `BACKLOG_PROJECTS` の許可チェックとキーから ID へのキャッシュ付き解決 |
| `crates/backlog-core/src/project_id_or_key.rs` | 数字の文字列も `EitherIdOrKey` になり得る。クエリ値は `Display` により単一文字列へ変換する |

HTTP 送信基盤や認証方式の変更は不要。
レビュー後の並列化では、すでに間接依存にある `futures-util` を MCP の直接依存に追加する。
追加のモックテストのため、CLI の dev-dependencies に既存 workspace の `wiremock` を追加する。

公開する操作と入力契約
----------------------

| 操作 | CLI | MCP ツールのベース名 | MCP 入力 |
| --- | --- | --- | --- |
| 一覧 | `blg document list` | `document_list_get` | `project_ids?: number[]`, `keyword?: string`, `sort?: created\|updated`, `order?: asc\|desc`, `offset?: u32`, `count?: u32` |
| 件数 | `blg document count --project-id <ID_OR_KEY>` | `document_count_get` | `project_id_or_key: string` |
| タグ追加 | `blg document tag-add <DOCUMENT_ID> --tag-name <NAME>` | `document_tag_add` | `document_id: string`, `tag_names: string[]` |
| タグ削除 | `blg document tag-remove <DOCUMENT_ID> --tag-name <NAME>` | `document_tag_remove` | `document_id: string`, `tag_names: string[]` |

MCP の実際のツール名には既存の `BACKLOG_PREFIX` が適用される。既定値は `backlog_`。
新規 CLI コマンドはいずれも `--json` に対応する。

- 一覧の `--project-id` / `-p` は任意・繰り返し可能な数値 ID とする。既存の単一 ID 指定はそのまま利用できる。
- CLI で ID 指定を省略した場合は参加プロジェクト横断検索になる。この挙動の拡張をヘルプと README に明記する。
- 一覧の `--keyword`, `--sort`, `--order`, `--offset`, `--count`, `--json` と既存の短縮オプションは保持する。
- `offset` は省略時に 0 を送る。負数は受け付けない。`count` は指定時 1〜100、省略時は API の既定値 20 を利用する。
- `sort` のヘルプから未対応の `title` を削除する。未指定の sort は勝手に決めず、order 未指定時も API に任せる（公式の既定値は desc）。
- MCP/ライブラリの `project_ids: []` は入力エラーにする。省略は全参加プロジェクト、空配列は指定ミスと区別する。
- `--tag-name` は繰り返し可能とし、カンマで自動分割しない。例: `--tag-name '設計,仕様' --tag-name '要確認'` は 2 タグ。
- タグ名配列は 1 要素以上、各要素は空文字・空白だけの文字列を不可とする。これは本クライアントの入力ルールであり、公式 API の必須条件としては記述しない。
- タグ名は空白だけかを判定する際にだけ trim を使い、送信値の前後空白・大小文字・重複・順序は自動変更しない。未記載の長さ・件数制限を作らない。
- 新規タグ操作のドキュメント ID は既存 `DocumentId::from_str` を使う。入力の前後空白は CLI/MCP で除去し、32 桁の小文字 hex という既存クライアントの制約を引き継ぐ。

出力は一覧がドキュメント配列、件数が `{"count": N}`、追加が API からのタグ配列、削除が `{"success": true}`。
削除の成功オブジェクトは CLI/MCP が生成するもので、Backlog API のレスポンスボディではない。
タグ追加レスポンスが付与済みタグ全体か今回追加分かは断定せず、受け取った配列を返す。
JSON 出力には進行メッセージを混ぜない。一覧の通常出力ではページ内件数を「総件数」と表示せず、
タグ操作へ ID をコピーできるよう完全なドキュメント ID とプロジェクト ID を表示する。
MCP は既存の `CallToolResult` / `ContentBlock::json` の経路を使い、独自のページングラッパーや推定 total は追加しない。

実装手順 1: 共通 API 層
-----------------------

対象は `crates/backlog-document` と `crates/backlog-api-client/src/lib.rs`。

1. 以下の型・メソッド・モジュールを追加する。

   | モジュール | パラメーター | 応答 / メソッド | feature |
   | --- | --- | --- | --- |
   | `api/get_document_count.rs` | `GetDocumentCountParams { project_id_or_key: ProjectIdOrKey }` | `GetDocumentCountResponse = DocumentCount`, `get_document_count()` | 読み取り |
   | `api/add_document_tag.rs` | `AddDocumentTagParams { document_id: DocumentId, tag_names: Vec<String> }` | `AddDocumentTagResponse = Vec<DocumentTag>`, `add_document_tag()` | `writable` |
   | `api/remove_document_tag.rs` | `RemoveDocumentTagParams { document_id: DocumentId, tag_names: Vec<String> }` | `RemoveDocumentTagResponse = ()`, `remove_document_tag()` | `writable` |

2. `DocumentCount { count: u32 }` を `models/document_count.rs` に追加する。Serde と optional な JsonSchema に対応させる。
   件数のクエリは `project_id_or_key.to_string()` または `ToFormParams` で文字列化する。
   `ProjectIdOrKey` 自体を Serde に渡して `EitherIdOrKey` が配列になる経路を使わない。
3. タグ操作のフォームは `#[form(skip)] document_id` と `#[form(array, name = "tagNames")] tag_names` を使う。
   マクロ生成の inherent `to_form()` だけで済ませず、`IntoRequest::to_form()` も実装して接続する。
   削除の `DocumentApi` メソッドは `execute_no_content()` を使用する。
4. 一覧とタグのパラメーターに共通に呼べる `validate()` を設ける。API メソッドから送信前に必ず検証し、
   MCP は許可プロジェクト解決などのリモート参照前にも同じ検証を呼ぶ。CLI の一覧は API 層の検証を利用する。
   エラーは既存 `backlog_core::Error::InvalidParameter` と API の変換経路を利用する。
   一覧は既存の公開 `offset: Option<u32>` と Builder を維持し、`None` から直接構築された場合も API メソッドで 0 を補完する。
5. `DocumentSortKey` / `DocumentOrder` に `Deserialize` と feature 付き `JsonSchema` を追加し、MCP の enum 制約に再利用する。
6. 一覧用 `Document` に `json: Option<serde_json::Value>` と `attachments: Vec<DocumentAttachment>` を追加する。
   欠落時の既定値を与え、既存の最小レスポンスも読めるようにする。
   公式例の `json` は JSON を含む文字列なので、文字列を内部オブジェクトへ勝手に再解釈しない。
   `Some` の値はその型のまま保持し、未提供の json は省略、attachments は欠落時に空配列とする。
   既存の `DocumentDetail` や作成・削除用 `DocumentResponse` に変更を波及させない。
7. `api/mod.rs`, `models/mod.rs`, 必要な再公開を更新する。
   facade から一覧の Params/Builder/Response、Document、DocumentTag、sort/order、件数・タグ関連型を利用できるようにする。
   新規 writable 型の再公開は既存の feature 条件に従う。

`Document` へのフィールド追加は JSON の読み取り互換性を維持する一方、
外部 Rust 利用者が構造体リテラルを作っている場合はソース変更が必要になる。
リポジトリ内のリテラル使用を検索して修正し、この点と JSON 出力の追加フィールドを変更説明に記載する。

実装手順 2: CLI
---------------

対象は `cli/src/commands/document/{args.rs,handler.rs,subcommands/*}`。

1. `List` の単一必須文字列を、既存フラグ名を保った任意の数値 ID リストへ変更する。
   オプション省略はライブラリの `project_ids=None` に変換し、空配列を明示送信しない。
2. `Count`, `TagAdd`, `TagRemove` の enum variant と dispatch を追加する。
   タグ操作の variant、import、handler は `document_writable` で gate する。
3. 件数処理を `subcommands/count.rs`、タグ処理を `subcommands/tags.rs` に置く。
   既存 get/tree/download/add/delete の大規模な配置変更は行わない。
4. count の `--project-id` は必須文字列で受け、trim 後に `ProjectIdOrKey` として parse する。
   一覧は数値のみであることを明記し、今回キーを ID に変換するための追加 API 呼び出しは導入しない。
5. 一覧の sort/order/count は clap の value parser でも早期検証する。
   タグ名の必須・繰り返し指定を設定し、共有バリデーションで空白だけの名前も検出する。
6. 通常表示と JSON を実装する。count の通常表示は件数、追加は戻ったタグ、削除は成功メッセージとする。
   API エラー時は非ゼロ終了し、成功したような出力を行わない。

使用例:

```sh
blg document list --project-id 123 --project-id 456 --keyword 設計 --offset 0 --count 20 --json
blg document list --keyword 設計 --json
blg document count --project-id MYPROJECT --json
blg document tag-add 01939983409c79d5a06a49859789e38f --tag-name 設計 --tag-name 要確認 --json
blg document tag-remove 01939983409c79d5a06a49859789e38f --tag-name 要確認 --json
```

実装手順 3: MCP とプロジェクト制限
----------------------------------

`document/request.rs`, `document/bridge.rs`, `server.rs` に 4 ツールを追加する。
タグ操作だけを `document_writable` で gate し、読み取り 2 ツールは feature 無効時も登録する。
count/offset の数値制約、project_ids とタグの最低要素数、sort/order の enum を JSON Schema と説明に示し、
実行時の検証も必ず行う。入力エラーは MCP の `invalid_params` に変換する。
共通 Error 変換の全体変更は避け、今回の bridge で validate/parse の失敗を `Error::Parameter` にマップする。

一覧には `AccessControl::scope_document_projects(requested_ids, client)` 相当の helper を追加する。
private の `allowed_projects` を外から直接読む設計にはせず、ここで次の規則を実装する。

| `BACKLOG_PROJECTS` の制限 | `project_ids` 入力 | 動作 |
| --- | --- | --- |
| 無効 | 省略 | プロジェクトフィルターなしで一覧 API を呼ぶ |
| 無効 | 非空配列 | 入力 ID をそのまま使う |
| 有効 | 省略 | 許可キーを既存 project cache で ID に解決し、非空の ID 配列として送る |
| 有効 | 非空配列 | 各 ID のアクセスを確認し、全件許可されている場合だけ入力 ID を送る |
| いずれも | 空配列 | API を呼ばず入力エラー |

- 指定 ID に未許可が混じる場合は全体を拒否する。黙って条件を減らさない。
- 許可キーの解決失敗はエラーとして止める。空の ID 配列やフィルター省略にフォールバックしない。
- 許可キーを重複排除して最大 8 件まで並列解決する。解決結果は入力キー順を保ち、ID の重複も除く。
- 許可キーの解決結果が空になった場合もエラーとし、全プロジェクト検索にしない。
- 制限が有効な場合はリクエストを絞ったうえで、応答中の異なる `project_id` をそれぞれ 1 回ずつ許可チェックする。
  未許可が含まれる場合は結果全体をエラーにし、部分データを返さない。
  応答からのフィルタリングをページングの実装に使わない。
- count は入力の `ProjectIdOrKey` の許可を確認してから count API を呼ぶ。
- タグ操作はまずローカル入力を検証する。制限が有効な場合は既存 get API で所属を特定し、許可確認後にのみ POST/DELETE を送る。
  詳細取得・許可確認に失敗した場合は更新を送らない。制限が無効な場合は詳細 GET を省略してタグ API を呼ぶ。
- 既存の client mutex を bridge で取得し、helper は `&BacklogApiClient` を受け取る。
  helper 内で同じ mutex を再取得しない。

テストは実際の env を並列に書き換えないようにする。
bridge テストはモジュール内で実行し、`AccessControl` に必要最小限の `#[cfg(test)] pub(crate)`
コンストラクター（許可キーを受け取り独立した cache を生成）を追加して設定を注入する。
プロトコルテストと CLI テストは子プロセス単位に env を渡す。
`BACKLOG_BASE_URL` と `BACKLOG_API_KEY` は必ずモック用の値で上書きし、
`BACKLOG_PREFIX` と `BACKLOG_PROJECTS` は各ケースで明示設定または `env_remove` する。
開発者シェルの許可プロジェクト設定や prefix を継承しない。

実装手順 4: 検証
-----------------

変更対象の振る舞いを検証するテストを先に追加し、その後に実装する。
CLI のコマンド定義をテスト専用 enum として複製しない。実際の型またはビルドされた `blg` を使う。

| 対象 | 必須の検証 |
| --- | --- |
| 一覧 API | 省略・複数 `projectId[]`、keyword、sort/order、必須 offset の補完、count の 1/100 と不正な 0/101、空 project_ids の拒否、空結果、既存 API エラー |
| 一覧応答 | 公式形の json 文字列・attachments・tags の保持、json/attachments 欠落時の互換性。json を追加でパースしない |
| 件数 API | 数値 ID とキー両方のクエリ、特に parse で `EitherIdOrKey` になる `"123"` が単一の `projectIdOrKey=123` になること。0 件、API エラー |
| タグ API | 正しい POST/DELETE のパスと Content-Type、複数の `tagNames[]` がクエリでなく body に入ること、日本語・空白・`+`, `&`, `,` のエンコード、空配列・空白タグの送信前拒否 |
| タグ応答 | 追加は `Vec<DocumentTag>`、削除は body なしの 204 が成功し JSON parse しない。削除の予期しない 200 や認証/権限/不存在などの API エラーは成功として扱わない |
| CLI | 旧 list 指定、複数 ID、無指定、enum・範囲のエラー、件数 ID/キー、複数タグ、カンマを含む単一タグ、JSON stdout の形、失敗時の exit code |
| MCP bridge | 上記スコープ表の全分岐、許可キー解決失敗、混在する未許可 ID、応答中の未許可 document。拒否した段階ごとの HTTP 呼び出し回数は下表で確認 |
| MCP protocol | 実 `tools/list` の新規ツール名、prefix、input schema。`tools/call` で一覧・件数・タグ追加・204 削除の応答を確認 |
| feature | document_writable 無効構成で list/count を利用でき、タグコマンド・ツールが公開されない。有効構成では両方公開される。完全 readonly の既存問題とは分離して確認 |

API のモックは既存 `wiremock` を使い、実際に受信した HTTP リクエストを検証する。
拒否ケースは、所属確認・許可キー解決の GET と、対象操作のリクエストを区別して検証する。

| 失敗する段階 | 対象操作の HTTP 呼び出し | 補助的な GET と結果 |
| --- | --- | --- |
| ローカル入力検証 | 一覧/count/タグ POST・DELETE すべて 0 件 | リモート GET も 0 件 |
| 一覧の事前許可確認・許可キー解決 | 一覧 0 件 | ID/キー解決の GET は必要に応じて発生。結果を返さない |
| 一覧応答中の未許可 document 検出 | 一覧 1 件 | 許可検査に必要な GET は発生し得る。部分結果を返さない |
| count の事前許可確認 | count 0 件 | 数値 ID の解決に GET が必要な場合がある |
| 制限有効時のタグの詳細取得・所属プロジェクト許可確認 | タグ POST・DELETE 0 件 | ドキュメント詳細 GET 1 件と、必要に応じたプロジェクト解決 GET |

未送信を要求する endpoint に `.expect(0)`、応答検査の一覧やタグの詳細取得には `.expect(1)` を設定する。
キャッシュの状態を各テストで固定し、プロジェクト解決 GET の期待数もそのケースに合わせる。
CLI は `env!("CARGO_BIN_EXE_blg")` を実行し、ダミー認証と MockServer URL を子プロセスに設定する。
CLI は現状 env 読み込みが clap parse より先なので、help/入力エラーのテストにもダミー env を与える。
CLI/MCP の新規プロセステストは既存 Tokio 依存の `tokio::process::Command::spawn()` を使い、
ハーネスが Child を保持する。stdout/stderr は並行して読み出し、MockServer の runtime をブロックしない。
MCP は既存 `tests/stdio_prefix_test.rs` の initialize → initialized → tools/list の手順を再利用する。
CLI の終了待ちと MCP の応答待ちに期限を設け、終了待ち・読み出しを含め無期限の待機を作らない。
タイムアウト時やテスト失敗時も、保持した Child を明示的に kill/wait して回収する。
このハーネスでは `Command::output()` やタイムアウトだけに依存する `spawn_blocking` は使わない。
実サービスの認証情報は使わない。

変更に直接関係する検証:

```sh
cargo test -p backlog-document
cargo test -p backlog-document --features writable,schemars
cargo test -p blg --all-features
cargo test -p mcp-backlog-server --all-features
cargo test -p blg --no-default-features --features document
cargo test -p blg --no-default-features --features document,document_writable
cargo test -p mcp-backlog-server --no-default-features --features issue_writable,git_writable,wiki_writable
cargo check -p backlog-api-client --no-default-features --features document
cargo check -p backlog-api-client --no-default-features --features document,document_writable,schemars
```

feature の各構成は独立した Cargo 呼び出しで実行する。
workspace 全体の all-features 実行による feature 統合だけで、無効時の検証を済ませない。
MCP の issue/git/wiki の書き込みを有効、document_writable だけを無効にした構成で、
ドキュメント list/count の登録・呼び出し成功と、tag-add/remove の非登録・呼び出し拒否を必須確認する。
この構成を「サーバー全体が読み取り専用」とは説明しない。

完全 readonly の基準確認として、実装前後に次も実行する:

```sh
cargo test -p mcp-backlog-server --no-default-features
```

現行の `issue/request.rs` と `issue/bridge.rs` には writable 型の無条件 import / TryFrom があり、
この構成は既存時点でビルドに失敗する可能性がある（計画作成時点ではソース確認のみ）。
再現した場合は既存問題として記録し、修正を別の作業項目に分ける。
その場合でも上記の document_writable 無効構成で今回の機能を検証し、
完全 readonly ビルドが通ったとは報告しない。今回の変更で増えた失敗は解消を必須とする。

最終的に `.github/workflows/ci.yml` と整合するチェックを行う:

```sh
cargo fmt --all -- --check
cargo clippy --all-features --all-targets -- -D warnings
cargo test --all-features --all-targets
cargo build --all-features --package blg
cargo build --all-features --package mcp-backlog-server
cargo doc --all-features --no-deps
cargo test --package backlog-api-client --no-default-features
cargo check --package blg --no-default-features
```

実装前に必要な feature 構成の現状を確認し、既存の失敗と新規変更による失敗を区別する。
無関係な既存不具合の修正に範囲を広げず、発生箇所・影響・未検証となる項目を報告する。
局所検証から全体検証へ一度広げ、変更や失敗がない限り同じチェックを繰り返さない。

実装手順 5: ドキュメントと完了条件
----------------------------------

- `cli/README.md`: 全コマンドの例、list の ID 指定省略・複数指定、count との検索条件の違い、JSON 出力、feature を記載する。
- `README.md`: 新規 MCP ツール、書き込み feature、入力制約、プロジェクト制限を追記する。
  ドキュメント系ツールは現在の 5 から 9（document_writable 無効では 3 から 5）になる。
- `API.md`: タグ追加・削除・件数取得を追加し、すでに実装済みだが表にない document add/delete も揃える。
  集計数は実際の一覧から再計算し、古い総数へ機械的に 3 を足さない。
- `project_structure.md`: ドキュメント API の説明と endpoint 数を実装に合わせる（現在の実装 6 + 新規 3 = 9）。
- `CLAUDE.md`: 変更に関係する機能・feature の説明だけを実装に合わせる。

完了条件は、4 操作の CLI/MCP 公開、既存 list の利用維持、範囲検証、
204 処理、MCP の許可プロジェクト制限、feature ごとの公開範囲、上記テストと説明の整合が確認できること。
実装は API 層 → CLI → MCP/access control → 全体検証・ドキュメントの順に進める。
各段階でその層のテストを追加して通し、後続段階へ進む。

実 API で確認する候補（実装着手の前提条件ではない）:

- 既に付与されたタグの再追加、未付与タグの削除、同じタグ名を重複送信した場合の挙動。
- タグ名の前後空白・大小文字の扱い、タグ追加応答が返す集合の意味。
- 件数がゴミ箱内ドキュメントを含むかどうか。

これらは公式ページだけでは確定しないため、推測で再試行・冪等性・補正処理を実装しない。
実環境での確認が必要になった場合は対象プロジェクト・ドキュメントを明確にし、
モックで確認した結果と実 API で確認した結果を分けて記録する。

Checker レビュー記録
---------------------

- 第 1 回: Checker が公式 4 API と現行ソースに照らしてレビュー。重大な API 設計・アクセス制御の指摘なし。
  中程度 1 件（document_writable 無効の独立した必須検証が不足）、軽微 1 件（応答検査後の拒否にも HTTP 0 件を要求）があり、両方反映した。
  任意改善の子プロセス環境変数の明示も採用した。
- 作成者の追加確認: 数値キーのクエリ文字列化、既存 feature 条件、CLI の env 読み込み順、
  子プロセスのタイムアウトと回収、project_ids の schema 制約を計画に織り込んだ。
- 第 2 回: 第 1 回の指摘と環境変数の改善が解消したことを Checker が確認。
  子プロセスの `Command::output()` と Child 保持の両立に関する軽微な指摘を受け、
  Tokio の spawn による保持・並行読み出し・期限付き待機・停止回収へ統一した。
- 第 3 回: Checker が最終修正を確認。子プロセス管理の矛盾は解消し、新たな問題なし。
  残る必須修正・任意改善はなく、実装着手可能と判定した。

計画作成中の検証は公式仕様・現行ソース・計画の照合と、文書の差分・参照先の静的確認。
上記に列挙した Cargo コマンドは実装時の実行計画であり、この計画作成中には実行していない。

実装時の記録
------------

- 共通 API、CLI、MCP とテストを実装した。新規の本番依存はなく、CLI のテスト依存に wiremock を追加した。
- 実装前の MCP `--no-default-features` を実行し、既存の issue writable 型の import とツール登録で失敗することを確認した。
- rmcp 3.2 の登録マクロはメソッドの cfg を考慮せず参照を生成するため、document の書き込み操作を
  impl 全体に feature 条件を付けた別ルーターにまとめ、feature 有効時だけ共通ルーターに合成した。
  既存の document add/delete もこのルーターへ移動した。他ドメインの既存問題は修正対象に含めていない。
- API と CLI のモックテスト、MCP bridge のアクセス制御テスト、および tools/list・tools/call のプロトコルテストが成功した。
- document_writable 無効構成の MCP でも、読み取りツールの利用とタグ操作の非公開・呼び出し拒否を確認した。
- ローカルのモック HTTP サーバーはサンドボックス内でポートを開けなかったため、許可されたサンドボックス外テストで実行した。
- 実装完了時の HTTP と MCP の動作確認はダミー認証を使ったローカルモックに対して実行した。
  その後、ユーザーの依頼と対象指定に基づいて実 API も検証した（次節）。

最終検証結果（2026-09-21）:

| 検証 | 結果 |
| --- | --- |
| `cargo fmt --all -- --check` | 成功 |
| `cargo clippy --all-features --all-targets -- -D warnings` | 成功、警告なし |
| `cargo test --all-features --all-targets` | ワークスペース全体で成功 |
| `cargo test -p backlog-document` および `--features writable,schemars` | 両構成で成功 |
| `cargo test -p blg --no-default-features --features document` および `--features document,document_writable` | 両構成で成功。タグコマンドの公開範囲も確認 |
| `cargo test -p mcp-backlog-server --all-features` | 成功。新規 bridge テスト 10 件、プロトコルテスト 3 件を含む |
| `cargo test -p mcp-backlog-server --no-default-features --features issue_writable,git_writable,wiki_writable` | 成功。document の書き込みツールの非登録・呼び出し拒否を確認 |
| `cargo check -p backlog-api-client --no-default-features --features document` および `--features document,document_writable,schemars` | 両構成で成功 |
| `cargo test -p backlog-api-client --no-default-features` | 成功 |
| `cargo check -p blg --no-default-features` | 成功 |
| `cargo doc --all-features --no-deps` | 成功 |
| `cargo build --all-features --package blg --package mcp-backlog-server` | 両バイナリの開発ビルド成功 |
| `cargo test -p mcp-backlog-server --no-default-features` | 既存問題で失敗。下記参照 |

完全 readonly の MCP は実装前後ともビルドに失敗した。残る原因は issue の writable 型の無条件 import / 変換実装と、
issue/git/wiki の cfg 無効メソッドを参照するツール登録であり、今回の document 拡張による新しい失敗はない。
document の書き込み登録は別ルーターへの移動で解消した。サーバー全体の完全 readonly ビルド成功は完了結果に含めない。
実サービス向けの既存 ignored テストは実行していない。

実 API 検証（2026-09-21）
------------------------

ユーザーが export した `BACKLOG_BASE_URL` / `BACKLOG_API_KEY` を継承して、
ビルド済みの `target/debug/blg` と `target/debug/mcp-backlog-server` を実行した。
認証情報の値はログ・検証記録に出していない。
書き込み先はユーザー指定の `OK1`（project ID: `57`）。
一時文書を作成し、その文書だけに検証用タグを追加・削除してから文書を削除した。

| 操作・条件 | 実 API の結果 |
| --- | --- |
| CLI 一覧取得 | プロジェクト無指定、複数 ID、sort/order、offset、count=100 が成功 |
| CLI 件数取得 | ID `57` とキー `OK1` の結果が一致。検証前は `{"count":1}` |
| MCP 一覧・件数取得 | 実 stdio の initialize / tools/list / tools/call が成功。CLI と結果が一致 |
| MCP アクセス制限 | `BACKLOG_PROJECTS=OK1` で ID 省略時も OK1 だけを取得し、別プロジェクト指定は拒否 |
| MCP 入力検証 | count=0 は InvalidParams（-32602）で拒否 |
| CLI タグ追加・削除 | 複数タグを追加し、詳細取得で反映を確認。削除後に元のタグ集合へ戻ったことを確認 |
| MCP タグ追加・削除 | `BACKLOG_PROJECTS=OK1` の状態で成功。詳細取得で追加・削除を確認 |
| タグ名のエンコード | 日本語・カンマ・プラス・アンパサンドを含むタグ名を、CLI と MCP の両方でそのまま保持 |
| キーワード付き一覧取得 | CLI・MCP とも HTTP 400。直接 HTTP GET でも再現（下記） |
| 後片付け | 一時文書の詳細取得は 404、キーワードなしの一覧からも消失。件数は検証前と同じ 1 件 |

キーワード検索の切り分けでは、同一環境・認証で次を直接送信した（認証パラメーターは省略）:

```text
GET /api/v2/documents?projectId[]=57&offset=0&count=1
  → HTTP 200
上記に keyword=test または keyword=ドキュメント を追加
  → HTTP 400
  → {"errors":[{"message":"","code":1,"moreInfo":""}]}
```

[公式仕様](https://developer.nulab.com/ja/docs/backlog/api/2/get-document-list/)には `keyword` が記載されている。
直接 HTTP GET でも失敗するため、今回の CLI/MCP の処理に固有の問題ではないことまでは確認できた。
この環境でキーワード付きリクエストが拒否される原因は未特定であり、検索の実 API 検証は成功扱いにしない。
検索条件を黙って省略する変更は行っていない。

タグ操作 4 回はすべて成功した。その後の検索確認と、検索を使った後片付け確認は上記 400 で失敗したが、
一時文書自体の削除は成功していた。キーワードなしの一覧・件数・詳細取得で削除済みを別途確認した。

コードレビュー後の改善（2026-09-23）
-----------------------------------

ユーザーが了承した方針に沿い、以下を適用した。今回の改善では実サービスへの操作は行っていない。

1. アクセス制限無効時のタグ操作は事前の詳細 GET を省略。有効時の所属確認と拒否動作は維持した。
2. タグ名の前後空白・大小文字などは変更しない。既存の exact-name 契約とテストを維持した。
3. 許可キーの重複を排除し、`futures-util` の buffered stream で最大 8 件まで並列解決する。
   許可キーの入力順を保ち、ID の重複を除去する。一部だけ成功した場合も失敗を返し、一覧 API は呼ばない。
   `futures-util` は既存 lockfile にあるバージョンを直接依存として使い、新しいパッケージやバージョンは追加していない。
4. 制限有効時の一覧応答を、異なる project ID ごとに 1 回検査する。重複する文書行と順序は保持し、
   未許可 ID が含まれる場合は引き続き応答全体を拒否する。
5. MCP の一覧パラメーターを Builder 経由で生成する。構造体リテラル利用者向けの API 層の offset=0 補完は維持した。
6. CLI 一覧の重複した validate 呼び出しを削除。MCP のプロジェクト解決前の検証は維持した。

回帰テストでは、無制限のタグ操作で詳細 GET が 0 回になることと、制限有効時の取得・拒否を確認する。
並列化は HTTP 応答を保留するテストサーバーで確認し、8 件が同時に開始され、応答前に 9 件目が開始されないこと、
重複キーを含む 17 プロジェクトの解決完了を検証する。キャッシュ再利用、部分解決失敗時の一覧未送信、
重複する文書行の保持と未許可文書の拒否も確認する。

検証結果:

| コマンド | 結果 |
| --- | --- |
| `cargo test -p mcp-backlog-server --lib document::tests` | 12 件成功。変更前には新規 2 テストの失敗を確認 |
| `cargo check --all-targets --all-features` | 成功 |
| `cargo test --all-features --all-targets` | ワークスペース全体で成功。MCP unit 45 件・document protocol 3 件を含む |
| `cargo clippy --all-features --all-targets -- -D warnings` | 成功、警告なし |
| `cargo fmt --all` | 成功 |

main の作業ツリーに適用し、コミットは作成していない。
