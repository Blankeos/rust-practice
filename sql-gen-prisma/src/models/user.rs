#[derive(Debug, Clone, sqlx::FromRow)]
pub struct User {
    pub id: String,
    pub username: String,
    // Issue 1: The database column is named "passwordHash" (camelCase)
    // while the Rust struct field follows snake_case ("password_hash").
    // The `#[sqlx(rename = "...")]` attribute is essential here:
    // it tells sqlx::FromRow (and assists `query_as!`) how to map the *result column*
    // named "passwordHash" from the SQL query to this Rust field "password_hash".
    //
    // Issue 2 (Previously): The SQL query itself must select the correct database column name.
    // The previous error "column “password_hash” does not exist" indicated that the
    // SQL query string was attempting to select a column named "password_hash".
    // The advice was to use `\"passwordHash\"` in the SQL query.
    //
    // Issue 3 (New): The "struct user::User has no field named createdTimestamp" error
    // occurs because `sqlx::query_as!` performs compile-time validation where it expects
    // the *selected column names (or their aliases) in the SQL query* to match the
    // *Rust struct's field names*.
    //
    // When the SQL query used `\"passwordHash\"` directly (as per previous Issue 2 advice),
    // `query_as!` looked for a Rust field literally named `passwordHash`. Since the Rust
    // field is `password_hash`, this caused the mismatch and the reported error.
    //
    // To resolve this for `sqlx::query_as!`, you must select the actual database column name
    // and then *alias* it to match the Rust struct's snake_case field name. The
    // `#[sqlx(rename = "...")]` attribute is still correct and necessary on this field
    // for `FromRow` to perform the runtime mapping if you weren't using `query_as!`,
    // or for `query_as!`'s internal mapping when aliases are used.
    //
    // Therefore, in your `sqlx::query_as!` call, you should change:
    // `\"passwordHash\"` to `\"passwordHash\" AS password_hash`
    #[sqlx(rename = "passwordHash")]
    pub password_hash: String,
    // Issue 1 & 3: The database column is named "createdTimestamp" (camelCase)
    // while the Rust struct field follows snake_case ("created_timestamp").
    // The `#[sqlx(rename = "...")]` attribute maps the result column "createdTimestamp"
    // from the SQL query to this Rust field "created_timestamp".
    //
    // As explained in Issue 3 above, for `sqlx::query_as!` to correctly validate at
    // compile time, the selected column in the SQL query *must be aliased* to the
    // Rust field name.
    //
    // In your `sqlx::query_as!` call, you should change:
    // `\"createdTimestamp\"` to `\"createdTimestamp\" AS created_timestamp`
    #[sqlx(rename = "createdTimestamp")]
    pub created_timestamp: chrono::NaiveDateTime,
    // Issue 1 & 3: The database column is named "updatedTimestamp" (camelCase)
    // while the Rust struct field follows snake_case ("updated_timestamp").
    // The `#[sqlx(rename = "...")]` attribute maps the result column "updatedTimestamp"
    // from the SQL query to this Rust field "updated_timestamp".
    //
    // As explained in Issue 3 above, for `sqlx::query_as!` to correctly validate at
    // compile time, the selected column in the SQL query *must be aliased* to the
    // Rust field name.
    //
    // In your `sqlx::query_as!` call, you should change:
    // `\"updatedTimestamp\"` to `\"updatedTimestamp\" AS updated_timestamp`
    #[sqlx(rename = "updatedTimestamp")]
    pub updated_timestamp: chrono::NaiveDateTime,
}
