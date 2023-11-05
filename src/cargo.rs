#![allow(dead_code)]

use serde::Deserialize;
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

#[derive(Debug, Deserialize)]
pub struct CargoToml {
    pub package: Package,
    pub dependencies: HashMap<String, Dependency>,
    #[serde(default)]
    pub dev_dependencies: HashMap<String, Dependency>,
    #[serde(default)]
    pub build_dependencies: HashMap<String, Dependency>,
    #[serde(default)]
    pub features: HashMap<String, Vec<String>>,
    #[serde(default)]
    pub workspace: Option<Workspace>,
    #[serde(default)]
    pub bin: Vec<Binary>,
}

#[derive(Debug, Deserialize)]
pub struct Package {
    pub name: String,
    pub version: String,
    pub authors: Option<Vec<String>>,
    pub edition: String,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub license: Option<String>,
    #[serde(default)]
    pub repository: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum Dependency {
    Version(String),
    Details(DependencyDetails),
}

#[derive(Debug, Deserialize)]
pub struct DependencyDetails {
    version: Option<String>,
    path: Option<String>,
    git: Option<String>,
    features: Option<Vec<String>>,
    optional: Option<bool>,
    default_features: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct Workspace {
    members: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct Binary {
    name: String,
    path: String,
}

pub fn parse_package_name(cargo_toml_path: &PathBuf) -> Result<String, Box<dyn std::error::Error>> {
    let cargo_toml_str = std::fs::read_to_string(cargo_toml_path.clone())?;
    let cargo_toml = toml::from_str::<CargoToml>(&cargo_toml_str)?;
    Ok(cargo_toml.package.name.replace("-", "_"))
}

pub fn find_cargo_toml(path: &Path) -> Option<PathBuf> {
    if path.join("Cargo.toml").exists() {
        return Some(path.join("Cargo.toml").to_path_buf());
    }

    if let Some(parent) = path.parent() {
        find_cargo_toml(parent)
    } else {
        None
    }
}
