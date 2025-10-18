mod commands;
mod naming;
mod produce;
mod config;
mod gitmeta;

mod directory_tree;




// Use Clap for command line argument parsing
use clap::{Parser,Subcommand};

#[derive(Parser)]
#[command(name = "cpthastings")]
#[command(about = "A command line tool for managing and automating lab results and simulations using Github and Slurm.")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Create a new repository
    Init {
        /// Name of the repository to create
        #[arg(short, long)]
        name: String,
         // use git defaults to true
        #[arg(long, default_value_t = true)]
        git: bool,
        // add a README file
        #[arg(long, default_value_t = true)]
        readme: bool,
        // authors either a string or a vector of strings, default to empty vector
        #[arg(long)]
        authors: Vec<String>,
        // force overwrite if repo already exists
        #[arg(short='f', long, default_value_t = false)]
        force: bool,
        // add_test flag to create a test directory
        #[arg(long, default_value_t = false)]    
        add_test: bool,
        //add_docs flag to create a docs directory
        // julia uses Documenters.jl for documentation
        // what should I use for rust? mdBook? Doctave?
        #[arg(long, default_value_t = false)]
        add_docs: bool,
        /* template to use for the repository, it should be a JSON string representing the directory structure
        If not provided, use DEFAULT_TEMPLATE*/
        #[arg(long)]
        template: Option<String>,   
    },
}



impl Commands {
    pub fn run(&self) {
        match self {
            Commands::Init { name, git, readme, authors, force, add_test, add_docs, template } => {
                commands::CMDInit::run_init(name, *git, *readme, authors.clone(), *force, *add_test, *add_docs, template.clone());
            }
        }
    }
}