extern crate clap;
extern crate diesel;

use clap::{App, SubCommand};
use microdon::connection;
use microdon::handlers;

fn main() {
    let args = App::new("myprog")
        .subcommands(vec![
            SubCommand::with_name("create").about("Add create activity to outbox."),
            SubCommand::with_name("announce").about("Add announce activity to outbox."),
            SubCommand::with_name("delete").about("Add delete activity from outbox."),
            SubCommand::with_name("follow").about("Add follow activity to outbox."),
            SubCommand::with_name("rx-create").about("Add create actiivity to inbox."),
            SubCommand::with_name("rx-announce").about("Add announce activity to inbox."),
            SubCommand::with_name("rx-accept").about("Add accept activity to inbox"),
            SubCommand::with_name("rx-delete").about("Add delete activity to inbox"),
            SubCommand::with_name("rx-follow").about("Add follow activity to inbox"),
        ])
        .get_matches();

    let db = connection::DbConn(connection::init_pool().get().unwrap());

    let activity = serde_json::from_reader(std::io::stdin().lock()).unwrap();

    match args.subcommand_name() {
        Some("create") => handlers::inbox::create(db, activity),
        Some("announce") => handlers::inbox::announce(db, activity),
        Some("delete") => handlers::inbox::delete(db, activity),
        Some("follow") => handlers::inbox::follow(db, activity),
        Some("accept") => Err("Not Implemented".to_string()),
        Some("rx-create") => Err("Not Implemented".to_string()),
        Some("rx-announce") => Err("Not Implemented".to_string()),
        Some("rx-accept") => Err("Not Implemented".to_string()),
        Some("rx-delete") => Err("Not Implemented".to_string()),
        Some("rx-follow") => Err("Not Implemented".to_string()),
        _ => Err("Invalid command.".to_string()),
    }
    .unwrap_or_else(|e| eprintln!("Error: {}", e))
}
