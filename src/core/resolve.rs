use std::collections::HashMap;

use super::registry::Registry;
use super::registry::model::VersionInfo;

use node_semver::{Version, Range};

struct Node<'a> {
    dependencies: &'a HashMap<String, Range>,
    resolved: HashMap<String, Version>
}

pub struct Resolver<'a, 'b> {
    registry: &'a Registry,
    package: &'b VersionInfo
}

impl Resolver<'_, '_> {
    /// Create a new `Resolver`, that works with
    /// a certain `registry`.
    pub fn new<'a, 'b>(registry: &'a Registry, package: &'b VersionInfo) -> Resolver<'a, 'b> {
        Resolver {
            registry,
            package
        }
    }

    /// For a number of `VersionInfo`, select only those 
    /// which lay in a certain (`semver`) `Range`.
    pub fn get_matching_versions<'a>(versions: &'a HashMap<Version, VersionInfo>, range: &Range) -> Vec<&'a VersionInfo> {
        let mut matching = Vec::new();
        for (version, pkg) in versions {
            if version.satisfies(range) {
                matching.push(pkg);
            }
        }

        matching
    }
}
