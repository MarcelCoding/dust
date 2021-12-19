use std::io;

use log::{info, LevelFilter};
use message_io::network::Transport;
use simplelog::{ColorChoice, Config, TerminalMode, TermLogger};

use crate::networking::{register_handler, Server};

mod networking;
mod package;

// Use Jemalloc only for musl-64 bits platforms
#[cfg(all(target_env = "musl", target_pointer_width = "64"))]
#[global_allocator]
static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;

fn main() -> io::Result<()> {
    TermLogger::init(
        LevelFilter::Info,
        Config::default(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    ).unwrap();

    let address = "0.0.0.0:1234".parse().unwrap();
    let transport = Transport::FramedTcp;

    let (mut server, listener) = Server::listen(transport, address)?;
    info!("Server is listening with {} on {}.", transport, address);

    register_handler(&mut server, listener);

    Ok(())
}
