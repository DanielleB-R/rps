table! {
    games (id) {
        id -> Int4,
        player_1 -> Int4,
        player_2 -> Int4,
        winner -> Nullable<Int4>,
        rounds -> Nullable<Int4>,
    }
}

table! {
    users (id) {
        id -> Int4,
        name -> Varchar,
        pronouns -> Varchar,
        age -> Int4,
        deleted -> Bool,
        username -> Varchar,
    }
}

allow_tables_to_appear_in_same_query!(
    games,
    users,
);
