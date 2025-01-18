// @generated automatically by Diesel CLI.

diesel::table! {
    user (id) {
        id -> Text,
        username -> Text,
        email -> Nullable<Text>,
        hashed_password -> Text,
    }
}
