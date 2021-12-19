use std::io;

use log::LevelFilter;
use message_io::network::Transport;
use simplelog::{ColorChoice, Config, TerminalMode, TermLogger};

use crate::networking::{Client, register_handler};

mod networking;

fn main() -> io::Result<()> {
    TermLogger::init(
        LevelFilter::Info,
        Config::default(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    ).unwrap();

    let address = "127.0.0.1:1234".parse().unwrap();
    let (mut client, listener) = Client::connect(Transport::FramedTcp, address)?;

    register_handler(&mut client, listener);

    Ok(())
}
