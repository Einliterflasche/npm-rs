
pub struct Package {
    name: String,
    version: Version,
    dependencies: Vec<Package>
}

pub struct Version {
    _semver: String
}

impl Version {
    pub fn new(semver: String) -> Version {
        Version { _semver: semver }
    }

    ///
    /// Check whether 
    /// 
    pub fn matches(&self, pattern: &str) -> bool {
        false
    }   
}
