

use cpthastings::Cli;
use clap::Parser;

fn main() {
    let cli = Cli::parse();
    
    // Execute the command
    cli.command.run();
}
