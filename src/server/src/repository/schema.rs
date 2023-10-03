// @generated automatically by Diesel CLI.

diesel::table! {
    location (id) {
        id -> Int4,
        #[max_length = 100]
        state -> Varchar,
        #[max_length = 2]
        acronym -> Bpchar,
        #[max_length = 100]
        county -> Varchar,
    }
}

diesel::table! {
    post (id) {
        #[max_length = 255]
        id -> Varchar,
        #[max_length = 100]
        title -> Varchar,
        imageurl -> Varchar,
        datepicture -> Nullable<Date>,
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
        publishedat -> Timestamp,
    }
}

diesel::joinable!(post -> location (location));

diesel::allow_tables_to_appear_in_same_query!(
    location,
    post,
);
