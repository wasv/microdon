extern crate clap;
extern crate diesel;
extern crate pretty_env_logger;
#[macro_use]
extern crate log;

use clap::{App, SubCommand};
use microdon::connection;
use microdon::handlers;

fn main() {
    pretty_env_logger::init();
    let args = App::new("myprog")
        .subcommands(vec![
            SubCommand::with_name("inbox").about("Add activity to inbox"),
            SubCommand::with_name("outbox").about("Add activity to outbox"),
        ])
        .get_matches();

    let db = connection::DbConn(connection::init_pool().get().unwrap());

    let activity = serde_json::from_reader(std::io::stdin().lock()).unwrap();

    match args.subcommand_name() {
        Some("inbox") => handlers::inbox::create(db, activity).and_then(|a| {
            info!("Activity:\n {:#?}", a);
            Ok(())
        }),
        Some("outbox") => handlers::outbox::create(db, activity).and_then(|a| {
            info!("Activity:\n {:#?}", a);
            Ok(())
        }),
        _ => Err("Invalid command.".to_string()),
    }
    .unwrap_or_else(|e| error!("{}", e))
}
