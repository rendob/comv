use clap::Parser;

/// compress videos in a directory.
#[derive(Parser)]
pub struct Args {
    /// path of the directory that contains uncompressed videos
    pub input_dir: String,

    /// whether to compress videos in descendant directories
    #[arg(short, long)]
    pub recursive: bool,
}

pub fn parse_args() -> Args {
    Args::parse()
}
