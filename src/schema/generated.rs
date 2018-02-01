table! {
    tasks (id) {
        id -> Integer,
        name -> Text,
        create_date -> Date,
        due_date -> Nullable<Date>,
        priority -> Integer,
        user_id -> Integer,
        done -> Bool,
    }
}

table! {
    users (id) {
        id -> Integer,
        slack_id -> Text,
        name -> Nullable<Text>,
    }
}

allow_tables_to_appear_in_same_query!(tasks, users,);
