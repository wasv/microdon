table! {
    activities (id) {
        id -> Varchar,
        acttype -> Varchar,
        author -> Varchar,
        published -> Nullable<Timestamp>,
        object -> Varchar,
        contents -> Nullable<Json>,
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
        since -> Nullable<Timestamp>,
    }
}

table! {
    objects (id) {
        id -> Varchar,
        objtype -> Varchar,
        author -> Varchar,
        published -> Nullable<Timestamp>,
        contents -> Nullable<Json>,
    }
}

joinable!(activities -> actors (author));
joinable!(activities -> objects (object));
joinable!(objects -> actors (author));

allow_tables_to_appear_in_same_query!(activities, actors, followings, objects,);
