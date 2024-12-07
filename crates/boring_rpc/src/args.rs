use clap::Parser;

/// The compiler for Boring RPC
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// The entry point schema file
    #[arg(short, long)]
    pub(crate) entry_point: String,
    #[arg(short, long)]
    /// Output directory
    pub(crate) out_dir: String,
}
