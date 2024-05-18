use std::collections::HashMap;

use lazy_static::lazy_static;
use primitypes::contest::Language;

use crate::configuration::{get_configuration, CmdStr, Settings};

pub const NSJAIL_DIR: &str = "/bin/nsjail";

lazy_static! {
    pub static ref CONFIGURATION: Settings =
        get_configuration().expect("Unable to get configuration");
    pub static ref RESOURCES: &'static str = CONFIGURATION.evaluator.resources.as_str();
    pub static ref PLAYGROUND: &'static str = CONFIGURATION.evaluator.playground.as_str();
    pub static ref LANGUAGE: &'static HashMap<Language, CmdStr> = &CONFIGURATION.language.0;
}
