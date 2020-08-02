table! {
    activities (id) {
        id -> Varchar,
        acttype -> Varchar,
        author -> Varchar,
        published -> Timestamp,
        object -> Varchar,
        contents -> Json,
    }
}

table! {
    actors (id) {
        id -> Varchar,
    }
}

table! {
    followings (target) {
        target -> Varchar,
        follower -> Varchar,
        since -> Timestamp,
    }
}

table! {
    objects (id) {
        id -> Varchar,
        objtype -> Varchar,
        author -> Varchar,
        published -> Timestamp,
        contents -> Json,
    }
}

joinable!(activities -> actors (author));
joinable!(activities -> objects (object));
joinable!(objects -> actors (author));

allow_tables_to_appear_in_same_query!(activities, actors, followings, objects,);
