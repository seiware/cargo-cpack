# cargo-cpack

`cargo-cpack`は、競技プログラミングのために`bin`ディレクトリ内のRustソースコードを一つのファイルにまとめるRust製ツールです。

## Features

- 提出のために複数のソースファイルを一つのファイルにまとる。
- 依存関係やモジュールを処理する。
- `rustfmt`を使ったオプショナルなコードフォーマット機能。
- 必要なモジュールのコードのみを生成する機能。

## Installation

`cargo-cpack`をインストールするには、`cargo`のインストールコマンドを利用してください。Cargoがインストールされていることを確認し、以下のコマンドをターミナルで実行します：

```sh
git clone https://github.com/your-username/cargo-cpack.git
cd cargo-cpack
cargo install --path .
```

## Usage

`cargo-cpack`を使用するには以下のコマンドを実行します：

```sh
cargo cpack [options] <path>
```

### Arguments

- `<path>`: バンドル対象の.rsファイルのパス。

### Options

- `--format`, `-f`（optional）: 出力されるコードを整形します（デフォルトは`false`）。
- `--gen-code-only`, `-g`（optional）: `bin`ディレクトリで使用されているモジュールに必要なコードのみを生成します（デフォルトは`false`）。

使用例：

```sh
cargo cpack -f ./src/bin/my_solution.rs
```

## Output

このツールは標準出力にバンドルされたソースコードを出力します。シェルのリダイレクションを使って出力をファイルにリダイレクトすることができます：

```sh
cargo cpack -f ./src/bin/my_solution.rs > packed_solution.rs
```

## Example

実際の使用例はExample Projectを参照してください。
[Example project](./example/README-jp.md)
