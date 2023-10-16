// @generated automatically by Diesel CLI.

diesel::table! {
    locations (id) {
        id -> Int4,
        #[max_length = 255]
        state_name -> Varchar,
        #[max_length = 2]
        state_abbreviation -> Bpchar,
        #[max_length = 255]
        city -> Varchar,
    }
}

diesel::table! {
    posts (id) {
        #[max_length = 255]
        id -> Varchar,
        #[max_length = 100]
        title -> Varchar,
        image_url -> Varchar,
        date_picture -> Nullable<Timestamp>,
        #[max_length = 255]
        description -> Nullable<Varchar>,
        #[max_length = 100]
        family -> Nullable<Varchar>,
        #[max_length = 100]
        gender -> Nullable<Varchar>,
        #[max_length = 100]
        specie -> Nullable<Varchar>,
        location -> Int4,
        #[max_length = 100]
        locality -> Varchar,
        verified -> Bool,
        published_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::joinable!(posts -> locations (location));

diesel::allow_tables_to_appear_in_same_query!(locations, posts,);
