//! This `hub` crate is the
//! entry point of the Rust logic.

mod messages;
// mod sample_functions;
mod client;

// Uncomment below to target the web.
// use tokio_with_wasm::alias as tokio;

rinf::write_interface!();

// You can go with any async library, not just `tokio`.
#[tokio::main(flavor = "current_thread")]
async fn main() {
    // Spawn concurrent tasks.
    // Always use non-blocking async functions like `tokio::fs::File::open`.
    // If you must use blocking code, use `tokio::task::spawn_blocking`
    // or the equivalent provided by your async library.
    // tokio::spawn(sample_functions::communicate());
    // bulletin_board_client::set_addr("192.168.30.20:7578");
    tokio::task::spawn(client::set_addr());
    tokio::task::spawn(client::start_server());
    tokio::task::spawn(client::stop_server());
    tokio::task::spawn(client::relabel());
    tokio::task::spawn(client::status());
    tokio::task::spawn(client::log());
    tokio::task::spawn(client::view_board());
    tokio::task::spawn(client::get_info());
    tokio::task::spawn(client::remove());
    tokio::task::spawn(client::archive());
    tokio::task::spawn(client::load());
    tokio::task::spawn(client::list_archive());
    tokio::task::spawn(client::rename_archive());
    tokio::task::spawn(client::delete_archive());
    tokio::task::spawn(client::dump());
    tokio::task::spawn(client::restore());
    tokio::task::spawn(client::clear_log());
    tokio::task::spawn(client::reset_server());
    tokio::task::spawn(client::terminate_server());
    tokio::task::spawn(client::key_input());

    // Keep the main function running until Dart shutdown.
    rinf::dart_shutdown().await;
}
