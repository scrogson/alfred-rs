extern crate chrono;
#[macro_use]
extern crate clap;
extern crate slack;

mod bot;

use bot::Bot;
use clap::{App, Arg};
use slack::RtmClient;

fn main() {
    let matches = App::new("alfred")
        .author(crate_authors!())
        .version(crate_version!())
        .arg(Arg::with_name("token")
            .help("Slack token")
            .required(true)
            .takes_value(true)
            .short("t")
            .long("token"))
        .get_matches();

    let token = matches.value_of("token").unwrap();
    let mut handler = Bot { last_ping: None };
    let mut client = RtmClient::new(&token);

    match client.login_and_run::<Bot>(&mut handler) {
        Ok(_) => {}
        Err(err) => panic!("Error: {}", err),
    }
}
