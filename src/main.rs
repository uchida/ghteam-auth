#[macro_use]
extern crate clap;
use clap::{Arg, App, SubCommand};
extern crate toml;
extern crate reqwest;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate glob;

#[macro_use]
extern crate lazy_static;

extern crate libc;

mod structs;
mod ghclient;
mod statics;
use statics::CLIENT;

fn main() {

    let matches = App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .arg(Arg::with_name("v")
                 .short("v")
                 .multiple(true)
                 .help("Sets the level of verbosity"))
        .subcommand(SubCommand::with_name("key")
                        .about("get user public key")
                        .arg(Arg::with_name("USER")
                                 .required(true)
                                 .index(1)
                                 .help("user name")))
        .subcommand(SubCommand::with_name("pam").about("execute pam check"))
        .subcommand(SubCommand::with_name("refresh").about("refresh cache"))
        .get_matches();


    if let Some(matches) = matches.subcommand_matches("key") {
        CLIENT.print_user_public_key(matches.value_of("USER").unwrap())
              .unwrap();
    } else if let Some(_) = matches.subcommand_matches("refresh") {
        CLIENT.clear_all_caches().unwrap();
    } else if let Some(_) = matches.subcommand_matches("pam") {
        match std::env::var("PAM_USER") {
            Ok(user) => {
                if CLIENT.check_pam(&user).unwrap() {
                    std::process::exit(0);
                } else {
                    std::process::exit(1)
                }
            }
            Err(e) => println!("PAM_USER: {}", e),
        }
    }

}
