pub mod commands;
use clap;

#[derive(clap::Parser)]
#[clap(
    name = "searchine",
    version = "0.1.0",
    about = "A simple local search engine."
)]
pub struct SearchineCli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(clap::Subcommand)]
pub enum Commands {
    Init {
        /// Path to the directory to initialize as a searchine index.
        path: String,
    },
    IndexCorpus {
        dir_path: String,
    },
    ListCorpus {
        dir_path: String,
    },
    CreateVocabulary {
        /// Path to the directory containing the documents.
        #[clap(short, long)]
        path: String,
        /// Path to the output vocabulary file.
        #[clap(short, long)]
        output: String,
    },
    Index {
        path: String,
    },
}
