pub use gpt::Client;
pub use source::{Source, SourceMap};
use std::path::PathBuf;
use structopt::StructOpt;

mod gpt;
mod promts;
mod source;

#[derive(Debug, StructOpt)]
#[structopt(name = "gpt-doc-gen", about = "A CLI to document Rust code")]
#[structopt(rename_all = "kebab-case")]
struct Cli {
    /// Print the output to stdout instead of writing it to a file.
    #[structopt(short, long)]
    dryrun: bool,

    #[structopt(subcommand)]
    command: Commands,

    #[structopt(short, long, env = "GPT_DOC_GEN_API_KEY")]
    api_key: String,
}

#[derive(Debug, StructOpt)]
enum Commands {
    /// Add doc comments to a single file.
    DocumentFile {
        /// Path to the file to be documented.
        path: PathBuf,
        /// Optional output file. Defaults to overwriting the file provided with `path`.
        #[structopt(short, long)]
        output: Option<PathBuf>,
    },
    /// Add doc comments to all Rust files in a crate.
    DocumentCrate {
        /// Path to the crate to be documented, defaults to `src`
        #[structopt(short, long, default_value = "src")]
        path: PathBuf,
        /// Output directory to write the result to, defaults to `src.documented`.
        #[structopt(short, long, default_value = "src.documented")]
        output: PathBuf,
    },
}

impl Cli {
    async fn run(self) -> color_eyre::Result<()> {
        let mut client = Client::new(&self.api_key);

        match self.command {
            Commands::DocumentFile { path, output } => {
                let source = Source::from_file(&path)?;
                let document = client.document(&source).await?;
                let output = output.unwrap_or_else(|| path.clone());
                if self.dryrun {
                    println!("{}", document)
                } else {
                    std::fs::write(output, document.as_bytes())?;
                }
            }
            Commands::DocumentCrate { path, output } => {
                let map = SourceMap::from_root(path)?;
                let output_dir = output;

                for (path, source) in &map.sources {
                    let path = output_dir.join(path.clone());
                    let document = client.document(source).await?;
                    let prefix = path
                        .parent()
                        .expect("document-crate must be passed a directory");
                    std::fs::create_dir_all(prefix)?;
                    std::fs::write(path, document.as_bytes())?;
                }
            }
        }
        Ok(())
    }
}

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    let cli = Cli::from_args();
    cli.run().await
}
