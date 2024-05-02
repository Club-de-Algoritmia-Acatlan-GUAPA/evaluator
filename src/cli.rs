use std::env;

use tracing::Level;

pub fn get_tracing_mode() -> Level {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 && &args[1] == "--debug" {
        return Level::DEBUG;
    }
    Level::INFO
}

pub fn init_tracing() {
    let tracing_mode = get_tracing_mode();
    tracing_subscriber::fmt()
        .with_max_level(tracing_mode)
        .init();
}
