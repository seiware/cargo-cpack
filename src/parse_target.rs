use syn::{File, Item};

use crate::dependency::{Dependency, ModPath};

pub fn parse_target_dependencies(target_syntax: &File, package_name: &str) -> Dependency {
    target_syntax
        .items
        .iter()
        .filter_map(|item| match item {
            Item::Use(item_use) => Some(parse_use_tree(
                &item_use.tree,
                package_name,
                &ModPath::new(),
            )),
            _ => None,
        })
        .flatten()
        .collect()
}

fn parse_use_tree(tree: &syn::UseTree, package_name: &str, path: &ModPath) -> Dependency {
    match tree {
        syn::UseTree::Path(v) => {
            if path.is_empty() && v.ident.to_string() == package_name {
                parse_use_tree(&v.tree, package_name, &path.join(&v.ident.to_string()))
            } else if path.is_empty() {
                Dependency::new()
            } else {
                parse_use_tree(&v.tree, package_name, &path.join(&v.ident.to_string()))
            }
        }
        syn::UseTree::Name(v) => Dependency::from(path.join(&v.ident.to_string())),
        syn::UseTree::Rename(v) => Dependency::from(path.join(&v.ident.to_string())),
        syn::UseTree::Glob(_) => Dependency::from(path.join("*")),
        syn::UseTree::Group(v) => v
            .items
            .iter()
            .map(|item| parse_use_tree(item, package_name, path))
            .flatten()
            .collect(),
    }
}
