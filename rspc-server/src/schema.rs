// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Nullable<Text>,
        username -> Text,
        email -> Nullable<Text>,
        hashed_password -> Nullable<Text>,
    }
}
