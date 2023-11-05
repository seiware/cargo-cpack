use std::path::PathBuf;

use proc_macro2::{Ident, Span, TokenStream};
use quote::TokenStreamExt;
use syn::{Item, ItemFn, ItemMacro};

use crate::dependency::{Dependency, ModPath};

#[derive(Debug)]
pub struct ModTree {
    pub name: Ident,
    pub path: PathBuf,
    pub tokens: TokenStream,
    pub children: Vec<ModTree>,
    pub is_gen_target: bool,
    macro_defines: Dependency,
}

impl ModTree {
    pub fn new(
        name: Ident,
        path: PathBuf,
        tokens: TokenStream,
        children: Vec<ModTree>,
        is_gen_target: bool,
        macro_defines: Dependency,
    ) -> Self {
        Self {
            name,
            path,
            tokens,
            children,
            is_gen_target,
            macro_defines,
        }
    }

    pub fn get_exported_macros(&self) -> Vec<String> {
        let child_macros = self
            .children
            .iter()
            .map(|child| child.get_exported_macros())
            .flatten();
        self.macro_defines
            .paths
            .iter()
            .map(|path| path.ident_list.last().unwrap().clone())
            .chain(child_macros)
            .collect()
    }
}

pub fn parse_mod_tree(
    path: &PathBuf,
    mod_path: ModPath,
    is_root: bool,
    target_dependencies: &Dependency,
) -> ModTree {
    let mut rs_path = path.clone();
    rs_path.set_extension("rs");
    let str = std::fs::read_to_string(rs_path).unwrap();
    let syntax = syn::parse_file(&str).unwrap();
    let mut tokens = TokenStream::new();
    tokens.append_all(syntax.items.iter().filter(|item| match item {
        Item::Mod(item_mod) => item_mod.semi.is_none() && !has_cfg_test(item_mod),
        Item::Fn(item_fn) => !has_test_attr(item_fn),
        _ => true,
    }));
    let children = syntax
        .items
        .iter()
        .filter_map(|item| {
            // mod m; 形式のみを対象とする
            let Item::Mod(item_mod) = item else {
                return None;
            };
            if item_mod.semi.is_none() {
                return None;
            }

            let item_ident_str = item_mod.ident.to_string();
            let mod_path_buf = if is_root {
                path.parent().unwrap().join(&item_ident_str)
            } else {
                path.join(&item_ident_str)
            };

            Some(parse_mod_tree(
                &mod_path_buf,
                mod_path.join(&item_ident_str),
                false,
                target_dependencies,
            ))
        })
        .collect();
    let macros_defines: Dependency = syntax
        .items
        .iter()
        .filter_map(|item| match item {
            Item::Macro(item_macro) => {
                if has_macro_export(item_macro) {
                    Some(mod_path.create_macro_path(&item_macro.ident.clone().unwrap().to_string()))
                } else {
                    None
                }
            }
            _ => None,
        })
        .collect();

    let is_gen_target = target_dependencies.include(&mod_path)
        || target_dependencies.contains_macro_dependency(&macros_defines)
        || macros_defines.is_not_empty()
            && target_dependencies.require_all_macro(&mod_path.ident_list[0]);

    ModTree::new(
        Ident::new(
            &path.file_name().unwrap().to_str().unwrap().to_string(),
            Span::call_site(),
        ),
        path.to_path_buf(),
        tokens,
        children,
        is_gen_target,
        macros_defines,
    )
}

fn has_macro_export(item_macro: &ItemMacro) -> bool {
    item_macro
        .attrs
        .iter()
        .any(|attr| attr.meta.path().is_ident("macro_export"))
}

fn has_test_attr(item_fn: &ItemFn) -> bool {
    item_fn
        .attrs
        .iter()
        .any(|attr| attr.meta.path().is_ident("test"))
}

fn has_cfg_test(item_mod: &syn::ItemMod) -> bool {
    item_mod.attrs.iter().any(|attr| {
        let syn::Meta::List(list) = &attr.meta else {
            return false;
        };

        list.path.segments.len() == 1
            && list.path.segments[0].ident == "cfg"
            && list.tokens.to_string().contains("test")
    })
}
