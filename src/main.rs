mod core;
mod cmd;

use cmd::args::NpmArgs;
use clap::Parser;
use cmd::handle_command;

#[tokio::main]
async fn main() {
    let args = NpmArgs::parse();

    handle_command(args.command_type).await;
}
