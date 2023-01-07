use crate::{cmd::args::RemoveCommand, core::manifest::Manifest};

/// This function handles the `remove` command.
pub async fn handle_remove(args: RemoveCommand) {
    // Make sure there are packages to remove
    if args.packages.len() == 0 {
        println!("Please specify packages to remove");
        return;
    }

    let current_dir = std::env::current_dir().unwrap();
    let manifest_path = current_dir.as_path().join("package.json");

    let mut manifest = Manifest::from_path(&manifest_path)
        .expect("Couldn't load `package.json`. Did you initialize the project?");

    // Remove all packages
    for i in args.packages {
        manifest.remove_dependency(&i);
    }

    // Save the manifest
    manifest.save(&manifest_path).expect("Failed to save `package.json`");

}
