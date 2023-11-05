use std::fmt::Display;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Dependency {
    pub paths: Vec<ModPath>,
}

impl Dependency {
    pub fn new() -> Self {
        Self { paths: Vec::new() }
    }

    pub fn append(&mut self, other: &Self) {
        self.paths.extend(other.paths.clone());
    }

    pub fn include(&self, mod_path: &ModPath) -> bool {
        self.paths.iter().any(|path| path.include(mod_path))
    }

    pub fn contains_macro_dependency(&self, macro_dependency: &Dependency) -> bool {
        self.paths
            .iter()
            .any(|path| macro_dependency.paths.contains(path))
    }

    pub fn is_not_empty(&self) -> bool {
        !self.paths.is_empty()
    }

    pub fn require_all_macro(&self, root_crate_name: &str) -> bool {
        self.paths.iter().any(|path| {
            path.ident_list.len() == 2
                && path.ident_list[0] == root_crate_name
                && path.ident_list[1] == "*"
        })
    }
}

impl FromIterator<Dependency> for Dependency {
    fn from_iter<T: IntoIterator<Item = Dependency>>(iter: T) -> Self {
        let mut dep = Self::new();
        for v in iter {
            dep.append(&v);
        }
        dep
    }
}

impl FromIterator<ModPath> for Dependency {
    fn from_iter<T: IntoIterator<Item = ModPath>>(iter: T) -> Self {
        Self {
            paths: iter.into_iter().collect(),
        }
    }
}

impl IntoIterator for Dependency {
    type Item = ModPath;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.paths.into_iter()
    }
}

impl From<ModPath> for Dependency {
    fn from(path: ModPath) -> Self {
        Self { paths: vec![path] }
    }
}

impl From<String> for Dependency {
    fn from(path: String) -> Self {
        Self {
            paths: vec![ModPath {
                ident_list: vec![path],
            }],
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
pub struct ModPath {
    pub ident_list: Vec<String>,
}

impl ModPath {
    pub fn new() -> Self {
        Self {
            ident_list: Vec::new(),
        }
    }

    // pub fn path(&self) -> String {
    //     self.ident_list.join("::")
    // }

    pub fn join(&self, path: &str) -> Self {
        let mut ident_list = self.ident_list.clone();
        ident_list.push(path.to_string());
        Self { ident_list }
    }

    pub fn is_empty(&self) -> bool {
        self.ident_list.is_empty()
    }

    pub fn create_macro_path(&self, macro_name: &str) -> Self {
        Self {
            ident_list: vec![self.ident_list[0].to_string(), macro_name.to_string()],
        }
    }

    pub fn include(&self, other: &Self) -> bool {
        if self.ident_list.len() <= 1 || self.ident_list.len() - 1 != other.ident_list.len() {
            return false;
        }

        self.ident_list[0..self.ident_list.len() - 1]
            .iter()
            .zip(other.ident_list.iter())
            .all(|(a, b)| a == b)
    }
}

impl From<&String> for ModPath {
    fn from(path: &String) -> Self {
        Self {
            ident_list: vec![path.to_string()],
        }
    }
}

impl From<&str> for ModPath {
    fn from(path: &str) -> Self {
        Self {
            ident_list: vec![path.to_string()],
        }
    }
}

impl Display for ModPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.ident_list.join("::"))
    }
}
