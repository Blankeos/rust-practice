## Setup

Prerequires:

- Just - `brew install just`
- Rust

```
cp .env.example .env

just migrate-up

cargo run
```

### Some guideline notes

Using the db, always pass it from the ctx. Upon router injection.

```rust
// Then in the closrue
t(async |ctx, input: () | {
    ctx.db // use it how u want it.
})
```

Throwing errors

```rs
Err(rspc::Error::new(
   rspc::ErrorCode::NotFound, // Code here.
   "Message here" // Message here.
))
```

Returning stuff (any primitive type)

```rs
t(async |ctx, _: ()| {

    Ok(2) // returns 2
})
```

Returning Json. A struct must derive two traits: `specta::Type, serde::Serialize`

```rs
#[derive(specta::Type, serde::Serialize)]
pub struct MyResponse {
    username: String,
    age: u32
}

t(async |ctx, _: ()| {
    Ok(MyResponse { username: String::from("Something"), age: 2 })
})
```

Passing parameters w/ typesafety. A struct must derive two traits: `sepcta::Type, serde::Deserialize`

```rs
#[derive(specta::Type, serde::Deserialize)]
pub struct MyInput {
    username: String,
    age: u32
}

t(async |ctx, input: MyInput| {
    // Use input however you want here.
})

// Important: Also note that passing this input via the url when .query is done via: ?input=%a-serialized-json% (you need to convert a json to a serialized json first)
```
