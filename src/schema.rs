table! {
    actors (id) {
        id -> Varchar,
        username -> Text,
        profile -> Varchar,
    }
}

table! {
    followers (actor) {
        actor -> Varchar,
        since -> Timestamp,
    }
}

table! {
    following (actor) {
        actor -> Varchar,
        since -> Timestamp,
    }
}

table! {
    inbox (id) {
        id -> Varchar,
        payload -> Nullable<Json>,
    }
}

table! {
    outbox (id) {
        id -> Varchar,
        actor -> Varchar,
        payload -> Nullable<Json>,
    }
}

joinable!(followers -> actors (actor));
joinable!(following -> actors (actor));
joinable!(outbox -> actors (actor));

allow_tables_to_appear_in_same_query!(actors, followers, following, inbox, outbox,);
