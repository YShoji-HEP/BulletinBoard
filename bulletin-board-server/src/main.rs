use bulletin_board_server::{BBServer, ServerOptions};
use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    /// Log to stdout
    #[arg(short, long)]
    debug: bool,
    /// Log level [0: No logging, 1: Error, 2: +Warn, 3: +Notice (default), 4: +Info, 5: +Debug]
    #[arg(short, long)]
    log_level: Option<u8>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let mut opt = ServerOptions::new();

    if args.debug {
        opt.set_debug();
    }

    if let Some(log_level) = args.log_level {
        opt.set_log_level(log_level);
    }

    opt.load_options();

    let mut server = BBServer::new()?;
    server.listen()?;
    Ok(())
}
