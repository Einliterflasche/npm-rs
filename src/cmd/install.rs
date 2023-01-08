use crate::{cmd::args::InstallCommand, core::{registry::Registry, manifest::Manifest}};

/// Handle the `install` command.
pub async fn handle_install(args: InstallCommand) {
    let n_pkg = args.packages.len();

    // Parse package.json
    let current_dir = std::env::current_dir().unwrap();
    let manifest_path = current_dir.as_path().join("package.json");
    let mut manifest = Manifest::from_path(&manifest_path)
        .expect("Couldn't load `package.json`. Did you initialize the project?");

    if n_pkg > 0 {
        let registry = Registry::default();

        let docs = registry
            .get_many_doc_json(args.packages.iter().collect())
            .await
            .expect("Failed to fetch package info");

        let deps = docs.iter().map(|i| {
            (&i.name, &i.dist_tags.latest)
        });

        manifest.add_dependencies(deps).expect("Failed to add dependencies");

        manifest.save(&manifest_path).expect("Failed to save `package.json`");
    }

    println!("Creating dependency tree...");
}
