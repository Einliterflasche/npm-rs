mod init;
mod install;
mod remove;
mod run;
pub mod args;

use args::CommandType;

use self::{install::handle_install, init::handle_init, remove::handle_remove};

/// This function delegates the command arguments to the 
/// specific handler function
pub async fn handle_command(command_type: CommandType) {
    match command_type {
        CommandType::Init(args) => handle_init(args),
        CommandType::Install(args) => handle_install(args).await,
        CommandType::Remove(args) => handle_remove(args).await,
        CommandType::Run(args) => ()
    };
}
