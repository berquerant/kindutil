use clap::Parser;
use kindutil::cmd;
use std::path::PathBuf;

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    cmd::delete_cluster(args.dir.as_path(), &args.name)?;
    if args.delete {
        return Ok(());
    }

    cmd::create_cluster(
        args.dir.as_path(),
        &args.name,
        args.control_plane,
        args.worker,
    )
}

/// Create kind cluster.
///
/// See: https://github.com/berquerant/kindutil
#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    /// Cluster name
    #[arg(short, long, default_value = "kindutil")]
    name: String,
    #[arg(short, long, default_value_t = 1)]
    control_plane: u8,
    #[arg(short, long, default_value_t = 1)]
    worker: u8,
    /// Kind config directory
    #[arg(short, long, default_value = ".kindutil")]
    dir: PathBuf,
    /// Delete cluster only
    #[arg(long = "delete")]
    delete: bool,
}
