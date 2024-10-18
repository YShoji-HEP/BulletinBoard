mod board;
mod bulletin;
mod server;
mod error;
use server::BBServer;
use std::sync::LazyLock;
use clap::Parser;

#[cfg(not(feature = "unix"))]
static LISTEN_ADDR: LazyLock<String> =
    LazyLock::new(|| std::env::var("BB_LISTEN_ADDR").unwrap_or("127.0.0.1:7578".to_string()));
#[cfg(feature = "unix")]
static LISTEN_ADDR: LazyLock<String> =
    LazyLock::new(|| std::env::var("BB_LISTEN_ADDR").unwrap_or("/tmp/bb.sock".to_string()));

static TMP_DIR: LazyLock<String> =
    LazyLock::new(|| std::env::var("BB_TMP_DIR").unwrap_or("./bb_tmp".to_string()));
static ACV_DIR: LazyLock<String> =
    LazyLock::new(|| std::env::var("BB_ACV_DIR").unwrap_or("./bb_acv".to_string()));
static TOT_MEM_LIMIT: LazyLock<u64> = LazyLock::new(|| {
    parse_size::parse_size(std::env::var("BB_TOT_MEM_LIMIT").unwrap_or("1GiB".to_string())).unwrap()
});
static FILE_THRETHOLD: LazyLock<u64> = LazyLock::new(|| {
    parse_size::parse_size(std::env::var("BB_FILE_THRETHOLD").unwrap_or("1MiB".to_string()))
        .unwrap()
});
static LOG_FILE: LazyLock<String> =
    LazyLock::new(|| std::env::var("BB_LOG_FILE").unwrap_or("./bulletin-board.log".to_string()));

#[derive(Parser)]
struct Args {
    /// Log to stdout
    #[arg(short, long)]
    debug: bool
}


fn main() -> Result<(), Box<dyn std::error::Error>> {
    LazyLock::force(&LISTEN_ADDR);
    LazyLock::force(&TMP_DIR);
    LazyLock::force(&ACV_DIR);
    LazyLock::force(&TOT_MEM_LIMIT);
    LazyLock::force(&FILE_THRETHOLD);
    LazyLock::force(&LOG_FILE);

    let args = Args::parse();

    let mut server = BBServer::new(args.debug)?;
    server.listen()?;
    Ok(())
}
