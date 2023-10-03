use std::env;

use tracing::Level;

pub fn get_tracing_mode() -> Level {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 && &args[1] == "--debug" {
        return Level::DEBUG;
    }
    Level::INFO
}
