use std::collections::HashMap;

use anyhow::{Error, bail};
use futures::future::try_join_all;
use serde_json::Value;
use thiserror::Error;
use reqwest::Client;

use self::model::PackageInfo;

pub mod model {
    use std::collections::HashMap;

    use node_semver::{Range, Version};
    use serde::Deserialize;

    #[derive(Deserialize)]
    pub struct PackageInfo {
        #[serde(rename = "dist-tags")]
        pub dist_tags: DistTags,
        pub versions: HashMap<Version, VersionInfo>,
        pub name: String
    }

    #[derive(Deserialize)]
    pub struct DistTags {
        pub latest: String,
    }

    #[derive(Deserialize)]
    pub struct VersionInfo {
        _id: String,
        name: String,
        version: String,
        #[serde(default = "HashMap::new")]
        dependencies: HashMap<String, Range>,
        #[serde(rename = "devDependencies", default = "HashMap::new")]
        dev_dependencies: HashMap<String, Range>,
        dist: Dist
    }

    impl VersionInfo {
        /// Get a reference to the `HashMap` containing this `Package`'
        /// dependencies.
        #[inline]
        pub fn get_dependencies(&self) -> &HashMap<String, Range> {
            &self.dependencies
        }
    }

    #[derive(Deserialize)]
    pub struct Dist {
        shasum: String,
        #[serde(rename = "tarball")]
        tarball_url: String,
    }

    
}



#[derive(Error, Debug)]
pub enum RegistryError {
    #[error("invalid package")]
    InvalidPackage,
    #[error("invalid version")]
    InvalidVersion
}

pub struct Registry {
    url: String,
    client: Client
}

impl Registry {
    /// Create a new `Registry` using the specified `url`. 
    /// 
    /// To use the default npm registry use `Registry::default`.
    pub fn new(mut url: String) -> Registry {
        if url.ends_with("/") {
            url.pop();
        }

        Registry {
            url,
            client: Client::new()
        }
    }

    /// Create a new `Registry` using the default npm regisry.
    pub fn default() -> Registry {
        Registry::new("https://registry.npmjs.org".to_owned())
    }

    /// Fetch the doc.json for a package
    pub async fn get_doc_json(&self, package: &String) -> Result<PackageInfo, Error> {
        // The url at from which we should get the packages' doc.json
        let url = format!("{}/{}", self.url, package);

        // Make the GET request
        let res = self.client.get(url).send().await?;
        let body = res.text().await?;

        // Parse the doc.json
        let body_values: Value = serde_json::from_str(&body)?;

        // Check whether an error occured
        if let Some(_) = body_values.get("error") {
            bail!(RegistryError::InvalidPackage)
        }

        // If no error occured, parse doc.json
        let doc_json: PackageInfo = serde_json::from_value(body_values)?;

        return Ok(doc_json);
    }

    /// Calls `get_doc_json` for multiple packages.
    /// 
    /// Returns an `Error` if at least one call fails.
    pub async fn get_many_doc_json(&self, packages: Vec<&String>) -> Result<Vec<PackageInfo>, Error> {
        let iter = packages.iter().map(|i| {
            self.get_doc_json(*i)
        });

        try_join_all(iter).await
    }

    /// Get a HashMap representing the dependencies of the specified package
    pub async fn get_dependencies(&self, package: &String, version: &String) -> Result<HashMap<String, String>, Error> {
        let url = format!("{}/{}/{}", self.url, package, version);

        bail!(RegistryError::InvalidPackage)
    }
}
