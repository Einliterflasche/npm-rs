use std::collections::HashMap;

use anyhow::{Error, bail};
use futures::future::try_join_all;
use serde_json::Value;
use thiserror::Error;
use reqwest::Client;

use self::res::DocJson;

mod res {
    use std::{collections::HashMap, str::FromStr};

    use node_semver::Range;
    use serde::Deserialize;

    #[derive(Deserialize)]
    pub struct DocJson {
        #[serde(rename = "dist-tags")]
        pub dist_tags: DistTags,
        pub versions: HashMap<String, PkgVersion>,
        pub name: String
    }

    impl DocJson {
        /// Get the latest version of a package
        pub fn get_latest_version(&self) -> Range {
            node_semver::Range::from_str(&self.dist_tags.latest).unwrap()
        }
    }

    #[derive(Deserialize)]
    pub struct DistTags {
        pub latest: String,
    }

    #[derive(Deserialize)]
    pub struct PkgVersion {
        _id: String,
        name: String,
        version: String,
        #[serde(default = "HashMap::new")]
        dependencies: HashMap<String, Range>,
        #[serde(rename = "devDependencies", default = "HashMap::new")]
        dev_dependencies: HashMap<String, Range>,
        dist: Dist
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
    pub async fn get_doc_json(&self, package: &String) -> Result<DocJson, Error> {
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
        let doc_json: DocJson = serde_json::from_value(body_values)?;

        return Ok(doc_json);
    }

    /// Calls `get_doc_json` for multiple packages.
    /// 
    /// Returns an `Error` if at least one call fails.
    pub async fn get_many_doc_json(&self, packages: Vec<&String>) -> Result<Vec<DocJson>, Error> {
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
