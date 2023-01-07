use clap::{
    Args,
    Parser, 
    Subcommand
};

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct NpmArgs {
    #[clap(subcommand)]
    pub command_type: CommandType
}

#[derive(Subcommand, Debug)]
pub enum CommandType {
    /// Initialize a project
    Init(InitCommand),
    /// Install one or more packages
    Install(InstallCommand),
    /// Remove one or more packages
    Remove(RemoveCommand),
    /// Run a command from the project
    Run(RunCommand)
}

#[derive(Debug, Args)]
pub struct InitCommand {
}

#[derive(Debug, Args)]
pub struct InstallCommand {
    /// The packages to install
    pub packages: Vec<String>
}

#[derive(Debug, Args)]
pub struct RemoveCommand {
    /// The packages to remove
    pub packages: Vec<String>
}

#[derive(Debug, Args)]
pub struct RunCommand {
    /// The command to run
    pub command: String
}
