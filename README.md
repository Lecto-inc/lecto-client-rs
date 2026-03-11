# lecto client

Lecto API の Rust クライアントライブラリ。

## リリース手順

[cargo-release](https://github.com/crate-ci/cargo-release) を使用してリリースします。

```bash
# インストール (初回のみ)
cargo install cargo-release

# パッチバージョンを上げてリリース (例: 0.11.0 → 0.11.1)
cargo release patch --execute

# マイナーバージョンを上げてリリース (例: 0.11.0 → 0.12.0)
cargo release minor --execute

# メジャーバージョンを上げてリリース (例: 0.11.0 → 1.0.0)
cargo release major --execute
```

`--execute` を付けない場合はドライランになり、実際の変更は行われません。

`cargo release` は以下を自動で行います:
1. `Cargo.toml` のバージョンを更新
2. コミットを作成
3. Git タグを作成
4. リモートに push
