mod cargo;
mod dependency;
mod format;
mod package;
mod parse_target;

use clap::Parser;
use package::{parse_mod_tree, ModTree};
use parse_target::parse_target_dependencies;
use proc_macro2::{Ident, TokenStream};
use quote::quote;
use std::path::Path;

use cargo::{find_cargo_toml, parse_package_name};

use crate::dependency::ModPath;

/// Parses the source code in the bin directory and bundles it into a single file.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to the source to be packed.
    path: String,

    /// Formatting output
    /// (default: false)
    #[clap(short, long, default_value = "false")]
    format: bool,

    /// Generated code only
    /// (default: false)
    #[clap(short, long, default_value = "false")]
    gen_code_only: bool,
}

fn main() {
    let args = Args::parse();
    match process(args) {
        Ok(()) => {}
        Err(e) => {
            println!("Error: {}", e);
        }
    }
}

fn process(args: Args) -> Result<(), Box<dyn std::error::Error>> {
    let path = Path::new(&args.path);

    // Cargo.tomlを探す
    let cargo_toml_path = find_cargo_toml(path).ok_or("Cargo.toml not found")?;
    // 変換対象のプロジェクト名を取得する
    let package_crate_name = parse_package_name(&cargo_toml_path)?;

    // pack対象のファイルを読み込む
    let target_str = std::fs::read_to_string(path)?;
    let target_syntax = syn::parse_file(&target_str)?;
    let target_dependencies = parse_target_dependencies(&target_syntax, &package_crate_name);
    // println!("{:#?}", target_dependencies);

    // Cargo.tomlの内容を元に、lib.rsを読み込む
    let lib_path = cargo_toml_path.parent().unwrap().join("src").join("lib");

    // lib.rsに記載されているmodの情報から再起的にModTreeを生成する
    let mut mod_tree = parse_mod_tree(
        &lib_path,
        ModPath::from(&package_crate_name),
        true,
        &target_dependencies,
    );
    mod_tree.name = syn::Ident::new(&package_crate_name, proc_macro2::Span::call_site());

    // TokenStreamを生成する
    let generated_token = gen_mod_token(&mod_tree, true);

    // macro_rules内の$crateの挙動が変わるので、$crateを$crate::package_crate_nameに置換する
    // (もっとスマートな書き方に直したい)
    let generated_token = generated_token.to_string().replace(
        "$ crate ::",
        format!("$crate::{}::", package_crate_name).as_str(),
    );

    // 対象ファイルの内容を出力する
    if !args.gen_code_only {
        println!("{}", target_str);
    }

    if args.format {
        // rustfmtを実行する
        let formatted_code = format::format_code(&generated_token)?;
        println!("{}", formatted_code);
    } else {
        // そのまま出力する
        println!("{}", generated_token);
    }

    Ok(())
}

fn gen_mod_token(mod_tree: &ModTree, is_root: bool) -> TokenStream {
    let mod_name = &mod_tree.name;
    let mod_tokens = if mod_tree.is_gen_target {
        mod_tree.tokens.clone()
    } else {
        TokenStream::new()
    };

    let mut child_tokens = TokenStream::new();
    for child in &mod_tree.children {
        child_tokens.extend(gen_mod_token(child, false));
    }

    if child_tokens.is_empty() && mod_tokens.is_empty() {
        return TokenStream::new();
    }

    let mut dummy_macro_tokens = TokenStream::new();
    if is_root {
        // マクロと同名の関数を定義する
        for macro_name in mod_tree.get_exported_macros() {
            let m = Ident::new(&macro_name, proc_macro2::Span::call_site());
            dummy_macro_tokens.extend(quote! {
                pub fn #m() {}
            });
        }
    }

    quote! {
        pub mod #mod_name {
            #dummy_macro_tokens

            #mod_tokens

            #child_tokens
        }
    }
}
