extern crate futures;
extern crate glob;
extern crate hyper;
extern crate hyper_rustls;
#[macro_use]
extern crate lazy_static;
extern crate libc;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
#[macro_use]
extern crate structopt;
extern crate tokio_core;
extern crate toml;

mod ghclient;
mod statics;
mod structs;
#[macro_use]
mod syslog;

use statics::CONF_PATH;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
enum Command {
    /// Gets user public key
    #[structopt(name = "key")]
    Key {
        #[structopt(parse(from_str))]
        user: String,
    },
    /// Executes pam check
    #[structopt(name = "pam")]
    Pam,
    /// Check configuration
    #[structopt(name = "check")]
    Check {
        #[structopt(parse(from_os_str))]
        confpath: std::path::PathBuf,
    },
    /// Cleans caches up
    #[structopt(name = "cleanup")]
    CleanUp,
    /// Displays version details
    #[structopt(name = "version")]
    Version,
}

fn main() {
    let command = Command::from_args();

    use std::env;
    use std::process;

    match command {
        Command::Check { confpath } => match structs::Config::new(&confpath) {
            Ok(_) => process::exit(0),
            Err(_) => process::exit(11),
        },
        Command::Key { user } => {
            match structs::Config::new(&CONF_PATH).and_then(|conf| Ok(ghclient::GithubClient::new(&conf)))
                                                  .and_then(|client| client.print_user_public_key(&user))
            {
                Ok(_) => process::exit(0),
                Err(_) => process::exit(21),
            }
        }
        Command::Pam => match env::var("PAM_USER") {
            Ok(user) => match structs::Config::new(&CONF_PATH).and_then(|conf| Ok(ghclient::GithubClient::new(&conf)))
                                                              .and_then(|client| client.check_pam(&user))
            {
                Ok(true) => process::exit(0),
                Ok(false) => process::exit(1),
                Err(_) => process::exit(31),
            },
            Err(_) => process::exit(41),
        },
        Command::CleanUp => {
            match structs::Config::new(&CONF_PATH).and_then(|conf| Ok(ghclient::GithubClient::new(&conf)))
                                                  .and_then(|client| client.clear_all_caches())
            {
                Ok(_) => process::exit(0),
                Err(_) => process::exit(51),
            }
        }
        Command::Version => {
            println!("{}",
                     concat!(env!("CARGO_PKG_VERSION"),
                             include_str!(concat!(env!("OUT_DIR"), "/commit-info.txt"))));
        }
    };
}
