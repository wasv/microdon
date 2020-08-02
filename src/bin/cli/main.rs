#[macro_use]
extern crate log;

use clap::{App, SubCommand};
use dotenv::dotenv;
use microdon::connection;
use microdon::handlers;
use std::env;

#[tokio::main]
async fn main() {
    dotenv().ok();
    pretty_env_logger::init();

    let args = App::new("microdon")
        .subcommands(vec![
            SubCommand::with_name("create")
                .about("Create an acitivity from stdin")
                .subcommands(vec![
                    SubCommand::with_name("inbox").about("Add activity to inbox"),
                    SubCommand::with_name("outbox").about("Add activity to outbox"),
                ]),
            SubCommand::with_name("list").about("Lists all known objects"),
        ])
        .get_matches();

    let db = connection::DbConn(connection::init_pool().get().unwrap());

    match args.subcommand() {
        ("create", Some(subargs)) => {
            let activity = serde_json::from_reader(std::io::stdin().lock()).unwrap();

            match subargs.subcommand_name() {
                Some("inbox") => handlers::inbox::create(db, activity).await.map(|a| {
                    info!("Activity:\n {:#?}", a);
                }),
                Some("outbox") => handlers::outbox::create(db, activity).await.map(|a| {
                    info!("Activity:\n {:#?}", a);
                }),
                _ => Err("Invalid create command.".to_string()),
            }
            .unwrap_or_else(|e| error!("{}", e));
        }
        ("list", _) => {
            let actor_id =
                env::var("SELF").unwrap_or_else(|_| "http://localhost:8000/".to_string());
            if let Ok(res) = serde_json::to_string(&handlers::inbox::get_all(db, actor_id)) {
                println!("{}", res);
            }
        }
        _ => error!("Invalid command."),
    }
}
