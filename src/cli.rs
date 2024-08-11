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
        dir_path: Option<String>,
    },
    IndexCorpus {
        dir_path: Option<String>,
    },
    ListCorpus {
        dir_path: Option<String>,
    },
    CreateVocabulary {
        dir_path: Option<String>,
    },
    Index {
        dir_path: Option<String>,
    },
    Status {
        dir_path: Option<String>,
    },
    Search {
        query: String,
        dir_path: Option<String>,
        top_n: Option<usize>,
    },
}
