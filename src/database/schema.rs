table! {
    users (user_id) {
        user_id -> Uuid,
        first_name -> Varchar,
        last_name -> Varchar,
        email -> Varchar,
        username -> Varchar,
        hash -> Varchar,
    }
}
