// config.rs
// CLI options and logging setup
use lazy_static::lazy_static;
use log::{info, trace, warn};
use serde_derive::Deserialize;
use std::env::{set_var, var};
use structopt::StructOpt;

/// deciduously-com backend
#[derive(Debug, Deserialize, StructOpt)]
#[structopt(name = "deciduously-com")]
pub struct Opt {
    /// Server address
    #[structopt(short, long, default_value = "127.0.0.1")]
    pub address: String,
    /// Server port 0-65535
    #[structopt(short, long, default_value = "3000")]
    pub port: u16,
}

lazy_static! {
    pub static ref OPT: Opt = {
        let toml_opt = include_str!("assets/config.toml");
        if std::env::args().nth(2).is_some() {
            // If anything was passed, override config.toml
            Opt::from_args()
        } else {
            toml::from_str(toml_opt).unwrap()
        }
    };
}

/// Start env_logger
pub fn init_logging(level: u8) {
    // if RUST_BACKTRACE is set, ignore the arg given and set `trace` no matter what
    let mut overridden = false;
    let verbosity = if std::env::var("RUST_BACKTRACE").unwrap_or_else(|_| "0".into()) == "1" {
        overridden = true;
        "trace"
    } else {
        match level {
            0 => "error",
            1 => "warn",
            2 => "info",
            3 => "debug",
            _ => "trace",
        }
    };
    set_var("RUST_LOG", verbosity);

    pretty_env_logger::init();

    if overridden {
        warn!("RUST_BACKTRACE is set, overriding user verbosity level");
    } else if verbosity == "trace" {
        set_var("RUST_BACKTRACE", "1");
        trace!("RUST_BACKTRACE has been set");
    };
    info!(
        "Set verbosity to {}",
        var("RUST_LOG").expect("Should set RUST_LOG environment variable")
    );
}
