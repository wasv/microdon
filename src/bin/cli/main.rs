extern crate clap;
extern crate diesel;
extern crate pretty_env_logger;
#[macro_use]
extern crate log;

use clap::{App, SubCommand};
use futures::executor::block_on;
use microdon::connection;
use microdon::handlers;

async fn execute_command(subcommand: Option<&str>) -> Result<(), String> {
    let db = connection::DbConn(connection::init_pool().get().unwrap());
    let activity = serde_json::from_reader(std::io::stdin().lock()).unwrap();

    match subcommand {
        Some("inbox") => handlers::inbox::create(db, activity).await.map(|a| {
            info!("Activity:\n {:#?}", a);
        }),
        Some("outbox") => handlers::outbox::create(db, activity).await.map(|a| {
            info!("Activity:\n {:#?}", a);
        }),
        _ => Err("Invalid command.".to_string()),
    }
}

fn main() {
    pretty_env_logger::init();
    let args = App::new("microdon")
        .subcommands(vec![
            SubCommand::with_name("inbox").about("Add activity to inbox"),
            SubCommand::with_name("outbox").about("Add activity to outbox"),
        ])
        .get_matches();

    block_on(execute_command(args.subcommand_name())).unwrap_or_else(|e| error!("{}", e))
}
