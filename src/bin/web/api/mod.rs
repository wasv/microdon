pub mod inbox;

use microdon::connection;

pub struct State {
    db: connection::Pool,
}

impl State {
    pub fn new() -> State {
        State {
            db: connection::init_pool(),
        }
    }
}
