use clap::Parser;

/// compress videos in a directory
#[derive(Parser)]
struct Args {
    /// path of the directory that contains uncompressed videos
    input_dir: String,
}

fn main() {
    let args = Args::parse();
    println!("{:?}", args.input_dir);
}
