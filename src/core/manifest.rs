use std::fs;
use std::path::Path;
use std::io::Write;
use std::str::FromStr;
use std::collections::HashMap;

use node_semver::Range;
use serde::{Serialize, Deserialize};
use serde_json;
use anyhow::{Error, bail};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ManifestError {
    #[error("already depends on incompatible version of same package")]
    IncompatibleVersions
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Manifest {
    name: String,
    dependencies: HashMap<String, Range>,
    dev_dependencies: HashMap<String, Range>
}

impl Manifest {
    /// Create a new manifest with the default options
    pub fn new(name: String) -> Manifest {
        Manifest {
            name: name,
            dependencies: HashMap::new(),
            dev_dependencies: HashMap::new()
        }
    }
    
    /// Try to parse a file into a manifest
    pub fn from_path<P>(path: P) -> Result<Manifest, Error>
    where 
        P: AsRef<Path>
    {
        let data = fs::read_to_string(path)?;
        let manifest = serde_json::from_str(&data)?;
        Ok(manifest)
    }

    /// Save the manifest to 
    pub fn save<P>(&self, path: P) -> Result<(), Error> 
    where 
        P: AsRef<Path> 
    {
        let data = serde_json::to_string_pretty(&self)?;
        let mut file = fs::File::create(path)?;
        write!(file, "{}", data)?;
        Ok(())
    }

    /// Check whether a package is already a dependency.
    ///
    /// Does not check for dev dependencies (see `dev_depends_on`).
    pub fn depends_on(&self, package: &String) -> bool {
        self.dependencies.contains_key(package) 
    }

    /// Check whether a package is already a dev dependency.
    pub fn dev_depends_on(&self, package: String) -> bool {
        self.dev_dependencies.contains_key(&package)
    }

    /// Add a dependency. 
    /// 
    /// Fails if an incompatible version 
    /// of the package is already depended upon.
    pub fn add_dependency(&mut self, package: &String, range: &String) -> Result<(), Error> {
        let range = &Range::from_str(range).unwrap();
        if let Some(old_range) = self.dependencies.get(package) {
            if let Some(intersect) = old_range.intersect(&range) {
                self.dependencies.insert(package.to_owned(), intersect);
                return Ok(());
            } else {
                bail!(ManifestError::IncompatibleVersions)
            }
        }

        self.dependencies.insert(package.to_owned(), range.to_owned());

        Ok(())
    }

    /// Remove a dependency.
    pub fn remove_dependency(&mut self, package: &String) -> Option<()> {
        if !self.depends_on(package) {
            return None;
        }
        self.dependencies.remove(package);
        Some(())
    }

    /// Same as `add_dependency` but for dev dependencies
    pub fn add_dev_dependency(&mut self, package: &String, range: &Range) -> Result<(), Error> {
        let package = package.to_owned();

        if let Some(old_range) = self.dev_dependencies.get(&package) {
            if let Some(intersect) = old_range.intersect(range) {
                self.dev_dependencies.insert(package, intersect);
                return Ok(());
            } else {
                bail!(ManifestError::IncompatibleVersions)
            }
        }

        self.dev_dependencies.insert(package, range.to_owned());

        Ok(())
    }

    /// Calls `add_dependency` for each item of an `Iterator`.
    /// 
    /// Returns an error if at least one call fails.
    pub fn add_dependencies<'a, I>(&mut self, dependencies: I) -> Result<(), Error> 
    where
        I: Iterator<Item = (&'a String, &'a String)>
    {
        for (name, version) in dependencies {
            if let Err(e) = self.add_dependency(&name, &version) {
                bail!(e)
            }
        }

        Ok(())
    }
}
